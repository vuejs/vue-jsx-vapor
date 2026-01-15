use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn text() {
  let code = transform(
    r#"<>foo</>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"("foo");"#);
}

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
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createBlock(_Fragment, null, [_normalizeVNode(() => foo)], 64);
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [
      _normalizeVNode(() => foo),
      _cache[0] || (_cache[0] = _normalizeVNode(" bar ", -1)),
      _normalizeVNode(() => baz)
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [
      _cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1)),
      _normalizeVNode(() => foo),
      _cache[1] || (_cache[1] = _normalizeVNode(" bar ", -1)),
      _normalizeVNode(() => baz),
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [..._cache[0] || (_cache[0] = [
      _createElementVNode("div", null, null, -1),
      _normalizeVNode("hello", -1),
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [..._cache[0] || (_cache[0] = [
      _createElementVNode("div", null, null, -1),
      _normalizeVNode("hello", -1),
      _createElementVNode("div", null, null, -1),
      _normalizeVNode(),
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [
      _cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1)),
      _normalizeVNode(() => foo),
      _cache[1] || (_cache[1] = _normalizeVNode(" bar ", -1)),
      _normalizeVNode(() => baz),
      _cache[2] || (_cache[2] = _createElementVNode("div", null, null, -1)),
      _cache[3] || (_cache[3] = _normalizeVNode("hello", -1)),
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))], 64))), 256);
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
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_foo = _resolveDirective("foo");
    return _withDirectives((_openBlock(), _createElementBlock("p", null, [_normalizeVNode(() => foo)])), [[_directive_foo]]);
  })();
  "#);
}

#[test]
fn condition_expression() {
  let code = transform(
    r#"<div>
      <div v-if={foo}/>
      {foo ? <div>{foo}bar</div> : bar}
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", null, [foo ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : _createCommentVNode("", true), foo ? (_openBlock(), _createElementBlock("div", { key: 1 }, [_normalizeVNode(() => foo), _cache[0] || (_cache[0] = _normalizeVNode("bar", -1))])) : (_openBlock(), _createBlock(_Fragment, { key: 2 }, [_normalizeVNode(bar)], 64))]);
  })();
  "#)
}

#[test]
fn logical_expression() {
  let code = transform(
    r#"<div>{foo && <div>{foo}</div>}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    let _temp;
    return _openBlock(), _createElementBlock("div", null, [(_temp = foo, _temp) ? (_openBlock(), _createElementBlock("div", { key: 0 }, [_normalizeVNode(() => foo)])) : _normalizeVNode((_openBlock(), _createBlock(_Fragment, { key: 1 }, [_normalizeVNode(_temp)], 64)))]);
  })();
  "#)
}

#[test]
fn logical_expression_or() {
  let code = transform(
    r#"<div>{foo || <div>{foo}</div>}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    let _temp;
    return _openBlock(), _createElementBlock("div", null, [(_temp = foo, _temp) ? _normalizeVNode((_openBlock(), _createBlock(_Fragment, { key: 1 }, [_normalizeVNode(_temp)], 64))) : (_openBlock(), _createElementBlock("div", { key: 0 }, [_normalizeVNode(() => foo)]))]);
  })();
  "#)
}

#[test]
fn logical_expression_coalesce() {
  let code = transform(
    r#"<div>{foo ?? <div>{foo}</div>}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    let _temp;
    return _openBlock(), _createElementBlock("div", null, [(_temp = foo, _temp == null) ? (_openBlock(), _createElementBlock("div", { key: 0 }, [_normalizeVNode(() => foo)])) : _normalizeVNode((_openBlock(), _createBlock(_Fragment, { key: 1 }, [_normalizeVNode(_temp)], 64)))]);
  })();
  "#)
}
