use oxc_ast::AstBuilder;
use oxc_ast::NONE;
use oxc_ast::ast::Expression;
use oxc_ast::ast::Statement;
use oxc_ast::ast::{Argument, ArrayExpressionElement, ObjectExpression, PropertyKind};
use oxc_span::SPAN;

use indexmap::IndexMap;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::generate::v_model::gen_v_model;
use crate::generate::v_show::gen_v_show;
use crate::ir::index::DirectiveIRNode;
use common::check::is_simple_identifier;
use common::expression::gen_getter;
use common::text::to_valid_asset_id;

/**
 * Run a helper call inside the once ambient: a helper that creates its own
 * effects at a v-once site has them run once like compiled ones do.
 */
fn gen_once<'a>(call: Expression<'a>, context: &'a CodegenContext<'a>) -> Expression<'a> {
  let ast = &context.ast;
  ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.str(context.options.helper("_withOnce"))),
    NONE,
    ast.vec1(gen_getter(call, ast).into()),
    false,
  )
}

pub fn gen_builtin_directive<'a>(
  oper: DirectiveIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Option<Statement<'a>> {
  let once = oper.once;
  let call = if oper.name.as_ref() == "show" {
    gen_v_show(oper, context)
  } else if oper.name.as_ref() == "model" {
    gen_v_model(oper, context)
  } else {
    return None;
  };
  Some(
    context
      .ast
      .statement_expression(SPAN, if once { gen_once(call, context) } else { call }),
  )
}

/**
 * user directives via `withVaporDirectives`, emitted at the end of the block
 * so the element's props, children and v-model are in place first
 */
pub fn gen_custom_directives<'a>(
  by_element: IndexMap<i32, Vec<DirectiveIRNode<'a>>>,
  context: &'a CodegenContext<'a>,
) -> Vec<Statement<'a>> {
  by_element
    .into_values()
    .map(|dirs| gen_element_directives(dirs, context))
    .collect()
}

fn gen_element_directives<'a>(
  mut opers: Vec<DirectiveIRNode<'a>>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let element = opers[0].element;
  // All directives on the same element share the same once ambient; the first
  // one mirrors upstream's `opers[0].once`.
  let once = opers[0].once;
  let mut directive_items = ast.vec();
  for item in &mut opers {
    let name = &item.name;
    let asset = item.asset;
    let directive_var = ast.alloc_identifier_reference(
      SPAN,
      if asset {
        ast.str(&to_valid_asset_id(name, "directive"))
      } else {
        ast.str(name)
      },
    );
    let value = if let Some(exp) = item.dir.exp.take() {
      let expression = gen_expression(exp, context, None, false);
      Some(gen_getter(expression, ast))
    } else {
      None
    };
    let argument = item
      .dir
      .arg
      .take()
      .map(|arg| gen_getter(gen_expression(arg, context, None, false), ast));
    let modifiers = if !item.dir.modifiers.is_empty() {
      Some(gen_directive_modifiers(item.dir.modifiers.clone(), ast))
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
              Some(value.into())
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
  let directives = ast.alloc_array_expression(SPAN, directive_items);
  let call = ast.expression_call(
    SPAN,
    ast.expression_identifier(
      SPAN,
      ast.str(context.options.helper("_withVaporDirectives")),
    ),
    NONE,
    ast.vec_from_array([
      Argument::Identifier(
        ast.alloc_identifier_reference(SPAN, ast.str(&format!("_n{}", element))),
      ),
      Argument::ArrayExpression(directives),
    ]),
    false,
  );
  let expression = if once { gen_once(call, context) } else { call };
  ast.statement_expression(SPAN, expression)
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
        ast.property_key_static_identifier(SPAN, ast.str(modifier)),
        ast.expression_boolean_literal(SPAN, true),
        false,
        false,
        false,
      )
    })),
  )
}
