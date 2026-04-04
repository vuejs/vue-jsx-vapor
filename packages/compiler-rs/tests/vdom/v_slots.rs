use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_slots_basic() {
  let code = transform(
    r#"<Comp v-slots={{ default: ({foo}) => <>{<input v-model={bar} onClick={() => foo} />}</> }}></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode, normalizeSlot as _normalizeSlot } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createBlock(Comp, null, {
  	_: 1,
  	default: _normalizeSlot(({ foo }) => (_openBlock(), _createElementBlock(_Fragment, null, [_normalizeVNode(() => (() => {
  		const _cache = _createVNodeCache("631d214bc2c8427c");
  		return _withDirectives((_openBlock(), _createElementBlock("input", {
  			"onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => bar = $event),
  			onClick: () => foo
  		}, null, 8, _hoisted_1)), [[_vModelText, bar]]);
  	})())], 64)))
  });
  "#);
}

#[test]
fn function_expression_children() {
  let code = transform(
    r#"<Comp>
      {({ foo }) => <div onClick={() => foo} />}
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeSlot as _normalizeSlot } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createBlock(Comp, null, {
  	_: 1,
  	default: _normalizeSlot(({ foo }) => (_openBlock(), _createElementBlock("div", { onClick: () => foo }, null, 8, _hoisted_1)))
  });
  "#);
}

#[test]
fn object_expression_children() {
  let code = transform(
    r#"<Comp>
      {{ default: ({ foo }) => <input v-model={bar} onClick={() => foo} /> }}
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeSlot as _normalizeSlot } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createBlock(Comp, null, {
  	_: 1,
  	default: _normalizeSlot(({ foo }) => (() => {
  		const _cache = _createVNodeCache("631d214bc2c8427c");
  		return _withDirectives((_openBlock(), _createElementBlock("input", {
  			"onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => bar = $event),
  			onClick: () => foo
  		}, null, 8, _hoisted_1)), [[_vModelText, bar]]);
  	})())
  });
  "#);
}

#[test]
fn object_expression_multiple_children() {
  let code = transform(
    r#"<Comp>
      {{ 
        default: ({ foo }) => <input v-model={bar} onClick={() => foo} />,
        other: () => <div>{foo}</div>
      }}
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode, normalizeSlot as _normalizeSlot } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createBlock(Comp, null, {
  	_: 1,
  	default: _normalizeSlot(({ foo }) => (() => {
  		const _cache = _createVNodeCache("631d214bc2c8427c");
  		return _withDirectives((_openBlock(), _createElementBlock("input", {
  			"onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => bar = $event),
  			onClick: () => foo
  		}, null, 8, _hoisted_1)), [[_vModelText, bar]]);
  	})()),
  	other: _normalizeSlot(() => (_openBlock(), _createElementBlock("div", null, [_normalizeVNode(() => foo)])))
  });
  "#);
}

#[test]
fn object_expression_children_with_computed_property() {
  let code = transform(
    r#"<Comp>
      {{ [foo]: () => <>foo</> }}
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createBlock(Comp, null, { [foo]: () => "foo" }, 1024);
  "#);
}

#[test]
fn v_slot_with_v_slots() {
  let code = transform(
    r#"({ bar }) => <VAutocomplete
      variant={ variant }
      density={ density }
      modelValue={['California']}
      label="selection slot"
      { ...v.props }
    >{{
      selection: ({ item }) => {
        return item + bar
      },
    }}
    </VAutocomplete>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  ({ bar }) => (_openBlock(), _createBlock(VAutocomplete, _mergeProps({
  	variant,
  	density,
  	modelValue: ["California"],
  	label: "selection slot"
  }, v.props), { selection: ({ item }) => {
  	return item + bar;
  } }, 1040, ["variant", "density"]));
  "#)
}

#[test]
fn should_raise_error_if_not_component() {
  let error = RefCell::new(None);
  transform(
    "<div v-slots={obj}></div>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMisplaced));
}

#[test]
fn should_raise_error_if_has_children() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slots={obj}> </Comp>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMixedSlotUsage));
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slots></Comp>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotsNoExpression));
}
