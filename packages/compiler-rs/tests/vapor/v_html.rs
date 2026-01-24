use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn should_convert_v_html_to_inner_html() {
  let code = transform("<div v-html={code.value}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setHtml as _setHtml, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setHtml(_n0, code.value));
  	return _n0;
  })();
  "#);
}

#[test]
fn work_with_component() {
  let code = transform("<Comp v-html={code.value} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { renderEffect as _renderEffect, setBlockHtml as _setBlockHtml } from "vue";
  (() => {
  	const _n0 = _createComponent(Comp, null, null, true);
  	_renderEffect(() => _setBlockHtml(_n0, code.value));
  	return _n0;
  })();
  "#);
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
