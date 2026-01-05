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
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(), _createBlock(Comp, null, { default: ({ foo }) => (() => {
      return _openBlock(), _createBlock(_Fragment, null, [_normalizeVNode(() => (() => {
        return _withDirectives((_openBlock(), _createElementBlock("input", {
          "onUpdate:modelValue": ($event) => bar = $event,
          onClick: () => foo
        }, null, 8, _hoisted_1)), [[_vModelText, bar]]);
      })())], 64);
    })() }, 1024);
  })();
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
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(), _createBlock(Comp, null, ({ foo }) => (() => {
      return _openBlock(), _createElementBlock("div", { onClick: () => foo }, null, 8, _hoisted_1);
    })(), 1024);
  })();
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
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(), _createBlock(Comp, null, { default: ({ foo }) => (() => {
      return _withDirectives((_openBlock(), _createElementBlock("input", {
        "onUpdate:modelValue": ($event) => bar = $event,
        onClick: () => foo
      }, null, 8, _hoisted_1)), [[_vModelText, bar]]);
    })() }, 1024);
  })();
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
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, { [foo]: () => (() => {
      const _cache = _createVNodeCache(0);
      return _openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))], 64);
    })() }, 1024);
  })();
  "#);
}

#[test]
fn v_slot_with_v_slots() {
  let code = transform(
    "<Comp bar={bar} v-slots={{
        bar,
        default: ({ foo }) => <>
          { foo + bar }
          {<Comp v-slot={{baz}}>{bar}{baz}</Comp>}
        </>
      }}>
    </Comp>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, { bar }, {
      bar,
      default: ({ foo }) => (() => {
        return _openBlock(), _createBlock(_Fragment, null, [_normalizeVNode(() => foo + bar), _normalizeVNode(() => (() => {
          return _openBlock(), _createBlock(Comp, null, {
            default: _withCtx(({ baz }) => [_normalizeVNode(() => bar), _normalizeVNode(() => baz)]),
            _: 2
          }, 1024);
        })())], 64);
      })()
    }, 1032, ["bar"]);
  })();
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
