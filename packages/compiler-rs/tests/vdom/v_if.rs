use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic_v_if() {
  let code = transform(
    r#"<div v-if={ok}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : _createCommentVNode("", true);
  "#);
}

#[test]
fn template_v_if() {
  let code = transform(
    r#"<template v-if={ok}><div/>hello<p/></template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, ok ? (_openBlock(), _createBlock(_Fragment, { key: 0 }, [..._cache[0] || (_cache[0] = [
      _createElementVNode("div", null, null, -1),
      _normalizeVNode("hello", -1),
      _createElementVNode("p", null, null, -1)
    ])], 64)) : _createCommentVNode("", true));
  })();
  "#);
}

#[test]
fn component_v_if() {
  let code = transform(
    r#"<Component v-if={ok}></Component>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createCommentVNode as _createCommentVNode, openBlock as _openBlock } from "vue";
  ok ? (_openBlock(), _createBlock(Component, { key: 0 })) : _createCommentVNode("", true);
  "#);
}

#[test]
fn v_if_v_else() {
  let code = transform(
    r#"<><div v-if={ok}/><p v-else/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  const _hoisted_2 = { key: 1 };
  _openBlock(), _createBlock(_Fragment, null, [ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : (_openBlock(), _createElementBlock("p", _hoisted_2))], 64);
  "#);
}

#[test]
fn v_if_v_else_if() {
  let code = transform(
    r#"<><div v-if={ok}/><p v-else-if={orNot}/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  const _hoisted_2 = { key: 1 };
  _openBlock(), _createBlock(_Fragment, null, [ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : orNot ? (_openBlock(), _createElementBlock("p", _hoisted_2)) : _createCommentVNode("", true)], 64);
  "#);
}

#[test]
fn v_if_v_else_if_v_else() {
  let code = transform(
    r#"<><div v-if={ok}/><p v-else-if={orNot}/><template v-else>fine</template></>"#,
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
  const _hoisted_2 = { key: 1 };
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : orNot ? (_openBlock(), _createElementBlock("p", _hoisted_2)) : (_openBlock(), _createBlock(_Fragment, { key: 2 }, [_cache[0] || (_cache[0] = _normalizeVNode("fine", -1))], 64))], 64);
  })();
  "#);
}

#[test]
fn comment_between_branches() {
  let code = transform(
    r#"<>
      div v-if={ok}/>
      <!--foo-->
      <p v-else-if={orNot}/>
      <!--bar-->
      <template v-else>fine</template>
    </>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @"");
}

