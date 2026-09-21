use std::borrow::Cow;

use common::{
  check::{is_keyboard_event, is_simple_identifier},
  directive::{Modifiers, resolve_modifiers},
  error::ErrorCodes,
  text::capitalize,
};
use oxc_ast::{
  NONE,
  ast::{
    Expression, FormalParameterKind, JSXAttribute, JSXAttributeName, JSXAttributeValue,
    PropertyKind,
  },
};
use oxc_span::SPAN;

use crate::transform::{DirectiveTransformResult, TransformContext};

pub fn transform_v_on<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = &context.ast;

  let (name, name_loc) = match &dir.name {
    JSXAttributeName::Identifier(name) => (name.name.as_ref(), name.span),
    JSXAttributeName::NamespacedName(name) => {
      (name.span.source_text(context.source_text), name.span)
    }
  };
  let replaced = format!("{}{}", name[2..3].to_lowercase(), &name[3..]);
  let mut splited = replaced.split("_").collect::<Vec<_>>();
  let mut event_name = Cow::Borrowed(splited.remove(0));
  let modifiers = splited;

  let value = &mut dir.value;
  if value.is_none() && modifiers.is_empty() {
    context.options.on_error.as_ref()(ErrorCodes::VOnNoExpression, dir.span);
  }

  if event_name.starts_with("vue:") {
    event_name = Cow::Owned(format!(
      "vnode{}",
      capitalize(Cow::Borrowed(&event_name[4..]))
    ));
  }

  let mut should_cache =
    context.options.optimize && value.is_none() && !*context.options.in_v_once.borrow();
  let mut has_jsx = false;
  // handler processing
  let mut exp = if let Some(JSXAttributeValue::ExpressionContainer(value)) = value {
    let (exp, has_scope_ref, has_this, _has_jsx) =
      context.process_expression(value.expression.to_expression_mut());
    has_jsx = _has_jsx;
    should_cache = context.options.optimize
      && !(exp.is_member_expression() || matches!(exp, Expression::Identifier(_)))
      && !(*context.options.in_v_once.borrow() || has_scope_ref || has_this);
    exp
  } else {
    ast.expression_arrow_function(
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
    )
  };

  if !modifiers.is_empty() {
    let Modifiers {
      keys: key_modifiers,
      non_keys: non_key_modifiers,
      options: event_option_modifiers,
    } = resolve_modifiers(&event_name, modifiers);

    let is_static_click = event_name == "click";

    // normalize click.right and click.middle since they don't actually fire
    if non_key_modifiers
      .iter()
      .any(|modifier| modifier == "middle")
      && is_static_click
    {
      event_name = Cow::Borrowed("mouseup")
    }
    if non_key_modifiers.iter().any(|modifier| modifier == "right") && is_static_click {
      event_name = Cow::Borrowed("contextmenu");
    }

    if !non_key_modifiers.is_empty() {
      exp = ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(context.options.helper("_withModifiers"))),
        NONE,
        ast.vec_from_array([
          exp.into(),
          ast
            .expression_array(
              SPAN,
              ast.vec_from_iter(non_key_modifiers.iter().map(|modifier| {
                ast
                  .expression_string_literal(SPAN, ast.str(modifier), None)
                  .into()
              })),
            )
            .into(),
        ]),
        false,
      )
    }

    if !key_modifiers.is_empty() && is_keyboard_event(&event_name) {
      exp = ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(context.options.helper("_withKeys"))),
        NONE,
        ast.vec_from_array([
          exp.into(),
          ast
            .expression_array(
              SPAN,
              ast.vec_from_iter(key_modifiers.into_iter().map(|key| {
                ast
                  .expression_string_literal(SPAN, ast.str(&key), None)
                  .into()
              })),
            )
            .into(),
        ]),
        false,
      );
    }

    if !event_option_modifiers.is_empty() {
      let modifier_postfix = event_option_modifiers
        .into_iter()
        .map(capitalize)
        .collect::<Vec<_>>()
        .join("");
      event_name = Cow::Owned(format!("{}{}", event_name, modifier_postfix));
    }
  }

  if should_cache {
    exp = context.cache(exp, false, false, false)
  }

  let mut on_event_name = format!("on{}", capitalize(event_name));
  if !is_simple_identifier(&on_event_name) {
    on_event_name = format!("\"{}\"", on_event_name);
  }
  Some(DirectiveTransformResult {
    props: vec![ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(name_loc, ast.str(&on_event_name)),
      exp,
      false,
      false,
      false,
    )],
    runtime: None,
    has_jsx,
  })
}
