use compiler_rs::transform::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "const A = defineComponent(() => {
      defineVaporComponent(() => <span />)
      return () => <div />
    })
    const B = defineVaporComponent(() => {
    const C = defineComponent(() => <div />)
    const D = <>{foo} <div /></>
    return <div />
    })",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}
