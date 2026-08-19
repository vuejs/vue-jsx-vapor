use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler::transform;
use insta::assert_snapshot;

#[test]
fn should_convert_v_text_to_set_text() {
  let code = transform("<div v-text={str.value}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setText as _setText, template as _template, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_renderEffect(() => _setText(_x0, _toDisplayString(str.value)));
  	return _n0;
  })();
  "#);
}

#[test]
fn work_with_component() {
  let code = transform(r#"<Comp v-text={foo} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { toDisplayString as _toDisplayString } from "vue";
  (() => {
  	const _n0 = _createComponent(Comp, { textContent: () => _toDisplayString(foo) }, null, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn should_preserve_constant_component_values() {
  let code = transform(r#"<><Comp v-text={1} /><Comp v-text={() => 1} /></>"#, None).code;
  assert_snapshot!(code.replace('\t', "  "), @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const _n0 = _createComponent(Comp, { textContent: 1 });
    const _n1 = _createComponent(Comp, { textContent: () => () => 1 });
    return [_n0, _n1];
  })();
  "#)
}

#[test]
fn should_set_literal_text_at_runtime_for_raw_text_elements() {
  let code = transform(r#"<script v-text={'&<'} />"#, None).code;
  assert_snapshot!(code.replace('\t', "  "), @r#"
  import { setText as _setText, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<script> ", 1);
  (() => {
    const _n0 = _t0();
    const _x0 = _txt(_n0);
    _setText(_x0, "&<");
    return _n0;
  })();
  "#)
}

#[test]
fn should_raise_error_and_ignore_children_when_v_text_is_present() {
  let error = RefCell::new(None);
  let code = transform(
    "<Comp v-text={test}>hello</Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(*error.borrow(), Some(ErrorCodes::VTextWithChildren));
  assert!(code.contains("textContent"));
  assert!(!code.contains("hello"));
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
