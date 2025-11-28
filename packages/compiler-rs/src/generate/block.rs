use std::mem;

use oxc_ast::NONE;
use oxc_ast::ast::{
  Argument, ArrayExpressionElement, BindingPatternKind, Expression, FormalParameter,
  FormalParameterKind, Statement, VariableDeclarationKind,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::operation::gen_operations;
use crate::generate::template::gen_self;
use crate::ir::index::BlockIRNode;
use crate::utils::text::to_valid_asset_id;

pub fn gen_block<'a>(
  oper: BlockIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  args: oxc_allocator::Vec<'a, FormalParameter<'a>>,
  root: bool,
) -> Expression<'a> {
  let ast = context.ast;
  ast.expression_arrow_function(
    SPAN,
    false,
    false,
    NONE,
    ast.alloc_formal_parameters(SPAN, FormalParameterKind::ArrowFormalParameters, args, NONE),
    NONE,
    ast.alloc_function_body(
      SPAN,
      ast.vec(),
      gen_block_content(Some(oper), context, context_block, root, None),
    ),
  )
}

type GenEffectsExtraFrag<'a> =
  Option<Box<dyn FnOnce(&mut oxc_allocator::Vec<'a, Statement<'a>>, &'a mut BlockIRNode<'a>) + 'a>>;

pub fn gen_block_content<'a>(
  block: Option<BlockIRNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  root: bool,
  gen_effects_extra_frag: GenEffectsExtraFrag<'a>,
) -> oxc_allocator::Vec<'a, Statement<'a>> {
  let ast = &context.ast;
  let mut statements = ast.vec();
  let mut reset_block = None;
  let context_block = context_block as *mut BlockIRNode;
  if let Some(block) = block {
    reset_block = Some(context.enter_block(block, unsafe { &mut *context_block }));
  }

  if root {
    for name in &context.ir.component {
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
              ast.expression_identifier(SPAN, ast.atom(&context.helper("resolveComponent"))),
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
    for name in &context.ir.directive {
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
              ast.expression_identifier(SPAN, ast.atom(&context.helper("resolveDirective"))),
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
  }

  for child in mem::take(&mut unsafe { &mut *context_block }.dynamic.children) {
    gen_self(&mut statements, child, context, unsafe {
      &mut *context_block
    });
  }

  gen_operations(
    &mut statements,
    mem::take(&mut unsafe { &mut *context_block }.operation),
    context,
    unsafe { &mut *context_block },
  );
  if let Some(statement) = gen_effects(context, unsafe { &mut *context_block }) {
    statements.push(statement);
  }
  if let Some(gen_extra_frag) = gen_effects_extra_frag {
    gen_extra_frag(&mut statements, unsafe { &mut *context_block })
  }

  let mut return_nodes = unsafe { &mut *context_block }.returns.iter().map(|n| {
    ast
      .expression_identifier(SPAN, ast.atom(&format!("n{n}")))
      .into()
  });
  statements.push(ast.statement_return(
    SPAN,
    Some(if return_nodes.len() > 1 {
      ast.expression_array(SPAN, ast.vec_from_iter(return_nodes))
    } else if let Some(node) = return_nodes.next()
      && let ArrayExpressionElement::Identifier(node) = node
    {
      ast.expression_identifier(SPAN, node.name)
    } else {
      ast.expression_null_literal(SPAN)
    }),
  ));

  if let Some(reset_block) = reset_block {
    reset_block();
  }
  statements
}

pub fn gen_effects<'a>(
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Statement<'a>> {
  let ast = &context.ast;
  let mut statements = ast.vec();
  let mut operations_count = 0;

  let effects = mem::take(&mut context_block.effect);
  let effects_is_empty = effects.is_empty();
  for effect in effects {
    operations_count += effect.operations.len();
    let _context_block = context_block as *mut BlockIRNode;
    gen_operations(&mut statements, effect.operations, context, unsafe {
      &mut *_context_block
    });
  }

  if effects_is_empty {
    None
  } else {
    Some(
      ast.statement_expression(
        SPAN,
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("renderEffect"))),
          NONE,
          ast.vec1(
            ast
              .expression_arrow_function(
                SPAN,
                operations_count == 1,
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
              )
              .into(),
          ),
          false,
        ),
      ),
    )
  }
}
