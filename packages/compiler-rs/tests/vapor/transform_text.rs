use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn static_template() {
  let code = transform(
    "<div>
      <div>hello</div>
      <input />
      <span />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>hello</div><input><span>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn interpolation() {
  let code = transform("<>{ 1 }{ 2 }{a +b +       c }</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes(1, 2, () => a + b + c);
  	return _n0;
  })();
  "#);
}

#[test]
fn on_consecutive_text() {
  let code = transform("<>{ \"hello world\" }</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes("hello world");
  	return _n0;
  })();
  "#);
}

#[test]
fn consecutive_text() {
  let code = transform("<>{ msg }  <div/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n2 = _t0();
  	const _n0 = _createNodes(() => msg, " ");
  	return [_n0, _n2];
  })();
  "#);
}

#[test]
fn escapes_raw_static_text() {
  let code = transform("<div>&nbsp;</div>", None).code;

  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>\xA0", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn escapes_raw_static_text_when_generating_the_template_string() {
  let code = transform(
    "<code>
      &nbsp;&lt;script&gt;&nbsp;
    </code>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<code>\xA0&lt;script&gt;\xA0", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_quotes_in_root_level_text_nodes() {
  let code = transform(r#"<>Howdy y'all</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("Howdy y'all", 2);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_double_quotes_in_root_level_text_nodes() {
  let code = transform(r#"<>Say "hello"</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("Say \"hello\"", 2);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_quotes_in_template_v_if_text() {
  // Text inside <template> tag also goes through createNode()
  let code = transform(r#"<template v-if="ok">Howdy y'all</template>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("Howdy y'all", 2);
  (() => {
  	const _n0 = _createIf(() => "ok", () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 50);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_quotes_in_component_slot_text() {
  // Text inside component (slot content) also goes through createNode()
  let code = transform("<Comp>Howdy y'all</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("Howdy y'all", 2);
  (() => {
  	const _n1 = _createComponent(Comp, null, () => {
  		const _n0 = _t0();
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn text_like() {
  let code = transform("<div>{ (2) }{`foo${1}`}{1}{1n}</div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>2foo111", 1);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn conditional_expression() {
  let code = transform(
    "<>{ok? (<span>{msg}</span>) : fail ? (<div>fail</div>)  : null }</>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  const _t1 = _template("<div>fail", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => msg);
  		return _n2;
  	}, () => _createIf(() => fail, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, () => {
  		const _n6 = _createNodes(null);
  		return _n6;
  	}, 545), 261);
  	return _n0;
  })();
  "#);
}

#[test]
fn multiple_conditional() {
  let code = transform("<>{ok? ok : fail} {foo ? foo : <span />}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template(" ", 2);
  const _t1 = _template("<span>", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _createNodes(() => ok);
  		return _n2;
  	}, () => {
  		const _n4 = _createNodes(() => fail);
  		return _n4;
  	}, 266);
  	const _n5 = _t0();
  	const _n6 = _createIf(() => foo, () => {
  		const _n8 = _createNodes(() => foo);
  		return _n8;
  	}, () => {
  		const _n10 = _t1();
  		return _n10;
  	}, 582);
  	return [
  		_n0,
  		_n5,
  		_n6
  	];
  })();
  "#);
}

#[test]
fn text_before_parenthesized_conditional_is_kept() {
  // `(ok ? a : b)` is a conditional, not an interpolation: the parentheses must
  // not make the leading text look like a text run that the conditional joins.
  let code = transform("<>a{(ok ? a : b)}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("a", 2);
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createIf(() => ok, () => {
  		const _n3 = _createNodes(() => a);
  		return _n3;
  	}, () => {
  		const _n5 = _createNodes(() => b);
  		return _n5;
  	}, 266);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn text_before_parenthesized_conditional_in_element_is_kept() {
  // Same as above, but inside a plain element whose later interpolation makes
  // the parent scan for text runs: the scan must not treat the conditional as
  // an interpolation either, or `a` is dropped from the template.
  let code = transform("<div>a{(ok ? a : b)}{x}<span/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>a<!> <span>", 1);
  (() => {
  	const _n7 = _t0();
  	const _n6 = _next(_child(_n7));
  	const _n5 = _next(_n6, true);
  	_setInsertionState(_n7, _n6);
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _createNodes(() => a);
  		return _n2;
  	}, () => {
  		const _n4 = _createNodes(() => b);
  		return _n4;
  	}, 266);
  	_setNodes(_n5, () => x);
  	return _n7;
  })();
  "#);
}

#[test]
fn logical_expression() {
  let code = transform("<>{ok && (<div>{msg}</div>)}</>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _createNodes(() => ok && (() => {
  		const _n0 = _t0();
  		const _x0 = _txt(_n0);
  		_setNodes(_x0, () => msg);
  		return _n0;
  	})());
  	return _n0;
  })();
  "#);
}

#[test]
fn logical_expression_or() {
  let code = transform(r#"<div>{foo || <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo || (() => {
  		const _n0 = _t0();
  		const _x0 = _txt(_n0);
  		_setNodes(_x0, () => foo);
  		return _n0;
  	})());
  	return _n0;
  })();
  "#)
}

#[test]
fn logical_expression_coalesce() {
  let code = transform(r#"<div>{foo ?? <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo ?? (() => {
  		const _n0 = _t0();
  		const _x0 = _txt(_n0);
  		_setNodes(_x0, () => foo);
  		return _n0;
  	})());
  	return _n0;
  })();
  "#)
}

