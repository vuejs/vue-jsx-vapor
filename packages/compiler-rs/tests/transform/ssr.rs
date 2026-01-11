use std::cell::RefCell;

use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
pub fn ssr_export() {
  let code = transform(
    "export const foo = () => {}",
    Some(TransformOptions {
      ssr: RefCell::new(true),
      in_ssr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { ssrRegisterHelper } from "/__vue-jsx-ssr-register-helper";
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
      ssr: RefCell::new(true),
      in_ssr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { ssrRegisterHelper } from "/__vue-jsx-ssr-register-helper";
  const __moduleId = "index.jsx";
  const Comp = () => {};
  export default Comp;
  ssrRegisterHelper(Comp, __moduleId);
  "#);
}
