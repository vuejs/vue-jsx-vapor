use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div onClick={handleClick}></div>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegateEvents as _delegateEvents, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtclick = handleClick;
  	return n0;
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
  _delegateEvents("click", "contextmenu", "keyup", "mouseup");
  import { delegateEvents as _delegateEvents, on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<a></a>");
  const t1 = _template("<form></form>");
  const t2 = _template("<div></div>");
  const t3 = _template("<input>");
  (() => {
  	const n0 = t0();
  	const n1 = t1();
  	const n2 = t0();
  	const n3 = t2();
  	const n4 = t2();
  	const n5 = t0();
  	const n6 = t2();
  	const n7 = t3();
  	const n8 = t3();
  	const n9 = t3();
  	const n10 = t3();
  	const n11 = t3();
  	const n12 = t3();
  	const n13 = t3();
  	const n14 = t3();
  	const n15 = t3();
  	const n16 = t3();
  	const n17 = t3();
  	const n18 = t3();
  	const n19 = t3();
  	const n20 = t3();
  	const n21 = t3();
  	n0.$evtclick = _withModifiers(handleEvent, ["stop"]);
  	_on(n1, "submit", _withModifiers(handleEvent, ["prevent"]));
  	n2.$evtclick = _withModifiers(handleEvent, ["stop", "prevent"]);
  	n3.$evtclick = _withModifiers(handleEvent, ["self"]);
  	_on(n4, "click", handleEvent, { capture: true });
  	_on(n5, "click", handleEvent, { once: true });
  	_on(n6, "scroll", handleEvent, { passive: true });
  	n7.$evtcontextmenu = _withModifiers(handleEvent, ["right"]);
  	n8.$evtclick = _withModifiers(handleEvent, ["left"]);
  	n9.$evtmouseup = _withModifiers(handleEvent, ["middle"]);
  	n10.$evtcontextmenu = _withKeys(_withModifiers(handleEvent, ["right"]), ["enter"]);
  	n11.$evtkeyup = _withKeys(handleEvent, ["enter"]);
  	n12.$evtkeyup = _withKeys(handleEvent, ["tab"]);
  	n13.$evtkeyup = _withKeys(handleEvent, ["delete"]);
  	n14.$evtkeyup = _withKeys(handleEvent, ["esc"]);
  	n15.$evtkeyup = _withKeys(handleEvent, ["space"]);
  	n16.$evtkeyup = _withKeys(handleEvent, ["up"]);
  	n17.$evtkeyup = _withKeys(handleEvent, ["down"]);
  	n18.$evtkeyup = _withKeys(handleEvent, ["left"]);
  	n19.$evtkeyup = _withModifiers(submit, ["middle"]);
  	n20.$evtkeyup = _withModifiers(submit, ["middle", "self"]);
  	n21.$evtkeyup = _withKeys(_withModifiers(handleEvent, ["self"]), ["enter"]);
  	return [
  		n0,
  		n1,
  		n2,
  		n3,
  		n4,
  		n5,
  		n6,
  		n7,
  		n8,
  		n9,
  		n10,
  		n11,
  		n12,
  		n13,
  		n14,
  		n15,
  		n16,
  		n17,
  		n18,
  		n19,
  		n20,
  		n21
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
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VOnNoExpression));
}

#[test]
fn should_not_error_if_no_expression_but_has_modifier() {
  let error = RefCell::new(None);
  let code = transform(
    "<div onClick_prevent />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert!(error.borrow().is_none());
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtclick = _withModifiers(() => {}, ["prevent"]);
  	return n0;
  })();
  "#);
}

#[test]
fn should_support_multiple_modifiers_and_event_options() {
  let code = transform("<div onClick_stop_prevent_capture_once={test} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_on(n0, "click", _withModifiers(test, ["stop", "prevent"]), {
  		capture: true,
  		once: true
  	});
  	return n0;
  })();
  "#);
}

#[test]
fn should_support_multiple_events_and_modifiers_options() {
  let code = transform("<div onClick_stop={test} onKeyup_enter={test} />", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click", "keyup");
  import { delegateEvents as _delegateEvents, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtclick = _withModifiers(test, ["stop"]);
  	n0.$evtkeyup = _withKeys(test, ["enter"]);
  	return n0;
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_keyboard_events_or_dynamic_events() {
  let code = transform("<div onKeydown_stop_capture_ctrl_a={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_on(n0, "keydown", _withKeys(_withModifiers(test, ["stop", "ctrl"]), ["a"]), { capture: true });
  	return n0;
  })();
  "#);
}

#[test]
fn should_not_wrap_keys_guard_if_no_key_modifier_is_present() {
  let code = transform("<div onKeyup_exact={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("keyup");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtkeyup = _withModifiers(test, ["exact"]);
  	return n0;
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_static_key_event_with_left_or_right_modifiers() {
  let code = transform("<div onKeyup_left={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("keyup");
  import { delegateEvents as _delegateEvents, template as _template, withKeys as _withKeys } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtkeyup = _withKeys(test, ["left"]);
  	return n0;
  })();
  "#);
}

#[test]
fn should_transform_click_right() {
  let code = transform("<div onClick_right={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("contextmenu");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtcontextmenu = _withModifiers(test, ["right"]);
  	return n0;
  })();
  "#);
}

#[test]
fn should_transform_click_middle() {
  let code = transform("<div onClick_middle={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("mouseup");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtmouseup = _withModifiers(test, ["middle"]);
  	return n0;
  })();
  "#);
}

#[test]
fn should_delegate_event() {
  let code = transform("<div onClick={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegateEvents as _delegateEvents, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtclick = test;
  	return n0;
  })();
  "#);
}

#[test]
fn should_use_delegate_helper_when_have_multiple_events_of_same_name() {
  let code = transform("<div onClick={test} onClick_stop={test} />", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegate as _delegate, delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_delegate(n0, "click", test);
  	_delegate(n0, "click", _withModifiers(test, ["stop"]));
  	return n0;
  })();
  "#);
}

#[test]
fn namespace_event_with_component() {
  let code = transform("<Comp onUpdate:modelValue={() => {}} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const n0 = _createComponent(Comp, { "onUpdate:modelValue": () => () => {} }, null, true);
  	return n0;
  })();
  "#);
}

#[test]
fn expression_with_type() {
  let code = transform("<div onClick={handleClick as any} />", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegateEvents as _delegateEvents, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	n0.$evtclick = handleClick as any;
  	return n0;
  })();
  "#);
}
