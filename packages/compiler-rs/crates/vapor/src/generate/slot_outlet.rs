use oxc_ast::NONE;
use oxc_ast::ast::{Expression, Statement, VariableDeclarationKind};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::gen_block;
use crate::generate::component::gen_raw_props;
use crate::generate::expression::gen_expression;
use crate::ir::index::{BlockIRNode, SlotOutletIRNode};

pub fn gen_slot_outlet<'a>(
  oper: SlotOutletIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let SlotOutletIRNode {
    id,
    name,
    fallback,
    props,
    flags,
    ..
  } = oper;

  Statement::VariableDeclaration(
    ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(
        ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{}", id))),
          NONE,
          Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.str(context.options.helper("_createSlot"))),
              NONE,
              ast.vec_from_iter(
                [
                  if matches!(&name, Expression::StringLiteral(name) if name.value == "default")
                    && props.is_empty()
                    && fallback.is_none()
                    && flags == 0
                  {
                    None
                  } else if let Expression::StringLiteral(name) = name {
                    Some(ast.expression_string_literal(SPAN, name.value, None).into())
                  } else {
                    Some(
                      ast
                        .expression_arrow_function(
                          SPAN,
                          true,
                          false,
                          NONE,
                          ast.formal_parameters(
                            SPAN,
                            oxc_ast::ast::FormalParameterKind::ArrowFormalParameters,
                            ast.vec(),
                            NONE,
                          ),
                          NONE,
                          ast.function_body(
                            SPAN,
                            ast.vec(),
                            ast.vec1(ast.statement_expression(
                              SPAN,
                              gen_expression(name, context, None, false),
                            )),
                          ),
                        )
                        .into(),
                    )
                  },
                  if !props.is_empty()
                    && let Some(props) = gen_raw_props(props, context)
                  {
                    Some(props.into())
                  } else if fallback.is_some() || flags > 0 {
                    Some(ast.expression_null_literal(SPAN).into())
                  } else {
                    None
                  },
                  if let Some(fallback) = fallback {
                    Some(gen_block(fallback, context, context_block, ast.vec()).into())
                  } else if flags > 0 {
                    Some(ast.expression_null_literal(SPAN).into())
                  } else {
                    None
                  },
                  if flags > 0 {
                    Some(
                      ast
                        .expression_numeric_literal(
                          SPAN,
                          flags as f64,
                          None,
                          oxc_ast::ast::NumberBase::Decimal,
                        )
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
            ),
          ),
          false,
        ),
      ),
      false,
    ),
  )
}
