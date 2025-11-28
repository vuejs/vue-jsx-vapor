use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn should_convert_v_text_to_set_text() {
  let code = transform("<div v-text={str.value}></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_raise_error_and_ignore_children_when_v_text_is_present() {
  let error = RefCell::new(None);
  transform(
    "<div v-text={test}>hello</div>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VTextWithChildren));
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-text></div>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VTextNoExpression));
}
