use compiler_rs::transform::{TransformOptions, transform};
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}
