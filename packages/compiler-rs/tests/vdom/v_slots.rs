use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_slot_basic() {
  let code = transform(
    r#"<Comp v-slots={{ default: () => <>{<span/>}</> }}></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, { default: () => (() => {
      return _openBlock(), _createBlock(_Fragment, null, [_normalizeVNode(() => (() => {
        return _openBlock(), _createElementBlock("span");
      })(), 1)], 64);
    })() }, 0);
  })();
  "#);
}

#[test]
fn v_slot_with_v_slots() {
  let code = transform(
    "<Comp bar={bar} v-slots={{
        bar,
        default: ({ foo })=> <>
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
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, { bar }, {
      bar,
      default: ({ foo }) => (() => {
        return _openBlock(), _createBlock(_Fragment, null, [_normalizeVNode(() => foo + bar, 1), _normalizeVNode(() => (() => {
          return _openBlock(), _createBlock(Comp, null, {
            default: _withCtx(({ baz }) => [_normalizeVNode(() => bar, 1), _normalizeVNode(() => baz, 1)]),
            _: 1
          });
        })(), 1)], 64);
      })()
    }, 8, ["bar"]);
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
