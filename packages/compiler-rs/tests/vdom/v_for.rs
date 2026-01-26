use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn number_expression() {
  let code = transform(
    r#"<span v-for={index in 5}>{index}</span>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(), _createElementBlock(_Fragment, null, _renderList(5, (index) => _createElementVNode("span", null, [_normalizeVNode(() => index)])), 64);
  "#);
}

#[test]
fn value() {
  let code = transform(
    r#"<span v-for={(item) in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn object_de_structured_value() {
  let code = transform(
    r#"<span v-for={({ id, value }) in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, ({ id, value }) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn array_de_structured_value() {
  let code = transform(
    r#"<span v-for={([ id, value ]) in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, ([id, value]) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn value_and_key() {
  let code = transform(
    r#"<span v-for={(item, key) in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item, key) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn value_and_key_and_index() {
  let code = transform(
    r#"<span v-for={(item, key, index) in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item, key, index) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn unbracketed_value() {
  let code = transform(
    r#"<span v-for={item in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn source_containing_string_expression_with_spaces() {
  let code = transform(
    r#"<span v-for={item in state ['my items']} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(state["my items"], (item) => (_openBlock(), _createElementBlock("span"))), 256);
  "#);
}

#[test]
fn missing_expression() {
  let code = transform(
    r#"<span v-for={item in items} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createElementBlock("span"))), 256);
  "#)
}

#[test]
fn v_for_source_with_complex_expression() {
  let code = transform(
    r#"<span v-for={i in list.concat([foo])} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list.concat([foo]), (i) => (_openBlock(), _createElementBlock("span"))), 256);
  "#)
}

#[test]
fn should_not_prefix_v_for_aliases() {
  let code = transform(
    r#"<span v-for={(i, j, k) in list}>{ i + j + k }{ l }</span>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i, j, k) => (_openBlock(), _createElementBlock("span", null, [_normalizeVNode(() => i + j + k), _normalizeVNode(() => l)]))), 256);
  "#)
}

#[test]
fn nested_v_for() {
  let code = transform(
    r#"<div v-for={i in list}>
      <div v-for={i in list}>{ i + j }</div>{ i }
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [(_openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_normalizeVNode(() => i + j)]))), 256)), _normalizeVNode(() => i)]))), 256);
  "#)
}

#[test]
fn v_for_aliases_with_complex_expressions() {
  let code = transform(
    r#"<div v-for={({ foo:foo = bar, baz: [qux = quux] }) in list}>
      { foo + bar + baz + qux + quux }
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, ({ foo = bar, baz: [qux = quux] }) => (_openBlock(), _createElementBlock("div", null, [_normalizeVNode(() => foo + bar + baz + qux + quux)]))), 256);
  "#)
}

#[test]
fn element_v_for_key_expression_prefixing() {
  let code = transform(
    r#"<div v-for={item in items} key={itemKey(item)}>test</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createElementBlock("div", { key: itemKey(item) }, "test"))), 128);
  "#)
}

#[test]
fn template_v_for_key_expression_prefixing() {
  let code = transform(
    r#"<template v-for={item in items} key={itemKey(item)}>test</template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, { key: itemKey(item) }, [_cache[0] || (_cache[0] = _normalizeVNode("test", -1))], 64))), 128);
  })();
  "#)
}

#[test]
fn template_v_for_key_no_prefixing_on_attribute_key() {
  let code = transform(
    r#"<template v-for={item in items} key="key">test</template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, { key: "key" }, [_cache[0] || (_cache[0] = _normalizeVNode("test", -1))], 64))), 128);
  })();
  "#)
}

#[test]
fn template_v_for_with_multiple_children() {
  let code = transform(
    r#"<template v-for={item in items}>hello<span/></template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, null, [..._cache[0] || (_cache[0] = [_normalizeVNode("hello", -1), _createElementVNode("span", null, null, -1)])], 64))), 256);
  })();
  "#)
}

#[test]
fn template_v_for_with_slotlet() {
  let code = transform(
    r#"<template v-for={item in items}><slot/></template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	let _slots = _useSlots();
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, null, [_renderSlot(_slots, "default")], 64))), 256);
  })();
  "#)
}

