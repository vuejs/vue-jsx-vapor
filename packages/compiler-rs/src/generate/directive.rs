use napi::bindgen_prelude::Either16;
use oxc_ast::AstBuilder;
use oxc_ast::NONE;
use oxc_ast::ast::Statement;
use oxc_ast::ast::{
  Argument, ArrayExpressionElement, FormalParameterKind, ObjectExpression, PropertyKind,
};
use oxc_span::GetSpan;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::generate::v_model::gen_v_model;
use crate::generate::v_show::gen_v_show;
use crate::ir::index::BlockIRNode;
use crate::ir::index::DirectiveIRNode;
use crate::utils::check::is_simple_identifier;
use crate::utils::text::to_valid_asset_id;

pub fn gen_builtin_directive<'a>(
  oper: DirectiveIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Option<Statement<'a>> {
  match oper.name.as_str() {
    "show" => Some(gen_v_show(oper, context)),
    "model" => Some(gen_v_model(oper, context)),
    _ => None,
  }
}

/**
 * user directives via `withVaporDirectives`
 * TODO the compiler side is implemented but no runtime support yet
 * it was removed due to perf issues
 */
pub fn gen_directives_for_element<'a>(
  id: i32,
  context: &'a CodegenContext<'a>,
  context_block: &mut BlockIRNode<'a>,
) -> Option<Statement<'a>> {
  let ast = &context.ast;
  let mut element = String::new();
  let mut directive_items = ast.vec();
  for item in &mut context_block.operation {
    if let Either16::M(item) = item
      && item.element == id
      && !item.builtin.unwrap_or(false)
    {
      if element.is_empty() {
        element = item.element.to_string();
      }
      let name = &item.name;
      let asset = item.asset;
      let directive_var = ast.alloc_identifier_reference(
        SPAN,
        if asset.unwrap_or(false) {
          ast.atom(&to_valid_asset_id(name, "directive"))
        } else {
          ast.atom(name)
        },
      );
      let value = if let Some(exp) = item.dir.exp.take() {
        let expression = gen_expression(exp, context, None, None);
        Some(ast.alloc_arrow_function_expression(
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
            ast.vec1(ast.statement_expression(expression.span(), expression)),
          ),
        ))
      } else {
        None
      };
      let argument = item
        .dir
        .arg
        .take()
        .map(|arg| gen_expression(arg, context, None, None));
      let modifiers = if !item.dir.modifiers.is_empty() {
        Some(gen_directive_modifiers(
          item.dir.modifiers.drain(..).map(|m| m.content).collect(),
          ast,
        ))
      } else {
        None
      };

      directive_items.push(ArrayExpressionElement::ArrayExpression(
        ast.alloc_array_expression(
          SPAN,
          ast.vec_from_iter(
            [
              Some(ArrayExpressionElement::Identifier(directive_var)),
              if let Some(value) = value {
                Some(ArrayExpressionElement::ArrowFunctionExpression(value))
              } else if argument.is_some() || modifiers.is_some() {
                Some(ArrayExpressionElement::Identifier(
                  ast.alloc_identifier_reference(SPAN, "void 0"),
                ))
              } else {
                None
              },
              if let Some(argument) = argument {
                Some(argument.into())
              } else if modifiers.is_some() {
                Some(ArrayExpressionElement::Identifier(
                  ast.alloc_identifier_reference(SPAN, "void 0"),
                ))
              } else {
                None
              },
              modifiers.map(ArrayExpressionElement::ObjectExpression),
            ]
            .into_iter()
            .flatten(),
          ),
        ),
      ));
    }
  }
  if directive_items.is_empty() {
    return None;
  }
  let directives = ast.alloc_array_expression(SPAN, directive_items);
  Some(ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("withVaporDirectives"))),
      NONE,
      ast.vec_from_array([
        Argument::Identifier(
          ast.alloc_identifier_reference(SPAN, ast.atom(&format!("n{}", element))),
        ),
        Argument::ArrayExpression(directives),
      ]),
      false,
    ),
  ))
}

pub fn gen_directive_modifiers<'a>(
  modifiers: Vec<String>,
  ast: &AstBuilder<'a>,
) -> oxc_allocator::Box<'a, ObjectExpression<'a>> {
  ast.alloc_object_expression(
    SPAN,
    ast.vec_from_iter(modifiers.into_iter().map(|modifier| {
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
}
