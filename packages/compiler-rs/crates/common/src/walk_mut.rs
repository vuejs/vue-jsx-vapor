use oxc_allocator::{FromIn, TakeIn};
use oxc_ast::ast::{
  AssignmentTargetMaybeDefault, AssignmentTargetProperty, AssignmentTargetPropertyProperty,
  Expression, IdentifierName, PropertyKey, SimpleAssignmentTarget,
};
use oxc_ast_visit::{
  VisitMut,
  walk_mut::{self, walk_expression, walk_simple_assignment_target},
};
use oxc_semantic::{NodeId, ScopeId};
use std::cell::Cell;

use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::SPAN;

use crate::{
  check::is_referenced_identifier,
  options::{RootJsx, TransformOptions},
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
  on_identifier: OnIdentifier<'a>,
  has_this: bool,
  pub options: &'a TransformOptions<'a>,
  pub roots: Vec<RootJsx<'a>>,
  pub root_scope_id: Option<ScopeId>,
}

impl<'a> WalkIdentifiersMut<'a> {
  pub fn new(on_identifier: OnIdentifier<'a>, options: &'a TransformOptions<'a>) -> Self {
    Self {
      options,
      on_identifier,
      root_scope_id: None,
      roots: vec![],
      has_this: false,
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
    let mut is_local = false;
    let mut scope_parent_id = Some(semantic.nodes().get_node(id.node_id()).scope_id());
    while let Some(scope_id) = if let Some(root_scope_id) = self.root_scope_id
      && let Some(scope_parent_id) = scope_parent_id
      && root_scope_id != scope_parent_id
    {
      Some(scope_parent_id)
    } else {
      None
    } {
      is_local = semantic.scoping().scope_has_binding(scope_id, id.name);
      if is_local {
        break;
      }
      scope_parent_id = semantic.scoping().scope_parent_id(scope_id);
    }
    let parent = semantic.nodes().parent_kind(id.node_id.get());
    let is_refed = is_referenced_identifier(id, Some(parent));
    if is_refed && !is_local {
      self.on_identifier.as_mut()(id, Some(parent))
    } else {
      None
    }
  }

  pub fn visit(&mut self, it: &mut Expression<'a>) -> bool {
    self.root_scope_id = Some(
      self
        .options
        .semantic
        .borrow()
        .nodes()
        .get_node(it.node_id())
        .scope_id(),
    );
    self.visit_expression(it);
    if let Some(on_exit_program) = self.options.on_exit_program.borrow().as_ref() {
      on_exit_program(std::mem::take(&mut self.roots));
    }
    self.has_this
  }
}

impl<'a> VisitMut<'a> for WalkIdentifiersMut<'a> {
  fn visit_expression(&mut self, node: &mut Expression<'a>) {
    if let Expression::Identifier(id) = node {
      if let Some(replacer) = self.on_identifier_reference(id) {
        *node = replacer;
      }
      return;
    } else if let Expression::ThisExpression(_) = node {
      self.has_this = true;
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

  fn visit_simple_assignment_target(&mut self, node: &mut SimpleAssignmentTarget<'a>) {
    if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = node {
      if let Some(exp) = self.on_identifier_reference(id) {
        match exp {
          Expression::Identifier(exp) => {
            *node = SimpleAssignmentTarget::AssignmentTargetIdentifier(exp);
          }
          Expression::StaticMemberExpression(exp) => {
            *node = SimpleAssignmentTarget::StaticMemberExpression(exp);
          }
          Expression::ComputedMemberExpression(exp) => {
            *node = SimpleAssignmentTarget::ComputedMemberExpression(exp);
          }
          Expression::PrivateFieldExpression(exp) => {
            *node = SimpleAssignmentTarget::PrivateFieldExpression(exp);
          }
          Expression::TSAsExpression(exp) => {
            *node = SimpleAssignmentTarget::TSAsExpression(exp);
          }
          Expression::TSNonNullExpression(exp) => {
            *node = SimpleAssignmentTarget::TSNonNullExpression(exp);
          }
          Expression::TSSatisfiesExpression(exp) => {
            *node = SimpleAssignmentTarget::TSSatisfiesExpression(exp);
          }
          Expression::TSTypeAssertion(exp) => {
            *node = SimpleAssignmentTarget::TSTypeAssertion(exp);
          }
          _ => {}
        };
      };
    }
    walk_simple_assignment_target(self, node);
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
}

/// Get the node_id for an AST node.
pub trait GetNodeId {
  /// Get the [`node_id`] for an AST node.
  fn node_id(&self) -> NodeId;
}

impl GetNodeId for Expression<'_> {
  fn node_id(&self) -> NodeId {
    match self {
      Self::BooleanLiteral(it) => it.node_id(),
      Self::NullLiteral(it) => it.node_id(),
      Self::NumericLiteral(it) => it.node_id(),
      Self::BigIntLiteral(it) => it.node_id(),
      Self::RegExpLiteral(it) => it.node_id(),
      Self::StringLiteral(it) => it.node_id(),
      Self::TemplateLiteral(it) => it.node_id(),
      Self::Identifier(it) => it.node_id(),
      Self::MetaProperty(it) => it.node_id(),
      Self::Super(it) => it.node_id(),
      Self::ArrayExpression(it) => it.node_id(),
      Self::ArrowFunctionExpression(it) => it.node_id(),
      Self::AssignmentExpression(it) => it.node_id(),
      Self::AwaitExpression(it) => it.node_id(),
      Self::BinaryExpression(it) => it.node_id(),
      Self::CallExpression(it) => it.node_id(),
      Self::ChainExpression(it) => it.node_id(),
      Self::ClassExpression(it) => it.node_id(),
      Self::ConditionalExpression(it) => it.node_id(),
      Self::FunctionExpression(it) => it.node_id(),
      Self::ImportExpression(it) => it.node_id(),
      Self::LogicalExpression(it) => it.node_id(),
      Self::NewExpression(it) => it.node_id(),
      Self::ObjectExpression(it) => it.node_id(),
      Self::ParenthesizedExpression(it) => it.node_id(),
      Self::SequenceExpression(it) => it.node_id(),
      Self::TaggedTemplateExpression(it) => it.node_id(),
      Self::ThisExpression(it) => it.node_id(),
      Self::UnaryExpression(it) => it.node_id(),
      Self::UpdateExpression(it) => it.node_id(),
      Self::YieldExpression(it) => it.node_id(),
      Self::PrivateInExpression(it) => it.node_id(),
      Self::JSXElement(it) => it.node_id(),
      Self::JSXFragment(it) => it.node_id(),
      Self::TSAsExpression(it) => it.node_id(),
      Self::TSSatisfiesExpression(it) => it.node_id(),
      Self::TSTypeAssertion(it) => it.node_id(),
      Self::TSNonNullExpression(it) => it.node_id(),
      Self::TSInstantiationExpression(it) => it.node_id(),
      Self::V8IntrinsicExpression(it) => it.node_id(),
      Self::ComputedMemberExpression(it) => it.node_id(),
      Self::StaticMemberExpression(it) => it.node_id(),
      Self::PrivateFieldExpression(it) => it.node_id(),
    }
  }
}
