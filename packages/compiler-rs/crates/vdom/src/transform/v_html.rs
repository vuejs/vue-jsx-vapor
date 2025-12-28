use common::{error::ErrorCodes, text::is_empty_text};
use oxc_ast::ast::{JSXAttribute, JSXElement, PropertyKind};
use oxc_span::{GetSpan, SPAN};

use crate::transform::{DirectiveTransformResult, TransformContext};

pub fn transform_v_html<'a>(
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let exp = if let Some(value) = &mut dir.value {
    context.jsx_attribute_value_to_expression(value)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VHtmlNoExpression, dir.span);
    return None;
  };

  if node.children.iter().any(|c| !is_empty_text(c)) {
    context.options.on_error.as_ref()(ErrorCodes::VHtmlWithChildren, node.span);
    return None;
  }

  let ast = &context.ast;
  Some(DirectiveTransformResult {
    props: vec![ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(dir.span(), "innerHTML"),
      exp,
      false,
      false,
      false,
    )],
    runtime: None,
  })
}
