use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn simple_expression() {
  let code = transform(
    r#"<input v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { "onUpdate:modelValue": ($event) => model = $event };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[_vModelText, model]]);
  })();
  "#)
}

#[test]
fn simple_expression_for_input_text() {
  let code = transform(
    r#"<input type="text" v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = {
    type: "text",
    "onUpdate:modelValue": ($event) => model = $event
  };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[_vModelText, model]]);
  })();
  "#);
}

#[test]
fn simple_expression_for_input_radio() {
  let code = transform(
    r#"<input type="radio" v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelRadio as _vModelRadio, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = {
    type: "radio",
    "onUpdate:modelValue": ($event) => model = $event
  };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[_vModelRadio, model]]);
  })();
  "#);
}

#[test]
fn simple_expression_for_input_checkbox() {
  let code = transform(
    r#"<input type="checkbox" v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelCheckbox as _vModelCheckbox, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = {
    type: "checkbox",
    "onUpdate:modelValue": ($event) => model = $event
  };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[_vModelCheckbox, model]]);
  })();
  "#);
}

#[test]
fn simple_expression_for_input_dynamic_type() {
  let code = transform(
    r#"<input type={foo} v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelDynamic as _vModelDynamic, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["type"];
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", {
      type: foo,
      "onUpdate:modelValue": ($event) => model = $event
    }, null, 8, _hoisted_1)), [[_vModelDynamic, model]]);
  })();
  "#);
}

#[test]
fn input_with_dynamic_v_bind() {
  let code = transform(
    r#"<input {...obj} v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock, vModelDynamic as _vModelDynamic, withDirectives as _withDirectives } from "vue";
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _mergeProps(obj, { "onUpdate:modelValue": ($event) => model = $event }), null, 16)), [[_vModelDynamic, model]]);
  })();
  "#);
}

#[test]
fn simple_expression_for_select() {
  let code = transform(
    r#"<select v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelSelect as _vModelSelect, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { "onUpdate:modelValue": ($event) => model = $event };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("select", _hoisted_1, null, 512)), [[_vModelSelect, model]]);
  })();
  "#);
}

#[test]
fn simple_expression_for_textarea() {
  let code = transform(
    r#"<textarea v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { "onUpdate:modelValue": ($event) => model = $event };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("textarea", _hoisted_1, null, 512)), [[_vModelText, model]]);
  })();
  "#);
}

#[test]
fn compound_expression() {
  let code = transform(
    r#"<input v-model={model[index]} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onUpdate:modelValue"];
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", { "onUpdate:modelValue": ($event) => model[index] = $event }, null, 8, _hoisted_1)), [[_vModelText, model[index]]]);
  })();
  "#)
}

#[test]
fn component_with_argument() {
  let code = transform(
    r#"<Comp v-model:foo-value={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, {
      "foo-value": model,
      "onUpdate:foo-value": ($event) => model = $event
    }, null, 8, ["foo-value"]);
  })();
  "#)
}

#[test]
fn component_with_dynamic_argument() {
  let code = transform(
    r#"<Comp v-model:$value$={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, normalizeProps as _normalizeProps, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, _normalizeProps({
      [value]: model,
      ["onUpdate:" + value]: ($event) => model = $event
    }), null, 16);
  })();
  "#)
}

#[test]
fn should_not_cache_update_handler_if_it_refers_v_for_scope_variables() {
  let code = transform(
    r#"<input v-for={i in list} v-model={foo[i]} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onUpdate:modelValue"];
  (() => {
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => _withDirectives((_openBlock(), _createElementBlock("input", { "onUpdate:modelValue": ($event) => foo[i] = $event }, null, 8, _hoisted_1)), [[_vModelText, foo[i]]])), 256);
  })();
  "#)
}

#[test]
fn should_not_cache_update_handler_if_it_inside_v_once() {
  let code = transform(
    r#"<div v-once><input v-model={foo} /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementVNode as _createElementVNode, setBlockTracking as _setBlockTracking, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div", null, [_withDirectives(_createElementVNode("input", { "onUpdate:modelValue": ($event) => foo = $event }, null, 512), [[_vModelText, foo]])])).cacheIndex = 0, _setBlockTracking(1), _cache[0]);
  })();
  "#)
}

