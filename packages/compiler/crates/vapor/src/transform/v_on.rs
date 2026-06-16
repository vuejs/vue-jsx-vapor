use std::borrow::Cow;

use common::{
  check::is_keyboard_event,
  directive::{Directives, Modifiers, resolve_modifiers},
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
};
use oxc_ast::{
  NONE,
  ast::{Expression, FormalParameterKind, JSXAttribute, JSXAttributeName},
};
use oxc_span::SPAN;

use crate::{
  ir::index::{BlockIRNode, OperationNode, SetEventIRNode},
  transform::{DirectiveTransformResult, TransformContext},
};

pub fn transform_v_on<'a>(
  directives: &Directives,
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = context.ast;
  let is_component = directives.is_component;

  let (name, name_loc) = match &dir.name {
    JSXAttributeName::Identifier(name) => (name.name.as_ref(), name.span),
    JSXAttributeName::NamespacedName(name) => {
      (name.span.source_text(context.source_text), name.span)
    }
  };
  let replaced = format!("{}{}", name[2..3].to_lowercase(), &name[3..]);
  let splited = replaced.split("_").collect::<Vec<_>>();
  let name_string = splited[0];
  let modifiers = splited[1..].to_vec();

  let value = &mut dir.value;
  if value.is_none() && modifiers.is_empty() {
    context.options.on_error.as_ref()(ErrorCodes::VOnNoExpression, dir.span);
  }

  let mut arg = ast.alloc_string_literal(
    name_loc,
    if let Some(name) = name_string.strip_prefix("vue:") {
      let mut s = String::with_capacity(5 + name.len());
      s.push_str("vnode");
      let mut chars = name.chars();
      if let Some(c) = chars.next() {
        s.push_str(&c.to_ascii_uppercase().to_string());
      }
      s.push_str(chars.as_str());
      ast.str(&s)
    } else {
      ast.str(name_string)
    },
    None,
  );

  let exp = value
    .as_mut()
    .map(|value| jsx_attribute_value_to_expression(value, ast))
    .unwrap_or(ast.expression_arrow_function(
      SPAN,
      false,
      false,
      NONE,
      ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ast.vec(),
        NONE,
      ),
      NONE,
      ast.function_body(SPAN, ast.vec(), ast.vec()),
    ));

  let Modifiers {
    keys: mut key_modifiers,
    non_keys: non_key_modifiers,
    options: event_option_modifiers,
  } = resolve_modifiers(&arg.value, modifiers);

  let is_static_click = arg.value == "click";

  // normalize click.right and click.middle since they don't actually fire
  if non_key_modifiers.iter().any(|modifier| modifier == "right") && is_static_click {
    arg.value = ast.str("contextmenu");
  } else if non_key_modifiers
    .iter()
    .any(|modifier| modifier == "middle")
    && is_static_click
  {
    arg.value = ast.str("mouseup");
  }

  // don't gen keys guard for non-keyboard events
  // if event name is dynamic, always wrap with keys guard
  if !key_modifiers.is_empty() && !is_keyboard_event(&arg.value) {
    key_modifiers.clear();
  }

  let modifiers = Modifiers {
    keys: key_modifiers
      .into_iter()
      .map(|m| Cow::Owned(Cow::into_owned(m)))
      .collect::<Vec<_>>(),
    non_keys: non_key_modifiers
      .into_iter()
      .map(|m| Cow::Owned(Cow::into_owned(m)))
      .collect::<Vec<_>>(),
    options: event_option_modifiers
      .into_iter()
      .map(|m| Cow::Owned(Cow::into_owned(m)))
      .collect::<Vec<_>>(),
  };

  if is_component {
    return Some(DirectiveTransformResult {
      key: Expression::StringLiteral(arg),
      value: exp,
      handler: true,
      handler_modifiers: Some(modifiers),
      model: false,
      model_modifiers: None,
      modifier: None,
      runtime_camelize: false,
    });
  }

  let element = context.reference(&mut context_block.dynamic);
  context.register_operation(
    context_block,
    OperationNode::SetEvent(SetEventIRNode {
      set_event: true,
      element,
      value: exp,
      modifiers,
      effect: false,
      key: Expression::StringLiteral(arg),
    }),
    None,
  );
  None
}
