use common::{
  check::{is_delegated_events, is_jsx_component},
  directive::{Modifiers, resolve_modifiers},
  error::ErrorCodes,
  expression::SimpleExpressionNode,
};
use napi::bindgen_prelude::Either3;
use oxc_ast::ast::{JSXAttribute, JSXAttributeName, JSXElement};

use crate::{
  ir::index::{BlockIRNode, OperationNode, SetEventIRNode},
  transform::{DirectiveTransformResult, TransformContext},
};

pub fn transform_v_on<'a>(
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let is_component = is_jsx_component(node);

  let (name, name_loc) = match &dir.name {
    JSXAttributeName::Identifier(name) => (name.name.as_ref(), name.span),
    JSXAttributeName::NamespacedName(name) => {
      (name.span.source_text(context.source_text), name.span)
    }
  };
  let replaced = format!("{}{}", name[2..3].to_lowercase(), &name[3..]);
  let splited = replaced.split("_").collect::<Vec<_>>();
  let name_string = splited[0].to_string();
  let modifiers = splited[1..].to_vec();

  let value = &mut dir.value;
  if value.is_none() && modifiers.is_empty() {
    context.options.on_error.as_ref()(ErrorCodes::VOnNoExpression, dir.span);
  }

  let mut arg = SimpleExpressionNode {
    content: name_string.clone(),
    is_static: true,
    loc: name_loc,
    ast: None,
  };
  let exp = value
    .as_mut()
    .map(|value| SimpleExpressionNode::new(Either3::C(value), context.source_text));

  let Modifiers {
    keys: key_modifiers,
    non_keys: non_key_modifiers,
    options: event_option_modifiers,
  } = resolve_modifiers(&arg.content, modifiers);

  let is_static_click = arg.is_static && arg.content.to_lowercase() == "click";

  // normalize click.right and click.middle since they don't actually fire
  if non_key_modifiers
    .iter()
    .any(|modifier| modifier == "middle")
    && is_static_click
  {
    arg.content = "mouseup".to_string()
  }
  if non_key_modifiers.iter().any(|modifier| modifier == "right") && is_static_click {
    arg.content = "contextmenu".to_string();
  }

  if is_component {
    return Some(DirectiveTransformResult {
      key: arg,
      value: exp.unwrap_or_default(),
      handler: true,
      handler_modifiers: Some(Modifiers {
        keys: key_modifiers,
        non_keys: non_key_modifiers,
        options: event_option_modifiers,
      }),
      model: false,
      model_modifiers: None,
      modifier: None,
      runtime_camelize: false,
    });
  }

  // Only delegate if:
  // - no dynamic event name
  // - no event option modifiers (passive, capture, once)
  // - is a delegatable
  let delegate =
    arg.is_static && event_option_modifiers.is_empty() && is_delegated_events(arg.content.as_str());

  let element = context.reference(&mut context_block.dynamic);
  context.register_effect(
    context_block,
    context.is_operation(vec![&arg]),
    OperationNode::SetEvent(SetEventIRNode {
      set_event: true,
      element,
      value: exp,
      modifiers: Modifiers {
        keys: key_modifiers,
        non_keys: non_key_modifiers,
        options: event_option_modifiers,
      },
      delegate,
      effect: !arg.is_static,
      key: arg,
    }),
    None,
    None,
  );
  None
}
