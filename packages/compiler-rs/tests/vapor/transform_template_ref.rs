use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn static_ref() {
  let code = transform("<div ref=\"foo\" />", None).code;
  assert_snapshot!(code, @r#"
  import { createTemplateRefSetter as _createTemplateRefSetter, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const n0 = t0();
  	_setTemplateRef(n0, "foo");
  	return n0;
  })();
  "#);
}

#[test]
fn dynamic_ref() {
  let code = transform("<div ref={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const n0 = t0();
  	let r0;
  	_renderEffect(() => r0 = _setTemplateRef(n0, foo, r0));
  	return n0;
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
  import { createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const n1 = _createComponent(Comp, null, { default: (_slotProps0) => {
  		const n0 = t0();
  		let r0;
  		_renderEffect(() => r0 = _setTemplateRef(n0, (bar) => {
  			foo.value = bar;
  			({baz: _slotProps0.baz, bar: _slotProps0.baz} = bar);
  			console.log(foo.value, _slotProps0.baz);
  		}, r0));
  		return n0;
  	} }, true);
  	return n1;
  })();
  "#);
}

#[test]
fn ref_v_if() {
  let code = transform("<div ref={foo} v-if={true} />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const n0 = _createIf(() => true, () => {
  		const n2 = t0();
  		let r2;
  		_renderEffect(() => r2 = _setTemplateRef(n2, foo, r2));
  		return n2;
  	}, null, true);
  	return n0;
  })();
  "#);
}

#[test]
fn ref_v_for() {
  let code = transform("<div ref={foo} v-for={item in [1,2,3]} />", None).code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createTemplateRefSetter as _createTemplateRefSetter, renderEffect as _renderEffect, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const _setTemplateRef = _createTemplateRefSetter();
  	const n0 = _createFor(() => [
  		1,
  		2,
  		3
  	], (_for_item0) => {
  		const n2 = t0();
  		let r2;
  		_renderEffect(() => r2 = _setTemplateRef(n2, foo, r2, true));
  		return n2;
  	}, void 0, 4);
  	return n0;
  })();
  "#);
}
