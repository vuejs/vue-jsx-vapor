use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
pub fn ssr_export() {
  let code = transform(
    "export const foo = () => {}",
    Some(TransformOptions {
      ssr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { ssrRegisterHelper } from "/vue-jsx-vapor/ssr";
  const __moduleId = "index.jsx";
  export const foo = () => {};
  ssrRegisterHelper(foo, __moduleId);
  "#);
}

#[test]
pub fn ssr_export_default() {
  let code = transform(
    "
    const Comp = () => {}
    export default Comp
    ",
    Some(TransformOptions {
      ssr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { ssrRegisterHelper } from "/vue-jsx-vapor/ssr";
  const __moduleId = "index.jsx";
  const Comp = () => {};
  export default Comp;
  ssrRegisterHelper(Comp, __moduleId);
  "#);
}

#[test]
pub fn ssr_define_vapor_component() {
  let code = transform(
    "const Comp = defineVaporComponent(() => <div />)",
    Some(TransformOptions {
      ssr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporSSRComponent as _defineVaporSSRComponent } from "/vue-jsx-vapor/vapor";
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  const Comp = defineVaporSSRComponent(() => (_openBlock(), _createBlock("div")));
  "#);
}
