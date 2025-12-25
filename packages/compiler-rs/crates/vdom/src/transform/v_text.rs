use common::{check::is_void_tag, error::ErrorCodes, text::is_empty_text};
use napi::Either;
use oxc_ast::{
  NONE,
  ast::{JSXAttribute, JSXElement, PropertyKind},
};
use oxc_span::{GetSpan, SPAN};

use crate::transform::{
  DirectiveTransformResult, TransformContext, cache_static::get_constant_type,
};

pub fn transform_v_text<'a>(
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let exp = if let Some(value) = &mut dir.value {
    context.jsx_attribute_value_to_expression(value)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VTextNoExpression, dir.span);
    return None;
  };

  if node.children.iter().filter(|c| !is_empty_text(c)).count() > 0 {
    context.options.on_error.as_ref()(ErrorCodes::VTextWithChildren, node.span);
    return None;
  };

  // v-text on void tags do nothing
  if let Some(name) = &node.opening_element.name.get_identifier_name()
    && is_void_tag(name)
  {
    return None;
  }

  let ast = &context.ast;
  Some(DirectiveTransformResult {
    props: vec![ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(dir.span(), "textContent"),
      if get_constant_type(
        Either::B(&exp),
        context,
        &mut context.codegen_map.borrow_mut(),
      ) as i32
        > 0
      {
        exp
      } else {
        ast.expression_call(
          exp.span(),
          ast.expression_identifier(SPAN, ast.atom(&context.helper("toDisplayString"))),
          NONE,
          ast.vec1(exp.into()),
          false,
        )
      },
      false,
      false,
      false,
    )],
    runtime: None,
  })
}
