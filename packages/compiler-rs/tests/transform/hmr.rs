use common::options::Hmr;
use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;
use napi::Either;

#[test]
pub fn export() {
  let code = transform(
    "export const foo = () => {}",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  export const foo = () => {};
  foo.__hmrId = "3b6957b69bea9439";
  __VUE_HMR_RUNTIME__.createRecord("3b6957b69bea9439", foo);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.foo.render ? "rerender" : "reload"](mod.foo.__hmrId, mod.foo.render || mod.foo);
  });
  "#);
}

#[test]
pub fn export_default() {
  let code = transform(
    "export default () => {}",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  const __default__ = () => {};
  export default __default__;
  __default__.__hmrId = "52164bac249078a3";
  __VUE_HMR_RUNTIME__.createRecord("52164bac249078a3", __default__);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  });
  "#);
}

#[test]
pub fn export_default_with_identifier() {
  let code = transform(
    "
    const Comp = () => {}
    export default Comp
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  const Comp = () => {};
  export default Comp;
  Comp.__hmrId = "52164bac249078a3";
  __VUE_HMR_RUNTIME__.createRecord("52164bac249078a3", Comp);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  });
  "#);
}

#[test]
pub fn export_default_with_function_declaration() {
  let code = transform(
    "
    export default function Comp() {}
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  export default function Comp() {}
  Comp.__hmrId = "52164bac249078a3";
  __VUE_HMR_RUNTIME__.createRecord("52164bac249078a3", Comp);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  });
  "#);
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
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  const Comp = () => {};
  function Comp1() {}
  export { Comp, Comp1 };
  export function Comp2() {}
  const __default__ = function() {};
  export default __default__;
  Comp.__hmrId = "8ed58763ca2bbfd5";
  __VUE_HMR_RUNTIME__.createRecord("8ed58763ca2bbfd5", Comp);
  Comp1.__hmrId = "f144a08cc37ed966";
  __VUE_HMR_RUNTIME__.createRecord("f144a08cc37ed966", Comp1);
  Comp2.__hmrId = "c36ea49ad2d3847e";
  __VUE_HMR_RUNTIME__.createRecord("c36ea49ad2d3847e", Comp2);
  __default__.__hmrId = "52164bac249078a3";
  __VUE_HMR_RUNTIME__.createRecord("52164bac249078a3", __default__);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  	__VUE_HMR_RUNTIME__[mod.Comp1.render ? "rerender" : "reload"](mod.Comp1.__hmrId, mod.Comp1.render || mod.Comp1);
  	__VUE_HMR_RUNTIME__[mod.Comp2.render ? "rerender" : "reload"](mod.Comp2.__hmrId, mod.Comp2.render || mod.Comp2);
  	__VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  });
  "#);
}

#[test]
pub fn exports_with_define_component() {
  let code = transform(
    "
    export const Comp = defineComponent(() => {})
    export default defineVaporComponent(() => {})
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  export const Comp = defineComponent(() => {});
  const __default__ = defineVaporComponent(() => {});
  export default __default__;
  Comp.__hmrId = "8ed58763ca2bbfd5";
  __VUE_HMR_RUNTIME__.createRecord("8ed58763ca2bbfd5", Comp);
  __default__.__hmrId = "52164bac249078a3";
  __VUE_HMR_RUNTIME__.createRecord("52164bac249078a3", __default__);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  	__VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  });
  "#);
}

#[test]
pub fn custom_define_component_name() {
  let code = transform(
    "
    export const Comp = createTemplate(() => {})
    export default createTemplate(() => {})
  ",
    Some(TransformOptions {
      hmr: Either::B(Hmr {
        define_component_name: vec![String::from("createTemplate")],
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  export const Comp = createTemplate(() => {});
  const __default__ = createTemplate(() => {});
  export default __default__;
  Comp.__hmrId = "8ed58763ca2bbfd5";
  __VUE_HMR_RUNTIME__.createRecord("8ed58763ca2bbfd5", Comp);
  __default__.__hmrId = "52164bac249078a3";
  __VUE_HMR_RUNTIME__.createRecord("52164bac249078a3", __default__);
  if (import.meta.hot) import.meta.hot.accept((mod) => {
  	__VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  	__VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  });
  "#);
}
