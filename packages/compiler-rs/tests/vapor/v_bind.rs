use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div id={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setProp(_n0, "id", id));
  	return _n0;
  })();
  "#);
}

#[test]
fn no_expression() {
  let code = transform("<div id />", None).code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_setProp(_n0, "id", true);
  	return _n0;
  })();
  "#);
}

#[test]
fn empty_expression() {
  let code = transform(r#"<div foo={} bar=""></div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div bar>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn camel_modifier() {
  let code = transform("<div foo-bar_camel={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setProp(_n0, "fooBar", id));
  	return _n0;
  })();
  "#);
}

#[test]
fn camel_modifier_with_no_expression() {
  let code = transform("<div foo-bar_camel />", None).code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_setProp(_n0, "fooBar", true);
  	return _n0;
  })();
  "#);
}

#[test]
fn prop_modifier() {
  let code = transform("<div fooBar_prop={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDOMProp as _setDOMProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDOMProp(_n0, "fooBar", id));
  	return _n0;
  })();
  "#);
}

#[test]
fn prop_modifier_with_no_expression() {
  let code = transform("<div fooBar_prop />", None).code;
  assert_snapshot!(code, @r#"
  import { setDOMProp as _setDOMProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_setDOMProp(_n0, "fooBar", true);
  	return _n0;
  })();
  "#);
}

#[test]
fn prop_modifier_with_text_content() {
  let code = transform("<div textContent_prop={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setElementText as _setElementText, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setElementText(_n0, foo));
  	return _n0;
  })();
  "#);
  assert!(code.contains("_setElementText(_n0, foo)"));
}

#[test]
fn text_content_binding() {
  let code = transform("<div textContent={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setElementText as _setElementText, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setElementText(_n0, foo));
  	return _n0;
  })();
  "#);
  assert!(code.contains("_setElementText(_n0, foo)"));
}

#[test]
fn attr_modifier() {
  let code = transform("<div foo-bar_attr={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setAttr as _setAttr, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setAttr(_n0, "foo-bar", id));
  	return _n0;
  })();
  "#);
}

#[test]
fn attr_modifier_with_no_expression() {
  let code = transform("<div foo-bar_attr />", None).code;
  assert_snapshot!(code, @r#"
  import { setAttr as _setAttr, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_setAttr(_n0, "foo-bar", true);
  	return _n0;
  })();
  "#);
}

#[test]
fn with_constant_value() {
  let code = transform(
    "<div
      a={void 0}
      b={1 > 2}
      c={1 + 2}
      d={1 ? 2 : 3}
      e={(2)}
      f={`foo${1}`}
      g={1.1}
      h={'1'}
      i={true}
      j={null}
      l={{ foo: 1 }}
      n={{ ...{ foo: 1 } }}
      o={[1, , 3]}
      p={[1, ...[2, 3]]}
      q={[1, 2]}
      r={/\\s+/}
    />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div e=2 f=foo1 g=1.1 h=1>", 1);
  (() => {
  	const _n0 = _t0();
  	_setProp(_n0, "a", void 0);
  	_setProp(_n0, "b", 1 > 2);
  	_setProp(_n0, "c", 1 + 2);
  	_setProp(_n0, "d", 1 ? 2 : 3);
  	_setProp(_n0, "i", true);
  	_setProp(_n0, "j", null);
  	_setProp(_n0, "l", { foo: 1 });
  	_setProp(_n0, "n", { ...{ foo: 1 } });
  	_setProp(_n0, "o", [
  		1,
  		,
  		3
  	]);
  	_setProp(_n0, "p", [1, ...[2, 3]]);
  	_setProp(_n0, "q", [1, 2]);
  	_setProp(_n0, "r", /\s+/);
  	return _n0;
  })();
  "#);
}

#[test]
fn number_value() {
  let code = transform(
    "<>
      <div depth={0} />
      <Comp depth={0} />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div depth=0>", 2);
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createComponent(Comp, { depth: 0 });
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn class_with_svg_elements() {
  let code = transform(r#"<svg class={cls}/>"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setAttr as _setAttr, template as _template } from "vue";
  const _t0 = _template("<svg>", 1, 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setAttr(_n0, "class", cls, true));
  	return _n0;
  })();
  "#);
}

#[test]
fn bind_with_svg_elements() {
  let code = transform(r#"<svg {...obj}/>"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<svg>", 1, 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [obj], true));
  	return _n0;
  })();
  "#);
}

#[test]
fn starts_with_underline() {
  let code = transform(
    r#"<div _id_prop={id} __id_prop={id} v-model:$_value_value$={model} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, renderEffect as _renderEffect, setDOMProp as _setDOMProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => {
  		_setDOMProp(_n0, "_id", id);
  		_setDOMProp(_n0, "__id", id);
  	});
  	_applyTextModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn namespace_prop() {
  let code = transform(
    r#"<div xmlns:xlink="http://www.w3.org/1999/xlink" foo:bar={foo} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div xmlns:xlink=http://www.w3.org/1999/xlink>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setProp(_n0, "foo:bar", foo));
  	return _n0;
  })();
  "#);
}

