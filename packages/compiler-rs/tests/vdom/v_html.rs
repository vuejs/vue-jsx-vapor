use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<span v-html={foo}></span>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["innerHTML"];
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createElementBlock("span", { innerHTML: foo }, null, 8, _hoisted_1);
  })();
  "#);
}
