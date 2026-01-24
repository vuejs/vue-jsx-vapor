use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "const A = defineComponent(() => {
      defineVaporComponent(() => <span />)
      return () => <><div /></>
    })
    const B = defineVaporComponent(() => {
      const C = defineComponent(() => <><div /></>)
      const D = <>{foo} <div /></>
      return <div />
    })",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, template as _template } from "vue";
  const _t0 = _template("<span></span>", true);
  const _t1 = _template("<div></div>");
  const _t2 = _template("<div></div>", true);
  const A = defineComponent(() => {
  	defineVaporComponent(() => (() => {
  		const _n0 = _t0();
  		return _n0;
  	})());
  	return () => (() => {
  		const _cache = _createVNodeCache(0);
  		return _openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))], 64);
  	})();
  });
  const B = defineVaporComponent(() => {
  	const C = defineComponent(() => (() => {
  		const _cache = _createVNodeCache(1);
  		return _openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))], 64);
  	})());
  	const D = (() => {
  		const _n2 = _t1();
  		const _n0 = _createNodes(() => foo, " ");
  		return [_n0, _n2];
  	})();
  	return (() => {
  		const _n0 = _t2();
  		return _n0;
  	})();
  });
  "#);
}
