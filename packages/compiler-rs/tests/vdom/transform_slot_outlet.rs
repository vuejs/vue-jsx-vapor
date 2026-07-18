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
  	const _cache = _createVNodeCache("631d214bc2c8427c");
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
  	const _cache = _createVNodeCache("631d214bc2c8427c");
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
  	const _cache = _createVNodeCache("631d214bc2c8427c");
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
  	const _cache = _createVNodeCache("631d214bc2c8427c");
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
  	const _cache = _createVNodeCache("631d214bc2c8427c");
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", { foo: bar }, () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]);
  })();
  "#);
}

#[test]
fn slots_component() {
  let code = transform(
    r#"<slots.foo foo={bar}><div /></slots.foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementVNode as _createElementVNode, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache("631d214bc2c8427c");
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", { foo: bar });
  })();
  "#);
}

#[test]
fn dollor_slots_component() {
  let code = transform(
    r#"<this.$slots.foo foo={bar}><div /></this.$slots.foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementVNode as _createElementVNode, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _cache = _createVNodeCache("631d214bc2c8427c");
  	const _slots = _useSlots();
  	return _renderSlot(_slots, "foo", { foo: bar });
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

#[test]
fn v_if_on_slot_with_v_bind() {
  let code = transform(
    r#"<slot v-if={ok} {...items}></slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, guardReactiveProps as _guardReactiveProps, normalizeProps as _normalizeProps, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	const _slots = _useSlots();
  	return ok ? _renderSlot(_slots, "default", _normalizeProps(_guardReactiveProps(items)), undefined, undefined, 0) : _createCommentVNode("", true);
  })();
  "#);
}
