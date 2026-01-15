use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn should_convert_v_text_to_set_text() {
  let code = transform("<div v-text={str.value}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { child as _child, renderEffect as _renderEffect, setText as _setText, template as _template, toDisplayString as _toDisplayString } from "vue";
  const t0 = _template("<div> </div>", true);
  (() => {
  	const n0 = t0();
  	const x0 = _child(n0);
  	_renderEffect(() => _setText(x0, _toDisplayString(str.value)));
  	return n0;
  })();
  "#);
}

#[test]
fn work_with_component() {
  let code = transform(r#"<Comp v-text={foo} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { renderEffect as _renderEffect, setBlockText as _setBlockText, toDisplayString as _toDisplayString } from "vue";
  (() => {
  	const n0 = _createComponent(Comp, null, null, true);
  	_renderEffect(() => _setBlockText(n0, _toDisplayString(foo)));
  	return n0;
  })();
  "#)
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
