use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_show() {
  let code = transform(
    r#"<div v-show={foo}>show</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementBlock as _createElementBlock, createTextVNode as _createTextVNode, openBlock as _openBlock, vShow as _vShow, withDirectives as _withDirectives } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _withDirectives((_openBlock(), _createElementBlock("div", null, _cache[0] || (_cache[0] = _createTextVNode("show", -1)), 512)), [[_vShow, foo]]);
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
