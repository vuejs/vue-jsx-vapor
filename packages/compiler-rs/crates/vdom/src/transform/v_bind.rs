use common::{
  check::is_reserved_prop, expression::jsx_attribute_value_to_expression, text::camelize,
};
use oxc_ast::ast::{JSXAttribute, JSXAttributeName, JSXElement, PropertyKind};
use oxc_span::{GetSpan, SPAN};

use crate::{
  ir::index::BlockIRNode,
  transform::{DirectiveTransformResult, TransformContext},
};

// v-bind without arg is handled directly in ./transformElement.ts due to its affecting
// codegen for the entire props object. This transform here is only for v-bind
// *with* args.
pub fn transform_v_bind<'a>(
  dir: &'a mut JSXAttribute<'a>,
  _: &JSXElement,
  context: &'a TransformContext<'a>,
  _: &mut BlockIRNode,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = &context.ast;

  let name_string = match &dir.name {
    JSXAttributeName::Identifier(name) => &name.name.to_string(),
    JSXAttributeName::NamespacedName(_) => return None,
  };
  let name_splited: Vec<&str> = name_string.split("_").collect();
  let modifiers = name_splited[1..].to_vec();
  let name_string = name_splited[0].to_string();

  let mut arg = name_string;
  if is_reserved_prop(&arg) {
    return None;
  }

  if modifiers.contains(&"camel") {
    arg = camelize(&arg)
  }

  if !context.options.in_ssr {
    if modifiers.contains(&"prop") {
      arg = format!(".{}", arg);
    } else if modifiers.contains(&"attr") {
      arg = format!("^{}", arg);
    }
  };

  let value = if let Some(value) = &mut dir.value {
    jsx_attribute_value_to_expression(value, ast.allocator)
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
    need_runtime: None,
  })
}
