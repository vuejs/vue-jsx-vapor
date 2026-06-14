use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn member_expression_component() {
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
fn emit_single_default_slot_as_raw_slot_function() {
  let code = transform("<Card><div/></Card>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n1 = _createComponent(Card, null, () => {
  		const _n0 = _t0();
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn component_generate_multi_root_component() {
  let code = transform("<><Comp/>123</>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("123", 2);
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
  		id: "foo",
  		class: "bar"
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_static_literal_bind_props() {
  let code = transform("<Foo literal={'bar'} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { literal: "bar" }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_constant_bind_props_are_direct_raw_prop_values() {
  let code = transform(
    r#"<Foo
      size={16}
      disabled={false}
      tabindex={0}
      nullable={null}
      missing={undefined}
      big={1n}
      label={`Save ${1}`}
      items={[1, "two", false, null, undefined]}
      options={{ placement: "bottom", offset: 8, nested: { enabled: true } }}
    />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, {
  		size: 16,
  		disabled: false,
  		tabindex: 0,
  		nullable: null,
  		missing: undefined,
  		big: 1n,
  		label: "Save 1",
  		items: [
  			1,
  			"two",
  			false,
  			null,
  			undefined
  		],
  		options: {
  			placement: "bottom",
  			offset: 8,
  			nested: { enabled: true }
  		}
  	}, null, true);
  	return _n0;
  })();
  "#);

  assert!(code.contains("size: 16"));
  assert!(code.contains("disabled: false"));
  assert!(code.contains("tabindex: 0"));
  assert!(code.contains("nullable: null"));
  assert!(code.contains("missing: undefined"));
  assert!(code.contains("big: 1n"));
  assert!(code.contains("label: \"Save 1\""));
  assert!(code.contains("placement: \"bottom\""));
  assert!(code.contains("offset: 8"));
  assert!(code.contains("nested: { enabled: true }"));
}

#[test]
fn component_dynamic_non_literal_prop_values_stay_as_getter_sources() {
  let code = transform(
    r#"<Foo foo={bar} obj={{ a: bar }} handler={onClick} formatter={v => v.toFixed(2)} fn={() => bar} onClick={foo} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, {
  		foo: () => bar,
  		obj: () => ({ a: bar }),
  		handler: () => onClick,
  		formatter: () => (v) => v.toFixed(2),
  		fn: () => () => bar,
  		onClick: () => foo
  	}, null, true);
  	return _n0;
  })();
  "#);

  assert!(code.contains("foo: () => bar"));
  assert!(code.contains("obj: () => ({ a: bar })"));
  assert!(code.contains("handler: () => onClick"));
  assert!(code.contains("formatter: () => (v) => v.toFixed(2)"));
  assert!(code.contains("fn: () => () => bar"));
  assert!(code.contains("onClick: () => foo"));
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
  		id: "foo",
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
  	const _n0 = _createComponent(Foo, { $: [() => obj, { id: "foo" }] }, null, true);
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
  		id: "foo",
  		$: [() => obj, { class: "bar" }]
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
fn static_props_unquoted_when_value_has_no_special_chars() {
  let code = transform("<div id=\"foo\" class=\"bar\" />", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div id=foo class=bar>", 3);
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
  const _t0 = _template("<div title=\"has whitespace\">", 3);
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
  const _t0 = _template("<div data-expr=\"a>b\">", 3);
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
  const _t0 = _template("<div data-expr=\"a<b\">", 3);
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
  const _t0 = _template("<div data-expr=\"a=b\">", 3);
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
  const _t0 = _template("<div title=\"it's\">", 3);
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
  const _t0 = _template("<div title=\"foo`bar\">", 3);
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
  const _t0 = _template("<div title=\"say &quot;hello&quot;\">", 3);
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
  const _t0 = _template("<div title=\"has whitespace\"inert data-targets=\"foo>bar\">", 3);
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
  const _t0 = _template("<div title=\"has whitespace\"alt=\"&quot;contains quotes&quot;\"data-targets=\"foo>bar\">", 3);
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
  const _t0 = _template("<div id=foo><span>", 3);
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
  const _t0 = _template("<div>", 1);
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
  const _t0 = _template("<div>", 1);
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
  const _t0 = _template("<div>", 1);
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
  const _t0 = _template("<div>", 1);
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
  const _t0 = _template("<div>", 1);
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
  const _t0 = _template("<div>", 1);
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
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isBar ? 1 : 0, " bar", "foo"));
  	return _n0;
  })();
  "#);
}

#[test]
fn v_on() {
  let code = transform("<div v-on={obj} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicEvents as _setDynamicEvents, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
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
  const _t1 = _template("<table>", 1);
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
fn zcustom_element() {
  let code = transform(r#"<my-custom-element>{foo}</my-custom-element>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createPlainElement as _createPlainElement } from "vue";
  (() => {
  	const _n1 = _createPlainElement("my-custom-element", null, () => {
  		const _n0 = _createNodes(() => foo);
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#)
}

#[test]
fn custom_element_with_v_model() {
  let code = transform(
    r#"<my-custom-element v-model={foo}></my-custom-element>"#,
    None,
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
    None,
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
    None,
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
    None,
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
  const _t0 = _template("<svg><circle r=40>", 3, 1);
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
  const _t0 = _template("<math><mrow><mi>x", 3, 2);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#)
}

#[test]
fn fragment_in_fragment() {
  let code = transform(r#"<>foo<>bar</>baz</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("foo", 2);
  const _t1 = _template("bar", 2);
  const _t2 = _template("baz", 2);
  (() => {
  	const _n0 = _t0();
  	const _n1 = _t1();
  	const _n2 = _t2();
  	return [
  		_n0,
  		_n1,
  		_n2
  	];
  })();
  "#)
}

#[test]
fn is_component() {
  let code = transform(
    r#"<>
      <组件 />
      <_foo />
      <$foo />
      <foo.bar />
    </>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(组件);
  	const _n1 = _createComponent(_foo);
  	const _n2 = _createComponent($foo);
  	const _n3 = _createComponent(foo.bar);
  	return [
  		_n0,
  		_n1,
  		_n2,
  		_n3
  	];
  })();
  "#)
}

#[test]
fn is_not_component() {
  let code = transform(
    r#"<>
      <foo-bar />
      <foo />
      <foo:bar />
    </>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement, template as _template } from "vue";
  const _t0 = _template("<foo>", 2);
  const _t1 = _template("<foo:bar>", 2);
  (() => {
  	const _n0 = _createPlainElement("foo-bar");
  	const _n1 = _t0();
  	const _n2 = _t1();
  	return [
  		_n0,
  		_n1,
  		_n2
  	];
  })();
  "#)
}

#[test]
fn component_vue_vnode_hooks() {
  let code = transform(r#"<Foo onVue:mounted={handleMounted} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { onVnodeMounted: () => handleMounted }, null, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn component_keeps_is_props() {
  let code = transform(r#"<><Comp is={'Parent'} /><Comp is="Parent" /></>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, { is: "Parent" });
  	const _n1 = _createComponent(Comp, { is: "Parent" });
  	return [_n0, _n1];
  })();
  "#)
}

#[test]
fn v_on_obj_before_static_event_keeps_handler_getters() {
  let code = transform(r#"<Foo v-on={obj} onFoo={bar} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { toHandlers as _toHandlers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { $: [() => _toHandlers(obj), { onFoo: () => bar }] }, null, true);
  	return _n0;
  })();
  "#)
}