#[test]
fn expression_map() {
  let code = transform(
    "<>{Array.from({ length: count.value }).map((_, index) => {
      if (index > 1) {
        return <div>1</div>
      } else {
        return [<span>({index}) lt 1</span>, <br />]
      }
    })}</>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div>1", 3);
  const _t1 = _template("<span> ", 1);
  const _t2 = _template("<br>", 3);
  (() => {
  	const _n0 = _createNodes(() => Array.from({ length: count.value }).map((_, index) => {
  		if (index > 1) {
  			return (() => {
  				const _n0 = _t0();
  				return _n0;
  			})();
  		} else {
  			return [(() => {
  				const _n0 = _t1();
  				const _x0 = _txt(_n0);
  				_setNodes(_x0, "(", () => index, ") lt 1");
  				return _n0;
  			})(), (() => {
  				const _n0 = _t2();
  				return _n0;
  			})()];
  		}
  	}));
  	return _n0;
  })();
  "#);
}

#[test]
fn expression_with_comment() {
  let code = transform(
    r#"<div>
      {foo}
      {/**/}
      <a></a>
    </div>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, template as _template } from "vue";
  const _t0 = _template("<div> <a>", 1);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _child(_n1, true);
  	_setNodes(_n0, () => foo);
  	return _n1;
  })();
  "#)
}

#[test]
fn slot_interpolation() {
  let code = transform(r#"<Comp>{Hello}</Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent, normalizeVaporSlots as _normalizeVaporSlots } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, null, { $: [() => _normalizeVaporSlots(Hello)] }, true);
  	return _n0;
  })();
  "#)
}

#[test]
fn slot_literal_interpolation() {
  let code = transform(r#"<Comp>{ "Hello" }</Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createNodes("Hello");
  		return _n0;
  	}, { _: 1 }), true);
  	return _n1;
  })();
  "#)
}

#[test]
fn fragment_with_interpolation() {
  let code = transform(r#"<>Message: { "Hello" }!</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes("Message: ", "Hello", "!");
  	return _n0;
  })();
  "#)
}

