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
      _cache.0 || (_cache.0 = _createElementVNode("span", null, "bar", -1)),
      baz
    ], 8, _hoisted_1);
  })();
  "#);
}