#[test]
fn template_v_for_with_slot() {
  let code = transform(
    r#"<template v-for={item in items}><slots.default /></template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, null, [_createVNode(slots.default)], 64))), 256);
  "#)
}

#[test]
fn template_v_for_key_injection_with_single_child() {
  let code = transform(
    r#"<template v-for={item in items} key={item.id}><span id={item.id} /></template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  const _hoisted_1 = ["id"];
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, { key: item.id }, [_createElementVNode("span", { id: item.id }, null, 8, _hoisted_1)], 64))), 128);
  "#)
}

#[test]
fn v_for_on_slotlet() {
  let code = transform(
    r#"<slot v-for={item in items}></slot>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList, renderSlot as _renderSlot, useSlots as _useSlots } from "vue";
  (() => {
  	let _slots = _useSlots();
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => _renderSlot(_slots, "default")), 256);
  })();
  "#)
}

#[test]
fn keyed_v_for() {
  let code = transform(
    r#"<span v-for={(item) in items} key={item} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createElementBlock("span", { key: item }))), 128);
  "#)
}

#[test]
fn keyed_template_v_for() {
  let code = transform(
    r#"<template v-for={item in items} key={item}>hello<span/></template>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, { key: item }, [..._cache[0] || (_cache[0] = [_normalizeVNode("hello", -1), _createElementVNode("span", null, null, -1)])], 64))), 128);
  })();
  "#)
}

#[test]
fn v_if_with_v_for() {
  let code = transform(
    r#"<div v-if={ok} v-for={i in list} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  ok ? (_openBlock(true), _createElementBlock(_Fragment, { key: 0 }, _renderList(list, (i) => (_openBlock(), _createElementBlock("div"))), 256)) : _createCommentVNode("", true);
  "#)
}

#[test]
fn v_if_with_v_for_on_template() {
  let code = transform(
    r#"<template v-if={ok} v-for={i in list} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(), _createBlock(_Fragment, null, ok ? (_openBlock(true), _createElementBlock(_Fragment, { key: 0 }, _renderList(list, (i) => (_openBlock(), _createBlock(_Fragment))), 256)) : _createCommentVNode("", true));
  "#)
}

#[test]
fn v_for_on_element_with_custom_directive() {
  let code = transform(
    r#"<div v-for={i in list} v-foo/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
  	const _directive_foo = _resolveDirective("foo");
  	return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[_directive_foo]])), 256);
  })();
  "#)
}

#[test]
fn template_v_for_key_with_key_on_div() {
  let code = transform(
    r#"<div v-for={key in keys} key={key}>test</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(keys, (key) => (_openBlock(), _createElementBlock("div", { key }, "test"))), 128);
  "#)
}

mod error {
  use common::error::ErrorCodes;
  use compiler_rs::{TransformOptions, transform};
  use std::cell::RefCell;

  #[test]
  fn missing_expression() {
    let error = RefCell::new(None);
    transform(
      r#"<span v-for />"#,
      Some(TransformOptions {
        interop: true,
        on_error: Box::new(|e, _| {
          *error.borrow_mut() = Some(e);
        }),
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(*error.borrow(), Some(ErrorCodes::VForNoExpression));
  }

  #[test]
  fn empty_expression() {
    let error = RefCell::new(None);
    transform(
      r#"<span v-for="" />"#,
      Some(TransformOptions {
        interop: true,
        on_error: Box::new(|e, _| {
          *error.borrow_mut() = Some(e);
        }),
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(*error.borrow(), Some(ErrorCodes::VForMalformedExpression));
  }

  #[test]
  fn invalid_expression() {
    let error = RefCell::new(None);
    transform(
      r#"<span v-for={items} />"#,
      Some(TransformOptions {
        interop: true,
        on_error: Box::new(|e, _| {
          *error.borrow_mut() = Some(e);
        }),
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(*error.borrow(), Some(ErrorCodes::VForMalformedExpression));
  }
}
