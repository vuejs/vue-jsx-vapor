pub mod block;
pub mod component;
pub mod directive;
pub mod dom;
pub mod event;
pub mod expression;
pub mod html;
pub mod operation;
pub mod prop;
pub mod slot;
pub mod template;
pub mod template_ref;
pub mod text;
pub mod v_for;
pub mod v_if;
pub mod v_model;
pub mod v_show;

use std::{cell::RefCell, collections::HashMap, mem};

use common::{options::TransformOptions, text::to_valid_asset_id};
use oxc_ast::{
  AstBuilder, NONE,
  ast::{
    Argument, BindingPatternKind, Expression, FormalParameterKind, JSXChild, Statement,
    VariableDeclarationKind,
  },
};
use oxc_span::{SPAN, Span};

use crate::{
  ast::NodeTypes,
  generate::block::gen_block_content,
  ir::index::{BlockIRNode, RootIRNode, RootNode},
  transform::TransformContext,
};

pub struct CodegenContext<'a> {
  pub options: &'a TransformOptions<'a>,
  pub identifiers: RefCell<HashMap<String, Vec<Expression<'a>>>>,
  pub ir: RootIRNode<'a>,
  pub block: RefCell<BlockIRNode<'a>>,
  pub scope_level: RefCell<i32>,
  pub ast: AstBuilder<'a>,
  pub root_node: JSXChild<'a>,
  pub codegen_map: HashMap<Span, NodeTypes<'a>>,
}

impl<'a> CodegenContext<'a> {
  pub fn new(context: &'a TransformContext<'a>) -> CodegenContext<'a> {
    let ir = context.ir.take();
    let block = context.block.take();
    let ast = AstBuilder::new(context.allocator);
    *context.options.in_v_for.borrow_mut() = *context.in_v_for.borrow();
    *context.options.in_v_once.borrow_mut() = *context.in_v_once.borrow();
    CodegenContext {
      options: context.options,
      identifiers: RefCell::new(HashMap::new()),
      block: RefCell::new(block),
      scope_level: RefCell::new(0),
      root_node: context.root_node.replace(RootNode::new(context.allocator)),
      codegen_map: context.codegen_map.take(),
      ir,
      ast,
    }
  }

  pub fn helper(&self, name: &str) -> String {
    self.options.helpers.borrow_mut().insert(name.to_string());
    format!("_{name}")
  }

  pub fn with_id(
    &self,
    _fn: impl FnOnce() -> Expression<'a>,
    mut id_map: HashMap<String, Option<Expression<'a>>>,
  ) -> Expression<'a> {
    for (id, value) in id_map.iter_mut() {
      let mut identifiers = self.identifiers.borrow_mut();
      if identifiers.get(id).is_none() {
        identifiers.insert(id.clone(), vec![]);
      }
      identifiers.get_mut(id).unwrap().insert(
        0,
        if value.is_some() {
          value.take().unwrap()
        } else {
          self.ast.expression_identifier(SPAN, self.ast.atom(id))
        },
      );
    }

    let ret = _fn();

    for id in id_map.keys() {
      if let Some(ids) = self.identifiers.borrow_mut().get_mut(id) {
        ids.clear();
      }
    }

    ret
  }

  pub fn enter_block(
    &self,
    block: BlockIRNode<'a>,
    context_block: &mut BlockIRNode<'a>,
  ) -> impl FnOnce() {
    let parent = mem::take(context_block);
    *context_block = block;
    || *context_block = parent
  }

  pub fn enter_scope(&self) -> (i32, impl FnOnce()) {
    let mut scope_level = self.scope_level.borrow_mut();
    let current = *scope_level;
    *scope_level += 1;
    (current, || *self.scope_level.borrow_mut() -= 1)
  }

  // IR -> JS codegen
  pub fn generate(self: &'a mut CodegenContext<'a>) -> Expression<'a> {
    let context = self as *mut CodegenContext;
    let ast = &self.ast;
    let mut statements = ast.vec();

    statements.push(Statement::VariableDeclaration(
      ast.alloc_variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ast.vec1(ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern(
            ast.binding_pattern_kind_binding_identifier(SPAN, "_cache"),
            NONE,
            false,
          ),
          Some(ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&self.helper("useVdomCache"))),
            NONE,
            ast.vec(),
            false,
          )),
          false,
        )),
        false,
      ),
    ));

    for name in &self.ir.component {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(
                ast.alloc_binding_identifier(SPAN, ast.atom(&to_valid_asset_id(name, "component"))),
              ),
              NONE,
              false,
            ),
            Some(ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&self.helper("resolveComponent"))),
              NONE,
              ast.vec_from_array([Argument::StringLiteral(ast.alloc_string_literal(
                SPAN,
                ast.atom(name),
                None,
              ))]),
              false,
            )),
            false,
          )),
          false,
        ),
      ));
    }

    for name in &self.ir.directive {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(
                ast.alloc_binding_identifier(SPAN, ast.atom(&to_valid_asset_id(name, "directive"))),
              ),
              NONE,
              false,
            ),
            Some(ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&self.helper("resolveDirective"))),
              NONE,
              ast.vec1(Argument::StringLiteral(ast.alloc_string_literal(
                SPAN,
                ast.atom(name),
                None,
              ))),
              false,
            )),
            false,
          )),
          false,
        ),
      ))
    }

    let context_block = &mut *self.block.borrow_mut() as *mut BlockIRNode;
    statements.extend(gen_block_content(None, unsafe { &mut *context }, unsafe {
      &mut *context_block
    }));

    ast.expression_call(
      SPAN,
      ast.expression_parenthesized(
        SPAN,
        ast.expression_arrow_function(
          SPAN,
          false,
          false,
          NONE,
          ast.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            ast.vec(),
            NONE,
          ),
          NONE,
          ast.function_body(SPAN, ast.vec(), statements),
        ),
      ),
      NONE,
      ast.vec(),
      false,
    )
  }
}
