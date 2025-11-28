use oxc_ast::NONE;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::ir::index::InsertNodeIRNode;

pub fn gen_insert_node<'a>(oper: InsertNodeIRNode, context: &CodegenContext<'a>) -> Statement<'a> {
  let ast = &context.ast;
  let InsertNodeIRNode {
    parent,
    elements,
    anchor,
    ..
  } = oper;

  let mut arguments = ast.vec();
  if elements.len() > 1 {
    arguments.push(
      ast
        .expression_array(
          SPAN,
          ast.vec_from_iter(elements.into_iter().map(|element| {
            ast
              .expression_identifier(SPAN, ast.atom(&format!("n{}", element)))
              .into()
          })),
        )
        .into(),
    );
  } else {
    arguments.push(
      ast
        .expression_identifier(SPAN, ast.atom(&format!("n{}", elements[0])))
        .into(),
    )
  }

  arguments.push(
    ast
      .expression_identifier(SPAN, ast.atom(&format!("n{parent}")))
      .into(),
  );

  if let Some(anchor) = anchor {
    arguments.push(
      ast
        .expression_identifier(SPAN, ast.atom(&format!("n{anchor}")))
        .into(),
    );
  }

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom("insert")),
      NONE,
      arguments,
      false,
    ),
  )
}
