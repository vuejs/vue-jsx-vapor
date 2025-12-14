use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_model() {
  let code = transform(
    r#"<input v-model_bar={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onUpdate:modelValue"];
  (() => {
    const _cache = _useVdomCache();
    return _withDirectives((_openBlock(), _createElementBlock("input", { "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => foo = $event) }, null, 8, _hoisted_1)), [[
      _vModelText,
      foo,
      void 0,
      { bar: true }
    ]]);
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
    }, null, 8, [
      "model",
      "onUpdate:model",
      "modelModifiers$"
    ]);
  })();
  "#);
}
