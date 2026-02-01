use oxc_ast::NONE;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::SetTemplateRefIRNode;

pub fn gen_set_template_ref<'a>(
  oper: SetTemplateRefIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;

  let SetTemplateRefIRNode {
    element,
    value,
    ref_for,
    ..
  } = oper;

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.atom(&format!("_n{element}")))
      .into(),
  );
  arguments.push(gen_expression(value, context, None, false).into());

  if ref_for {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom("_setTemplateRef")), // will be generated in root scope
      NONE,
      arguments,
      false,
    ),
  )
}
