use std::cell::RefCell;

use common::error::ErrorCodes;
use compiler::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn default_slot_outlet() {
  let code = transform(
    r#"<slot>
    </slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot();
  	return _n0;
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet() {
  let code = transform(
    r#"<slot name="foo" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("foo");
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet() {
  let code = transform(
    r#"<slot name={foo} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot(() => foo);
  	return _n0;
  })();
  "#);
}

#[test]
fn slot_outlet_with_number_literal_props() {
  // slot props are passed along as raw values, so number literals must not
  // be stringified the way they are for plain element attributes
  let code = transform(
    r#"<slot count={0} level={1} ratio={1.5} str="1" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains("count: 0"), "{code}");
  assert!(code.contains("level: 1"), "{code}");
  assert!(code.contains("ratio: 1.5"), "{code}");
  assert!(code.contains(r#"str: "1""#), "{code}");
}

#[test]
fn default_slot_outlet_with_props() {
  let code = transform(
    r#"<slot foo="bar" baz={qux} foo-bar={foo-bar} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("default", {
  		foo: "bar",
  		baz: () => qux,
  		"foo-bar": () => foo - bar
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet_with_props() {
  let code = transform(
    r#"<slot name="foo" foo="bar" baz={qux} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("foo", {
  		foo: "bar",
  		baz: () => qux
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet_with_props() {
  let code = transform(
    r#"<slot name={foo} foo="bar" baz={qux} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot(() => foo, {
  		foo: "bar",
  		baz: () => qux
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_fallback() {
  let code = transform(
    r#"<slot><div /></slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("default", null, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn named_slot_outlet_with_fallback() {
  let code = transform(
    r#"<slot name="foo"><div /></slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("foo", null, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_props_and_fallback() {
  let code = transform(
    r#"<slot foo={bar}><div /></slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("default", { foo: () => bar }, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn named_slot_outlet_with_props_and_fallback() {
  let code = transform(
    r#"<slot name="foo" foo={bar}><div /></slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("foo", { foo: () => bar }, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn root_v_if_fallbck() {
  let code = transform(
    r#"<slot><span v-if={ok}/></slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  (() => {
  	const _n0 = _createSlot("default", null, () => {
  		const _n1 = _createIf(() => ok, () => {
  			const _n3 = _t0();
  			return _n3;
  		});
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn nested_root_v_for_fallbck() {
  let code = transform(
    r#"<slot><template v-if={ok}><span v-for={item in items}>{ item }</span></template></slot>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx/vapor";
  import { createFor as _createFor, createIf as _createIf, createSlot as _createSlot, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  (() => {
  	const _n0 = _createSlot("default", null, () => {
  		const _n1 = _createIf(() => ok, () => {
  			const _n3 = _createFor(() => items, (_for_item0) => {
  				const _n5 = _t0();
  				const _x5 = _txt(_n5);
  				_setNodes(_x5, () => _for_item0.value);
  				return _n5;
  			}, void 0, 8);
  			return _n3;
  		});
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn does_not_mark_non_root_fallback_v_if_as_slot_root() {
  let code = transform(
    r#"<div><span v-if={ok}/></div>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  const _t1 = _template("<div>", 1);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3);
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	return _n3;
  })();
  "#);
}

#[test]
fn error_on_unexpected_cunstom_directive_on_slot() {
  let error = RefCell::new(None);
  compiler::transform(
    r#"<slot v-foo />"#,
    Some(TransformOptions {
      on_error: Box::new(|e, _| *error.borrow_mut() = Some(e)),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(
    *error.borrow(),
    Some(ErrorCodes::VSlotUnexpectedDirectiveOnSlotOutlet.message())
  );
}

#[test]
fn slot_outlet_key_is_also_a_slot_prop() {
  let code = transform(
    r#"<slot key={id} x={1} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  // same as vdom: the outlet keeps its key as a prop and is keyed as well
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createKeyedFragment(() => id, () => {
  		const _n2 = _createSlot("default", {
  			key: () => id,
  			x: 1
  		});
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn static_slot_outlet_key_is_also_a_slot_prop() {
  let code = transform(
    r#"<slot key="s" x={1} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("default", {
  		key: "s",
  		x: 1
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_slot_outlet_key_is_also_a_slot_prop() {
  let code = transform(
    r#"<slot v-for={i in list} key={i} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  // the loop keys the rows itself, so the outlet gets no keyed fragment
  assert!(!code.contains("_createKeyedFragment"));
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createSlot("default", { key: () => _for_item0.value });
  		return _n2;
  	}, (i) => i, 16);
  	return _n0;
  })();
  "#);
}

#[test]
fn normalizes_static_slot_keys_inside_their_own_getters() {
  let code = transform(
    r#"<slot class={classes()} style={styles()} title={title()} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(
    code.contains("class: () => _normalizeClass(classes())"),
    "{code}"
  );
  assert!(
    code.contains("style: () => _normalizeStyle(styles())"),
    "{code}"
  );
  assert!(code.contains("title: () => title()"), "{code}");
  assert!(!code.contains("_normalizeProps"), "{code}");
  assert!(!code.contains("_mergeProps"), "{code}");
}

#[test]
fn keeps_modified_class_and_style_keys_as_ordinary_props() {
  let code = transform(
    r#"<slot class_prop={classes} style_attr={styles} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(code.contains(r#"".class": () => classes"#), "{code}");
  assert!(code.contains(r#""^style": () => styles"#), "{code}");
  assert!(!code.contains("_normalizeClass"), "{code}");
  assert!(!code.contains("_normalizeStyle"), "{code}");
}

#[test]
fn trims_a_static_class_but_not_a_static_style() {
  let code = transform(
    r#"<slot class=" a  b " style="color: red" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(code.contains(r#"class: "a  b""#), "{code}");
  assert!(code.contains(r#"style: "color: red""#), "{code}");
}

#[test]
fn merges_a_static_and_a_dynamic_class_into_one_normalized_prop() {
  let code = transform(
    r#"<slot class="a" class={c} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(
    code.contains(r#"class: () => _normalizeClass(["a", c])"#),
    "{code}"
  );
}

#[test]
fn preserves_the_normalization_boundary_of_a_spread() {
  for (src, expected) in [
    (
      r#"<slot style={s} {...{ title: 't' }} />"#,
      r#"_mergeProps({ style: s }, { title: "t" })"#,
    ),
    (
      r#"<slot {...{ title: 't' }} style={s} />"#,
      r#"_mergeProps({ title: "t" }, { style: s })"#,
    ),
    (
      r#"<slot {...{ class: c, style: s }} />"#,
      "_normalizeProps(_guardReactiveProps({",
    ),
    (
      r#"<slot {...(void 0)} />"#,
      "_normalizeProps(_guardReactiveProps(void 0))",
    ),
    (
      r#"<slot {...obj} />"#,
      "_normalizeProps(_guardReactiveProps(obj))",
    ),
  ] {
    let code = transform(
      src,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert!(code.contains(expected), "{src}\n{code}");
  }
}

#[test]
fn slot_listeners_are_props() {
  let code = transform(
    r#"<slot onClick={foo} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(code.contains("onClick: () => foo"), "{code}");
  assert!(!code.contains("_on("), "{code}");
}

#[test]
fn slot_v_on_object_is_a_props_group() {
  let code = transform(
    r#"<slot v-on={obj} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(code.contains("{ $: [() => _toHandlers(obj)] }"), "{code}");
}

#[test]
fn merges_a_slot_listener_with_a_v_on_object() {
  let code = transform(
    r#"<slot onClick={foo} v-on={bar} baz="qux" />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;

  assert!(
    code.contains(r#"_mergeProps({ onClick: foo }, _toHandlers(bar), { baz: "qux" })"#),
    "{code}"
  );
}
