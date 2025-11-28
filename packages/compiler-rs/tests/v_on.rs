use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div onClick={handleClick}></div>", None).code;
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}

#[test]
fn should_support_multiple_modifiers_and_event_options() {
  let code = transform("<div onClick_stop_prevent_capture_once={test} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_multiple_events_and_modifiers_options() {
  let code = transform("<div onClick_stop={test} onKeyup_enter={test} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_wrap_keys_guard_for_keyboard_events_or_dynamic_events() {
  let code = transform("<div onKeydown_stop_capture_ctrl_a={test}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_not_wrap_keys_guard_if_no_key_modifier_is_present() {
  let code = transform("<div onKeyup_exact={test}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_wrap_keys_guard_for_static_key_event_with_left_or_right_modifiers() {
  let code = transform("<div onKeyup_left={test}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_transform_click_right() {
  let code = transform("<div onClick_right={test}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_transform_click_middle() {
  let code = transform("<div onClick_middle={test}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_delegate_event() {
  let code = transform("<div onClick={test}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_use_delegate_helper_when_have_multiple_events_of_same_name() {
  let code = transform("<div onClick={test} onClick_stop={test} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn namespace_event_with_component() {
  let code = transform("<Comp onUpdate:modelValue={() => {}} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn expression_with_type() {
  let code = transform("<div onClick={handleClick as any} />", None).code;
  assert_snapshot!(code);
}
