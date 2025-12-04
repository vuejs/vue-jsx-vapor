use oxc_ast::NONE;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::SetHtmlIRNode;

pub fn gen_set_html<'a>(oper: SetHtmlIRNode<'a>, context: &'a CodegenContext<'a>) -> Statement<'a> {
  let ast = &context.ast;
  let SetHtmlIRNode { value, element, .. } = oper;

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("setHtml"))),
      NONE,
      ast.vec_from_array([
        ast
          .expression_identifier(SPAN, ast.atom(&format!("n{element}")))
          .into(),
        gen_expression(value, context, None, None).into(),
      ]),
      false,
    ),
  )
}
