use oxc_ast::NONE;
use oxc_ast::ast::BindingPatternKind;
use oxc_ast::ast::Expression;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::PropertyKind;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::DirectiveIRNode;
use crate::ir::index::DirectiveNode;
use crate::ir::index::SimpleExpressionNode;
use crate::utils::check::is_simple_identifier;

// This is only for built-in v-model on native elements.
pub fn gen_v_model<'a>(
  oper: DirectiveIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let DirectiveIRNode {
    model_type,
    element,
    dir: DirectiveNode { exp, modifiers, .. },
    ..
  } = oper;
  let exp = exp.unwrap();

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(
        SPAN,
        ast.atom(&context.helper(match model_type.unwrap().as_str() {
          "text" => "applyTextModel",
          "radio" => "applyRadioModel",
          "checkbox" => "applyCheckboxModel",
          "select" => "applySelectModel",
          "dynamic" => "applyDynamicModel",
          _ => panic!("Unsupported model type"),
        })),
      ),
      NONE,
      ast.vec_from_iter(
        [
          Some(
            ast
              .expression_identifier(SPAN, ast.atom(&format!("n{element}")))
              .into(),
          ),
          // getter
          Some(
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
                    ast
                      .statement_expression(SPAN, gen_expression(exp.clone(), context, None, None)),
                  ),
                ),
              )
              .into(),
          ),
          // setter
          Some(gen_model_handler(exp, context).into()),
          // modifiers
          if !modifiers.is_empty() {
            Some(
              ast
                .expression_object(
                  SPAN,
                  ast.vec_from_iter(modifiers.into_iter().map(|modifier| {
                    let modifier = modifier.content;
                    let modifier = if is_simple_identifier(&modifier) {
                      &modifier
                    } else {
                      &format!("\"{}\"", modifier)
                    };
                    ast.object_property_kind_object_property(
                      SPAN,
                      PropertyKind::Init,
                      ast.property_key_static_identifier(SPAN, ast.atom(modifier)),
                      ast.expression_boolean_literal(SPAN, true),
                      false,
                      false,
                      false,
                    )
                  })),
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
  )
}

pub fn gen_model_handler<'a>(
  exp: SimpleExpressionNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  ast.expression_arrow_function(
    SPAN,
    true,
    false,
    NONE,
    ast.formal_parameters(
      SPAN,
      FormalParameterKind::ArrowFormalParameters,
      ast.vec1(ast.formal_parameter(
        SPAN,
        ast.vec(),
        ast.binding_pattern(
          BindingPatternKind::BindingIdentifier(
            ast.alloc_binding_identifier(SPAN, ast.atom("_value")),
          ),
          NONE,
          false,
        ),
        None,
        false,
        false,
      )),
      NONE,
    ),
    NONE,
    ast.function_body(
      SPAN,
      ast.vec(),
      ast.vec1(ast.statement_expression(
        SPAN,
        gen_expression(
          exp,
          context,
          Some(ast.expression_identifier(SPAN, ast.atom("_value"))),
          None,
        ),
      )),
    ),
  )
}
