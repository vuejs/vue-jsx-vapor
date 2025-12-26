use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<div onClick={onClick}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = (...args) => onClick(...args)) });
  })();
  "#)
}

#[test]
fn call_expression() {
  let code = transform(
    r#"<div onClick={foo($event)}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = foo($event)) });
  })();
  "#)
}

#[test]
fn arrow_function_expression() {
  let code = transform(
    r#"<div onClick={$event => foo($event)}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = ($event) => foo($event)) });
  })();
  "#)
}

#[test]
fn async_arrow_function_expression() {
  let code = transform(
    r#"<div onClick={async $event => foo($event)}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = async ($event) => foo($event)) });
  })();
  "#)
}

#[test]
fn function_expression() {
  let code = transform(
    r#"<div onClick={function($event) {
      foo($event)
    }}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = function($event) {
      foo($event);
    }) });
  })();
  "#)
}

#[test]
fn complex_memeber_expression() {
  let code = transform(
    r#"<div onClick={a['b' + c]}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = (...args) => a["b" + c](...args)) });
  })();
  "#)
}

#[test]
fn should_error_if_no_expression_and_no_modifier() {
  let error = RefCell::new(None);
  transform(
    r#"<input onClick />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VOnNoExpression));
}

#[test]
fn should_not_error_if_no_expression_but_has_modifier() {
  let code = transform(
    r#"<input onClick_prevent />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("input", { onClick: _cache[0] || (_cache[0] = _withModifiers(() => {}, ["prevent"])) });
  })();
  "#);
}

#[test]
fn do_not_case_conversion_for_kebab_case_events() {
  let code = transform(
    r#"<input onFoo-bar="onMount />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @"");
}

#[test]
fn vue_prefixed_events() {
  let code = transform(
    r#"<div onVue:mounted={onMount} onVue:beforeUpdate={onBeforeUpdate} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", {
      onVnodeMounted: _cache[0] || (_cache[0] = (...args) => onMount(...args)),
      onVnodeBeforeUpdate: _cache[1] || (_cache[1] = (...args) => onBeforeUpdate(...args))
    }, null, 512);
  })();
  "#);
}

#[test]
fn empty_handler() {
  let code = transform(
    r#"<div onClick_prevent />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = _withModifiers(() => {}, ["prevent"])) });
  })();
  "#);
}

#[test]
fn member_expression_handler() {
  let code = transform(
    r#"<div onClick={foo.bar} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = (...args) => foo.bar(...args)) });
  })();
  "#);
}

#[test]
fn bail_on_component_member_expression_handler() {
  let code = transform(
    r#"<comp onClick={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(comp, { onClick: foo }, null, 8, ["onClick"]);
  })();
  "#);
}

#[test]
fn should_not_be_cached_inside_v_once() {
  let code = transform(
    r#"<div v-once><div onClick={foo}/></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementVNode as _createElementVNode, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div", null, [_createElementVNode("div", { onClick: foo }, null, 8, ["onClick"])])).cacheIndex = 0, _setBlockTracking(1), _cache[0]);
  })();
  "#);
}

#[test]
fn unicode_identifier_from_v_for_should_not_be_cached() {
  let code = transform(
    r#"<div v-for={项 in items} key={value}><div onClick={foo(项)}/></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(items, (项) => (_openBlock(), _createElementBlock("div", { key: value }, [_createElementVNode("div", { onClick: foo(项) }, null, 8, _hoisted_1)]))), 128);
  })();
  "#);
}

#[test]
fn identifier_from_v_slot_should_not_be_cached() {
  let code = transform(
    r#"<Comp v-slot={{ item }}><div onClick={foo(item)}/></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx(({ item }) => [_createElementVNode("div", { onClick: foo(item) }, null, 8, _hoisted_1)]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn should_support_multiple_modifiers() {
  let code = transform(
    r#"<div onClick_stop_prevent={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = _withModifiers((...args) => test(...args), ["stop", "prevent"])) });
  })();
  "#)
}

#[test]
fn should_support_multiple_events_and_modifiers_options() {
  let code = transform(
    r#"<div onClick_stop={test} onKeyup_enter={test} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", {
      onClick: _cache[0] || (_cache[0] = _withModifiers((...args) => test(...args), ["stop"])),
      onKeyup: _cache[1] || (_cache[1] = _withKeys((...args) => test(...args), ["enter"]))
    }, null, 32);
  })();
  "#);
}

#[test]
fn should_support_multiple_modifiers_and_event_options() {
  let code = transform(
    r#"<div onClick_stop_capture_once={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onClickCaptureOnce: _cache[0] || (_cache[0] = _withModifiers((...args) => test(...args), ["stop"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_keyboard_events_or_dynamic_events() {
  let code = transform(
    r#"<div onKeydown_stop_capture_ctrl_a={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withKeys as _withKeys, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onKeydownCapture: _cache[0] || (_cache[0] = _withKeys(_withModifiers((...args) => test(...args), ["stop", "ctrl"]), ["a"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_not_wrap_keys_guard_if_no_key_modifier_is_present() {
  let code = transform(
    r#"<div onKeyup_exact={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onKeyup: _cache[0] || (_cache[0] = _withModifiers((...args) => test(...args), ["exact"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_wrap_keys_guard_for_static_key_event_with_left_right_modifiers() {
  let code = transform(
    r#"<div onKeyup_left={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withKeys as _withKeys } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onKeyup: _cache[0] || (_cache[0] = _withKeys((...args) => test(...args), ["left"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_not_wrap_normal_guard_if_there_is_only_keys_guard() {
  let code = transform(
    r#"<div onKeyup_enter={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withKeys as _withKeys } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onKeyup: _cache[0] || (_cache[0] = _withKeys((...args) => test(...args), ["enter"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_transform_click_right() {
  let code = transform(
    r#"<div onClick_right={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onContextmenu: _cache[0] || (_cache[0] = _withModifiers((...args) => test(...args), ["right"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_transform_click_middle() {
  let code = transform(
    r#"<div onClick_middle={test}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withModifiers as _withModifiers } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onMouseup: _cache[0] || (_cache[0] = _withModifiers((...args) => test(...args), ["middle"])) }, null, 32);
  })();
  "#);
}

#[test]
fn cache_handler_with_modifiers() {
  let code = transform(
    r#"<div onKeyup_enter_capture={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withKeys as _withKeys } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onKeyupCapture: _cache[0] || (_cache[0] = _withKeys((...args) => foo(...args), ["enter"])) }, null, 32);
  })();
  "#);
}

#[test]
fn should_not_have_props_patch_flag_for_constant_v_on_handlers_with_modifiers() {
  let code = transform(
    r#"<div onKeydown_up={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withKeys as _withKeys } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createElementBlock("div", { onKeydown: _cache[0] || (_cache[0] = _withKeys((...args) => foo(...args), ["up"])) }, null, 32);
  })();
  "#);
}
