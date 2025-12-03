use oxc_ast::NONE;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::DirectiveIRNode;

pub fn gen_v_show<'a>(oper: DirectiveIRNode<'a>, context: &'a CodegenContext<'a>) -> Statement<'a> {
  let ast = &context.ast;
  let DirectiveIRNode { dir, element, .. } = oper;

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("applyVShow"))),
      NONE,
      ast.vec_from_array([
        ast
          .expression_identifier(SPAN, ast.atom(&format!("n{element}")))
          .into(),
        ast
          .expression_arrow_function(
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
              ast.vec1(
                ast.statement_expression(
                  SPAN,
                  gen_expression(dir.exp.unwrap(), context, None, None),
                ),
              ),
            ),
          )
          .into(),
      ]),
      false,
    ),
  )
}
