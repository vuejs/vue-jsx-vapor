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

use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  mem,
};

use oxc_ast::{
  AstBuilder, NONE,
  ast::{Expression, FormalParameterKind, Program, Statement, VariableDeclarationKind},
};
use oxc_span::SPAN;

use crate::{
  compile::Template,
  generate::block::gen_block_content,
  ir::index::{BlockIRNode, RootIRNode},
  transform::{TransformContext, TransformOptions},
};

pub struct CodegenContext<'a> {
  pub options: &'a TransformOptions<'a>,
  pub identifiers: RefCell<HashMap<String, Vec<Expression<'a>>>>,
  pub ir: RootIRNode<'a>,
  pub block: RefCell<BlockIRNode<'a>>,
  pub scope_level: RefCell<i32>,
  pub ast: AstBuilder<'a>,
  pub transform_cotext: &'a TransformContext<'a>,
}

impl<'a> CodegenContext<'a> {
  pub fn new(context: &'a TransformContext<'a>) -> CodegenContext<'a> {
    let ir = context.ir.take();
    let block = context.block.take();
    let ast = AstBuilder::new(context.allocator);
    CodegenContext {
      transform_cotext: context,
      options: context.options,
      identifiers: RefCell::new(HashMap::new()),
      block: RefCell::new(block),
      scope_level: RefCell::new(0),
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
  pub fn generate(self: &'a CodegenContext<'a>) -> Expression<'a> {
    let ast = &self.ast;
    let mut statements = ast.vec();

    if self.ir.has_template_ref {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            ast.binding_pattern(
              ast.binding_pattern_kind_binding_identifier(SPAN, ast.atom("_setTemplateRef")),
              NONE,
              false,
            ),
            Some(ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&self.helper("createTemplateRefSetter"))),
              NONE,
              ast.vec(),
              false,
            )),
            false,
          )),
          false,
        ),
      ));
    }
    let context_block = &mut *self.block.borrow_mut() as *mut BlockIRNode;
    statements.extend(gen_block_content(
      None,
      self,
      unsafe { &mut *context_block },
      true,
      None,
    ));

    if !self.options.delegates.borrow().is_empty() {
      self.helper("delegateEvents");
    }
    if !&self.options.templates.borrow().is_empty() {
      self.helper("template");
    }

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

pub struct VaporCodegenResult<'a> {
  pub helpers: HashSet<String>,
  pub templates: Vec<Template>,
  pub delegates: HashSet<String>,
  pub program: Program<'a>,
}