#[test]
fn should_mark_update_handler_dynamic_if_it_refers_slot_scope_variables() {
  let code = transform(
    r#"<Comp v-slot={{ foo }}><input v-model={foo.bar}/></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, vModelText as _vModelText, withCtx as _withCtx, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = ["onUpdate:modelValue"];
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx(({ foo }) => [_withDirectives(_createElementVNode("input", { "onUpdate:modelValue": ($event) => foo.bar = $event }, null, 8, _hoisted_1), [[_vModelText, foo.bar]])]),
      _: 1
    });
  })();
  "#)
}

#[test]
fn should_generate_model_modifiers_for_component_v_model() {
  let code = transform(
    r#"<Comp v-model_trim_bar-baz={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, {
      modelValue: foo,
      "onUpdate:modelValue": ($event) => foo = $event,
      modelModifiers: {
        trim: true,
        "bar-baz": true
      }
    }, null, 8, ["modelValue"]);
  })();
  "#)
}

#[test]
fn should_generate_model_modifiers_for_component_v_model_with_arguments() {
  let code = transform(
    r#"<Comp v-model:foo_trim={foo} v-model:bar_number={bar} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, {
      foo,
      "onUpdate:foo": ($event) => foo = $event,
      fooModifiers: { trim: true },
      bar,
      "onUpdate:bar": ($event) => bar = $event,
      barModifiers: { number: true }
    }, null, 8, ["foo", "bar"]);
  })();
  "#)
}

#[test]
fn should_generate_model_modifiers_dollar_for_component_v_model_model_with_arguments() {
  let code = transform(
    r#"<Comp v-model:model_trim={foo} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, {
      model: foo,
      "onUpdate:model": ($event) => foo = $event,
      modelModifiers$: { trim: true }
    }, null, 8, ["model"]);
  })();
  "#)
}

#[test]
fn modifiers_number() {
  let code = transform(
    r#"<input v-model_number={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { "onUpdate:modelValue": ($event) => model = $event };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[
      _vModelText,
      model,
      void 0,
      { number: true }
    ]]);
  })();
  "#);
}

#[test]
fn modifiers_trim() {
  let code = transform(
    r#"<input v-model_trim={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { "onUpdate:modelValue": ($event) => model = $event };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[
      _vModelText,
      model,
      void 0,
      { trim: true }
    ]]);
  })();
  "#);
}

#[test]
fn modifiers_lazy() {
  let code = transform(
    r#"<input v-model_lazy={model} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { "onUpdate:modelValue": ($event) => model = $event };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[
      _vModelText,
      model,
      void 0,
      { lazy: true }
    ]]);
  })();
  "#);
}

#[test]
fn should_raise_error_if_plain_elements_with_argument() {
  let error = RefCell::new(None);
  transform(
    r#"<input v-model:value={model} />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelArgOnElement));
}

#[test]
fn should_raise_error_if_invalid_element() {
  let error = RefCell::new(None);
  transform(
    r#"<span v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnInvalidElement));
}

#[test]
fn should_raise_error_if_used_file_input_element() {
  let error = RefCell::new(None);
  transform(
    r#"<input type="file" v-model={test}/>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnFileInputElement));
}

#[test]
fn should_error_on_dynamic_value_binding_alongside_v_model() {
  let error = RefCell::new(None);
  transform(
    r#"<input v-model={test} value={test}/>"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelUnnecessaryValue));
}

#[test]
fn should_not_error_on_static_value_binding_alongside_v_model() {
  let code = transform(
    r#"<input v-model={test} value="test"/>"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag.starts_with("my-")),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = {
    "onUpdate:modelValue": ($event) => test = $event,
    value: "test"
  };
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("input", _hoisted_1, null, 512)), [[_vModelText, test]]);
  })();
  "#);
}

#[test]
fn should_allow_usage_on_custom_element() {
  let code = transform(
    r#"<my-input v-model={model} />"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag.starts_with("my-")),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, vModelText as _vModelText, withDirectives as _withDirectives } from "vue";
  (() => {
    return _withDirectives((_openBlock(), _createElementBlock("my-input", { "onUpdate:modelValue": ($event) => model = $event }, null, 512)), [[_vModelText, model]]);
  })();
  "#);
}

#[test]
fn should_error_if_empty_expression() {
  let error = RefCell::new(None);
  transform(
    r#"<input v-model={} />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelMalformedExpression));
}

#[test]
fn should_error_if_mal_formed_expression() {
  let error = RefCell::new(None);
  transform(
    r#"<input v-model={a + b} />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelMalformedExpression));
}

#[test]
fn used_on_scope_variable() {
  let error = RefCell::new(None);
  transform(
    r#"<span v-for={i in list} v-model={i} />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnScopeVariable));
}
