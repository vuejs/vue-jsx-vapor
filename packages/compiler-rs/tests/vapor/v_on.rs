use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div onClick={handleClick}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", handleClick);
  	return _n0;
  })();
  "#);
}

#[test]
fn event_modifier() {
  let code = transform(
    "<>
      <a onClick_stop={handleEvent}></a>
      <form onSubmit_prevent={handleEvent}></form>
      <a onClick_stop_prevent={handleEvent}></a>
      <div onClick_self={handleEvent}></div>
      <div onClick_capture={handleEvent}></div>
      <a onClick_once={handleEvent}></a>
      <div onScroll_passive={handleEvent}></div>
      <input onClick_right={handleEvent} />
      <input onClick_left={handleEvent} />
      <input onClick_middle={handleEvent} />
      <input onClick_enter_right={handleEvent} />
      <input onKeyup_enter={handleEvent} />
      <input onKeyup_tab={handleEvent} />
      <input onKeyup_delete={handleEvent} />
      <input onKeyup_esc={handleEvent} />
      <input onKeyup_space={handleEvent} />
      <input onKeyup_up={handleEvent} />
      <input onKeyup_down={handleEvent} />
      <input onKeyup_left={handleEvent} />
      <input onKeyup_middle={submit} />
      <input onKeyup_middle_self={submit} />
      <input onKeyup_self_enter={handleEvent} />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<a>");
  const _t1 = _template("<form>");
  const _t2 = _template("<div>");
  const _t3 = _template("<input>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _t1();
  	const _n2 = _t0();
  	const _n3 = _t2();
  	const _n4 = _t2();
  	const _n5 = _t0();
  	const _n6 = _t2();
  	const _n7 = _t3();
  	const _n8 = _t3();
  	const _n9 = _t3();
  	const _n10 = _t3();
  	const _n11 = _t3();
  	const _n12 = _t3();
  	const _n13 = _t3();
  	const _n14 = _t3();
  	const _n15 = _t3();
  	const _n16 = _t3();
  	const _n17 = _t3();
  	const _n18 = _t3();
  	const _n19 = _t3();
  	const _n20 = _t3();
  	const _n21 = _t3();
  	_on(_n0, "click", _withModifiers(handleEvent, ["stop"]));
  	_on(_n1, "submit", _withModifiers(handleEvent, ["prevent"]));
  	_on(_n2, "click", _withModifiers(handleEvent, ["stop", "prevent"]));
  	_on(_n3, "click", _withModifiers(handleEvent, ["self"]));
  	_on(_n4, "click", handleEvent, { capture: true });
  	_on(_n5, "click", handleEvent, { once: true });
  	_on(_n6, "scroll", handleEvent, { passive: true });
  	_on(_n7, "contextmenu", _withModifiers(handleEvent, ["right"]));
  	_on(_n8, "click", _withModifiers(handleEvent, ["left"]));
  	_on(_n9, "mouseup", _withModifiers(handleEvent, ["middle"]));
  	_on(_n10, "contextmenu", _withModifiers(handleEvent, ["right"]));
  	_on(_n11, "keyup", _withKeys(handleEvent, ["enter"]));
  	_on(_n12, "keyup", _withKeys(handleEvent, ["tab"]));
  	_on(_n13, "keyup", _withKeys(handleEvent, ["delete"]));
  	_on(_n14, "keyup", _withKeys(handleEvent, ["esc"]));
  	_on(_n15, "keyup", _withKeys(handleEvent, ["space"]));
  	_on(_n16, "keyup", _withKeys(handleEvent, ["up"]));
  	_on(_n17, "keyup", _withKeys(handleEvent, ["down"]));
  	_on(_n18, "keyup", _withKeys(handleEvent, ["left"]));
  	_on(_n19, "keyup", _withModifiers(submit, ["middle"]));
  	_on(_n20, "keyup", _withModifiers(submit, ["middle", "self"]));
  	_on(_n21, "keyup", _withKeys(_withModifiers(handleEvent, ["self"]), ["enter"]));
  	return [
  		_n0,
  		_n1,
  		_n2,
  		_n3,
  		_n4,
  		_n5,
  		_n6,
  		_n7,
  		_n8,
  		_n9,
  		_n10,
  		_n11,
  		_n12,
  		_n13,
  		_n14,
  		_n15,
  		_n16,
  		_n17,
  		_n18,
  		_n19,
  		_n20,
  		_n21
  	];
  })();
  "#);
}

