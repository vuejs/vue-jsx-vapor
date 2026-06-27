use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
pub fn runtime_module_name() {
  let code = transform(
    "<div>{foo}</div>",
    Some(TransformOptions {
      runtime_module_name: Some(String::from("vue-jsx-vapor")),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "vue-jsx-vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo);
  	return _n0;
  })();
  "#);
}

#[test]
pub fn filename() {
  let code = transform(
    "<div>{foo}</div>",
    Some(TransformOptions {
      filename: "routes/index.tsx?tsr-split=component",
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo);
  	return _n0;
  })();
  "#);
}

#[test]
pub fn optimize_slots() {
  let code = transform(
    "<Comp>{foo}</Comp>",
    Some(TransformOptions {
      interop: true,
      optimize: false,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeSlots as _normalizeSlots } from "/vue-jsx-vapor/vdom";
  import { createVNode as _createVNode } from "vue";
  _createVNode(Comp, null, _normalizeSlots(foo));
  "#);
}
