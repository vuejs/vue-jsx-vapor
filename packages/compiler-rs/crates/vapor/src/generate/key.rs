use oxc_ast::NONE;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::Statement;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::gen_block;
use crate::generate::expression::gen_expression;
use crate::ir::index::BlockIRNode;
use crate::ir::index::KeyIRNode;

pub fn gen_key<'a>(
  oper: KeyIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let KeyIRNode {
    id, value, block, ..
  } = oper;

  let expr = ast.expression_arrow_function(
    SPAN,
    true,
    false,
    NONE,
    ast.formal_parameters(
      SPAN,
      FormalParameterKind::ArrowFormalParameters,
      ast.vec(),
      NONE,
    ),
    NONE,
    ast.function_body(
      SPAN,
      ast.vec(),
      ast.vec1(ast.statement_expression(SPAN, gen_expression(value, context, None, false))),
    ),
  );

  let _context_block = context_block as *mut BlockIRNode;
  let block_fn = gen_block(block, context, unsafe { &mut *_context_block }, ast.vec());

  let expression = ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.atom(&context.helper("createKeyedFragment"))),
    NONE,
    ast.vec_from_array([expr.into(), block_fn.into()]),
    false,
  );

  Statement::VariableDeclaration(ast.alloc_variable_declaration(
    SPAN,
    VariableDeclarationKind::Const,
    ast.vec1(ast.variable_declarator(
      SPAN,
      VariableDeclarationKind::Const,
      ast.binding_pattern_binding_identifier(SPAN, ast.atom(&format!("_n{}", id))),
      NONE,
      Some(expression),
      false,
    )),
    false,
  ))
}
