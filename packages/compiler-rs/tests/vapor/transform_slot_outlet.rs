use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn default_slot_outlet() {
  let code = transform(
    r#"<slot>
    </slot>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("default");
  	return _n0;
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet() {
  let code = transform(r#"<slot name="foo" />"#, None).code;
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
  let code = transform(r#"<slot name={foo} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot(() => foo);
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_props() {
  let code = transform(r#"<slot foo="bar" baz={qux} foo-bar={foo-bar} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("default", {
  		foo: () => "bar",
  		baz: () => qux,
  		"foo-bar": () => foo - bar
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet_with_props() {
  let code = transform(r#"<slot name="foo" foo="bar" baz={qux} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("foo", {
  		foo: () => "bar",
  		baz: () => qux
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet_with_props() {
  let code = transform(r#"<slot name={foo} foo="bar" baz={qux} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot(() => foo, {
  		foo: () => "bar",
  		baz: () => qux
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_fallback() {
  let code = transform(r#"<slot><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>");
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
  let code = transform(r#"<slot name="foo"><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>");
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
  let code = transform(r#"<slot foo={bar}><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>");
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
  let code = transform(r#"<slot name="foo" foo={bar}><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>");
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
fn error_on_unexpected_cunstom_directive_on_slot() {
  let error = RefCell::new(None);
  transform(
    r#"<slot v-foo />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(
    *error.borrow(),
    Some(ErrorCodes::VSlotUnexpectedDirectiveOnSlotOutlet)
  );
}
