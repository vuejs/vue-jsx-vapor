use oxc_ast::NONE;
use oxc_ast::ast::{Statement, VariableDeclarationKind};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::gen_block;
use crate::generate::component::gen_raw_props;
use crate::generate::expression::gen_expression;
use crate::ir::index::{BlockIRNode, SlotOutletNodeIRNode};

pub fn gen_slot_outlet<'a>(
  oper: SlotOutletNodeIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let SlotOutletNodeIRNode {
    id,
    name,
    fallback,
    props,
    once,
    no_slotted,
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
          ast.binding_pattern_binding_identifier(SPAN, ast.atom(&format!("_n{}", id))),
          NONE,
          Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("createSlot"))),
              NONE,
              ast.vec_from_iter(
                [
                  if name.is_static {
                    Some(
                      ast
                        .expression_string_literal(SPAN, ast.atom(&name.content), None)
                        .into(),
                    )
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
                  } else if fallback.is_some() {
                    Some(ast.expression_null_literal(SPAN).into())
                  } else {
                    None
                  },
                  if let Some(fallback) = fallback {
                    Some(gen_block(fallback, context, context_block, ast.vec()).into())
                  } else if no_slotted || once {
                    Some(ast.expression_identifier(SPAN, "void 0").into())
                  } else {
                    None
                  },
                  if no_slotted {
                    Some(ast.expression_boolean_literal(SPAN, true).into())
                  } else if once {
                    Some(ast.expression_boolean_literal(SPAN, false).into())
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
            ),
          ),
          false,
        ),
      ),
      false,
    ),
  )
}
