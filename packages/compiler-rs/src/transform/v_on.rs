use napi::{
  Either,
  bindgen_prelude::{Either3, Either16},
};
use oxc_ast::ast::{JSXAttribute, JSXAttributeName, JSXElement};
use oxc_span::SPAN;
use phf::phf_set;

use crate::{
  ir::index::{BlockIRNode, Modifiers, SetEventIRNode, SimpleExpressionNode},
  transform::{DirectiveTransformResult, TransformContext},
  utils::{check::is_jsx_component, error::ErrorCodes},
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
      (name.span.source_text(context.ir.borrow().source), name.span)
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
    .map(|value| SimpleExpressionNode::new(Either3::C(value), context));

  let Modifiers {
    keys: key_modifiers,
    non_keys: non_key_modifiers,
    options: event_option_modifiers,
  } = resolve_modifiers(
    if arg.is_static {
      Either::B(format!("on{}", name_string))
    } else {
      Either::A(&arg)
    },
    modifiers
      .iter()
      .map(|modifier| SimpleExpressionNode {
        content: modifier.to_string(),
        is_static: false,
        loc: SPAN,
        ast: None,
      })
      .collect(),
  );

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
      handler: Some(true),
      handler_modifiers: Some(Modifiers {
        keys: key_modifiers,
        non_keys: non_key_modifiers,
        options: event_option_modifiers,
      }),
      model: None,
      model_modifiers: None,
      modifier: None,
      runtime_camelize: None,
    });
  }

  // Only delegate if:
  // - no dynamic event name
  // - no event option modifiers (passive, capture, once)
  // - is a delegatable
  let delegate = arg.is_static
    && event_option_modifiers.is_empty()
    && DELEGATED_EVENTS.contains(arg.content.as_str());

  let element = context.reference(&mut context_block.dynamic);
  context.register_effect(
    context_block,
    context.is_operation(vec![&arg]),
    Either16::H(SetEventIRNode {
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

static DELEGATED_EVENTS: phf::Set<&'static str> = phf_set! {
    "beforeinput",
    "click",
    "dblclick",
    "contextmenu",
    "focusin",
    "focusout",
    "input",
    "keydown",
    "keyup",
    "mousedown",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "pointerdown",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "touchend",
    "touchmove",
    "touchstart"
};

fn is_event_option_modifier(modifier: &str) -> bool {
  matches!(modifier, "passive" | "once" | "capture")
}

fn is_non_key_modifier(modifier: &str) -> bool {
  matches!(
    modifier,
    // event propagation management
    "stop" | "prevent" | "self" |
    // system modifiers + exact
    "ctrl" | "shift" | "alt" | "meta" | "exact" |
    // mouse
    "middle"
  )
}

// left & right could be mouse or key modifiers based on event type
fn maybe_key_modifier(modifier: &str) -> bool {
  matches!(modifier, "left" | "right")
}

fn is_keyboard_event(key: &str) -> bool {
  matches!(key, "onkeyup" | "onkeydown" | "onkeypress")
}

pub fn resolve_modifiers(
  key: Either<&SimpleExpressionNode, String>,
  modifiers: Vec<SimpleExpressionNode>,
) -> Modifiers {
  let mut key_modifiers: Vec<String> = vec![];
  let mut non_key_modifiers: Vec<String> = vec![];
  let mut event_option_modifiers: Vec<String> = vec![];

  for modifier in modifiers {
    let modifier = modifier.content;
    if is_event_option_modifier(&modifier) {
      // eventOptionModifiers: modifiers for addEventListener() options,
      // e.g. .passive & .capture
      event_option_modifiers.push(modifier);
    } else {
      let key_string = match &key {
        Either::A(node) => {
          if node.is_static {
            &node.content
          } else {
            ""
          }
        }
        Either::B(string) => string,
      };

      // runtimeModifiers: modifiers that needs runtime guards
      if maybe_key_modifier(&modifier) {
        if !key_string.is_empty() {
          if is_keyboard_event(&key_string.to_lowercase()) {
            key_modifiers.push(modifier);
          } else {
            non_key_modifiers.push(modifier)
          }
        } else {
          key_modifiers.push(modifier.clone());
          non_key_modifiers.push(modifier)
        }
      } else if is_non_key_modifier(&modifier) {
        non_key_modifiers.push(modifier)
      } else {
        key_modifiers.push(modifier)
      }
    }
  }

  Modifiers {
    keys: key_modifiers,
    non_keys: non_key_modifiers,
    options: event_option_modifiers,
  }
}
