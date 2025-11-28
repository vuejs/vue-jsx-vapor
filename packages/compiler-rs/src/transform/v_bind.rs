use napi::bindgen_prelude::Either3;
use oxc_ast::ast::{JSXAttribute, JSXAttributeName, JSXElement};
use oxc_span::SPAN;

use crate::{
  ir::index::{BlockIRNode, SimpleExpressionNode},
  transform::{DirectiveTransformResult, TransformContext},
  utils::{check::is_reserved_prop, text::camelize},
};

pub fn transform_v_bind<'a>(
  dir: &'a mut JSXAttribute<'a>,
  _: &JSXElement,
  context: &'a TransformContext<'a>,
  _: &mut BlockIRNode,
) -> Option<DirectiveTransformResult<'a>> {
  let name_string = match &dir.name {
    JSXAttributeName::Identifier(name) => &name.name.to_string(),
    JSXAttributeName::NamespacedName(_) => return None,
  };
  let name_splited: Vec<&str> = name_string.split("_").collect();
  let modifiers = name_splited[1..].to_vec();
  let name_string = name_splited[0].to_string();

  let exp = if let Some(value) = &mut dir.value {
    SimpleExpressionNode::new(Either3::C(value), context)
  } else {
    SimpleExpressionNode {
      content: String::from("true"),
      is_static: false,
      loc: SPAN,
      ast: None,
    }
  };

  let mut arg = SimpleExpressionNode {
    content: name_string,
    is_static: true,
    loc: SPAN,
    ast: None,
  };
  if is_reserved_prop(&arg.content) {
    return None;
  }

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
    runtime_camelize: Some(false),
    modifier,
    handler: None,
    handler_modifiers: None,
    model: None,
    model_modifiers: None,
  })
}
