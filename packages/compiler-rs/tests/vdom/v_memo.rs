use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn on_root_element() {
  let code = transform(
    r#"<div v-memo={[x]}></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withMemo as _withMemo } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _withMemo([x], () => (_openBlock(), _createElementBlock("div")), _cache, 0);
  })();
  "#)
}

#[test]
fn on_normal_element() {
  let code = transform(
    r#"<div v-memo={[x]}></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withMemo as _withMemo } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _withMemo([x], () => (_openBlock(), _createElementBlock("div")), _cache, 0);
  })();
  "#)
}

#[test]
fn on_component() {
  let code = transform(
    r#"<Comp v-memo={[x]}></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withMemo as _withMemo } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _withMemo([x], () => (_openBlock(), _createBlock(Comp)), _cache, 0);
  })();
  "#)
}

#[test]
fn on_v_if() {
  let code = transform(
    r#"<>
      <div v-if={ok} v-memo={[x]}><span>foo</span>bar</div>
      <Comp v-else v-memo={[x]}></Comp>
    </>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock, withCtx as _withCtx, withMemo as _withMemo } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [ok ? _withMemo([x], () => (_openBlock(), _createElementBlock("div", null, [_createElementVNode("span", null, "foo"), _normalizeVNode("bar")])), _cache, 0) : _withMemo([x], () => _createVNode(Comp), _cache, 1)], 64);
  })();
  "#)
}

#[test]
fn on_v_for() {
  let code = transform(
    r#"<div v-for={{ x, y } in list} key={x} v-memo={[x, y === z]}>
      <span>foobar</span>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, isMemoSame as _isMemoSame, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, ({ x, y }, __, ___, _cached) => {
      const _memo = [x, y === z];
      if (_cached && _cached.key === x && _isMemoSame(_cached, _memo)) return _cached;
      const _item = (_openBlock(), _createElementBlock("div", { key: x }, [_cache[0] || (_cache[0] = _createElementVNode("span", null, "foobar", -1))]));
      _item.memo = _memo;
      return _item;
    }, _cache, 1), 128);
  })();
  "#)
}

#[test]
fn on_template_v_for() {
  let code = transform(
    r#"<template v-for={{ x, y } in list} key={x} v-memo={[x, y === z]}>
      <span>foobar</span>
    </template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, isMemoSame as _isMemoSame, openBlock as _openBlock, renderList as _renderList, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, ({ x, y }, __, ___, _cached) => {
      const _memo = [x, y === z];
      if (_cached && _cached.key === x && _isMemoSame(_cached, _memo)) return _cached;
      const _item = (_openBlock(), _createBlock(_Fragment, { key: x }, [_cache[0] || (_cache[0] = _createElementVNode("span", null, "foobar", -1))], 64));
      _item.memo = _memo;
      return _item;
    }, _cache, 1), 128);
  })();
  "#)
}

#[test]
fn element_v_for_key_expression_v_memo() {
  let code = transform(
    r#"<span v-for={data in tableData} key={getId(data)} v-memo={getLetter(data)}></span>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, isMemoSame as _isMemoSame, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(tableData, (data, __, ___, _cached) => {
      const _memo = getLetter(data);
      if (_cached && _cached.key === getId(data) && _isMemoSame(_cached, _memo)) return _cached;
      const _item = (_openBlock(), _createElementBlock("span", { key: getId(data) }));
      _item.memo = _memo;
      return _item;
    }, _cache, 0), 128);
  })();
  "#)
}
