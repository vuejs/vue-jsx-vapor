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
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtclick = handleClick;
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
  _delegateEvents("click", "contextmenu", "keyup", "mouseup");
  import { delegateEvents as _delegateEvents, on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<a></a>");
  const _t1 = _template("<form></form>");
  const _t2 = _template("<div></div>");
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
  	_n0.$evtclick = _withModifiers(handleEvent, ["stop"]);
  	_on(_n1, "submit", _withModifiers(handleEvent, ["prevent"]));
  	_n2.$evtclick = _withModifiers(handleEvent, ["stop", "prevent"]);
  	_n3.$evtclick = _withModifiers(handleEvent, ["self"]);
  	_on(_n4, "click", handleEvent, { capture: true });
  	_on(_n5, "click", handleEvent, { once: true });
  	_on(_n6, "scroll", handleEvent, { passive: true });
  	_n7.$evtcontextmenu = _withModifiers(handleEvent, ["right"]);
  	_n8.$evtclick = _withModifiers(handleEvent, ["left"]);
  	_n9.$evtmouseup = _withModifiers(handleEvent, ["middle"]);
  	_n10.$evtcontextmenu = _withKeys(_withModifiers(handleEvent, ["right"]), ["enter"]);
  	_n11.$evtkeyup = _withKeys(handleEvent, ["enter"]);
  	_n12.$evtkeyup = _withKeys(handleEvent, ["tab"]);
  	_n13.$evtkeyup = _withKeys(handleEvent, ["delete"]);
  	_n14.$evtkeyup = _withKeys(handleEvent, ["esc"]);
  	_n15.$evtkeyup = _withKeys(handleEvent, ["space"]);
  	_n16.$evtkeyup = _withKeys(handleEvent, ["up"]);
  	_n17.$evtkeyup = _withKeys(handleEvent, ["down"]);
  	_n18.$evtkeyup = _withKeys(handleEvent, ["left"]);
  	_n19.$evtkeyup = _withModifiers(submit, ["middle"]);
  	_n20.$evtkeyup = _withModifiers(submit, ["middle", "self"]);
  	_n21.$evtkeyup = _withKeys(_withModifiers(handleEvent, ["self"]), ["enter"]);
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
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtclick = _withModifiers(() => {}, ["prevent"]);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_multiple_modifiers_and_event_options() {
  let code = transform("<div onClick_stop_prevent_capture_once={test} />", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
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
  _delegateEvents("click", "keyup");
  import { delegateEvents as _delegateEvents, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtclick = _withModifiers(test, ["stop"]);
  	_n0.$evtkeyup = _withKeys(test, ["enter"]);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_keyboard_events_or_dynamic_events() {
  let code = transform("<div onKeydown_stop_capture_ctrl_a={test}/>", None).code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
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
  _delegateEvents("keyup");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtkeyup = _withModifiers(test, ["exact"]);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_static_key_event_with_left_or_right_modifiers() {
  let code = transform("<div onKeyup_left={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("keyup");
  import { delegateEvents as _delegateEvents, template as _template, withKeys as _withKeys } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtkeyup = _withKeys(test, ["left"]);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_transform_click_right() {
  let code = transform("<div onClick_right={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("contextmenu");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtcontextmenu = _withModifiers(test, ["right"]);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_transform_click_middle() {
  let code = transform("<div onClick_middle={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("mouseup");
  import { delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtmouseup = _withModifiers(test, ["middle"]);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_delegate_event() {
  let code = transform("<div onClick={test}/>", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegateEvents as _delegateEvents, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtclick = test;
  	return _n0;
  })();
  "#);
}

#[test]
fn should_use_delegate_helper_when_have_multiple_events_of_same_name() {
  let code = transform("<div onClick={test} onClick_stop={test} />", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegate as _delegate, delegateEvents as _delegateEvents, template as _template, withModifiers as _withModifiers } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_delegate(_n0, "click", test);
  	_delegate(_n0, "click", _withModifiers(test, ["stop"]));
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
  _delegateEvents("click");
  import { delegateEvents as _delegateEvents, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_n0.$evtclick = handleClick as any;
  	return _n0;
  })();
  "#);
}
