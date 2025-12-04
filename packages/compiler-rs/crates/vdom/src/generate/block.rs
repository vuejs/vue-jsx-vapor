use oxc_ast::{
  NONE,
  ast::{Expression, FormalParameter, FormalParameterKind, Statement},
};
use oxc_span::SPAN;

use crate::{generate::CodegenContext, ir::index::BlockIRNode};

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
      gen_block_content(Some(oper), context, context_block),
    ),
  )
}

pub fn gen_block_content<'a>(
  block: Option<BlockIRNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> oxc_allocator::Vec<'a, Statement<'a>> {
  let ast = &context.ast;
  let mut statements = ast.vec();
  let mut reset_block = None;
  let context_block = context_block as *mut BlockIRNode;
  if let Some(block) = block {
    reset_block = Some(context.enter_block(block, unsafe { &mut *context_block }));
  }

  statements.push(ast.statement_expression(
    SPAN,
    ast.expression_parenthesized(
      SPAN,
      ast.expression_sequence(
        SPAN,
        ast.vec_from_array([
          ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&context.helper("openBlock"))),
            NONE,
            ast.vec(),
            false,
          ),
          // gen_self,
        ]),
      ),
    ),
  ));

  if let Some(reset_block) = reset_block {
    reset_block();
  }
  statements
}
