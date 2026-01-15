use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-example></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[_directive_example]]);
  })();
  "#);
}

#[test]
fn binding_value() {
  let code = transform(
    "<div v-example={msg}></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[_directive_example, msg]]);
  })();
  "#);
}

#[test]
fn static_parameters() {
  let code = transform(
    "<div v-example:foo={msg}></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
      _directive_example,
      msg,
      foo
    ]]);
  })();
  "#);
}

#[test]
fn modifiers() {
  let code = transform(
    "<div v-example_bar={msg}></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
      _directive_example,
      msg,
      void 0,
      { bar: true }
    ]]);
  })();
  "#);
}

#[test]
fn modifiers_with_binding() {
  let code = transform(
    "<div v-example_foo-bar></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
      _directive_example,
      void 0,
      void 0,
      { foo-bar: true }
    ]]);
  })();
  "#);
}

#[test]
fn static_argument_and_modifiers() {
  let code = transform(
    "<div v-example:foo_bar={msg}></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
      _directive_example,
      msg,
      foo,
      { bar: true }
    ]]);
  })();
  "#);
}

#[test]
fn dynamic_argument() {
  let code = transform(
    "<div v-example:$foo$={msg}></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
    const _directive_example = _resolveDirective("example");
    return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
      _directive_example,
      msg,
      foo
    ]]);
  })();
  "#);
}

#[test]
fn component() {
  let code = transform(
    "<Comp v-test>
      <div v-if={true}>
        <Bar v-hello_world />
      </div>
    </Comp>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, resolveDirective as _resolveDirective, withCtx as _withCtx, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { key: 0 };
  (() => {
    const _directive_test = _resolveDirective("test");
    const _directive_hello = _resolveDirective("hello");
    return _withDirectives((_openBlock(), _createBlock(Comp, null, {
      default: _withCtx(() => [true ? (_openBlock(), _createElementBlock("div", _hoisted_1, [_withDirectives(_createVNode(Bar, null, null, 512), [[
        _directive_hello,
        void 0,
        void 0,
        { world: true }
      ]])])) : _createCommentVNode("", true)]),
      _: 1
    })), [[_directive_test]]);
  })();
  "#);
}

#[test]
fn none_resolve_directive() {
  let code = transform(
    "<div vExample={msg}></div>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withDirectives as _withDirectives } from "vue";
  _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
    vExample,
    msg,
    vExample
  ]]);
  "#);
}