#[test]
fn error_on_v_else_missing_adjacent_v_if() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-else/>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn error_on_v_else_if_missing_adjacent_v_if_or_v_else_if() {
  let error = RefCell::new(None);
  transform(
    r#"<div v-else-if={foo}/>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn error_on_adjacent_v_else() {
  let error = RefCell::new(None);
  transform(
    r#"<><div v-if={false}/><div v-else/><div v-else/></>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn user_key() {
  let code = transform(
    r#"<><div v-if={ok} key={a + 1} /><div v-else key={a + 1} /></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createBlock(_Fragment, null, [ok ? (_openBlock(), _createElementBlock("div", { key: a + 1 })) : (_openBlock(), _createElementBlock("div", { key: a + 1 }))], 64);
  "#)
}

#[test]
fn multiple_v_if_that_are_sibling_nodes_should_have_different_keys() {
  let code = transform(
    r#"<><div v-if={ok}/><p v-if={orNot}/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  const _hoisted_2 = { key: 1 };
  _openBlock(), _createBlock(_Fragment, null, [ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : _createCommentVNode("", true), orNot ? (_openBlock(), _createElementBlock("p", _hoisted_2)) : _createCommentVNode("", true)], 64);
  "#)
}

#[test]
fn increasing_key_v_if_v_else_if_v_else() {
  let code = transform(
    r#"<><div v-if={ok}/><p v-else/><div v-if={another}/><p v-else-if={orNot}/><p v-else/></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  const _hoisted_2 = { key: 1 };
  const _hoisted_3 = { key: 2 };
  const _hoisted_4 = { key: 3 };
  const _hoisted_5 = { key: 4 };
  _openBlock(), _createBlock(_Fragment, null, [ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : (_openBlock(), _createElementBlock("p", _hoisted_2)), another ? (_openBlock(), _createElementBlock("div", _hoisted_3)) : orNot ? (_openBlock(), _createElementBlock("p", _hoisted_4)) : (_openBlock(), _createElementBlock("p", _hoisted_5))], 64);
  "#)
}

#[test]
fn key_injection_only_v_bind() {
  let code = transform(
    r#"<div v-if={ok} {...obj}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, guardReactiveProps as _guardReactiveProps, mergeProps as _mergeProps, normalizeProps as _normalizeProps, openBlock as _openBlock } from "vue";
  ok ? (_openBlock(), _createElementBlock("div", _mergeProps({ key: 0 }, obj), null, 16)) : _createCommentVNode("", true);
  "#)
}

#[test]
fn key_injection_before_v_bind() {
  let code = transform(
    r#"<div v-if={ok} id="foo" {...obj}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  ok ? (_openBlock(), _createElementBlock("div", _mergeProps({
    key: 0,
    id: "foo"
  }, obj), null, 16)) : _createCommentVNode("", true);
  "#)
}

#[test]
fn key_injection_after_v_bind() {
  let code = transform(
    r#"<div v-if={ok} {...obj} id="foo"/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  ok ? (_openBlock(), _createElementBlock("div", _mergeProps({ key: 0 }, obj, { id: "foo" }), null, 16)) : _createCommentVNode("", true);
  "#)
}

#[test]
fn key_injection_custom_directive() {
  let code = transform(
    r#"<div v-if={ok} v-foo />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { key: 0 };
  (() => {
    const _directive_foo = _resolveDirective("foo");
    return ok ? _withDirectives((_openBlock(), _createElementBlock("div", _hoisted_1, null, 512)), [[_directive_foo]]) : _createCommentVNode("", true);
  })();
  "#)
}

#[test]
fn avoid_duplicate_keys() {
  let code = transform(
    r#"<div v-if={ok} key="custom_key" {...obj}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  ok ? (_openBlock(), _createElementBlock("div", _mergeProps({ key: "custom_key" }, obj), null, 16)) : _createCommentVNode("", true);
  "#)
}

#[test]
fn with_spaces_between_branches() {
  let code = transform(
    r#"<><div v-if={ok}/> <div v-else-if={no}/> <div v-else/></>"#,
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
  const _hoisted_2 = { key: 1 };
  const _hoisted_3 = { key: 2 };
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, [
      ok ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : no ? (_openBlock(), _createElementBlock("div", _hoisted_2)) : (_openBlock(), _createElementBlock("div", _hoisted_3)),
      _cache[0] || (_cache[0] = _normalizeVNode()),
      _cache[1] || (_cache[1] = _normalizeVNode())
    ], 64);
  })();
  "#)
}

#[test]
fn with_comments() {
  let code = transform(
    r#"<template v-if={ok}>
      {/*comment1*/}
      <div v-if={ok2}>
        {/*comment2*/}
      </div>
      {/*comment3*/}
      <b v-else/>
      {/*comment4*/}
      <p/>
    </template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { key: 0 };
  const _hoisted_2 = { key: 1 };
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(_Fragment, null, ok ? (_openBlock(), _createBlock(_Fragment, { key: 0 }, [ok2 ? (_openBlock(), _createElementBlock("div", _hoisted_1)) : (_openBlock(), _createElementBlock("b", _hoisted_2)), _cache[0] || (_cache[0] = _createElementVNode("p", null, null, -1))], 64)) : _createCommentVNode("", true));
  })();
  "#)
}

#[test]
fn v_on_with_v_if() {
  let code = transform(
    r#"<button v-on={{ click: clickEvent }} v-if={true}>w/ v-if</button>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock, toHandlers as _toHandlers } from "vue";
  true ? (_openBlock(), _createElementBlock("button", _mergeProps({ key: 0 }, _toHandlers({ click: clickEvent }, true)), "w/ v-if", 16)) : _createCommentVNode("", true);
  "#)
}
