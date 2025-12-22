use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_model() {
  let code = transform(
    r#"<input onClick_right={() => {}} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createElementBlock("input", { onContextmenu: _cache[0] || (_cache[0] = _withModifiers(() => {}, ["right"])) }, null, 32);
  })();
  "#)
}

#[test]
fn v_model_component() {
  let code = transform(
    r#"<Comp v-model:model_foo={foo.value} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, {
      model: foo.value,
      "onUpdate:model": _cache[0] || (_cache[0] = ($event) => foo.value = $event),
      modelModifiers$: { foo: true }
    }, null, 8, ["model"]);
  })();
  "#);
}
