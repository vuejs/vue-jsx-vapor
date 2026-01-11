use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div v-show={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyVShow as _applyVShow, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _applyVShow(n0, () => foo);
    return n0;
  })();
  "#);
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
