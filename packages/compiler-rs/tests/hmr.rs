use compiler_rs::transform::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
pub fn export() {
  let code = transform(
    "export const foo = () => {}",
    Some(TransformOptions {
      hmr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}

#[test]
pub fn export_default() {
  let code = transform(
    "export default () => {}",
    Some(TransformOptions {
      hmr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}

#[test]
pub fn export_default_with_identifier() {
  let code = transform(
    "
    const Comp = () => {}
    export default Comp
  ",
    Some(TransformOptions {
      hmr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}

#[test]
pub fn export_default_with_function_declaration() {
  let code = transform(
    "
    export default function Comp() {}
  ",
    Some(TransformOptions {
      hmr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}

#[test]
pub fn exports() {
  let code = transform(
    "
    const Comp = () => {}
    function Comp1 () {}
    export { Comp, Comp1 }
    export function Comp2() {}
    export default function() {}
  ",
    Some(TransformOptions {
      hmr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}

#[test]
pub fn exports_with_define_component() {
  let code = transform(
    "
    export const Comp = defineComponent(() => {})
    export default defineVaporComponent(() => {})
  ",
    Some(TransformOptions {
      hmr: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}
