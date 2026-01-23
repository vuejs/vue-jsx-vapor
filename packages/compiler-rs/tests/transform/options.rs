use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
pub fn export() {
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
  const t0 = _template("<div> </div>", true);
  (() => {
  	const n0 = t0();
  	const x0 = _txt(n0);
  	_setNodes(x0, () => foo);
  	return n0;
  })();
  "#);
}