#[test]
fn fragment_with_empty_interpolation() {
  let code = transform(
    r#"<>
      Parent
      {/* ... */}
    </>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("Parent", 2);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#)
}

#[test]
fn text_references_among_element_children() {
  let code = transform(r#"<p>{before}<br id={id}/>{between}<br/>{after}</p>"#, None).code;
  assert!(code.contains("const _n0 = _child(_n4, true)"));
  assert!(code.contains("const _n1 = _next(_n0)"));
  assert!(code.contains("const _n2 = _next(_n1, true)"));
  assert!(code.contains("const _n3 = _nthChild(_n4, 4, true)"));
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, nthChild as _nthChild, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<p> <br> <br> ", 1);
  (() => {
  	const _n4 = _t0();
  	const _n0 = _child(_n4, true);
  	const _n1 = _next(_n0);
  	const _n2 = _next(_n1, true);
  	const _n3 = _nthChild(_n4, 4, true);
  	_setNodes(_n0, () => before);
  	_setNodes(_n2, () => between);
  	_setNodes(_n3, () => after);
  	_renderEffect(() => _setProp(_n1, "id", id));
  	return _n4;
  })();
  "#)
}

#[test]
fn empty_literal_next_to_an_element_in_slot_content() {
  // Unlike element children, slot content keeps its empty text node.
  let code = transform("<Comp>{''}<b>{msg}</b></Comp>", None).code;
  assert!(code.contains(r#"_createNodes("")"#), "{code}");
  assert!(code.contains("return [_n0, _n1]"), "{code}");
}

#[test]
fn leading_lt_in_root_level_text_is_materialized() {
  // The runtime parses a template string that starts with "<" as HTML, so a
  // raw text node whose content starts with "<" has to be created imperatively.
  let code = transform("<>&lt;b&gt;<i/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<i>", 2);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _createNodes("<b>");
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn leading_lt_in_component_and_custom_element_text_is_materialized() {
  let code = transform("<Comp>&lt;b&gt;foo&lt;/b&gt;</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createNodes("<b>foo</b>");
  		return _n0;
  	}, { _: 1 }), true);
  	return _n1;
  })();
  "#);

  let code = transform("<my-el>&lt;b&gt;x&lt;/b&gt;</my-el>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createPlainElement as _createPlainElement, extend as _extend } from "vue";
  (() => {
  	const _n1 = _createPlainElement("my-el", null, _extend(() => {
  		const _n0 = _createNodes("<b>x</b>");
  		return _n0;
  	}, { _: 1 }), true);
  	return _n1;
  })();
  "#);
}

#[test]
fn leading_lt_text_adjacent_to_interpolation_is_merged() {
  // Text that an interpolation would collect must not be materialized twice.
  let code = transform("<>&lt;b&gt;{x}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes("<b>", () => x);
  	return _n0;
  })();
  "#);

  let code = transform("<>{x}&lt;b&gt;</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes(() => x, "<b>");
  	return _n0;
  })();
  "#);
}

#[test]
fn leading_lt_text_in_transparent_fragment_inside_template_is_merged() {
  // A `<>` is spliced into its parent while `transform_children` walks the
  // children, i.e. after the parent's own text pre-pass has already run, so the
  // marked-before-interpolation decision can only be made at the child level.
  let code = transform(r#"<template v-if="ok"><>&lt;b&gt;{x}</></template>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf } from "vue";
  (() => {
  	const _n0 = _createIf(() => "ok", () => {
  		const _n2 = _createNodes("<b>", () => x);
  		return _n2;
  	}, null, 17);
  	return _n0;
  })();
  "#);
}

#[test]
fn leading_text_in_transparent_fragment_counts_as_one_unit() {
  // The text is consumed by the interpolation, so it materializes no node of
  // its own in the parent template and must not consume a logical unit.
  // Otherwise the append below would claim unit 2 while the parent template
  // only renders a single text node before it.
  let code = transform(r#"<div><>a{x}<span v-if="ok"/></></div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  const _t1 = _template("<div> ", 1);
  (() => {
  	const _n4 = _t1();
  	const _n0 = _child(_n4, true);
  	_setNodes(_n0, "a", () => x);
  	_setInsertionState(_n4, 1);
  	const _n1 = _createIf(() => "ok", () => {
  		const _n3 = _t0();
  		return _n3;
  	}, null, 49);
  	return _n4;
  })();
  "#);
}

#[test]
fn leading_lt_in_element_text_stays_escaped_in_template() {
  let code = transform("<div>&lt;b&gt;foo&lt;/b&gt;</div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>&lt;b&gt;foo&lt;/b&gt;", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn leading_lt_in_template_directive_text_is_materialized() {
  let code = transform(
    r#"<template v-if="ok">&lt;b&gt;foo&lt;/b&gt;</template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf } from "vue";
  (() => {
  	const _n0 = _createIf(() => "ok", () => {
  		const _n2 = _createNodes("<b>foo</b>");
  		return _n2;
  	}, null, 18);
  	return _n0;
  })();
  "#);

  let code = transform(
    "<template v-for={item in list}>&lt;b&gt;x&lt;/b&gt;</template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor } from "vue";
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createNodes("<b>x</b>");
  		return _n2;
  	}, void 0, 64);
  	return _n0;
  })();
  "#);
}

#[test]
fn leading_lt_in_bare_template_is_not_materialized() {
  // A bare <template> is inlined into the surrounding template and shares the
  // block that locates it, so materializing there would redeclare its variable.
  let code = transform("<div><template>&lt;b&gt;x&lt;/b&gt;</template></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><template>&lt;b&gt;x&lt;/b&gt;", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
  assert_eq!(code.matches("const _n0").count(), 1, "{code}");
}
