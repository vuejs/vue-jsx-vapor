use common::{
  check::{is_boolean_attr, is_reserved_prop, is_special_boolean_attr},
  directive::{Directives, resolve_prop_name},
  expression::jsx_attribute_value_to_expression,
  text::{camelize, get_text_like_value},
};
use oxc_allocator::TakeIn;
use oxc_ast::ast::{BigintBase, Expression, JSXAttribute, JSXAttributeName, JSXAttributeValue};
use oxc_span::SPAN;

use crate::transform::{DirectiveTransformResult, TransformContext};

// `true-value` / `false-value` are only read back by `v-model` on a checkbox,
// and a dynamic `type` can still make the element a checkbox at runtime.
fn is_checkbox_value_prop(directives: &Directives, key: &str) -> bool {
  if directives.tag_name != "input" || !matches!(key, "true-value" | "false-value") {
    return false;
  }
  match &directives._type {
    Some(attr) => match &attr.value {
      Some(JSXAttributeValue::ExpressionContainer(_)) => true,
      Some(JSXAttributeValue::StringLiteral(value)) => value.value == "checkbox",
      _ => false,
    },
    None => directives.has_dynamic_key_v_bind,
  }
}

// Props `v-model` reads back off the element as raw values (`_value`,
// `_trueValue`, `_falseValue`), so a bound number literal has to keep its type.
fn is_model_value_prop(directives: &Directives, key: &str) -> bool {
  (key == "value"
    && matches!(
      directives.tag_name,
      "input" | "option" | "textarea" | "select"
    ))
    || is_checkbox_value_prop(directives, key)
}

pub fn transform_v_bind<'a>(
  directives: &Directives,
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = context.ast;
  let name_string = match &dir.name {
    JSXAttributeName::Identifier(name) => name.name.as_str(),
    JSXAttributeName::NamespacedName(name) => name.span.source_text(context.source_text),
  };
  let name_splited: Vec<&str> = resolve_prop_name(name_string);
  if is_reserved_prop(name_splited[0]) {
    return None;
  }
  let modifiers = name_splited[1..].to_vec();
  let modifier = if modifiers.contains(&"prop") {
    Some(".")
  } else if modifiers.contains(&"attr") {
    Some("^")
  } else {
    None
  };

  let mut arg = ast.alloc_string_literal(SPAN, ast.str(name_splited[0]), None);
  if modifiers.contains(&"camel") {
    arg.value = ast.str(&camelize(arg.value.into()))
  }

  let value = if let Some(value) = dir.value.as_mut() {
    if let Some(value) = match value {
      JSXAttributeValue::ExpressionContainer(value) => {
        let expression = value.expression.as_expression_mut()?;
        // A number literal loses its type as soon as it is stringified into
        // the template, so hold it back wherever the value is consumed as a
        // raw value: component, slot outlet and custom element props, boolean
        // attributes are folded from the type of the value itself, and v-model
        // reads its value props back off the element. With `.attr`, `setAttr`
        // still checks special boolean attributes and stores raw checkbox
        // true/false values before calling `setAttribute`.
        let exclude_number = (directives.is_component || directives.tag_name == "slot")
          || is_special_boolean_attr(&arg.value)
          || is_checkbox_value_prop(directives, &arg.value)
          || (!modifiers.contains(&"attr")
            && (is_boolean_attr(&arg.value) || is_model_value_prop(directives, &arg.value)));
        if exclude_number && expression.is_number_literal() {
          if let Expression::NumericLiteral(_) = expression {
            Some(expression.take_in(ast.allocator))
          } else if let Expression::BigIntLiteral(node) = expression
            && let Some(raw) = node.raw
          {
            Some(ast.expression_big_int_literal(
              SPAN,
              ast.str(raw.as_str()),
              None,
              BigintBase::Decimal,
            ))
          } else {
            None
          }
        } else {
          get_text_like_value(expression)
            .map(|value| ast.expression_string_literal(SPAN, ast.str(value.as_ref()), None))
        }
      }
      JSXAttributeValue::StringLiteral(value) => {
        Some(ast.expression_string_literal(SPAN, ast.str(value.value.as_ref()), None))
      }
      _ => None,
    } {
      return Some(DirectiveTransformResult::new(
        Expression::StringLiteral(arg),
        value,
        modifier,
      ));
    } else {
      jsx_attribute_value_to_expression(value, ast)?
    }
  } else {
    let value =
      if !directives.is_component && !directives.is_custom_element && is_boolean_attr(&arg.value) {
        ast.expression_string_literal(SPAN, "", None)
      } else {
        ast.expression_boolean_literal(SPAN, true)
      };
    return Some(DirectiveTransformResult::new(
      Expression::StringLiteral(arg),
      value,
      modifier,
    ));
  };

  Some(DirectiveTransformResult {
    key: Expression::StringLiteral(arg),
    value,
    to_display_string: false,
    runtime_camelize: false,
    modifier,
    handler: false,
    handler_modifiers: None,
    model: false,
    model_modifiers: None,
  })
}
