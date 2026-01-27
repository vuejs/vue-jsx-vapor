use oxc_allocator::{FromIn, TakeIn};
use oxc_ast::ast::{
  ArrowFunctionExpression, AssignmentTarget, AssignmentTargetMaybeDefault,
  AssignmentTargetProperty, AssignmentTargetPropertyProperty, BindingIdentifier, BindingPattern,
  BlockStatement, CatchClause, Expression, ForInStatement, ForOfStatement, ForStatement,
  ForStatementInit, ForStatementLeft, Function, FunctionBody, IdentifierName, PropertyKey,
  Statement, VariableDeclarationKind,
};
use oxc_ast_visit::{
  VisitMut,
  walk_mut::{self, walk_assignment_target, walk_expression},
};
use std::collections::{HashMap, HashSet};

use napi::bindgen_prelude::Either3;
use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::{SPAN, Span};

use crate::{
  check::is_referenced_identifier,
  options::{RootJsx, TransformOptions},
};

type OnIdentifier<'a> = Box<
  dyn FnMut(
      &mut IdentifierReference<'a>,
      Option<&AstKind<'a>>,
      &Vec<AstKind<'a>>,
      bool,
      bool,
    ) -> Option<Expression<'a>>
    + 'a,
>;

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
  pub parents: Vec<AstKind<'a>>,
  pub roots: Vec<RootJsx<'a>>,
}

impl<'a> WalkIdentifiersMut<'a> {
  pub fn new(on_identifier: OnIdentifier<'a>, options: &'a TransformOptions<'a>) -> Self {
    Self {
      options,
      on_identifier,
      known_ids: HashMap::new(),
      scope_ids_map: HashMap::new(),
      parents: vec![],
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
    let is_local = self.known_ids.contains_key(id.name.as_str());
    let is_refed = is_referenced_identifier(id, &self.parents);
    if is_refed && !is_local {
      self.on_identifier.as_mut()(id, self.parents.last(), &self.parents, is_refed, is_local)
    } else {
      None
    }
  }

  pub fn on_visit_expression(&self, node: &mut Expression<'a>) -> Option<bool> {
    if !matches!(node, Expression::JSXElement(_) | Expression::JSXFragment(_)) {
      return None;
    }
    if self.options.interop {
      let mut has_define_vapor_component = false;
      for parent in self.parents.iter().rev() {
        if let AstKind::CallExpression(parent) = parent
          && let Expression::Identifier(name) = &parent.callee
        {
          if ["defineVaporComponent", "defineVaporCustomElement"].contains(&name.name.as_ref()) {
            has_define_vapor_component = true;
            break;
          } else if ["defineComponent", "defineCustomElement"].contains(&name.name.as_ref()) {
            return Some(true);
          }
        }
      }
      if !has_define_vapor_component {
        return Some(true);
      }
    }
    Some(false)
  }

  pub fn visit(&mut self, it: &mut Expression<'a>) {
    self.visit_expression(it);
  }
}

impl<'a> VisitMut<'a> for WalkIdentifiersMut<'a> {
  fn enter_node(&mut self, kind: AstKind<'a>) {
    self.parents.push(kind);
  }

  fn visit_expression(&mut self, node: &mut Expression<'a>) {
    if let Expression::Identifier(id) = node {
      if let Some(replacer) = self.on_identifier_reference(id) {
        *node = replacer;
      }
    } else if let Some(vdom) = self.on_visit_expression(node) {
      let node_ref = node as *mut _;
      self.roots.push(RootJsx {
        node_ref,
        node: unsafe { &mut *node_ref }.take_in(&self.options.allocator),
        vdom,
      });
    }
    walk_expression(self, node);
  }

