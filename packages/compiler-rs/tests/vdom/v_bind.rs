use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<div id={id}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["id"];
  _openBlock(), _createElementBlock("div", { id }, null, 8, _hoisted_1);
  "#);
}

#[test]
fn no_expression() {
  let code = transform(
    r#"<div id />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { id: true };
  _openBlock(), _createElementBlock("div", _hoisted_1);
  "#);
}

#[test]
fn empty_expression() {
  let code = transform("<div foo={}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    return n0;
  })();
  "#);
}

#[test]
fn shoud_not_error_if_empty_expression() {
  let code = transform(
    r#"<div arg="" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { arg: "" };
  _openBlock(), _createElementBlock("div", _hoisted_1);
  "#);
}

#[test]
fn camel_modifier() {
  let code = transform(
    r#"<div foo-bar_camel={id} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["fooBar"];
  _openBlock(), _createElementBlock("div", { fooBar: id }, null, 8, _hoisted_1);
  "#);
}

#[test]
fn prop_modifier() {
  let code = transform(
    r#"<div foo-bar_prop={id} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = [".foo-bar"];
  _openBlock(), _createElementBlock("div", { ".foo-bar": id }, null, 40, _hoisted_1);
  "#);
}

#[test]
fn attr_modifier() {
  let code = transform(
    r#"<div foo-bar_attr={id} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["^foo-bar"];
  _openBlock(), _createElementBlock("div", { "^foo-bar": id }, null, 8, _hoisted_1);
  "#);
}
