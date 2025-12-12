use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<template v-for={{ x, y } in list} key={x}>
      <span>foobar</span>
    </template>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList, resolveComponent as _resolveComponent } from "vue";
  (() => {
    const _cache = _useVdomCache();
    const _component__Fragment = _resolveComponent("_Fragment");
    return _openBlock(true), _createElementBlock("_Fragment", null, _renderList(list, ({ x, y }) => (_openBlock(), _createBlock(_component__Fragment, { key: x }, [_createElementVNode("span", null, "foobar")]))), 128);
  })();
  "#);
}
