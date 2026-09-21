use compiler::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn member_expression_component() {
  let code = transform(
    "<Foo.Example/>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo.Example, null, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_generate_single_root_component() {
  let code = transform(
    "<Comp/>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, null, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn emit_single_default_slot_as_raw_slot_function() {
  let code = transform(
    "<Card><div/></Card>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "<><Comp/>123</>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "<><Comp/></>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Comp);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_v_for_should_not_mark_as_single_root() {
  let code = transform(
    "<Comp v-for={item in items} key={item}/>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "<Foo id=\"foo\" class=\"bar\" />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "<Foo literal={'bar'} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "{...obj}",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#""#);
}

#[test]
fn component_dynamic_props_after_static_prop() {
  let code = transform(
    "<Foo id=\"foo\" {...obj} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "<Foo {...obj} id=\"foo\" />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { $: [() => obj, { id: "foo" }] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_dynamic_props_between_static_prop() {
  let code = transform(
    "<Foo id=\"foo\" {...obj} class=\"bar\" />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
  let code = transform(
    "<Foo onClick_foo={a} onClick_bar={e => b(e)} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { onClick: () => [a, (e) => b(e)] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_props_merging_event_handlers_with_modifiers() {
  let code = transform(
    "<Foo onKeydown_enter_prevent={a} onKeydown_esc_prevent={b} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { onKeydown: () => [_withKeys(_withModifiers(a, ["prevent"]), ["enter"]), _withKeys(_withModifiers(b, ["prevent"]), ["esc"])] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_props_merging_style() {
  let code = transform(
    "<Foo style=\"color: green\" style={{ color: 'red' }} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { style: () => ["color: green", { color: "red" }] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_props_merging_class() {
  let code = transform(
    "<Foo class=\"foo\" class={{ bar: isBar }} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { class: () => ["foo", { bar: isBar }] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_v_on() {
  let code = transform(
    "<Foo v-on={obj} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { toHandlers as _toHandlers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { $: [() => _toHandlers(obj)] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_event_with_once_modifier() {
  let code = transform(
    "<Foo onFoo_once={bar} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { onFooOnce: () => bar }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_event_with_multiple_modifier_and_event_options() {
  let code = transform(
    "<Foo onFoo_enter_stop_prevent_capture_once={bar} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { withModifiers as _withModifiers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { onFooCaptureOnce: () => _withModifiers(bar, ["stop", "prevent"]) }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_unquoted_when_value_has_no_special_chars() {
  let code = transform(
    "<div id=\"foo\" class=\"bar\" />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div title="has whitespace" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div data-expr="a>b" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div data-expr="a<b" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div data-expr="a=b" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div title="it's" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div title="foo`bar" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<div title='say "hello"' />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"has whitespace\" inert data-targets=\"foo>bar\">", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn static_props_space_kept_after_quoted_attribute() {
  let code = transform(
    r#"<div title="has whitespace" alt='"contains quotes"' data-targets="foo>bar" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div title=\"has whitespace\" alt=\"&quot;contains quotes&quot;\" data-targets=\"foo>bar\">", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn props_children() {
  let code = transform(
    "<div id=\"foo\"><span/></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    "<div {...obj} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    "<div id=\"foo\" {...obj} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    "<div {...obj} id=\"foo\" />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    "<div id=\"foo\" {...obj} class=\"bar\" />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    "<div onClick_foo={a} onClick_bar={b} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { on as _on, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_on(_n0, "click", a);
  	_on(_n0, "click", b);
  	return _n0;
  })();
  "#);
}

#[test]
fn props_merging_style() {
  let code = transform(
    "<div style=\"color: green\" style={{ color: 'red' }} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
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
  let code = transform(
    "<div class=\"foo\" class={{ bar: isBar }} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    "<div v-on={obj} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
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
  	insert(_n0, _n1);
  	const _n3 = _t2();
  	const _n2 = _t2();
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx/vapor";
  import { child as _child, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<tr><td> ");
  const _t1 = _template("<table>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _t0();
  	insert(_n1, _n2);
  	const _n0 = _child(_n1);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => msg);
  	return _n2;
  })();
  "#);
}

#[test]
fn custom_element() {
  let code = transform(
    r#"<my-custom-element>{foo}</my-custom-element>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVaporSlots as _normalizeVaporSlots } from "/vue-jsx/vapor";
  import { createPlainElement as _createPlainElement } from "vue";
  (() => {
  	const _n0 = _createPlainElement("my-custom-element", null, { $: [() => _normalizeVaporSlots(foo)] }, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn nested_custom_element_with_dynamic_child() {
  let code = transform(
    r#"<div><my-custom-element><span>{msg}</span></my-custom-element></div>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx/vapor";
  import { createPlainElement as _createPlainElement, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  const _t1 = _template("<div>", 1);
  (() => {
  	const _n2 = _t1();
  	_setInsertionState(_n2);
  	const _n1 = _createPlainElement("my-custom-element", null, () => {
  		const _n0 = _t0();
  		const _x0 = _txt(_n0);
  		_setNodes(_x0, () => msg);
  		return _n0;
  	});
  	return _n2;
  })();
  "#);

  assert!(code.contains("_createPlainElement(\"my-custom-element\""));
  assert!(code.contains("_setInsertionState("));
  assert!(code.contains("_setNodes("));
  assert!(!code.contains("_nthChild("));
}

#[test]
fn custom_element_with_v_model() {
  let code = transform(
    r#"<my-custom-element v-model={foo}></my-custom-element>"#,
    Some(TransformOptions {
      vapor: true,
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
      vapor: true,
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
      vapor: true,
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
      vapor: true,
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
  let code = transform(
    r#"<svg><circle r="40"></circle></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<math><mrow><mi>x</mi></mrow></math>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
  let code = transform(
    r#"<>foo<>bar</>baz</>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
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
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
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
  let code = transform(
    r#"<Foo onVue:mounted={handleMounted} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Foo, { onVnodeMounted: () => handleMounted }, null, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn component_keeps_is_props() {
  let code = transform(
    r#"<><Comp is={'Parent'} /><Comp is="Parent" /></>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, { is: "Parent" });
  	const _n1 = _createComponent(Comp, { is: "Parent" });
  	return [_n0, _n1];
  })();
  "#)
}

#[test]
fn component_in_svg_get_namespace() {
  let code = transform(
    r#"<svg><Comp/></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<svg>", 1, 1);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1);
  	const _n0 = _createComponent(Comp, null, null, null, null, 1);
  	return _n1;
  })();
  "#)
}

#[test]
fn component_in_svg_with_v_if_get_namespace() {
  // v-if wraps the element in a synthetic fragment; the namespace must still be
  // inherited from the real `<svg>` parent via the namespace stack.
  let code = transform(
    r#"<svg><Comp v-if={ok}/></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<svg>", 1, 1);
  (() => {
  	const _n3 = _t0();
  	_setInsertionState(_n3);
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _createComponent(Comp, null, null, null, null, 1);
  		return _n2;
  	});
  	return _n3;
  })();
  "#)
}

#[test]
fn component_in_svg_with_v_for_get_namespace() {
  let code = transform(
    r#"<svg><G v-for={i in list}/></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<svg>", 1, 1);
  (() => {
  	const _n3 = _t0();
  	_setInsertionState(_n3);
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createComponent(G, null, null, null, null, 1);
  		return _n2;
  	}, void 0, 3);
  	return _n3;
  })();
  "#)
}

#[test]
fn custom_element_in_svg_get_namespace() {
  let code = transform(
    r#"<svg><custom-el/></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<svg>", 1, 1);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1);
  	const _n0 = _createPlainElement("custom-el", null, null, null, null, 1);
  	return _n1;
  })();
  "#)
}

#[test]
fn component_in_nested_svg_child_get_namespace() {
  let code = transform(
    r#"<svg><g><Comp/></g></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { child as _child, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<svg><g>", 1, 1);
  (() => {
  	const _n2 = _t0();
  	const _n1 = _child(_n2);
  	_setInsertionState(_n1);
  	const _n0 = _createComponent(Comp, null, null, null, null, 1);
  	return _n2;
  })();
  "#)
}

#[test]
fn component_in_math_get_namespace() {
  let code = transform(
    r#"<math><Comp/></math>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<math>", 1, 2);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1);
  	const _n0 = _createComponent(Comp, null, null, null, null, 2);
  	return _n1;
  })();
  "#)
}

#[test]
fn dynamic_component_in_container_does_not_inherit_namespace() {
  // TODO: a dynamic component expression is compiled by an independent
  // `TransformContext` at codegen time, so it currently does not inherit the
  // namespace of the enclosing `<svg>`. This test pins the known limitation;
  // update once the ambient namespace is propagated to nested roots.
  let code = transform(
    r#"<svg>{<D.value/>}</svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<svg> ", 1, 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => (() => {
  		const _n0 = _createComponent(D.value, null, null, true);
  		return _n0;
  	})());
  	return _n0;
  })();
  "#)
}

#[test]
fn component_in_foreign_object_falls_back_to_html_namespace() {
  let code = transform(
    r#"<svg><foreignObject><Comp/></foreignObject></svg>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { child as _child, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<svg><foreignObject>", 1, 1);
  (() => {
  	const _n2 = _t0();
  	const _n1 = _child(_n2);
  	_setInsertionState(_n1);
  	const _n0 = _createComponent(Comp);
  	return _n2;
  })();
  "#)
}

#[test]
fn component_in_html_annotation_xml_falls_back_to_html_namespace() {
  let code = transform(
    r#"<math><annotation-xml encoding="text/html"><Comp/></annotation-xml></math>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { createPlainElement as _createPlainElement, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<math>", 1, 2);
  (() => {
  	const _n2 = _t0();
  	_setInsertionState(_n2);
  	const _n1 = _createPlainElement("annotation-xml", { encoding: "text/html" }, () => {
  		const _n0 = _createComponent(Comp);
  		return _n0;
  	}, null, null, 2);
  	return _n2;
  })();
  "#)
}

#[test]
fn component_in_math_text_integration_point_falls_back_to_html_namespace() {
  let code = transform(
    r#"<math><mi><Comp/></mi></math>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { child as _child, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<math><mi>", 1, 2);
  (() => {
  	const _n2 = _t0();
  	const _n1 = _child(_n2);
  	_setInsertionState(_n1);
  	const _n0 = _createComponent(Comp);
  	return _n2;
  })();
  "#)
}

#[test]
fn v_on_obj_before_static_event_keeps_handler_getters() {
  let code = transform(
    r#"<Foo v-on={obj} onFoo={bar} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { toHandlers as _toHandlers } from "vue";
  (() => {
  	const _n0 = _createComponent(Foo, { $: [() => _toHandlers(obj), { onFoo: () => bar }] }, null, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn keep_the_leading_newline_of_pre_and_textarea() {
  // The compiler already dropped the first newline after these start tags, and
  // the template string is parsed as HTML again at runtime, which drops one
  // more - so a newline surviving into the template has to be doubled.
  for (source, template) in [
    ("<pre>{'\\n\\nline'}</pre>", "<pre>\n\n\nline"),
    ("<pre>{'\\n\\n\\nline'}</pre>", "<pre>\n\n\n\nline"),
    ("<pre>{'\\nline'}</pre>", "<pre>\n\nline"),
    (
      "<textarea>{'\\n\\nline'}</textarea>",
      "<textarea>\n\n\nline",
    ),
    ("<pre>{'\\r\\n\\r\\nline'}</pre>", "<pre>\n\r\n\r\nline"),
    ("<pre v-text={'\\n\\nline'}/>", "<pre>\n\n\nline"),
    // untouched
    ("<pre>{'line\\n\\nmore'}</pre>", "<pre>line\n\nmore"),
    ("<pre>{'line'}</pre>", "<pre>line"),
    ("<div>{'\\n\\nline'}</div>", "<div>\n\nline"),
  ] {
    let code = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert!(
      code.contains(&format!("_template({template:?}")),
      "`{source}` should compile to a template starting with {template:?}, got:\n{code}"
    );
  }
}

#[test]
fn leading_newline_outside_the_html_namespace_is_untouched() {
  let code = transform(
    "<svg><pre>{'\\n\\nline'}</pre></svg>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains(r#"_template("<pre>\n\nline")"#), "{code}");
}

// upstream: `props the template string cannot carry`. `<textarea>` / `<select>`
// ignore a `value` content attribute, the value only takes effect as a dom
// property. Upstream also takes `true-value` / `false-value` out of a checkbox
// template here; that half only matters for hydration, which this repo has no
// vapor path for, so only the inert `value` is covered.
#[test]
fn props_the_template_string_cannot_carry() {
  for (source, template) in [
    // `<textarea>` / `<select>` ignore a `value` content attribute
    (r#"<textarea value="1"></textarea>"#, "<textarea>"),
    (r#"<textarea value={'x'}></textarea>"#, "<textarea>"),
    (r#"<select value="b"></select>"#, "<select>"),
    // untouched
    (r#"<div value="1"></div>"#, "<div value=1>"),
    (r#"<input value="1" />"#, "<input value=1>"),
    (r#"<option value="1"></option>"#, "<option value=1>"),
  ] {
    let code = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert!(
      code.contains(&format!("_template({template:?}")),
      "`{source}` should compile to a template starting with {template:?}, got:\n{code}"
    );
  }
}
