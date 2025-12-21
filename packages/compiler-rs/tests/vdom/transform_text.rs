use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn no_consecutive_text() {
  let code = transform(
    r#"<>{foo}</>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(_Fragment, null, [foo], 64);
  })();
  "#);
}

#[test]
fn consecutive_text() {
  let code = transform(
    r#"<>{foo} bar {baz}</>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(_Fragment, null, [
      foo,
      _cache[0] || (_cache[0] = _createTextVNode(" bar ", -1)),
      baz
    ], 64);
  })();
  "#);
}

#[test]
fn consecutive_text_between_elements() {
  let code = transform(
    r#"<><div/>{foo} bar {baz}<div/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(_Fragment, null, [
      _cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1)),
      foo,
      _cache[1] || (_cache[1] = _createTextVNode(" bar ", -1)),
      baz,
      _cache[2] || (_cache[2] = _createElementVNode("div", null, null, -1))
    ], 64);
  })();
  "#);
}

#[test]
fn text_between_elements_static() {
  let code = transform(
    r#"<><div/>hello<div/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(_Fragment, null, [..._cache[0] || (_cache[0] = [
      _createElementVNode("div", null, null, -1),
      _createTextVNode("hello", -1),
      _createElementVNode("div", null, null, -1)
    ])], 64);
  })();
  "#);
}

#[test]
fn whitespace_text() {
  let code = transform(
    r#"<><div/>hello<div/>  <div/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(_Fragment, null, [..._cache[0] || (_cache[0] = [
      _createElementVNode("div", null, null, -1),
      _createTextVNode("hello", -1),
      _createElementVNode("div", null, null, -1),
      _createTextVNode(),
      _createElementVNode("div", null, null, -1)
    ])], 64);
  })();
  "#);
}

#[test]
fn consecutive_text_mixed_with_elements() {
  let code = transform(
    r#"<><div/>{ foo } bar { baz }<div/>hello<div/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(_Fragment, null, [
      _cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1)),
      foo,
      _cache[1] || (_cache[1] = _createTextVNode(" bar ", -1)),
      baz,
      _cache[2] || (_cache[2] = _createElementVNode("div", null, null, -1)),
      _cache[3] || (_cache[3] = _createTextVNode("hello", -1)),
      _cache[4] || (_cache[4] = _createElementVNode("div", null, null, -1))
    ], 64);
  })();
  "#);
}

#[test]
fn template_v_for() {
  let code = transform(
    r#"<template v-for={i in list}>foo</template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createTextVNode as _createTextVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))], 64))), 256);
  })();
  "#);
}

#[test]
fn element_with_custom_directives_and_only_one_text_child_node() {
  let code = transform(
    r#"<p v-foo>{foo}</p>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withDirectives as _withDirectives } from "vue";
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("p", null, foo)), [[vFoo]]);
  })();
  "#);
}
