use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"export default () => <div foo={foo}>
      <Comp foo-bar_attr={bar} />
      <span>bar</span>
      {baz}
    </div>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock, resolveComponent as _resolveComponent } from "vue";
  const _hoisted_1 = ["foo"];
  export default () => (() => {
    const _cache = _useVdomCache();
    const _component_Comp = _resolveComponent("Comp");
    return _openBlock(), _createElementBlock("div", { foo }, [
      _createVNode(_component_Comp, { "^foo-bar": bar }, null, 8, ["^foo-bar"]),
      _cache[0] || (_cache[0] = _createElementVNode("span", null, "bar", -1)),
      baz
    ], 8, _hoisted_1);
  })();
  "#);
}

#[test]
fn jsx_expression() {
  let code = transform(
    r#"<div>
      foo
      {<Foo />}
    </div>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, createTextVNode as _createTextVNode, openBlock as _openBlock, resolveComponent as _resolveComponent } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createTextVNode("foo", -1)), (() => {
      const _component_Foo = _resolveComponent("Foo");
      return _openBlock(), _createBlock(_component_Foo);
    })()]);
  })();
  "#);
}

#[test]
fn jsx_fragment() {
  let code = transform(
    r#"<><span /></>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))], 64);
  })();
  "#)
}
