use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn should_convert_v_html_to_inner_html() {
  let code = transform(
    r#"<div v-html={test}>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["innerHTML"];
  (() => {
    return _openBlock(), _createElementBlock("div", { innerHTML: test }, null, 8, _hoisted_1);
  })();
  "#);
}

#[test]
fn should_raise_error_if_has_children() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-html={test}> </div>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(*error.borrow(), Some(ErrorCodes::VHtmlWithChildren));
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-html></div>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(*error.borrow(), Some(ErrorCodes::VHtmlNoExpression));
}
