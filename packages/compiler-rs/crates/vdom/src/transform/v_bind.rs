use std::borrow::Cow;

use common::{check::is_simple_identifier, directive::resolve_prop_name, text::camelize};
use oxc_ast::ast::{JSXAttribute, JSXAttributeName, JSXElement, PropertyKind};
use oxc_span::{GetSpan, SPAN};

use crate::transform::{DirectiveTransformResult, TransformContext};

// v-bind without arg is handled directly in ./transformElement.ts due to its affecting
// codegen for the entire props object. This transform here is only for v-bind
// *with* args.
pub fn transform_v_bind<'a>(
  dir: &'a mut JSXAttribute<'a>,
  _: &JSXElement<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = &context.ast;

  let name_string = match &dir.name {
    JSXAttributeName::Identifier(name) => name.name.as_ref(),
    JSXAttributeName::NamespacedName(_) => return None,
  };
  let name_splited: Vec<&str> = resolve_prop_name(name_string);
  let modifiers = name_splited[1..].to_vec();
  let mut arg = Cow::Borrowed(name_splited[0]);

  if modifiers.contains(&"camel") {
    arg = camelize(arg)
  }

  if !context.options.ssr {
    if modifiers.contains(&"prop") {
      arg = Cow::Owned(format!(".{}", arg));
    } else if modifiers.contains(&"attr") {
      arg = Cow::Owned(format!("^{}", arg));
    }
  };

  if !is_simple_identifier(&arg) {
    arg = Cow::Owned(format!("\"{}\"", arg));
  }

  let value = if let Some(value) = &mut dir.value {
    context.jsx_attribute_value_to_expression(value)
  } else {
    ast.expression_boolean_literal(SPAN, true)
  };

  Some(DirectiveTransformResult {
    props: vec![ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(dir.name.span(), ast.atom(&arg)),
      value,
      false,
      false,
      false,
    )],
    runtime: None,
  })
}
