use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn static_ref() {
  let code = transform("<div ref=\"foo\" />", None).code;
  assert_snapshot!(code, @r#"
  import { createTemplateRefSetter as _createTemplateRefSetter, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const _n0 = _t0();
  	_setTemplateRef(_n0, "foo");
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_ref() {
  let code = transform("<div ref={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const _n0 = _t0();
  	let _r0;
  	_renderEffect(() => _r0 = _setTemplateRef(_n0, foo, _r0));
  	return _n0;
  })();
  "#);
}

#[test]
fn function_ref() {
  let code = transform(
    "<Comp v-slot={{baz}}>
      <div ref={bar => {
        foo.value = bar
        ;({ baz, bar: baz } = bar)
        console.log(foo.value, baz)
      }} />
  </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx((_slotProps0) => {
  		const _n0 = _t0();
  		let _r0;
  		_renderEffect(() => _r0 = _setTemplateRef(_n0, (bar) => {
  			foo.value = bar;
  			({baz: _slotProps0.baz, bar: _slotProps0.baz} = bar);
  			console.log(foo.value, _slotProps0.baz);
  		}, _r0));
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn ref_v_if() {
  let code = transform("<div ref={foo} v-if={true} />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const _n0 = _createIf(() => true, () => {
  		const _n2 = _t0();
  		let _r2;
  		_renderEffect(() => _r2 = _setTemplateRef(_n2, foo, _r2));
  		return _n2;
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn ref_v_for() {
  let code = transform("<div ref={foo} v-for={item in [1,2,3]} />", None).code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const _n0 = _createFor(() => [
  		1,
  		2,
  		3
  	], (_for_item0) => {
  		const _n2 = _t0();
  		let _r2;
  		_renderEffect(() => _r2 = _setTemplateRef(_n2, foo, _r2, true));
  		return _n2;
  	}, void 0, 4);
  	return _n0;
  })();
  "#);
}
