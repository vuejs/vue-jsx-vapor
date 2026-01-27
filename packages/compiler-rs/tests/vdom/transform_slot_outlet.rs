use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn default_slot_outlet() {
  let code = transform(
    r#"<slot>
    </slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "default", {});
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet() {
  let code = transform(
    r#"<slot name="foo">foo</slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", {}, () => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))]);
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet() {
  let code = transform(
    r#"<slot name={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _slots = _useSlots();
  	return _renderSlot(_slots, foo);
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_props() {
  let code = transform(
    r#"<slot foo="bar" baz={qux} foo-bar={foo-bar} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "default", {
  		foo: "bar",
  		baz: qux,
  		"foo-bar": foo - bar
  	});
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet_with_props() {
  let code = transform(
    r#"<slot name="foo" foo="bar" baz={qux} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", {
  		foo: "bar",
  		baz: qux
  	});
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet_with_props() {
  let code = transform(
    r#"<slot name={foo} foo="bar" baz={qux} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _slots = _useSlots();
  	return _renderSlot(_slots, foo, {
  		foo: "bar",
  		baz: qux
  	});
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_fallback() {
  let code = transform(
    r#"<slot><div /></slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementVNode as _createElementVNode, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "default", {}, () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]);
  })();
  "#);
}

#[test]
fn named_slot_outlet_with_fallback() {
  let code = transform(
    r#"<slot name="foo"><div /></slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementVNode as _createElementVNode, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", {}, () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]);
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_props_and_fallback() {
  let code = transform(
    r#"<slot foo={bar}><div /></slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementVNode as _createElementVNode, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "default", { foo: bar }, () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]);
  })();
  "#);
}

#[test]
fn named_slot_outlet_with_props_and_fallback() {
  let code = transform(
    r#"<slot name="foo" foo={bar}><div /></slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementVNode as _createElementVNode, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", { foo: bar }, () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]);
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