#[test]
fn deduped_props() {
  let code = transform(r#"<div foo="foo" foo={foo} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div foo=foo>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn simple_object_class_name_helper() {
  let code = transform(r#"<div class={{ active: isActive }}/>"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isActive ? 1 : 0, "active"));
  	return _n0;
  })();
  "#);
}

#[test]
fn ternary_string_class_name_helper() {
  let code = transform(
    r#"<div class={selected === row.id ? 'danger' : ''}/>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, selected === row.id ? 1 : 0, "danger"));
  	return _n0;
  })();
  "#);
}

#[test]
fn reverse_ternary_string_class_name_helper() {
  let code = transform(
    r#"<div class={selected === row.id ? '' : 'danger'}/>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, selected === row.id ? 0 : 1, "danger"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, selected === row.id ? 0 : 1, "danger")"#));
}

#[test]
fn static_class_after_conditional_uses_class_name_helper_with_suffix() {
  let code = transform(
    r#"<div class={selected === row.id ? 'danger' : ''} class="foo" />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, selected === row.id ? 1 : 0, "danger", "", "foo"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, selected === row.id ? 1 : 0, "danger", "", "foo")"#));
}

#[test]
fn static_class_with_simple_object_class_name_helper() {
  let code = transform(r#"<div class="foo" class={{ bar: isBar }} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isBar ? 1 : 0, " bar", "foo"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, isBar ? 1 : 0"#));
  assert!(code.contains(r#"" bar", "foo""#));
  assert!(!code.contains("{ bar:"));
}

#[test]
fn static_class_in_reverse_order_uses_class_name_helper_with_suffix() {
  let code = transform(r#"<div class={{ bar: isBar }} class="foo" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isBar ? 1 : 0, "bar", "", "foo"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, isBar ? 1 : 0, "bar", "", "foo")"#));
}

#[test]
fn static_class_after_multiple_object_class_name_helper_uses_suffix() {
  let code = transform(
    r#"<div class={{ active: ok, foo: bar }} class="tail" />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, (ok ? 1 : 0) | (bar ? 2 : 0), [" active", " foo"], "", "tail"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(
    r#"_setClassName(_n0, (ok ? 1 : 0) | (bar ? 2 : 0), [" active", " foo"], "", "tail")"#
  ));
}

#[test]
fn multiple_simple_object_class_name_helper() {
  let code = transform(r#"<div class={{ active: ok, foo: bar }} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, (ok ? 1 : 0) | (bar ? 2 : 0), [" active", " foo"]));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, (ok ? 1 : 0) | (bar ? 2 : 0)"#));
  assert!(code.contains(r#"[" active", " foo"]"#));
  assert!(!code.contains("{ active:"));
}

#[test]
fn static_class_with_multiple_object_class_name_helper() {
  let code = transform(
    r#"<div class="foo" class={{ danger: selected === row.id, 'is-active': active }} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, (selected === row.id ? 1 : 0) | (active ? 2 : 0), [" danger", " is-active"], "foo"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, (selected === row.id ? 1 : 0) | (active ? 2 : 0), [" danger", " is-active"], "foo")"#));
  assert!(!code.contains("{ danger:"));
}

#[test]
fn object_class_with_multi_token_key() {
  let code = transform(r#"<div class={{ 'foo bar': isActive }} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isActive ? 1 : 0, "foo bar"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, isActive ? 1 : 0"#));
  assert!(code.contains(r#""foo bar""#));
  assert!(!code.contains("'foo bar':"));
}

#[test]
fn static_class_with_overlapping_object_class() {
  let code = transform(r#"<div class="bar" class={{ bar: isBar }} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isBar ? 1 : 0, " bar", "bar"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, isBar ? 1 : 0"#));
  assert!(code.contains(r#"" bar", "bar""#));
  assert!(!code.contains("{ bar:"));
}

#[test]
fn static_class_with_overlapping_multi_token_object_class() {
  let code = transform(
    r#"<div class="foo" class={{ 'foo bar': isActive }} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, isActive ? 1 : 0, " foo bar", "foo"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, isActive ? 1 : 0"#));
  assert!(code.contains(r#"" foo bar", "foo""#));
  assert!(!code.contains("'foo bar':"));
}

#[test]
fn class_name_helper_normalizes_static_and_string_class_values() {
  let code = transform(
    r#"<div class=" foo  bar " class={ok ? ' baz ' : ''} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, ok ? 1 : 0, " baz", "foo bar"));
  	return _n0;
  })();
  "#);

  assert!(code.contains(r#"_setClassName(_n0, ok ? 1 : 0, " baz", "foo bar")"#));
}

#[test]
fn class_name_helper_falls_back_when_bit_flags_are_exhausted() {
  let entries = (0..32)
    .map(|i| format!("c{i}: a{i}"))
    .collect::<Vec<_>>()
    .join(", ");
  let source = format!("<div class={{{{ {} }}}}/>", entries);
  let code = transform(&source, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClass(_n0, {
  		c0: a0,
  		c1: a1,
  		c2: a2,
  		c3: a3,
  		c4: a4,
  		c5: a5,
  		c6: a6,
  		c7: a7,
  		c8: a8,
  		c9: a9,
  		c10: a10,
  		c11: a11,
  		c12: a12,
  		c13: a13,
  		c14: a14,
  		c15: a15,
  		c16: a16,
  		c17: a17,
  		c18: a18,
  		c19: a19,
  		c20: a20,
  		c21: a21,
  		c22: a22,
  		c23: a23,
  		c24: a24,
  		c25: a25,
  		c26: a26,
  		c27: a27,
  		c28: a28,
  		c29: a29,
  		c30: a30,
  		c31: a31
  	}));
  	return _n0;
  })();
  "#);
  assert!(code.contains("_setClass(_n0, {"));
  assert!(!code.contains("_setClassName"));
}

#[test]
fn class_name_helper_supports_the_max_safe_bit_flag() {
  let entries = (0..31)
    .map(|i| format!("c{i}: a{i}"))
    .collect::<Vec<_>>()
    .join(", ");
  let source = format!("<div class={{{{ {} }}}}/>", entries);
  let code = transform(&source, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClassName(_n0, (a0 ? 1 : 0) | (a1 ? 2 : 0) | (a2 ? 4 : 0) | (a3 ? 8 : 0) | (a4 ? 16 : 0) | (a5 ? 32 : 0) | (a6 ? 64 : 0) | (a7 ? 128 : 0) | (a8 ? 256 : 0) | (a9 ? 512 : 0) | (a10 ? 1024 : 0) | (a11 ? 2048 : 0) | (a12 ? 4096 : 0) | (a13 ? 8192 : 0) | (a14 ? 16384 : 0) | (a15 ? 32768 : 0) | (a16 ? 65536 : 0) | (a17 ? 131072 : 0) | (a18 ? 262144 : 0) | (a19 ? 524288 : 0) | (a20 ? 1048576 : 0) | (a21 ? 2097152 : 0) | (a22 ? 4194304 : 0) | (a23 ? 8388608 : 0) | (a24 ? 16777216 : 0) | (a25 ? 33554432 : 0) | (a26 ? 67108864 : 0) | (a27 ? 134217728 : 0) | (a28 ? 268435456 : 0) | (a29 ? 536870912 : 0) | (a30 ? 1073741824 : 0), [
  		" c0",
  		" c1",
  		" c2",
  		" c3",
  		" c4",
  		" c5",
  		" c6",
  		" c7",
  		" c8",
  		" c9",
  		" c10",
  		" c11",
  		" c12",
  		" c13",
  		" c14",
  		" c15",
  		" c16",
  		" c17",
  		" c18",
  		" c19",
  		" c20",
  		" c21",
  		" c22",
  		" c23",
  		" c24",
  		" c25",
  		" c26",
  		" c27",
  		" c28",
  		" c29",
  		" c30"
  	]));
  	return _n0;
  })();
  "#);
  assert!(code.contains("_setClassName"));
  assert!(code.contains("(a30 ? 1073741824 : 0)"));
  assert!(!code.contains("_setClass(_n0, {"));
}

#[test]
fn computed_object_class_key_falls_back_to_set_class() {
  let code = transform(r#"<div class={{ [name]: active }} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClass(_n0, { [name]: active }));
  	return _n0;
  })();
  "#);

  assert!(code.contains("_setClass(_n0, { [name]: active })"));
  assert!(!code.contains("_setClassName"));
}

#[test]
fn array_class_falls_back_to_set_class() {
  let code = transform(r#"<div class={[foo, { danger: active }]} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setClass(_n0, [foo, { danger: active }]));
  	return _n0;
  })();
  "#);

  assert!(code.contains("_setClass(_n0, [foo, { danger: active }])"));
  assert!(!code.contains("_setClassName"));
}

#[test]
fn class_with_v_bind_object_falls_back_to_dynamic_props() {
  let code = transform(
    r#"<div class="foo" class={{ bar: isBar }} {...mayBeHasClass} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setDynamicProps(_n0, [{ class: ["foo", { bar: isBar }] }, mayBeHasClass]));
  	return _n0;
  })();
  "#);

  assert!(
    code.contains(r#"_setDynamicProps(_n0, [{ class: ["foo", { bar: isBar }] }, mayBeHasClass])"#)
  );
  assert!(!code.contains("_setClassName"));
}

#[test]
fn custom_element_number_literals() {
  // Custom element props are passed along as raw values instead of being
  // stringified into the template, so number literals must keep their type.
  let code = transform(
    r#"<number-probe count={0} ratio={1.5} bigint={1n} str={'0'} text={`0`} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createPlainElement as _createPlainElement } from "vue";
  (() => {
  	const _n0 = _createPlainElement("number-probe", {
  		count: 0,
  		ratio: 1.5,
  		bigint: 1n,
  		str: "0",
  		text: "0"
  	}, null, true);
  	return _n0;
  })();
  "#);

  assert!(code.contains("count: 0"));
  assert!(code.contains("ratio: 1.5"));
  assert!(code.contains("bigint: 1n"));
  assert!(code.contains(r#"str: "0""#));
  assert!(code.contains(r#"text: "0""#));
}

#[test]
fn custom_element_number_literals_with_dynamic_key() {
  // jsx cannot express a dynamic attribute name, so the equivalent of upstream
  // `:[key]="0"` is a computed key inside a spread.
  let code = transform("<number-probe {...{[key]: 0}} />", None).code;
  assert!(code.contains("[key]: 0"), "{code}");
}

#[test]
fn custom_element_number_literals_with_spread_props() {
  let code = transform("<number-probe {...props} count={0} />", None).code;
  assert!(code.contains("{ count: 0 }"), "{code}");
}

#[test]
fn custom_element_number_literals_with_v_bind_object() {
  let code = transform("<number-probe {...{count: 0}} />", None).code;
  assert!(code.contains("{ count: 0 }"), "{code}");
}

// upstream: `v-model value number literals`
#[test]
fn model_value_prop_number_literals_stay_raw() {
  let code = transform(
    r#"<input type="checkbox" value={1} true-value={1} false-value={0} />"#,
    None,
  )
  .code;

  assert!(code.contains("_setValue(_n0, 1)"), "{code}");
  assert!(code.contains(r#"_setAttr(_n0, "true-value", 1)"#), "{code}");
  assert!(
    code.contains(r#"_setAttr(_n0, "false-value", 0)"#),
    "{code}"
  );
}

// upstream: `textarea and select value literals`. The `value` content attribute
// is inert on both tags, so it has to be assigned as a dom property. Upstream
// compiles the two roots as siblings; jsx needs a fragment to spell that.
#[test]
fn textarea_and_select_value_literals() {
  let code = transform(
    r#"<><textarea value={'hello'}></textarea><select value={'b'}><option value="b"></option></select></>"#,
    None,
  )
  .code;

  assert!(code.contains(r#"_template("<textarea>")"#), "{code}");
  assert!(
    code.contains(r#"_template("<select><option value=b>")"#),
    "{code}"
  );
  assert!(code.contains(r#"_setValue(_n0, "hello")"#), "{code}");
  assert!(code.contains(r#"_setValue(_n1, "b")"#), "{code}");
}

// upstream: `number literals with %s` (jsx spelling of each case)
#[test]
fn number_literals_with_v_model_value_props() {
  for (source, expected) in [
    // v-model reads these back off the element, so they stay raw values
    (r#"<input value={1} />"#, "_setValue(_n0, 1)"),
    (r#"<input value={1n} />"#, "_setValue(_n0, 1n)"),
    (r#"<option value={1}></option>"#, "_setValue(_n0, 1)"),
    (r#"<textarea value={1}></textarea>"#, "_setValue(_n0, 1)"),
    (r#"<select value={1}></select>"#, "_setValue(_n0, 1)"),
    (
      r#"<input type="checkbox" true-value={1} />"#,
      r#"_setAttr(_n0, "true-value", 1)"#,
    ),
    (
      r#"<input type="checkbox" false-value={0} />"#,
      r#"_setAttr(_n0, "false-value", 0)"#,
    ),
    // the type is only known at runtime, so it may still be a checkbox
    (
      r#"<input type={type} true-value={1} />"#,
      r#"_setAttr(_n0, "true-value", 1)"#,
    ),
    // a spread may carry the `type` that makes it a checkbox
    (
      r#"<input {...attrs} true-value={1} />"#,
      r#""true-value": 1"#,
    ),
    // `.prop` goes through the same `setValue`
    (r#"<input value_prop={1} />"#, "_setValue(_n0, 1)"),
    // boolean attributes are folded from the value's own type
    (
      r#"<input disabled={0} />"#,
      r#"_setProp(_n0, "disabled", 0)"#,
    ),
    (r#"<div hidden={0} />"#, r#"_setProp(_n0, "hidden", 0)"#),
    // still stringified into the template
    (r#"<div value={1} />"#, r#"_template("<div value=1>""#),
    (r#"<input value={'1'} />"#, r#"_template("<input value=1>""#),
    (r#"<input size={2} />"#, r#"_template("<input size=2>""#),
    // `true-value` is only read back on a checkbox
    (
      r#"<input true-value={1} />"#,
      r#"_template("<input true-value=1>""#,
    ),
    (
      r#"<input type="text" true-value={1} />"#,
      "_template(\"<input type=text true-value=1>\"",
    ),
    // checkbox values stay raw even when forced through `setAttr`
    (
      r#"<input type="checkbox" true-value_attr={1} />"#,
      r#"_setAttr(_n0, "true-value", 1)"#,
    ),
    (
      r#"<input type="checkbox" false-value_attr={0} />"#,
      r#"_setAttr(_n0, "false-value", 0)"#,
    ),
    // special boolean attributes still inspect the raw value in `setAttr`
    (
      r#"<input readonly_attr={0} />"#,
      r#"_setAttr(_n0, "readonly", 0)"#,
    ),
    // these `.attr` bindings only need the serialized attribute value
    (
      r#"<input value_attr={1} />"#,
      r#"_template("<input value=1>""#,
    ),
    (
      r#"<input disabled_attr={0} />"#,
      r#"_template("<input disabled=0>""#,
    ),
  ] {
    let code = transform(source, None).code;
    assert!(code.contains(expected), "{source}\n{code}");
  }
}

// upstream: `:[key]="0"` and `v-bind="{...}"` cannot be expressed as a dynamic
// attribute name in jsx; a dynamic key inside a spread is the equivalent and is
// always applied at runtime, so it never reaches the template.
#[test]
fn number_literals_with_dynamic_key() {
  let code = transform(r#"<div {...{[key]: 0}} />"#, None).code;
  assert!(code.contains("[key]: 0"), "{code}");
}
