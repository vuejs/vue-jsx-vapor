use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn component_import_resolve_component() {
  let code = transform(
    "<foo-bar/>",
    Some(TransformOptions {
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponentWithFallback as _createComponentWithFallback } from "/vue-jsx-vapor/vapor";
  import { resolveComponent as _resolveComponent } from "vue";
  (() => {
  	const _component_foo_bar = _resolveComponent("foo-bar");
  	const _n0 = _createComponentWithFallback(_component_foo_bar, null, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_resolve_namespaced_component() {
  let code = transform("<Foo.Example/>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo.Example, null, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_() {
  let code = transform("", None).code;
  assert_snapshot!(code, @r#""#);
}

#[test]
fn component_generate_single_root_component() {
  let code = transform("<Comp/>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, null, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_generate_multi_root_component() {
  let code = transform("<><Comp/>123</>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("123");
  (() => {
  	const _n0 = _createComponent(Comp);
  	const _n1 = _t0();
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn component_fragment_should_not_mark_as_single_root() {
  let code = transform("<><Comp/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_v_for_should_not_mark_as_single_root() {
  let code = transform("<Comp v-for={item in items} key={item}/>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor } from "vue";
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createComponent(Comp);
  		return _n2;
  	}, (item) => item, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_static_props() {
  let code = transform("<Foo id=\"foo\" class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, {
  		id: () => "foo",
  		class: () => "bar"
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_dynamic_props() {
  let code = transform("{...obj}", None).code;
  assert_snapshot!(code, @r#""#);
}

#[test]
fn component_dynamic_props_after_static_prop() {
  let code = transform("<Foo id=\"foo\" {...obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, {
  		id: () => "foo",
  		$: [() => obj]
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_dynamic_props_before_static_prop() {
  let code = transform("<Foo {...obj} id=\"foo\" />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { $: [() => obj, { id: () => "foo" }] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_dynamic_props_between_static_prop() {
  let code = transform("<Foo id=\"foo\" {...obj} class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, {
  		id: () => "foo",
  		$: [() => obj, { class: () => "bar" }]
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_props_merging_style() {
  let code = transform(
    "<Foo style=\"color: green\" style={{ color: 'red' }} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { style: () => ["color: green", { color: "red" }] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_props_merging_class() {
  let code = transform("<Foo class=\"foo\" class={{ bar: isBar }} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { class: () => ["foo", { bar: isBar }] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_v_on() {
  let code = transform("<Foo v-on={obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { toHandlers as _toHandlers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { $: [() => _toHandlers(obj)] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_event_with_once_modifier() {
  let code = transform("<Foo onFoo_once={bar} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { onFooOnce: () => bar }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_with_fallback() {
  let code = transform("<foo-bar />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponentWithFallback as _createComponentWithFallback } from "/vue-jsx-vapor/vapor";
  import { resolveComponent as _resolveComponent } from "vue";
  (() => {
  	const _component_foo_bar = _resolveComponent("foo-bar");
  	const _n0 = _createComponentWithFallback(_component_foo_bar, null, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props() {
  let code = transform("<div id=\"foo\" class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div id=\"foo\" class=\"bar\"></div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn props_children() {
  let code = transform("<div id=\"foo\"><span/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div id=\"foo\"><span></span></div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props() {
  let code = transform("<div {...obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [obj], true));
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props_after_static_prop() {
  let code = transform("<div id=\"foo\" {...obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [{ id: "foo" }, obj], true));
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props_before_static_prop() {
  let code = transform("<div {...obj} id=\"foo\" />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [obj, { id: "foo" }], true));
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props_between_static_prop() {
  let code = transform("<div id=\"foo\" {...obj} class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [
  		{ id: "foo" },
  		obj,
  		{ class: "bar" }
  	], true));
  	return _n0;
  })();
  "#);
}

#[test]
fn props_merging_event_handlers() {
  let code = transform("<div onClick_foo={a} onClick_bar={b} />", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegate as _delegate, delegateEvents as _delegateEvents, template as _template, withKeys as _withKeys } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_delegate(_n0, "click", _withKeys(a, ["foo"]));
  	_delegate(_n0, "click", _withKeys(b, ["bar"]));
  	return _n0;
  })();
  "#);
}

#[test]
fn props_merging_style() {
  let code = transform(
    "<div style=\"color: green\" style={{ color: 'red' }} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setStyle as _setStyle, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_setStyle(_n0, ["color: green", { color: "red" }]);
  	return _n0;
  })();
  "#);
}

#[test]
fn props_merging_class() {
  let code = transform("<div class=\"foo\" class={{ bar: isBar }} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClass(_n0, ["foo", { bar: isBar }]));
  	return _n0;
  })();
  "#);
}

#[test]
fn v_on() {
  let code = transform("<div v-on={obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicEvents as _setDynamicEvents, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicEvents(_n0, obj));
  	return _n0;
  })();
  "#);
}

#[test]
fn invalid_html_nesting() {
  let code = transform(
    "<><p><div>123</div></p>
    <form><form/></form></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>123</div>");
  const _t1 = _template("<p></p>");
  const _t2 = _template("<form></form>");
  (() => {
  	const _n1 = _t1();
  	const _n0 = _t0();
  	const _n3 = _t2();
  	const _n2 = _t2();
  	insert(_n0, _n1);
  	insert(_n2, _n3);
  	return [_n1, _n3];
  })();
  "#);
}

#[test]
fn svg() {
  let code = transform(r#"<svg><circle r="40"></circle></svg>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<svg><circle r=\"40\"></circle></svg>", true, 1);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#)
}

#[test]
fn math_ml() {
  let code = transform(r#"<math><mrow><mi>x</mi></mrow></math>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("x");
  const _t1 = _template("<math></math>", true, 2);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, null, true);
  	const _n2 = _createComponent(mrow, null, { default: _withVaporCtx(() => {
  		const _n1 = _createComponent(mi, null, { default: () => {
  			const _n0 = _t0();
  			return _n0;
  		} });
  		return _n1;
  	}) });
  	return _n3;
  })();
  "#)
}
