use oxc_allocator::{FromIn, TakeIn};
use oxc_ast::ast::{
  ArrowFunctionExpression, AssignmentTarget, AssignmentTargetMaybeDefault,
  AssignmentTargetProperty, AssignmentTargetPropertyProperty, BlockStatement, CatchClause,
  Expression, ForInStatement, ForOfStatement, ForStatement, Function, FunctionBody, IdentifierName,
  PropertyKey,
};
use oxc_ast_visit::{
  VisitMut,
  walk_mut::{self, walk_assignment_target, walk_expression},
};
use oxc_semantic::NodeId;
use std::{
  cell::Cell,
  collections::{HashMap, HashSet},
};

use napi::bindgen_prelude::Either3;
use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::{SPAN, Span};

use crate::{
  check::is_referenced_identifier,
  options::{RootJsx, TransformOptions},
  walk::{
    extract_identifiers, mark_known_ids, mark_scope_identifier, remove_known_ids,
    walk_block_declarations, walk_for_statement,
  },
};

type OnIdentifier<'a> =
  Box<dyn FnMut(&mut IdentifierReference<'a>, Option<AstKind<'a>>) -> Option<Expression<'a>> + 'a>;

// Modified from https://github.com/vuejs/core/blob/main/packages/compiler-core/src/babelUtils.ts
// To support browser environments and JSX.
//
// https://github.com/vuejs/core/blob/main/LICENSE
//
// Return value indicates whether the AST walked can be a constant
pub struct WalkIdentifiersMut<'a> {
  known_ids: HashMap<String, u32>,
  on_identifier: OnIdentifier<'a>,
  scope_ids_map: HashMap<Span, HashSet<String>>,
  pub options: &'a TransformOptions<'a>,
  pub roots: Vec<RootJsx<'a>>,
}

impl<'a> WalkIdentifiersMut<'a> {
  pub fn new(on_identifier: OnIdentifier<'a>, options: &'a TransformOptions<'a>) -> Self {
    Self {
      options,
      on_identifier,
      known_ids: HashMap::new(),
      scope_ids_map: HashMap::new(),
      roots: vec![],
    }
  }

  pub fn on_identifier_reference(
    &mut self,
    id: &mut IdentifierReference<'a>,
  ) -> Option<Expression<'a>> {
    if id.span.eq(&SPAN) && !id.name.eq("_cache") {
      return None;
    }
    let semantic = &self.options.semantic.borrow();
    let parent = semantic.nodes().parent_kind(id.node_id.get());
    let is_local = self.known_ids.contains_key(id.name.as_str());
    let is_refed = is_referenced_identifier(id, Some(parent));
    if is_refed && !is_local {
      self.on_identifier.as_mut()(id, Some(parent))
    } else {
      None
    }
  }

  pub fn visit(&mut self, it: &mut Expression<'a>) {
    self.visit_expression(it);
    if let Some(on_exit_program) = self.options.on_exit_program.borrow().as_ref() {
      on_exit_program(std::mem::take(&mut self.roots));
    }
  }
}

