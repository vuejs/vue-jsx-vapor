use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn simple_expression() {
  let code = transform(
    r#"<div v-show={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vShow as _vShow, withDirectives as _withDirectives } from "vue";
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[_vShow, foo]]);
  })();
  "#)
}

#[test]
fn should_raise_errror_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-show />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VShowNoExpression));
}
