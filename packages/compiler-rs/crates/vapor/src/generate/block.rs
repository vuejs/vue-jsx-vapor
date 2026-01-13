use std::mem;

use oxc_ast::NONE;
use oxc_ast::ast::{
  ArrayExpressionElement, Expression, FormalParameter, FormalParameterKind, Statement,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::operation::gen_operations;
use crate::generate::template::gen_self;
use crate::ir::index::BlockIRNode;

pub fn gen_block<'a>(
  oper: BlockIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  args: oxc_allocator::Vec<'a, FormalParameter<'a>>,
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
      gen_block_content(Some(oper), context, context_block, None),
    ),
  )
}

type GenEffectsExtraFrag<'a> =
  Option<Box<dyn FnOnce(&mut oxc_allocator::Vec<'a, Statement<'a>>, &'a mut BlockIRNode<'a>) + 'a>>;

pub fn gen_block_content<'a>(
  block: Option<BlockIRNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  gen_effects_extra_frag: GenEffectsExtraFrag<'a>,
) -> oxc_allocator::Vec<'a, Statement<'a>> {
  let ast = &context.ast;
  let block_is_none = block.is_none();
  let mut statements = ast.vec();
  let mut reset_block = None;
  let context_block = context_block as *mut BlockIRNode;
  if let Some(block) = block {
    reset_block = Some(context.enter_block(block, unsafe { &mut *context_block }));
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

  if block_is_none && context.ir.has_deferred_v_show {
    statements.push(
      ast.statement_expression(
        SPAN,
        ast.expression_call(
          SPAN,
          ast
            .member_expression_static(
              SPAN,
              ast.expression_identifier(SPAN, "deferredApplyVShows"),
              ast.identifier_name(SPAN, "forEach"),
              false,
            )
            .into(),
          NONE,
          ast.vec1(
            ast
              .expression_arrow_function(
                SPAN,
                true,
                false,
                NONE,
                ast.formal_parameters(
                  SPAN,
                  FormalParameterKind::ArrowFormalParameters,
                  ast.vec1(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, "fn"),
                  )),
                  NONE,
                ),
                NONE,
                ast.function_body(
                  SPAN,
                  ast.vec(),
                  ast.vec1(ast.statement_expression(
                    SPAN,
                    ast.expression_call(
                      SPAN,
                      ast.expression_identifier(SPAN, "fn"),
                      NONE,
                      ast.vec(),
                      false,
                    ),
                  )),
                ),
              )
              .into(),
          ),
          false,
        ),
      ),
    )
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
