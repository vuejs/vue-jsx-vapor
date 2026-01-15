use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn as_root_node() {
  let code = transform(
    r#"<div id={foo} v-once />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementVNode as _createElementVNode, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div", { id: foo }, null, 8, ["id"])).cacheIndex = 0, _setBlockTracking(1), _cache[0]);
  })();
  "#)
}

#[test]
fn on_nested_plain_element() {
  let code = transform(
    r#"<div><div id={foo} v-once /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div", { id: foo }, null, 8, ["id"])).cacheIndex = 0, _setBlockTracking(1), _cache[0])]);
  })();
  "#)
}

#[test]
fn on_component() {
  let code = transform(
    r#"<div><Comp id={foo} v-once /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createVNode(Comp, { id: foo }, null, 8, ["id"])).cacheIndex = 0, _setBlockTracking(1), _cache[0])]);
  })();
  "#)
}

#[test]
fn inside_v_once() {
  // v-once inside v-once should not be cached
  let code = transform(
    r#"<div v-once><div v-once/></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementVNode as _createElementVNode, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div", null, [_createElementVNode("div")])).cacheIndex = 0, _setBlockTracking(1), _cache[0]);
  })();
  "#)
}

#[test]
fn with_hoist_static() {
  // cached nodes should be ignored by hoistStatic transform
  let code = transform(
    r#"<div><div v-once /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div")).cacheIndex = 0, _setBlockTracking(1), _cache[0])]);
  })();
  "#)
}

#[test]
fn with_v_if_else() {
  let code = transform(
    r#"<><div v-if={BOOLEAN} v-once /><p v-else/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, setBlockTracking as _setBlockTracking } from "vue";
  const _hoisted_1 = { key: 1 };
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(_Fragment, null, [BOOLEAN ? _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div")).cacheIndex = 0, _setBlockTracking(1), _cache[0]) : (_openBlock(), _createElementBlock("p", _hoisted_1))], 64);
  })();
  "#)
}

#[test]
fn with_v_for() {
  let code = transform(
    r#"<div v-for={i in list} v-once />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createVNode(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div"))), 256)).cacheIndex = 0, _setBlockTracking(1), _cache[0]);
  })();
  "#)
}
