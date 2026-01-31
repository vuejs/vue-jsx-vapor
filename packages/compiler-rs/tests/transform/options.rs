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
  const _t0 = _template("<div> ", true);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo);
  	return _n0;
  })();
  "#);
}
