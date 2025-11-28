use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn should_convert_v_html_to_inner_html() {
  let code = transform("<div v-html={code.value}></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_raise_error_and_ignore_children_when_v_html_is_present() {
  let error = RefCell::new(None);
  transform(
    "<div v-html={test.value}>hello</div>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VHtmlWithChildren));
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-html></div>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VHtmlNoExpression));
}
