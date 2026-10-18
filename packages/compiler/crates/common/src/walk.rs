use oxc_ast::ast::Expression;
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;

use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::SPAN;

use crate::{
  check::is_referenced_identifier,
  options::{RootJsx, TransformOptions},
};

type OnIdentifier<'a> =
  Box<dyn FnMut(&IdentifierReference<'a>, Option<AstKind<'a>>, &Vec<AstKind<'a>>) + 'a>;

// Modified from https://github.com/vuejs/core/blob/main/packages/compiler-core/src/babelUtils.ts
// To support browser environments and JSX.
//
// https://github.com/vuejs/core/blob/main/LICENSE
//
// Return value indicates whether the AST walked can be a constant
pub struct WalkIdentifiers<'a, 'opt> {
  on_identifier: OnIdentifier<'a>,
  pub options: &'a TransformOptions<'opt>,
  pub parents: Vec<AstKind<'a>>,
  pub roots: Vec<RootJsx<'a>>,
  pub root_scope_id: Option<ScopeId>,
}

impl<'a, 'opt> WalkIdentifiers<'a, 'opt> {
  pub fn new(on_identifier: OnIdentifier<'a>, options: &'a TransformOptions<'opt>) -> Self {
    Self {
      options,
      on_identifier,
      parents: vec![],
      roots: vec![],
      root_scope_id: None,
    }
  }

  pub fn on_identifier_reference(&mut self, id: &IdentifierReference<'a>) {
    if id.span.eq(&SPAN) && !id.name.eq("_cache") {
      return;
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
    let parent = self.parents.last().copied();
    let is_refed = is_referenced_identifier(id, parent);
    if is_refed && !is_local {
      self.on_identifier.as_mut()(id, parent, &self.parents);
    }
  }

  pub fn visit(&mut self, it: &Expression<'a>) {
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
  }
}

impl<'a, 'opt> Visit<'a> for WalkIdentifiers<'a, 'opt> {
  fn enter_node(&mut self, kind: AstKind<'a>) {
    self.parents.push(kind);
  }
  fn leave_node(&mut self, _: AstKind<'a>) {
    self.parents.pop();
  }

  fn visit_identifier_reference(&mut self, id: &IdentifierReference<'a>) {
    self.on_identifier_reference(id);
  }
}
