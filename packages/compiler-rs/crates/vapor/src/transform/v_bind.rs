use common::{
  check::is_reserved_prop, directive::resolve_prop_name,
  expression::jsx_attribute_value_to_expression, text::camelize,
};
use oxc_ast::ast::{Expression, JSXAttribute, JSXAttributeName};
use oxc_span::SPAN;

use crate::transform::{DirectiveTransformResult, TransformContext};

pub fn transform_v_bind<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = context.ast;
  let name_string = match &dir.name {
    JSXAttributeName::Identifier(name) => name.name.as_str(),
    JSXAttributeName::NamespacedName(_) => return None,
  };
  let name_splited: Vec<&str> = resolve_prop_name(name_string);
  let modifiers = name_splited[1..].to_vec();
  if is_reserved_prop(name_splited[0]) {
    return None;
  }

  let mut arg = ast.alloc_string_literal(SPAN, ast.atom(name_splited[0]), None);

  let exp = if let Some(value) = &mut dir.value {
    jsx_attribute_value_to_expression(value, ast)
  } else {
    ast.expression_boolean_literal(SPAN, true)
  };

  if modifiers.contains(&"camel") {
    arg.value = ast.atom(&camelize(arg.value.into()))
  }

  let modifier = if modifiers.contains(&"prop") {
    Some(".")
  } else if modifiers.contains(&"attr") {
    Some("^")
  } else {
    None
  };

  Some(DirectiveTransformResult {
    key: Expression::StringLiteral(arg),
    value: exp,
    runtime_camelize: false,
    modifier,
    handler: false,
    handler_modifiers: None,
    model: false,
    model_modifiers: None,
  })
}
