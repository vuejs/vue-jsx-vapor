use common::{check::is_reserved_prop, expression::SimpleExpressionNode, text::camelize};
use napi::bindgen_prelude::Either3;
use oxc_ast::ast::{JSXAttribute, JSXAttributeName};
use oxc_span::SPAN;

use crate::transform::{DirectiveTransformResult, TransformContext};

pub fn transform_v_bind<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let name_string = match &dir.name {
    JSXAttributeName::Identifier(name) => &name.name.to_string(),
    JSXAttributeName::NamespacedName(_) => return None,
  };
  let name_splited: Vec<&str> = name_string.split("_").collect();
  let modifiers = name_splited[1..].to_vec();
  if is_reserved_prop(name_splited[0]) {
    return None;
  }

  let mut arg = SimpleExpressionNode {
    content: name_splited[0].to_string(),
    is_static: true,
    loc: SPAN,
    ast: None,
  };

  let exp = if let Some(value) = &mut dir.value {
    SimpleExpressionNode::new(Either3::C(value), context.source_text)
  } else {
    SimpleExpressionNode {
      content: String::from("true"),
      is_static: false,
      loc: SPAN,
      ast: None,
    }
  };

  if modifiers.contains(&"camel") {
    arg.content = camelize(&arg.content)
  }

  let modifier = if modifiers.contains(&"prop") {
    Some(String::from("."))
  } else if modifiers.contains(&"attr") {
    Some(String::from("^"))
  } else {
    None
  };

  Some(DirectiveTransformResult {
    key: arg,
    value: exp,
    runtime_camelize: false,
    modifier,
    handler: false,
    handler_modifiers: None,
    model: false,
    model_modifiers: None,
  })
}
