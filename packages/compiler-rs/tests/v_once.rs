use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-once>
      { msg }
      <span class={clz} />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn as_root_node() {
  let code = transform("<div id={foo} v-once />", None).code;
  assert_snapshot!(code);
}

#[test]
fn on_nested_plain_element() {
  let code = transform("<div><div id={foo} v-once /></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn on_component() {
  let code = transform("<div><Comp id={foo} v-once /></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn inside_v_once() {
  let code = transform("<div v-once><div v-once/></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn with_v_if() {
  let code = transform("<div v-if={expr} v-once />", None).code;
  assert_snapshot!(code);
}

#[test]
fn with_v_if_else() {
  let code = transform("<><div v-if={expr} v-once /><p v-else/></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn with_conditional_expression() {
  let code = transform(
    "<div v-once>{ok? <span>{msg}</span> : <div>fail</div> }</div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn with_v_for() {
  let code = transform("<div v-for={i in list} v-once />", None).code;
  assert_snapshot!(code);
}

#[test]
fn execution_order() {
  let code = transform(
    "<div>
      <span v-once>{ foo }</span>
      { bar }<br/>
      { baz }
      <div foo={true}>{foo}</div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-show />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VShowNoExpression));
}
