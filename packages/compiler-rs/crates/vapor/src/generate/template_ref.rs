use oxc_ast::NONE;
use oxc_ast::ast::{
  AssignmentOperator, AssignmentTarget, BindingPatternKind, Statement, VariableDeclarationKind,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::{DeclareOldRefIRNode, SetTemplateRefIRNode};

pub fn gen_set_template_ref<'a>(
  oper: SetTemplateRefIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;

  let SetTemplateRefIRNode {
    effect,
    element,
    value,
    ref_for,
    ..
  } = oper;

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.atom(&format!("n{element}")))
      .into(),
  );
  arguments.push(gen_expression(value, context, None, None).into());

  if effect {
    arguments.push(
      ast
        .expression_identifier(SPAN, ast.atom(&format!("r{element}")))
        .into(),
    );
  } else if ref_for {
    arguments.push(ast.expression_identifier(SPAN, "void 0").into());
  }
  if ref_for {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }

  let right = ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.atom("_setTemplateRef")), // will be generated in root scope
    NONE,
    arguments,
    false,
  );
  if effect {
    ast.statement_expression(
      SPAN,
      ast.expression_assignment(
        SPAN,
        AssignmentOperator::Assign,
        AssignmentTarget::AssignmentTargetIdentifier(
          ast.alloc_identifier_reference(SPAN, ast.atom(&format!("r{element}"))),
        ),
        right,
      ),
    )
  } else {
    ast.statement_expression(SPAN, right)
  }
}

pub fn gen_declare_old_ref<'a>(
  oper: DeclareOldRefIRNode,
  context: &CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  Statement::VariableDeclaration(ast.alloc_variable_declaration(
    SPAN,
    VariableDeclarationKind::Let,
    ast.vec1(ast.variable_declarator(
      SPAN,
      VariableDeclarationKind::Let,
      ast.binding_pattern(
        BindingPatternKind::BindingIdentifier(
          ast.alloc_binding_identifier(SPAN, ast.atom(&format!("r{}", oper.id))),
        ),
        NONE,
        false,
      ),
      None,
      false,
    )),
    false,
  ))
}
