use oxc_ast::NONE;
use oxc_ast::ast::Argument;
use oxc_ast::ast::BindingPatternKind;
use oxc_ast::ast::Statement;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::CreateNodesIRNode;
use crate::ir::index::GetTextChildIRNode;
use crate::ir::index::SetNodesIRNode;
use crate::ir::index::SetTextIRNode;
use crate::ir::index::SimpleExpressionNode;
use crate::utils::check::is_constant_node;

pub fn gen_set_text<'a>(oper: SetTextIRNode<'a>, context: &'a CodegenContext<'a>) -> Statement<'a> {
  let ast = &context.ast;
  let SetTextIRNode {
    element,
    values,
    generated,
    ..
  } = oper;
  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(
        SPAN,
        ast.atom(&format!(
          "{}{}",
          if generated.unwrap_or(false) { "x" } else { "n" },
          element
        )),
      )
      .into(),
  );
  combine_values(&mut arguments, values, context, true, true);
  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("setText"))),
      NONE,
      arguments,
      false,
    ),
  )
}

pub fn gen_get_text_child<'a>(
  oper: GetTextChildIRNode,
  context: &CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;

  Statement::VariableDeclaration(
    ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(
        ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(
              ast.alloc_binding_identifier(SPAN, ast.atom(&format!("x{}", oper.parent))),
            ),
            NONE,
            false,
          ),
          Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("child"))),
              NONE,
              ast.vec1(
                ast
                  .expression_identifier(SPAN, ast.atom(&format!("n{}", oper.parent)))
                  .into(),
              ),
              false,
            ),
          ),
          false,
        ),
      ),
      false,
    ),
  )
}

pub fn gen_set_nodes<'a>(
  oper: SetNodesIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;

  let SetNodesIRNode {
    element,
    values,
    generated,
    once,
    ..
  } = oper;

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(
        SPAN,
        ast.atom(&format!(
          "{}{}",
          if generated.unwrap_or(false) {
            "x".to_string()
          } else {
            "n".to_string()
          },
          element
        )),
      )
      .into(),
  );
  combine_values(&mut arguments, values, context, once, false);

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("setNodes"))),
      NONE,
      arguments,
      false,
    ),
  )
}

pub fn gen_create_nodes<'a>(
  oper: CreateNodesIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;

  let CreateNodesIRNode {
    id, values, once, ..
  } = oper;

  let mut arguments = ast.vec();
  combine_values(&mut arguments, values, context, once, false);

  Statement::VariableDeclaration(ast.alloc_variable_declaration(
    SPAN,
    VariableDeclarationKind::Const,
    ast.vec1(ast.variable_declarator(
      SPAN,
      VariableDeclarationKind::Const,
      ast.binding_pattern(
        ast.binding_pattern_kind_binding_identifier(SPAN, ast.atom(&format!("n{id}"))),
        NONE,
        false,
      ),
      Some(ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.atom(&context.helper("createNodes"))),
        NONE,
        arguments,
        false,
      )),
      false,
    )),
    false,
  ))
}

fn combine_values<'a>(
  arguments: &mut oxc_allocator::Vec<'a, Argument<'a>>,
  values: Vec<SimpleExpressionNode<'a>>,
  context: &'a CodegenContext<'a>,
  once: bool,
  is_set_text: bool,
) {
  let ast = &context.ast;

  for value in values {
    let should_wrap = !once
      && !is_set_text
      && !value.content.is_empty()
      && !value.is_static
      && !is_constant_node(&value.ast.as_deref());
    let literal_expression_value = &value.get_literal_expression_value();
    let exp = gen_expression(value, context, None, Some(should_wrap));
    if is_set_text && literal_expression_value.is_none() {
      // dynamic, wrap with toDisplayString
      arguments.push(
        ast
          .expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&context.helper("toDisplayString"))),
            NONE,
            ast.vec1(exp.into()),
            false,
          )
          .into(),
      )
    } else {
      arguments.push(exp.into());
    }
  }
}
