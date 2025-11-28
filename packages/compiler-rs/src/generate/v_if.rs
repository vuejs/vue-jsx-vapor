use napi::Either;
use oxc_ast::NONE;
use oxc_ast::ast::BindingPatternKind;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::Statement;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::gen_block;
use crate::generate::expression::gen_expression;
use crate::ir::index::BlockIRNode;
use crate::ir::index::IfIRNode;

pub fn gen_if<'a>(
  oper: IfIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  is_nested: bool,
) -> Statement<'a> {
  let ast = &context.ast;
  let IfIRNode {
    condition,
    positive,
    negative,
    once,
    ..
  } = oper;

  let condition_expr = ast.expression_arrow_function(
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
      ast.vec1(ast.statement_expression(SPAN, gen_expression(condition, context, None, None))),
    ),
  );

  let _context_block = context_block as *mut BlockIRNode;
  let positive_arg = gen_block(
    positive,
    context,
    unsafe { &mut *_context_block },
    ast.vec(),
    false,
  );

  let mut negative_arg = None;
  if let Some(negative) = negative {
    let negative = *negative;
    negative_arg = Some(match negative {
      Either::A(negative) => gen_block(negative, context, context_block, ast.vec(), false),
      Either::B(negative) => ast.expression_arrow_function(
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
          ast.vec1(gen_if(negative, context, context_block, true)),
        ),
      ),
    });
  }

  let expression = ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.atom(&context.helper("createIf"))),
    NONE,
    ast.vec_from_iter(
      [
        Some(condition_expr.into()),
        Some(positive_arg.into()),
        if let Some(negative_arg) = negative_arg {
          Some(negative_arg.into())
        } else if once {
          Some(ast.expression_null_literal(SPAN).into())
        } else {
          None
        },
        if once {
          Some(ast.expression_boolean_literal(SPAN, true).into())
        } else {
          None
        },
      ]
      .into_iter()
      .flatten(),
    ),
    false,
  );

  if !is_nested {
    Statement::VariableDeclaration(ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        ast.binding_pattern(
          BindingPatternKind::BindingIdentifier(
            ast.alloc_binding_identifier(SPAN, ast.atom(&format!("n{}", oper.id))),
          ),
          NONE,
          false,
        ),
        Some(expression),
        false,
      )),
      false,
    ))
  } else {
    ast.statement_expression(SPAN, expression)
  }
}
