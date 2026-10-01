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
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(foo.__hmrId = "3b6957b69bea9439", foo);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.foo) __VUE_HMR_RUNTIME__[mod.foo.render ? "rerender" : "reload"](mod.foo.__hmrId, mod.foo.render || mod.foo);
  	});
  }
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
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  const __default__ = _defineVaporHmrComponent(() => {});
  export default __default__;
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}
#[test]
pub fn export_default_with_identifier() {
  let code = transform(
    "\n    const Comp = () => {}\n    export default Comp\n  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  const Comp = _defineVaporHmrComponent(() => {});
  export default Comp;
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "52164bac249078a3", Comp);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn export_default_with_function_declaration() {
  let code = transform(
    "\n    export default function Comp() {}\n  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  export default function Comp() {}
  Comp = _defineVaporHmrComponent(Comp);
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "52164bac249078a3", Comp);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn exports() {
  let code = transform(
    "\n    const Comp = () => {}\n    function Comp1 () {}\n    export { Comp, Comp1 }\n    export function Comp2() {}\n    export default function() {}\n  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  const Comp = _defineVaporHmrComponent(() => {});
  function Comp1() {}
  Comp1 = _defineVaporHmrComponent(Comp1);
  export { Comp, Comp1 };
  export function Comp2() {}
  Comp2 = _defineVaporHmrComponent(Comp2);
  const __default__ = _defineVaporHmrComponent(function() {});
  export default __default__;
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(Comp1.__hmrId = "f144a08cc37ed966", Comp1);
  	__VUE_HMR_RUNTIME__.createRecord(Comp2.__hmrId = "c36ea49ad2d3847e", Comp2);
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.Comp1) __VUE_HMR_RUNTIME__[mod.Comp1.render ? "rerender" : "reload"](mod.Comp1.__hmrId, mod.Comp1.render || mod.Comp1);
  		if (mod.Comp2) __VUE_HMR_RUNTIME__[mod.Comp2.render ? "rerender" : "reload"](mod.Comp2.__hmrId, mod.Comp2.render || mod.Comp2);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn function_components_get_a_stable_shim() {
  let code = transform(
    "export const Arrow = () => {}
    Decl()
    export function Decl() {}
    const Named = () => {}
    export { Named }
    export default function () {}
    export const Defined = defineVaporComponent(() => {})
    ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  export const Arrow = _defineVaporHmrComponent(() => {});
  Decl();
  export function Decl() {}
  Decl = _defineVaporHmrComponent(Decl);
  const Named = _defineVaporHmrComponent(() => {});
  export { Named };
  const __default__ = _defineVaporHmrComponent(function() {});
  export default __default__;
  export const Defined = defineVaporComponent(() => {});
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Arrow.__hmrId = "787edc77c0d75d87", Arrow);
  	__VUE_HMR_RUNTIME__.createRecord(Decl.__hmrId = "31d4e20a213cbc11", Decl);
  	__VUE_HMR_RUNTIME__.createRecord(Named.__hmrId = "16232d353d5e663b", Named);
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	__VUE_HMR_RUNTIME__.createRecord(Defined.__hmrId = "becca27b30f74606", Defined);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Arrow) __VUE_HMR_RUNTIME__[mod.Arrow.render ? "rerender" : "reload"](mod.Arrow.__hmrId, mod.Arrow.render || mod.Arrow);
  		if (mod.Decl) __VUE_HMR_RUNTIME__[mod.Decl.render ? "rerender" : "reload"](mod.Decl.__hmrId, mod.Decl.render || mod.Decl);
  		if (mod.Named) __VUE_HMR_RUNTIME__[mod.Named.render ? "rerender" : "reload"](mod.Named.__hmrId, mod.Named.render || mod.Named);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  		if (mod.Defined) __VUE_HMR_RUNTIME__[mod.Defined.render ? "rerender" : "reload"](mod.Defined.__hmrId, mod.Defined.render || mod.Defined);
  	});
  }
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
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
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
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn mixed_declarations() {
  let code = transform(
    "
    export const Comp = () => {}, Defined = defineVaporComponent(() => {})
    const Other = () => {}, Kept = defineComponent({ setup: () => () => {} })
    export { Other, Kept }
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  export const Comp = _defineVaporHmrComponent(() => {}), Defined = defineVaporComponent(() => {});
  const Other = _defineVaporHmrComponent(() => {}), Kept = defineComponent({ setup: () => () => {} });
  export { Other, Kept };
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(Defined.__hmrId = "becca27b30f74606", Defined);
  	__VUE_HMR_RUNTIME__.createRecord(Other.__hmrId = "6f77318b6db355a5", Other);
  	__VUE_HMR_RUNTIME__.createRecord(Kept.__hmrId = "d7eeb8f813dfaba1", Kept);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.Defined) __VUE_HMR_RUNTIME__[mod.Defined.render ? "rerender" : "reload"](mod.Defined.__hmrId, mod.Defined.render || mod.Defined);
  		if (mod.Other) __VUE_HMR_RUNTIME__[mod.Other.render ? "rerender" : "reload"](mod.Other.__hmrId, mod.Other.render || mod.Other);
  		if (mod.Kept) __VUE_HMR_RUNTIME__[mod.Kept.render ? "rerender" : "reload"](mod.Kept.__hmrId, mod.Kept.render || mod.Kept);
  	});
  }
  "#);
}

#[test]
pub fn export_specifiers_resolve_through_the_local_binding() {
  let code = transform(
    "
    const Comp = () => {}
    export { Comp as Alias }
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  const Comp = _defineVaporHmrComponent(() => {});
  export { Comp as Alias };
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8f87c76b959d4619", Comp);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Alias) __VUE_HMR_RUNTIME__[mod.Alias.render ? "rerender" : "reload"](mod.Alias.__hmrId, mod.Alias.render || mod.Alias);
  	});
  }
  "#);
}

