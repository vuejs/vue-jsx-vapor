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
fn component_props_merging_event_handlers() {
  let code = transform("<Foo onClick_foo={a} onClick_bar={e => b(e)} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { onClick: () => [a, (e) => b(e)] }, null, true);
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
fn component_event_with_multiple_modifier_and_event_options() {
  let code = transform("<Foo onFoo_enter_stop_prevent_capture_once={bar} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { withModifiers as _withModifiers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { onFooCaptureOnce: () => _withModifiers(bar, ["stop", "prevent"]) }, null, true);
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
fn static_props_unquoted_when_value_has_no_special_chars() {
  let code = transform("<div id=\"foo\" class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div id=foo class=bar>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_quoted_when_value_contains_whitespace() {
  let code = transform(r#"<div title="has whitespace" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"has whitespace\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_quoted_when_value_contains_right_angle_bracket() {
  let code = transform(r#"<div data-expr="a>b" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div data-expr=\"a>b\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_quoted_when_value_contains_left_angle_bracket() {
  let code = transform(r#"<div data-expr="a<b" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div data-expr=\"a<b\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_quoted_when_value_contains_equal_bracket() {
  let code = transform(r#"<div data-expr="a=b" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div data-expr=\"a=b\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_quoted_when_value_contains_single_quote() {
  let code = transform(r#"<div title="it's" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"it's\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_quoted_when_value_contains_backtick() {
  let code = transform(r#"<div title="foo`bar" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"foo`bar\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_escapes_double_quotes_in_value() {
  let code = transform(r#"<div title='say "hello"' />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"say &quot;hello&quot;\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_mixed_quoting_with_boolean_attribute() {
  let code = transform(
    r#"<div title="has whitespace" inert data-targets="foo>bar" />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"has whitespace\"inert data-targets=\"foo>bar\">", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_space_omitted_after_quoted_attribute() {
  let code = transform(
    r#"<div title="has whitespace" alt='"contains quotes"' data-targets="foo>bar" />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"has whitespace\"alt=\"&quot;contains quotes&quot;\"data-targets=\"foo>bar\">", true);
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
  const _t0 = _template("<div id=foo><span>", true);
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
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [obj]));
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props_after_static_prop() {
  let code = transform("<div id=\"foo\" {...obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [{ id: "foo" }, obj]));
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props_before_static_prop() {
  let code = transform("<div {...obj} id=\"foo\" />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [obj, { id: "foo" }]));
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_props_between_static_prop() {
  let code = transform("<div id=\"foo\" {...obj} class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [
  		{ id: "foo" },
  		obj,
  		{ class: "bar" }
  	]));
  	return _n0;
  })();
  "#);
}

#[test]
fn props_merging_event_handlers() {
  let code = transform("<div onClick_foo={a} onClick_bar={b} />", None).code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { delegate as _delegate, delegateEvents as _delegateEvents, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	_delegate(_n0, "click", a);
  	_delegate(_n0, "click", b);
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
  const _t0 = _template("<div>", true);
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
  const _t0 = _template("<div>", true);
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
  const _t0 = _template("<div>", true);
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
  const _t0 = _template("<div>123");
  const _t1 = _template("<p>");
  const _t2 = _template("<form>");
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
fn invalid_table_nesting_with_dynamic_child() {
  let code = transform(
    "<table>
      <tr>
        <td>{msg}</td>
      </tr>
    </table>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<tr><td> ");
  const _t1 = _template("<table>", true);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _t0();
  	const _n0 = _child(_n1);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => msg);
  	insert(_n1, _n2);
  	return _n2;
  })();
  "#);
}

#[test]
fn custom_element() {
  let code = transform(
    r#"<my-custom-element>{foo}</my-custom-element>"#,
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createPlainElement as _createPlainElement, withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createPlainElement("my-custom-element", null, { default: _withVaporCtx(() => {
  		const _n0 = _createNodes(() => foo);
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#)
}

#[test]
fn custom_element_with_v_model() {
  let code = transform(
    r#"<my-custom-element v-model={foo}></my-custom-element>"#,
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement } from "vue";
  (() => {
  	const _n0 = _createPlainElement("my-custom-element", {
  		modelValue: () => foo,
  		"onUpdate:modelValue": () => (_value) => foo = _value
  	}, null, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn custom_element_with_v_on() {
  let code = transform(
    r#"<my-custom-element onFoo={foo}></my-custom-element>"#,
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement } from "vue";
  (() => {
  	const _n0 = _createPlainElement("my-custom-element", { onFoo: () => foo }, null, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn custom_element_with_v_html() {
  let code = transform(
    r#"<my-custom-element v-html={foo}></my-custom-element>"#,
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement, renderEffect as _renderEffect, setHtml as _setHtml } from "vue";
  (() => {
  	const _n0 = _createPlainElement("my-custom-element", null, null, true);
  	_renderEffect(() => _setHtml(_n0, foo));
  	return _n0;
  })();
  "#)
}

#[test]
fn custom_element_with_v_text() {
  let code = transform(
    r#"<my-custom-element v-text={foo}></my-custom-element>"#,
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement, renderEffect as _renderEffect, setText as _setText, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  (() => {
  	const _n0 = _createPlainElement("my-custom-element", null, null, true);
  	const _x0 = _txt(_n0);
  	_renderEffect(() => _setText(_x0, _toDisplayString(foo)));
  	return _n0;
  })();
  "#)
}

#[test]
fn svg() {
  let code = transform(r#"<svg><circle r="40"></circle></svg>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<svg><circle r=40>", true, 1);
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
  import { template as _template } from "vue";
  const _t0 = _template("<math><mrow><mi>x", true, 2);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#)
}
