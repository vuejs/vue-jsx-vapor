use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::ast::{
  ArrowFunctionExpression, AssignmentTargetMaybeDefault, AssignmentTargetProperty,
  BindingIdentifier, BindingPattern, BindingPatternKind, BlockStatement, CatchClause, Expression,
  ForInStatement, ForOfStatement, ForStatement, ForStatementInit, ForStatementLeft, Function,
  FunctionBody, Program, Statement, VariableDeclarationKind,
};
use oxc_semantic::SemanticBuilder;
use oxc_traverse::{Ancestor, Traverse, TraverseAncestry, TraverseCtx, traverse_mut};
use std::collections::{HashMap, HashSet};

use napi::bindgen_prelude::Either3;
use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  generate::CodegenContext, transform::TransformContext, utils::check::is_referenced_identifier,
};

type OnIdentifier<'a> = Box<
  dyn FnMut(
      &mut IdentifierReference<'a>,
      &Ancestor,
      &TraverseAncestry<'a>,
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
pub struct WalkIdentifiers<'a, 'ctx> {
  known_ids: HashMap<String, u32>,
  include_all: bool,
  context: &'a CodegenContext<'ctx>,
  on_identifier: OnIdentifier<'a>,
  scope_ids_map: HashMap<Span, HashSet<String>>,
  roots: Vec<*mut Expression<'a>>,
}

impl<'a, 'ctx> WalkIdentifiers<'a, 'ctx> {
  pub fn new(
    context: &'a CodegenContext<'ctx>,
    on_identifier: OnIdentifier<'a>,
    include_all: bool,
  ) -> Self {
    Self {
      context,
      on_identifier,
      include_all,
      known_ids: HashMap::new(),
      scope_ids_map: HashMap::new(),
      roots: vec![],
    }
  }

  pub fn traverse(&mut self, expression: Expression<'a>) -> Expression<'a> {
    let ast = &self.context.ast;
    let source_text = expression.span().source_text(self.context.ir.source);
    let program = &mut ast.program(
      SPAN,
      self.context.options.source_type,
      source_text,
      ast.vec(),
      None,
      ast.vec(),
      ast.vec_from_array([ast.statement_expression(SPAN, expression)]),
    );
    traverse_mut(
      self,
      ast.allocator,
      program,
      SemanticBuilder::new()
        .build(program)
        .semantic
        .into_scoping(),
      (),
    );
    let Statement::ExpressionStatement(stmt) = &mut program.body[0] else {
      unreachable!();
    };
    if self.roots.is_empty() {
      stmt.expression.take_in(ast.allocator)
    } else {
      stmt.expression.clone_in(ast.allocator)
    }
  }

  fn exit_node(&mut self, span: &Span, ctx: &mut oxc_traverse::TraverseCtx<'a, ()>) {
    let known_ids = &mut self.known_ids;
    if !matches!(ctx.parent(), Ancestor::None)
      && let Some(scope_ids) = self.scope_ids_map.get(span)
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

  fn on_identifier_reference(
    &mut self,
    id: &mut IdentifierReference<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
  ) -> Option<Expression<'a>> {
    if id.span.eq(&SPAN) {
      return None;
    }
    let is_local = self.known_ids.contains_key(id.name.as_str());
    let is_refed = is_referenced_identifier(id, &ctx.ancestry);
    if self.include_all || (is_refed && !is_local) {
      self.on_identifier.as_mut()(id, &ctx.parent(), &ctx.ancestry, is_refed, is_local)
    } else {
      None
    }
  }
}

impl<'a> Traverse<'a, ()> for WalkIdentifiers<'a, '_> {
  fn enter_assignment_target_property(
    &mut self,
    node: &mut AssignmentTargetProperty<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
  ) {
    let ast = self.context.ast;
    match node {
      // ;({ baz } = bar)
      AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(id) => {
        if let Some(replacer) = self.on_identifier_reference(&mut id.binding, ctx) {
          *node = ast.assignment_target_property_assignment_target_property_property(
            SPAN,
            ast.property_key_static_identifier(id.binding.span, id.binding.name),
            match replacer {
              Expression::Identifier(replacer) => {
                AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(replacer)
              }
              Expression::StaticMemberExpression(replacer) => {
                AssignmentTargetMaybeDefault::StaticMemberExpression(replacer)
              }
              _ => unimplemented!(),
            },
            false,
          );
        };
      }
      // ;({ baz: baz } = bar)
      AssignmentTargetProperty::AssignmentTargetPropertyProperty(property) => {
        match &mut property.binding {
          AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(id) => {
            if let Some(replacer) = self.on_identifier_reference(id, ctx) {
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
  }
  fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    if let Expression::Identifier(id) = node {
      if let Some(replacer) = self.on_identifier_reference(id, ctx) {
        *node = replacer;
      }
    } else if matches!(node, Expression::JSXElement(_) | Expression::JSXFragment(_)) {
      if self.context.options.interop {
        for node in ctx.ancestors() {
          if let Ancestor::CallExpressionArguments(node) = node {
            let name = node.callee().span().source_text(self.context.ir.source);
            if name == "defineVaporComponent" {
              break;
            } else if name == "defineComponent" {
              return;
            }
          }
        }
      }
      self.roots.push(node as *mut Expression);
    }
  }
  fn exit_program(&mut self, _: &mut Program<'a>, _: &mut TraverseCtx<'a, ()>) {
    let allocator = self.context.ast.allocator;
    let transform_context = self.context.transform_cotext;
    unsafe {
      for root in &mut self.roots {
        let root = &mut **root;
        let context: *mut TransformContext =
          &mut TransformContext::new(allocator, self.context.options);
        *(&mut *context).in_v_once.borrow_mut() = *transform_context.in_v_once.borrow();
        *(&mut *context).in_v_for.borrow_mut() = *transform_context.in_v_for.borrow();
        let source = &self.context.ir.source[..root.span().end as usize];
        *root = (&*context).transform(root.take_in(allocator), source);
      }
    }
  }
  fn exit_identifier_reference(
    &mut self,
    node: &mut IdentifierReference<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
  ) {
    self.exit_node(&node.span, ctx)
  }

  fn enter_function(&mut self, node: &mut Function<'a>, _: &mut TraverseCtx<'a, ()>) {
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
  }
  fn exit_function(&mut self, node: &mut Function<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    self.exit_node(&node.span, ctx)
  }

  fn enter_arrow_function_expression(
    &mut self,
    node: &mut ArrowFunctionExpression<'a>,
    _: &mut TraverseCtx<'a, ()>,
  ) {
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
  }
  fn exit_arrow_function_expression(
    &mut self,
    node: &mut ArrowFunctionExpression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
  ) {
    self.exit_node(&node.span, ctx)
  }

  fn enter_function_body(&mut self, node: &mut FunctionBody<'a>, _: &mut TraverseCtx<'a, ()>) {
    if let Some(scope_ids) = self.scope_ids_map.get(&node.span) {
      for id in scope_ids {
        mark_known_ids(id.clone(), &mut self.known_ids);
      }
    } else {
      walk_block_declarations(&node.statements, |id| {
        mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
      });
    }
  }
  fn exit_function_body(&mut self, node: &mut FunctionBody<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    self.exit_node(&node.span, ctx);
  }

  fn enter_block_statement(&mut self, node: &mut BlockStatement<'a>, _: &mut TraverseCtx<'a, ()>) {
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
  }
  fn exit_block_statement(&mut self, node: &mut BlockStatement<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    self.exit_node(&node.span, ctx);
  }

  fn enter_catch_clause(&mut self, node: &mut CatchClause<'a>, _: &mut TraverseCtx<'a, ()>) {
    if let Some(param) = &node.param {
      for id in extract_identifiers(&param.pattern, vec![]) {
        mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
      }
    }
  }
  fn exit_catch_clause(&mut self, node: &mut CatchClause<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    self.exit_node(&node.span, ctx);
  }

  fn enter_for_statement(&mut self, node: &mut ForStatement<'a>, _: &mut TraverseCtx<'a, ()>) {
    walk_for_statement(Either3::A(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    })
  }
  fn exit_for_statement(&mut self, node: &mut ForStatement<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    self.exit_node(&node.span, ctx);
  }
  fn enter_for_in_statement(&mut self, node: &mut ForInStatement<'a>, _: &mut TraverseCtx<'a, ()>) {
    walk_for_statement(Either3::B(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    })
  }
  fn exit_for_in_statement(
    &mut self,
    node: &mut ForInStatement<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
  ) {
    self.exit_node(&node.span, ctx);
  }

  fn enter_for_of_statement(&mut self, node: &mut ForOfStatement<'a>, _: &mut TraverseCtx<'a, ()>) {
    walk_for_statement(Either3::C(node), true, &mut |id| {
      mark_scope_identifier(node.span, id, &mut self.known_ids, &mut self.scope_ids_map);
    })
  }
  fn exit_for_of_statement(
    &mut self,
    node: &mut ForOfStatement<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
  ) {
    self.exit_node(&node.span, ctx);
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
  match &node.kind {
    BindingPatternKind::BindingIdentifier(node) => identifiers.push(node.as_ref()),
    BindingPatternKind::ObjectPattern(node) => {
      if let Some(rest) = &node.rest {
        identifiers = extract_identifiers(&rest.argument, identifiers);
      } else {
        for prop in &node.properties {
          identifiers = extract_identifiers(&prop.value, identifiers)
        }
      }
    }
    BindingPatternKind::ArrayPattern(node) => {
      for element in (&node.elements).into_iter().flatten() {
        identifiers = extract_identifiers(element, identifiers);
      }
    }
    BindingPatternKind::AssignmentPattern(node) => {
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