#[test]
fn should_error_if_no_expression_and_no_modifier() {
  let error = RefCell::new(None);
  transform(
    "<div onClick />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| *error.borrow_mut() = Some(e)),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VOnNoExpression.message()));
}

#[test]
fn should_not_error_if_no_expression_but_has_modifier() {
  let error = RefCell::new(None);
  let code = transform(
    "<div onClick_prevent />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| *error.borrow_mut() = Some(e)),
      ..Default::default()
    }),
  )
  .code;
  assert!(error.borrow().is_none());
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", _withModifiers(() => {}, ["prevent"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_multiple_modifiers_and_event_options() {
  let code = transform("<div onClick_stop_prevent_capture_once={test} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", _withModifiers(test, ["stop", "prevent"]), {
  		capture: true,
  		once: true
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_multiple_events_and_modifiers_options() {
  let code = transform("<div onClick_stop={test} onKeyup_enter={test} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", _withModifiers(test, ["stop"]));
  	_on(_n0, "keyup", _withKeys(test, ["enter"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_keyboard_events_or_dynamic_events() {
  let code = transform("<div onKeydown_stop_capture_ctrl_a={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "keydown", _withKeys(_withModifiers(test, ["stop", "ctrl"]), ["a"]), { capture: true });
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_wrap_keys_guard_if_no_key_modifier_is_present() {
  let code = transform("<div onKeyup_exact={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "keyup", _withModifiers(test, ["exact"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_static_key_event_with_left_or_right_modifiers() {
  let code = transform("<div onKeyup_left={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "keyup", _withKeys(test, ["left"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_transform_click_right() {
  let code = transform("<div onClick_right={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "contextmenu", _withModifiers(test, ["right"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_transform_click_middle() {
  let code = transform("<div onClick_middle={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "mouseup", _withModifiers(test, ["middle"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_use_direct_event_listener_by_default() {
  let code = transform("<div onClick={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", test);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_delegate_event_with_delegate_modifier() {
  let code = transform("<div onClick_delegate={test}/>", None).code;
  assert!(code.contains("_delegateEvents(\"click\")"));
  assert!(code.contains("delegateEvents as _delegateEvents"));
  assert!(!code.contains("on as _on"));
}

#[test]
fn should_delegate_empty_handler() {
  let error = RefCell::new(None);
  transform(
    "<div onClick_delegate />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| *error.borrow_mut() = Some(e)),
      ..Default::default()
    }),
  )
  .code;
  assert!(error.borrow().is_none());
}

#[test]
fn should_delegate_multiple_handlers_of_same_name() {
  let code = transform(
    "<div onClick_delegate_foo={a} onClick_delegate_bar={b} />",
    None,
  )
  .code;
  assert!(code.contains("_delegateEvents(\"click\")"));
  assert!(code.contains("delegate as _delegate"));
  assert!(code.contains("_delegate(_n0, \"click\", a)"));
  assert!(code.contains("_delegate(_n0, \"click\", b)"));
}

#[test]
fn should_fallback_to_direct_listener_with_event_options() {
  let code = transform("<div onClick_delegate_capture={test} />", None).code;
  assert!(code.contains("_on(_n0, \"click\", test, { capture: true })"));
  assert!(!code.contains("_delegateEvents"));
}

#[test]
fn should_fallback_to_direct_listener_for_unsupported_event() {
  let warning = RefCell::new(None);
  let code = transform(
    "<div onScroll_delegate={test} />",
    Some(TransformOptions {
      on_warn: Box::new(|message, _| {
        *warning.borrow_mut() = Some(message);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(
    warning.borrow().as_deref(),
    Some(
      ".delegate modifier is not supported on the \"scroll\" event. The listener will be attached directly."
    )
  );
  assert!(code.contains("_on(_n0, \"scroll\", test)"));
  assert!(!code.contains("_delegateEvents"));
}

#[test]
fn should_ignore_delegate_modifier_on_component_event() {
  let warning = RefCell::new(None);
  let code = transform(
    "<Comp onClick_delegate={test} />",
    Some(TransformOptions {
      on_warn: Box::new(|message, _| {
        *warning.borrow_mut() = Some(message);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(
    warning.borrow().as_deref(),
    Some(
      ".delegate modifier is only supported on native DOM elements. The modifier will be ignored."
    )
  );
  assert!(code.contains("{ onClick: () => test }"));
  assert!(!code.contains("_delegateEvents"));
}

#[test]
fn should_not_delegate_stop_when_have_multiple_events_of_same_name() {
  let code = transform("<div onClick_delegate={test} onClick_stop={test} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", test);
  	_on(_n0, "click", _withModifiers(test, ["stop"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_delegate_when_delegate_handler_uses_stop() {
  let code = transform("<div onClick_stop_delegate={test} />", None).code;
  assert!(code.contains("_on(_n0, \"click\", _withModifiers(test, [\"stop\"]))"));
  assert!(!code.contains("_delegateEvents"));
}

#[test]
fn should_not_delegate_normalized_static_event_when_sibling_uses_stop() {
  let code = transform(
    r#"<div onClick_right_delegate={test} onContextmenu_stop={test} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "contextmenu", _withModifiers(test, ["right"]));
  	_on(_n0, "contextmenu", _withModifiers(test, ["stop"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn namespace_event_with_component() {
  let code = transform("<Comp onUpdate:modelValue={() => {}} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, { "onUpdate:modelValue": () => () => {} }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn expression_with_type() {
  let code = transform("<div onClick={handleClick as any} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", handleClick as any);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_prioritize_right_over_middle_for_click_event_normalization() {
  let code = transform(
    "<><div onClick_middle_right={test}/><div onClick_right_middle={test}/></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _t0();
  	_on(_n0, "contextmenu", _withModifiers(test, ["middle", "right"]));
  	_on(_n1, "contextmenu", _withModifiers(test, ["right", "middle"]));
  	return [_n0, _n1];
  })();
  "#);
}

// vapor has no per-element vnode hooks, so a hook on a plain element can only
// register a listener that never fires. upstream (#15807) drops `@vue:*` there
// and warns; the port does the same, and also warns for `onVnodeXxx`, which
// resolves to the same event without the `vue:` prefix.
//
// upstream's guard is `tagType === ElementTypes.ELEMENT`, i.e. the tag is a
// known HTML/SVG tag. `!is_component` is the port's closest equivalent, so a
// component, a custom element (`<my-el>`, routed to the component path by
// `transform.rs`), a `<slot>`, and a `<template>` with structural directives
// are all left alone, while `<div>`/`<svg>`/`<input>` warn.
const VNODE_HOOK_MESSAGE: &str = "onVue:xxx / onVnodeXxx per-element lifecycle events are not supported in Vapor mode and will be ignored.";

fn vue_hook_warning(source: &str) -> Option<(String, usize)> {
  let warning: RefCell<Option<(String, usize)>> = RefCell::new(None);
  transform(
    source,
    Some(TransformOptions {
      on_warn: Box::new(|message, span| {
        *warning.borrow_mut() = Some((message, span.start as usize));
      }),
      ..Default::default()
    }),
  );
  warning.into_inner()
}

#[test]
fn warns_and_drops_vue_hooks_on_elements() {
  let cases = [
    "<div onVue:mounted={onMounted}>{bar}</div>",
    "<input onVue:beforeUnmount={onMounted} v-model={foo} />",
    "<svg v-for={item in items} onVue:updated={onMounted} />",
    "<textarea onVnodeUpdated={onMounted} />",
    // `onVnodeXxx` reaches the same event without the `vue:` prefix
    "<div onVnodeMounted={onMounted} />",
    // any `vue:` arg names a hook the element cannot host
    "<div onVue:whatever={onMounted} />",
  ];

  for source in cases {
    let (message, start) =
      vue_hook_warning(source).unwrap_or_else(|| panic!("no warning for {source}"));
    assert_eq!(message, VNODE_HOOK_MESSAGE, "{source}");
    // the span points at the attribute, not at the whole element
    assert!(
      source[start..].starts_with("onVue:") || source[start..].starts_with("onVnode"),
      "{source} @ {start}"
    );
  }
}

#[test]
fn keeps_vue_hooks_where_a_vnode_exists() {
  for source in [
    "<Comp onVue:mounted={onMounted} />",
    "<Comp onVnodeMounted={onMounted} />",
    "<slot onVue:mounted={onMounted} />",
    // custom elements go through the component path, like upstream's
    // `_createAssetComponent`, so they are not warned about either
    "<my-el onVue:mounted={onMounted} />",
    "<template v-if={ok} onVue:mounted={onMounted}><div /></template>",
  ] {
    assert!(vue_hook_warning(source).is_none(), "{source}");
  }
}

// dropping the listener is the only effect
#[test]
fn a_vue_hook_on_an_element_leaves_no_listener() {
  let code = transform("<div onVue:mounted={onMounted}>{bar}</div>", None).code;
  assert!(!code.contains("_on("));
  assert!(!code.contains("vnodeMounted"));
  assert!(!code.contains("onVnodeMounted"));
#[test]
fn camel_case_event_modifiers() {
  let code = transform(
    "<><div onClickCapture={a} onClickOnce={b} onScrollPassive={c} onClickStopPrevent={d} onClickRight={e}/><a onClickStop={f}/></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<a>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _t1();
  	_on(_n0, "click", a, { capture: true });
  	_on(_n0, "click", b, { once: true });
  	_on(_n0, "scroll", c, { passive: true });
  	_on(_n0, "click", _withModifiers(d, ["stop", "prevent"]));
  	_on(_n0, "contextmenu", _withModifiers(e, ["right"]));
  	_on(_n1, "click", _withModifiers(f, ["stop"]));
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn component_events_keep_option_suffixes_in_prop_names() {
  let code = transform(
    "<><Comp onClick={a} onClickCapture={b} /><Comp onClick_capture={c} onClick={d} /></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, {
  		onClickCapture: () => b,
  		onClick: () => a
  	});
  	const _n1 = _createComponent(Comp, {
  		onClick: () => d,
  		onClickCapture: () => c
  	});
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn component_events_keep_custom_camel_case_names() {
  let code = transform("<Comp onMoveLeft={handler} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, { onMoveLeft: () => handler }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn camel_case_modifiers_are_ignored_for_unknown_event_names() {
  let code = transform("<div onMoveLeft={handler} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "moveLeft", handler);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_delegate_normalized_camel_event_when_sibling_uses_stop() {
  let code = transform(
    r#"<div onContextmenu_delegate={a} onClickRightStop={b} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "contextmenu", a);
  	_on(_n0, "contextmenu", _withModifiers(b, ["right", "stop"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn camel_case_event_modifiers_normalize_middle() {
  let code = transform("<div onClickMiddleStop={handler} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "mouseup", _withModifiers(handler, ["middle", "stop"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn camel_case_event_modifiers_apply_to_dragexit_and_key_aliases() {
  let code = transform("<div onDragexitStop={a} onKeyupEnter={b} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "dragexit", _withModifiers(a, ["stop"]));
  	_on(_n0, "keyup", _withKeys(b, ["enter"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn camel_case_key_names_only_alias_on_keyboard_events() {
  let code = transform(
    "<div onDragEnter={a} onDropDown={b} onKeyupEnter={c} onKeydownUp={d} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "dragEnter", a);
  	_on(_n0, "dropDown", b);
  	_on(_n0, "keyup", _withKeys(c, ["enter"]));
  	_on(_n0, "keydown", _withKeys(d, ["up"]));
  	return _n0;
  })();
  "#);

// vapor has no per-element vnode hooks, so a hook on a plain element can only
// register a listener that never fires. upstream (#15807) drops `@vue:*` there
// and warns; the port does the same, and also warns for `onVnodeXxx`, which
// resolves to the same event without the `vue:` prefix.
//
// upstream's guard is `tagType === ElementTypes.ELEMENT`, i.e. the tag is a
// known HTML/SVG tag. `!is_component` is the port's closest equivalent, so a
// component, a custom element (`<my-el>`, routed to the component path by
// `transform.rs`), a `<slot>`, and a `<template>` with structural directives
// are all left alone, while `<div>`/`<svg>`/`<input>` warn.
const VNODE_HOOK_MESSAGE: &str = "onVue:xxx / onVnodeXxx per-element lifecycle events are not supported in Vapor mode and will be ignored.";

fn vue_hook_warning(source: &str) -> Option<(String, usize)> {
  let warning: RefCell<Option<(String, usize)>> = RefCell::new(None);
  transform(
    source,
    Some(TransformOptions {
      on_warn: Box::new(|message, span| {
        *warning.borrow_mut() = Some((message, span.start as usize));
      }),
      ..Default::default()
    }),
  );
  warning.into_inner()
}

#[test]
fn warns_and_drops_vue_hooks_on_elements() {
  let cases = [
    "<div onVue:mounted={onMounted}>{bar}</div>",
    "<input onVue:beforeUnmount={onMounted} v-model={foo} />",
    "<svg v-for={item in items} onVue:updated={onMounted} />",
    "<textarea onVnodeUpdated={onMounted} />",
    // `onVnodeXxx` reaches the same event without the `vue:` prefix
    "<div onVnodeMounted={onMounted} />",
    // any `vue:` arg names a hook the element cannot host
    "<div onVue:whatever={onMounted} />",
  ];

  for source in cases {
    let (message, start) =
      vue_hook_warning(source).unwrap_or_else(|| panic!("no warning for {source}"));
    assert_eq!(message, VNODE_HOOK_MESSAGE, "{source}");
    // the span points at the attribute, not at the whole element
    assert!(
      source[start..].starts_with("onVue:") || source[start..].starts_with("onVnode"),
      "{source} @ {start}"
    );
  }
}

#[test]
fn keeps_vue_hooks_where_a_vnode_exists() {
  for source in [
    "<Comp onVue:mounted={onMounted} />",
    "<Comp onVnodeMounted={onMounted} />",
    "<slot onVue:mounted={onMounted} />",
    // custom elements go through the component path, like upstream's
    // `_createAssetComponent`, so they are not warned about either
    "<my-el onVue:mounted={onMounted} />",
    "<template v-if={ok} onVue:mounted={onMounted}><div /></template>",
  ] {
    assert!(vue_hook_warning(source).is_none(), "{source}");
  }
}

// dropping the listener is the only effect
#[test]
fn a_vue_hook_on_an_element_leaves_no_listener() {
  let code = transform("<div onVue:mounted={onMounted}>{bar}</div>", None).code;
  assert!(!code.contains("_on("));
  assert!(!code.contains("vnodeMounted"));
  assert!(!code.contains("onVnodeMounted"));
}
