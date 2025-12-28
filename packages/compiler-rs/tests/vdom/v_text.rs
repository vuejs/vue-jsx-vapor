use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn should_convert_v_text_to_text_content() {
  let code = transform(
    r#"<div v-text={test}>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, toDisplayString as _toDisplayString } from "vue";
  const _hoisted_1 = ["textContent"];
  (() => {
    return _openBlock(), _createElementBlock("div", { textContent: _toDisplayString(test) }, null, 8, _hoisted_1);
  })();
  "#)
}

#[test]
fn should_raise_error_if_has_children() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-text={test}>hello</div>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(*error.borrow(), Some(ErrorCodes::VTextWithChildren));
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-text></div>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(*error.borrow(), Some(ErrorCodes::VTextNoExpression));
}