impl<'a> VisitMut<'a> for WalkIdentifiersMut<'a> {
  fn visit_expression(&mut self, node: &mut Expression<'a>) {
    if let Expression::Identifier(id) = node {
      if let Some(replacer) = self.on_identifier_reference(id) {
        *node = replacer;
      }
      return;
    } else if let Some(on_enter_expression) = self.options.on_enter_expression.borrow().as_ref()
      && let Some((node_ref, vdom)) = on_enter_expression(node)
    {
      self.roots.push(RootJsx {
        node_ref,
        node: unsafe { &mut *node_ref }.take_in(&self.options.allocator),
        vdom,
      });
    }
    walk_expression(self, node);
    if let Expression::CallExpression(node) = node
      && let Some(on_leave_expression) = self.options.on_leave_expression.borrow().as_ref()
    {
      on_leave_expression(node)
    }
  }

  fn visit_assignment_target(&mut self, node: &mut AssignmentTarget<'a>) {
    if let AssignmentTarget::AssignmentTargetIdentifier(id) = node {
      self.on_identifier_reference(id);
    } else if let AssignmentTarget::StaticMemberExpression(node) = node
      && let Expression::Identifier(id) = {
        let mut object = &mut node.object;
        loop {
          if let Expression::StaticMemberExpression(member) = object {
            object = &mut member.object;
            continue;
          }
          break;
        }
        object
      }
    {
      self.on_identifier_reference(id);
    } else if let AssignmentTarget::ComputedMemberExpression(node) = node
      && let Expression::Identifier(id) = &mut node.object
    {
      self.on_identifier_reference(id);
    }
    walk_assignment_target(self, node);
  }

  fn visit_assignment_target_property(&mut self, node: &mut AssignmentTargetProperty<'a>) {
    match node {
      // ;({ baz } = bar)
      AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(id) => {
        if let Some(replacer) = self.on_identifier_reference(&mut id.binding) {
          *node =
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(oxc_allocator::Box::new_in(
              AssignmentTargetPropertyProperty {
                node_id: Cell::new(NodeId::DUMMY),
                span: SPAN,
                name: PropertyKey::StaticIdentifier(oxc_allocator::Box::from_in(
                  IdentifierName {
                    node_id: Cell::new(NodeId::DUMMY),
                    span: id.binding.span,
                    name: id.binding.name,
                  },
                  &self.options.allocator,
                )),
                binding: match replacer {
                  Expression::Identifier(replacer) => {
                    AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(replacer)
                  }
                  Expression::StaticMemberExpression(replacer) => {
                    AssignmentTargetMaybeDefault::StaticMemberExpression(replacer)
                  }
                  _ => unimplemented!(),
                },
                computed: false,
              },
              &self.options.allocator,
            ));
        };
      }
      // ;({ baz: baz } = bar)
      AssignmentTargetProperty::AssignmentTargetPropertyProperty(property) => {
        match &mut property.binding {
          AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(id) => {
            if let Some(replacer) = self.on_identifier_reference(id) {
              property.binding = match replacer {
                Expression::Identifier(replacer) => {
                  AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(replacer)
                }
                Expression::StaticMemberExpression(replacer) => {
                  AssignmentTargetMaybeDefault::StaticMemberExpression(replacer)
                }
                _ => unimplemented!(),
              };
            }
          }
          _ => unreachable!(),
        };
      }
    }
    walk_mut::walk_assignment_target_property(self, node);
  }

  fn visit_function(&mut self, node: &mut Function<'a>, flags: oxc_semantic::ScopeFlags) {
    if let Some(scope_ids) = self.scope_ids_map.get(&node.span) {
      for id in scope_ids {
        mark_known_ids(id.clone(), &mut self.known_ids);
      }
    } else {
      // walk function expressions and add its arguments to known identifiers
      // so that we don't prefix them
      for p in &node.params.items {
        for id in extract_identifiers(&p.pattern, Vec::new()) {
          mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map)
        }
      }
    }
    walk_mut::walk_function(self, node, flags);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }

  fn visit_arrow_function_expression(&mut self, node: &mut ArrowFunctionExpression<'a>) {
    if let Some(scope_ids) = self.scope_ids_map.get(&node.span) {
      for id in scope_ids {
        mark_known_ids(id.clone(), &mut self.known_ids);
      }
    } else {
      // walk function expressions and add its arguments to known identifiers
      // so that we don't prefix them
      for p in &node.params.items {
        for id in extract_identifiers(&p.pattern, Vec::new()) {
          mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map)
        }
      }
    }
    walk_mut::walk_arrow_function_expression(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }

  fn visit_function_body(&mut self, node: &mut FunctionBody<'a>) {
    if let Some(scope_ids) = self.scope_ids_map.get(&node.span) {
      for id in scope_ids {
        mark_known_ids(id.clone(), &mut self.known_ids);
      }
    } else {
      walk_block_declarations(&node.statements, |id| {
        mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
      });
    }
    walk_mut::walk_function_body(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }

  fn visit_block_statement(&mut self, node: &mut BlockStatement<'a>) {
    if let Some(scope_ids) = self.scope_ids_map.get(&node.span) {
      for id in scope_ids {
        mark_known_ids(id.clone(), &mut self.known_ids);
      }
    } else {
      // #3445 record block-level local variables
      walk_block_declarations(&node.body, |id| {
        mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
      });
    }
    walk_mut::walk_block_statement(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }

  fn visit_catch_clause(&mut self, node: &mut CatchClause<'a>) {
    if let Some(param) = &node.param {
      for id in extract_identifiers(&param.pattern, vec![]) {
        mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
      }
    }
    walk_mut::walk_catch_clause(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }

  fn visit_for_statement(&mut self, node: &mut ForStatement<'a>) {
    walk_for_statement(Either3::A(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    });
    walk_mut::walk_for_statement(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }
  fn visit_for_in_statement(&mut self, node: &mut ForInStatement<'a>) {
    walk_for_statement(Either3::B(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    });
    walk_mut::walk_for_in_statement(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }
  fn visit_for_of_statement(&mut self, node: &mut ForOfStatement<'a>) {
    walk_for_statement(Either3::C(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    });
    walk_mut::walk_for_of_statement(self, node);
    remove_known_ids(node.span, &mut self.known_ids, &mut self.scope_ids_map);
  }
}