  fn visit_assignment_target(&mut self, node: &mut AssignmentTarget<'a>) {
    if let AssignmentTarget::AssignmentTargetIdentifier(id) = node {
      self.on_identifier_reference(id);
    } else if let AssignmentTarget::StaticMemberExpression(node) = node
      && let Expression::Identifier(id) = {
        let mut object = &mut node.object;
        loop {
          match object {
            Expression::StaticMemberExpression(member) => {
              object = &mut member.object;
              continue;
            }
            _ => {}
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
                span: SPAN,
                name: PropertyKey::StaticIdentifier(oxc_allocator::Box::from_in(
                  IdentifierName {
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
  }

  fn visit_catch_clause(&mut self, node: &mut CatchClause<'a>) {
    if let Some(param) = &node.param {
      for id in extract_identifiers(&param.pattern, vec![]) {
        mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
      }
    }
    walk_mut::walk_catch_clause(self, node);
  }

  fn visit_for_statement(&mut self, node: &mut ForStatement<'a>) {
    walk_for_statement(Either3::A(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    });
    walk_mut::walk_for_statement(self, node);
  }
  fn visit_for_in_statement(&mut self, node: &mut ForInStatement<'a>) {
    walk_for_statement(Either3::B(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    });
    walk_mut::walk_for_in_statement(self, node);
  }
  fn visit_for_of_statement(&mut self, node: &mut ForOfStatement<'a>) {
    walk_for_statement(Either3::C(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    });
    walk_mut::walk_for_of_statement(self, node);
  }

  fn leave_node(&mut self, kind: AstKind<'a>) {
    self.parents.pop();
    if let Some(span) = match kind {
      AstKind::IdentifierReference(node) => Some(node.span),
      AstKind::Function(node) => Some(node.span),
      AstKind::FunctionBody(node) => Some(node.span),
      AstKind::ArrowFunctionExpression(node) => Some(node.span),
      AstKind::BlockStatement(node) => Some(node.span),
      AstKind::CatchClause(node) => Some(node.span),
      AstKind::ForStatement(node) => Some(node.span),
      AstKind::ForOfStatement(node) => Some(node.span),
      AstKind::ForInStatement(node) => Some(node.span),
      _ => None,
    } {
      let known_ids = &mut self.known_ids;
      if !self.parents.is_empty()
        && let Some(scope_ids) = self.scope_ids_map.get(&span)
      {
        for id in scope_ids {
          if let Some(size) = known_ids.get(id) {
            known_ids.insert(id.clone(), size - 1);
            if known_ids[id] == 0 {
              known_ids.remove(id);
            }
          }
        }
      }
    }

    if let Some(on_exit_program) = self.options.on_exit_program.borrow().as_ref()
      && self.parents.is_empty()
    {
      on_exit_program(std::mem::take(&mut self.roots));
    }
  }
}

pub fn mark_known_ids(name: String, known_ids: &mut HashMap<String, u32>) {
  if let Some(ids) = known_ids.get(&name) {
    known_ids.insert(name, ids + 1);
  } else {
    known_ids.insert(name, 1);
  }
}

pub fn mark_scope_identifier(
  node_span: Span,
  child: &BindingIdentifier,
  known_ids: &mut HashMap<String, u32>,
  scope_ids_map: &mut HashMap<Span, HashSet<String>>,
) {
  let name = child.name.to_string();
  if let Some(scope_ids) = scope_ids_map.get_mut(&node_span) {
    if scope_ids.contains(&name) {
      return;
    } else {
      scope_ids.insert(name.clone());
    }
  } else {
    scope_ids_map.insert(node_span, HashSet::from([name.clone()]));
  }
  mark_known_ids(name, known_ids);
}

pub fn walk_function_params<'a>(
  node: &'a AstKind,
  mut on_ident: impl FnMut(&'a BindingIdentifier) + 'a,
) {
  let params = match node {
    AstKind::Function(node) => &node.params.items,
    AstKind::ArrowFunctionExpression(node) => &node.params.items,
    _ => panic!(""),
  };
  for p in params {
    for id in extract_identifiers(&p.pattern, Vec::new()) {
      on_ident(id)
    }
  }
}

pub fn extract_identifiers<'a>(
  node: &'a BindingPattern<'a>,
  mut identifiers: Vec<&'a BindingIdentifier<'a>>,
) -> Vec<&'a BindingIdentifier<'a>> {
  match node {
    BindingPattern::BindingIdentifier(node) => identifiers.push(node.as_ref()),
    BindingPattern::ObjectPattern(node) => {
      if let Some(rest) = &node.rest {
        identifiers = extract_identifiers(&rest.argument, identifiers);
      } else {
        for prop in &node.properties {
          identifiers = extract_identifiers(&prop.value, identifiers)
        }
      }
    }
    BindingPattern::ArrayPattern(node) => {
      for element in (&node.elements).into_iter().flatten() {
        identifiers = extract_identifiers(element, identifiers);
      }
    }
    BindingPattern::AssignmentPattern(node) => {
      identifiers = extract_identifiers(&node.left, identifiers);
    }
  }
  identifiers
}

pub fn walk_block_declarations<'a>(
  body: &'a oxc_allocator::Vec<Statement>,
  mut on_ident: impl FnMut(&'a BindingIdentifier) + 'a,
) {
  for stmt in body {
    if let Statement::VariableDeclaration(stmt) = stmt {
      if stmt.declare {
        continue;
      }
      for decl in &stmt.declarations {
        for id in extract_identifiers(&decl.id, Vec::new()) {
          on_ident(id)
        }
      }
    } else if let Statement::FunctionDeclaration(stmt) = stmt {
      if stmt.declare {
        continue;
      }
      if let Some(id) = &stmt.id {
        on_ident(id);
      }
    } else if let Statement::ClassDeclaration(stmt) = stmt {
      if stmt.declare {
        continue;
      }
      if let Some(id) = &stmt.id {
        on_ident(id);
      }
    } else if let Statement::ForStatement(stmt) = stmt {
      walk_for_statement(Either3::A(stmt), true, &mut on_ident);
    } else if let Statement::ForInStatement(stmt) = stmt {
      walk_for_statement(Either3::B(stmt), true, &mut on_ident);
    } else if let Statement::ForOfStatement(stmt) = stmt {
      walk_for_statement(Either3::C(stmt), true, &mut on_ident);
    }
  }
}

pub fn walk_for_statement<'a>(
  stmt: Either3<&'a ForStatement, &'a ForInStatement, &'a ForOfStatement>,
  is_var: bool,
  on_ident: &mut impl FnMut(&'a BindingIdentifier),
) {
  let variable = if let Either3::A(stmt) = stmt
    && let Some(ForStatementInit::VariableDeclaration(stmt)) = &stmt.init
  {
    Some(stmt.as_ref())
  } else if let Either3::B(stmt) = stmt
    && let ForStatementLeft::VariableDeclaration(stmt) = &stmt.left
  {
    Some(stmt.as_ref())
  } else if let Either3::C(stmt) = stmt
    && let ForStatementLeft::VariableDeclaration(stmt) = &stmt.left
  {
    Some(stmt.as_ref())
  } else {
    None
  };
  if let Some(variable) = variable
    && if let VariableDeclarationKind::Var = variable.kind {
      is_var
    } else {
      !is_var
    }
  {
    for decl in &variable.declarations {
      for id in extract_identifiers(&decl.id, Vec::new()) {
        on_ident(id)
      }
    }
  }
}
