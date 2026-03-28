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
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
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

#[test]
fn starts_with_underline() {
  let code = transform(
    r#"<div _id_prop={id} __id_prop="" v-model:$_value_value$={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, normalizeProps as _normalizeProps, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["._id"];
  (() => {
  	const _cache = _createVNodeCache("631d214bc2c8427c");
  	return _openBlock(), _createElementBlock("div", _normalizeProps({
  		"._id": id,
  		".__id": "",
  		[_value.value]: model,
  		["onUpdate:" + _value.value]: _cache[0] || (_cache[0] = ($event) => model = $event)
  	}), null, 48, _hoisted_1);
  })();
  "#);
}

#[test]
fn prevent_hoisted_expression_with_this() {
  let code = transform(
    r#"<div class={this.foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, normalizeClass as _normalizeClass, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", { class: _normalizeClass(this.foo) }, null, 2);
  "#);
}

#[test]
fn prevent_cache_expression_with_this() {
  let code = transform(
    r#"<div onMousedown={this.onMousedown}>{this.foo}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onMousedown"];
  _openBlock(), _createElementBlock("div", { onMousedown: this.onMousedown }, [_normalizeVNode(() => this.foo)], 40, _hoisted_1);
  "#);
}

#[test]
fn jsx_in_expression_container() {
  let code = transform(
    r#"<><div foo={<div />} /></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { foo: (_openBlock(), _createElementBlock("div")) };
  _openBlock(), _createElementBlock(_Fragment, null, [(_openBlock(), _createElementBlock("div", _hoisted_1))], 64);
  "#);
}