#[test]
pub fn interop_function_components_are_wrapped() {
  let code = transform(
    "
    export const Comp = () => {}
    export default defineComponent({ setup: () => () => {} })
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineHmrComponent as _defineHmrComponent } from "/vue-jsx-vapor/vdom";
  export const Comp = _defineHmrComponent(() => {});
  const __default__ = defineComponent({ setup: () => () => {} });
  export default __default__;
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn only_component_like_bindings_are_wrapped() {
  let code = transform(
    "
    export const Comp = () => {}
    export const useFoo = () => {}
    export function helper() {}
    const lower = () => {}
    export { lower as Alias }
    export const _private = () => {}
    export const $dollar = () => {}
    export default function app() {}
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  export const Comp = _defineVaporHmrComponent(() => {});
  export const useFoo = () => {};
  export function helper() {}
  const lower = _defineVaporHmrComponent(() => {});
  export { lower as Alias };
  export const _private = _defineVaporHmrComponent(() => {});
  export const $dollar = _defineVaporHmrComponent(() => {});
  export default function app() {}
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(useFoo.__hmrId = "1a5024d4348ffa63", useFoo);
  	__VUE_HMR_RUNTIME__.createRecord(helper.__hmrId = "90f84a5d44a33252", helper);
  	__VUE_HMR_RUNTIME__.createRecord(lower.__hmrId = "8f87c76b959d4619", lower);
  	__VUE_HMR_RUNTIME__.createRecord(_private.__hmrId = "4044ed32b2c9ceeb", _private);
  	__VUE_HMR_RUNTIME__.createRecord($dollar.__hmrId = "1b0266d63d161463", $dollar);
  	__VUE_HMR_RUNTIME__.createRecord(app.__hmrId = "52164bac249078a3", app);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.useFoo) __VUE_HMR_RUNTIME__[mod.useFoo.render ? "rerender" : "reload"](mod.useFoo.__hmrId, mod.useFoo.render || mod.useFoo);
  		if (mod.helper) __VUE_HMR_RUNTIME__[mod.helper.render ? "rerender" : "reload"](mod.helper.__hmrId, mod.helper.render || mod.helper);
  		if (mod.Alias) __VUE_HMR_RUNTIME__[mod.Alias.render ? "rerender" : "reload"](mod.Alias.__hmrId, mod.Alias.render || mod.Alias);
  		if (mod._private) __VUE_HMR_RUNTIME__[mod._private.render ? "rerender" : "reload"](mod._private.__hmrId, mod._private.render || mod._private);
  		if (mod.$dollar) __VUE_HMR_RUNTIME__[mod.$dollar.render ? "rerender" : "reload"](mod.$dollar.__hmrId, mod.$dollar.render || mod.$dollar);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn interop_default_function_component_is_wrapped() {
  let code = transform(
    "
    export const Comp = () => {}
    export default () => {}
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineHmrComponent as _defineHmrComponent } from "/vue-jsx-vapor/vdom";
  export const Comp = _defineHmrComponent(() => {});
  const __default__ = _defineHmrComponent(() => {});
  export default __default__;
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Comp.__hmrId = "8ed58763ca2bbfd5", Comp);
  	__VUE_HMR_RUNTIME__.createRecord(__default__.__hmrId = "52164bac249078a3", __default__);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Comp) __VUE_HMR_RUNTIME__[mod.Comp.render ? "rerender" : "reload"](mod.Comp.__hmrId, mod.Comp.render || mod.Comp);
  		if (mod.default) __VUE_HMR_RUNTIME__[mod.default.render ? "rerender" : "reload"](mod.default.__hmrId, mod.default.render || mod.default);
  	});
  }
  "#);
}

#[test]
pub fn local_components_are_not_registered() {
  let code = transform(
    "
    const Comp = () => {}
    function Decl() {}
    export const Page = () => {}
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { defineVaporHmrComponent as _defineVaporHmrComponent } from "/vue-jsx-vapor/vapor";
  const Comp = () => {};
  function Decl() {}
  export const Page = _defineVaporHmrComponent(() => {});
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Page.__hmrId = "d6ea2e3457f2b7c4", Page);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Page) __VUE_HMR_RUNTIME__[mod.Page.render ? "rerender" : "reload"](mod.Page.__hmrId, mod.Page.render || mod.Page);
  	});
  }
  "#);
}

#[test]
pub fn module_with_only_local_components_is_not_a_boundary() {
  let code = transform(
    "const Comp = defineComponent({ setup: () => () => {} })",
    Some(TransformOptions {
      hmr: Either::A(true),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  const Comp = defineComponent({ setup: () => () => {} });
  "#);
}

#[test]
pub fn local_components_in_interop() {
  let code = transform(
    "
    const Comp = () => {}
    const ObjComp = defineComponent({ setup: () => () => {} })
    export const Page = defineComponent({ setup: () => () => {} })
  ",
    Some(TransformOptions {
      hmr: Either::A(true),
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  const Comp = () => {};
  const ObjComp = defineComponent({ setup: () => () => {} });
  export const Page = defineComponent({ setup: () => () => {} });
  if (typeof __VUE_HMR_RUNTIME__ !== "undefined") {
  	__VUE_HMR_RUNTIME__.createRecord(Page.__hmrId = "d6ea2e3457f2b7c4", Page);
  	if (import.meta.hot) import.meta.hot.accept((mod) => {
  		if (!mod) return;
  		if (mod.Page) __VUE_HMR_RUNTIME__[mod.Page.render ? "rerender" : "reload"](mod.Page.__hmrId, mod.Page.render || mod.Page);
  	});
  }
  "#);
}
