use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<Transition>
      <h1 v-show={show}>foo</h1>
    </Transition>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, vShow as _vShow, withCtx as _withCtx, withDirectives as _withDirectives } from "vue";
  _openBlock(), _createBlock(Transition, { persisted: true }, {
    default: _withCtx(() => [_withDirectives(_createElementVNode("h1", null, "foo", 512), [[_vShow, show]])]),
    _: 2
  }, 1024);
  "#);
}

#[test]
fn v_show_with_appear() {
  let code = transform(
    r#"<Transition appear onAppear={() => {}}>
      <h1 v-show={show}>foo</h1>
    </Transition>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, vShow as _vShow, withCtx as _withCtx, withDirectives as _withDirectives } from "vue";
  _openBlock(), _createBlock(Transition, {
    appear: true,
    onAppear: () => {},
    persisted: true
  }, {
    default: _withCtx(() => [_withDirectives(_createElementVNode("h1", null, "foo", 512), [[_vShow, show]])]),
    _: 2
  }, 1024);
  "#);
}

#[test]
fn work_with_v_if() {
  let code = transform(
    r#"<Transition>
      <h1 v-if={show}>foo</h1>
    </Transition>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  const _hoisted_1 = { key: 0 };
  _openBlock(), _createBlock(Transition, null, {
    default: _withCtx(() => [show ? (_openBlock(), _createElementBlock("h1", _hoisted_1, "foo")) : _createCommentVNode("", true)]),
    _: 2
  }, 1024);
  "#);
}

#[test]
fn transition_work_with_dynamic_keyed_children() {
  let code = transform(
    "<Transition>
      <h1 key={foo}>foo</h1>
    </Transition>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  _openBlock(), _createBlock(Transition, null, {
    default: _withCtx(() => [(_openBlock(), _createElementBlock("h1", { key: foo }, "foo"))]),
    _: 2
  }, 1024);
  "#);
}
