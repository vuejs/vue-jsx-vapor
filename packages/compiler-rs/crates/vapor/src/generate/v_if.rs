use common::patch_flag::VaporBlockShape;
use common::patch_flag::VaporIfFlags;
use napi::Either;
use oxc_ast::NONE;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::NumberBase;
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
    id,
    condition,
    positive,
    negative,
    once,
    index,
    block_shape,
    ..
  } = oper;
  let flags = gen_if_flags(
    block_shape,
    once,
    if negative.is_some() {
      Some(index)
    } else {
      None
    },
  );

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
      ast.vec1(ast.statement_expression(SPAN, gen_expression(condition, context, None, false))),
    ),
  );

  let _context_block = context_block as *mut BlockIRNode;
  let positive_arg = gen_block(
    positive,
    context,
    unsafe { &mut *_context_block },
    ast.vec(),
  );

  let mut negative_arg = None;
  if let Some(negative) = negative {
    let negative = *negative;
    negative_arg = Some(match negative {
      Either::A(negative) => gen_block(negative, context, context_block, ast.vec()),
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
    ast.expression_identifier(SPAN, ast.str(context.options.helper("_createIf"))),
    NONE,
    ast.vec_from_iter(
      [
        Some(condition_expr.into()),
        Some(positive_arg.into()),
        if let Some(negative_arg) = negative_arg {
          Some(negative_arg.into())
        } else if flags.is_some() {
          Some(ast.expression_null_literal(SPAN).into())
        } else {
          None
        },
        if let Some(flags) = flags {
          Some(
            ast
              .expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Decimal)
              .into(),
          )
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
        ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{}", id))),
        NONE,
        Some(expression),
        false,
      )),
      false,
    ))
  } else {
    ast.statement_expression(SPAN, expression)
  }
}

fn gen_if_flags(block_shape: i32, once: bool, index: Option<i32>) -> Option<i32> {
  let mut flags = block_shape;
  if once {
    flags |= VaporIfFlags::Once as i32;
  } else if let Some(index) = index {
    // The encoded index is shifted by +1 so runtime can use 0 as the unkeyed
    // sentinel while preserving source index 0.
    flags |= (index + 1) << VaporIfFlags::IndexShift as i32;
  }

  // This is the only omitted-flags case: true branch is single-root, false
  // branch is empty, and there is no once/index metadata.
  if flags == VaporBlockShape::SingleRoot as i32 {
    return None;
  }

  return Some(flags);
}
