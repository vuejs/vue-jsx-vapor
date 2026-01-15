use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn should_not_cache_root_node() {
  // if the whole tree is static, the root still needs to be a block
  // so that it's patched in optimized mode to skip children
  let code = transform(
    r#"<div/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div");
  "#);
}

#[test]
fn cache_root_node_children() {
  // we don't have access to the root codegenNode during the transform
  // so we only cache each child individually
  let code = transform(
    r#"<><span class="inline">hello</span><span class="inline">hello</span></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(_Fragment, null, [..._cache[0] || (_cache[0] = [_createElementVNode("span", { class: "inline" }, "hello", -1), _createElementVNode("span", { class: "inline" }, "hello", -1)])], 64);
  })();
  "#);
}

#[test]
fn cache_single_children_array() {
  let code = transform(
    r#"<div><span id="inline">hello</span></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("span", { id: "inline" }, "hello", -1))]);
  })();
  "#);
}

#[test]
fn cache_nested_children_array() {
  let code = transform(
    r#"<div><p><span/><span/></p><p><span/><span/></p></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [..._cache[0] || (_cache[0] = [_createElementVNode("p", null, [_createElementVNode("span"), _createElementVNode("span")], -1), _createElementVNode("p", null, [_createElementVNode("span"), _createElementVNode("span")], -1)])]);
  })();
  "#);
}

#[test]
fn cache_nested_static_tree_with_comments() {
  let code = transform(
    r#"<div><div>{/*comment*/}</div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]);
  })();
  "#);
}

#[test]
fn cache_siblings_including_text_with_common_non_hoistable_parent() {
  let code = transform(
    r#"<div><span/>foo<div/></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [..._cache[0] || (_cache[0] = [
  		_createElementVNode("span", null, null, -1),
  		_normalizeVNode("foo", -1),
  		_createElementVNode("div", null, null, -1)
  	])]);
  })();
  "#);
}

#[test]
fn cache_inside_default_slot() {
  let code = transform(
    r#"<Foo>{x}<span/></Foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Foo, null, {
  		default: _withCtx(() => [_normalizeVNode(() => x), _cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]),
  		_: 2
  	}, 1024);
  })();
  "#);
}

#[test]
fn cache_default_slot_as_whole() {
  let code = transform(
    r#"<Foo><span/><span/></Foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Foo, null, {
  		default: _withCtx(() => [..._cache[0] || (_cache[0] = [_createElementVNode("span", null, null, -1), _createElementVNode("span", null, null, -1)])]),
  		_: 1
  	});
  })();
  "#);
}

#[test]
fn cache_inside_named_slot() {
  let code = transform(
    r#"<Foo><template v-slot:foo>{x}<span/></template></Foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Foo, null, {
  		foo: _withCtx(() => [_normalizeVNode(() => x), _cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]),
  		_: 2
  	}, 1024);
  })();
  "#);
}

#[test]
fn cache_named_slot_as_a_whole() {
  let code = transform(
    r#"<Foo><template v-slot:foo><span/><span/></template></Foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Foo, null, {
  		foo: _withCtx(() => [..._cache[0] || (_cache[0] = [_createElementVNode("span", null, null, -1), _createElementVNode("span", null, null, -1)])]),
  		_: 1
  	});
  })();
  "#);
}

#[test]
fn cache_dynamically_named_slot_as_a_whole() {
  let code = transform(
    r#"<Foo><template v-slot:$foo$><span/><span/></template></Foo>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Foo, null, {
  		[foo]: _withCtx(() => [..._cache[0] || (_cache[0] = [_createElementVNode("span", null, null, -1), _createElementVNode("span", null, null, -1)])]),
  		_: 2
  	}, 1024);
  })();
  "#);
}

#[test]
fn cache_should_not_cache_components() {
  let code = transform(
    r#"<div><Comp/></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", null, [_createVNode(Comp)]);
  "#);
}

#[test]
fn cache_should_not_cache_element_with_dynamic_props_but_hoist_the_props_list() {
  let code = transform(
    r#"<div><div id={foo} /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["id"];
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("div", { id: foo }, null, 8, _hoisted_1)]);
  "#);
}

#[test]
fn cache_element_with_static_key() {
  let code = transform(
    r#"<div><div key="foo" /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("div", { key: "foo" }, null, -1))]);
  })();
  "#);
}

#[test]
fn should_not_cache_element_with_dynamic_key() {
  let code = transform(
    r#"<div><div key={foo} /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", null, [(_openBlock(), _createElementBlock("div", { key: foo }))]);
  "#);
}

#[test]
fn should_not_cache_element_with_dynamic_ref() {
  let code = transform(
    r#"<div><div ref={foo} /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("div", { ref: foo }, null, 512)]);
  "#);
}

#[test]
fn hoist_static_props_for_elements_with_directives() {
  let code = transform(
    r#"<div><div id="foo" v-foo /></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  const _hoisted_1 = { id: "foo" };
  (() => {
  	const _directive_foo = _resolveDirective("foo");
  	return _openBlock(), _createElementBlock("div", null, [_withDirectives(_createElementVNode("div", _hoisted_1, null, 512), [[_directive_foo]])]);
  })();
  "#);
}

#[test]
fn hoist_static_props_for_elements_with_dynamic_text_children() {
  let code = transform(
    r#"<div><div id="foo">{hello}</div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { id: "foo" };
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("div", _hoisted_1, [_normalizeVNode(() => hello)])]);
  "#);
}

#[test]
fn hoist_static_props_for_elements_with_unhoistable_children() {
  let code = transform(
    r#"<div><div id="foo"><Comp/></div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { id: "foo" };
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("div", _hoisted_1, [_createVNode(Comp)])]);
  "#);
}

#[test]
fn should_cache_v_if_props_or_children_if_static() {
  let code = transform(
    r#"<div><div v-if="ok" id="foo"><span/></div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = {
  	key: 0,
  	id: "foo"
  };
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, ["ok" ? (_openBlock(), _createElementBlock("div", _hoisted_1, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))])) : _createCommentVNode("", true)]);
  })();
  "#);
}

#[test]
fn should_hoist_v_for_children_if_static() {
  let code = transform(
    r#"<div><div v-for={i in list} id="foo"><span/></div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  const _hoisted_1 = { id: "foo" };
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [(_openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", _hoisted_1, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]))), 256))]);
  })();
  "#);
}

#[test]
fn should_hoist_props_for_root_with_single_element_excluding_comments() {
  // deeply nested div to trigger stringification condition
  let code = transform(
    r#"<>{/*comment*/}<div id="a"><div id="b"><div id="c"><div id="d"><div id="e">hello</div></div></div></div></div></>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(_Fragment, null, [_cache[0] || (_cache[0] = _createElementVNode("div", { id: "a" }, [_createElementVNode("div", { id: "b" }, [_createElementVNode("div", { id: "c" }, [_createElementVNode("div", { id: "d" }, [_createElementVNode("div", { id: "e" }, "hello")])])])], -1))], 64);
  })();
  "#);
}

#[test]
fn cache_nested_static_tree_with_static_interpolation() {
  // deeply nested div to trigger stringification condition
  let code = transform(
    r#"<div><span>foo { 1 } { true }</span></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("span", null, [
  		_normalizeVNode("foo "),
  		_normalizeVNode(1),
  		_normalizeVNode(),
  		_normalizeVNode(true)
  	], -1))]);
  })();
  "#);
}

#[test]
fn cache_nested_static_tree_with_static_prop_value() {
  let code = transform(
    r#"<div><span foo={0}>{1}</span></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("span", { foo: 0 }, 1, -1))]);
  })();
  "#);
}

#[test]
fn cache_class_with_static_object_value() {
  let code = transform(
    r#"<div><span class={{ foo: true }}>{bar}</span></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, normalizeClass as _normalizeClass, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { class: _normalizeClass({ foo: true }) };
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("span", _hoisted_1, [_normalizeVNode(() => bar)])]);
  "#);
}

#[test]
fn should_not_cache_expressions_that_refer_scope_variables() {
  let code = transform(
    r#"<div><p v-for={o in list}><span>{o}</span></p></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(), _createElementBlock("div", null, [(_openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (o) => (_openBlock(), _createElementBlock("p", null, [_createElementVNode("span", null, [_normalizeVNode(() => o)])]))), 256))]);
  "#);
}

#[test]
fn should_not_cache_expressions_that_refer_scope_variables_2() {
  let code = transform(
    r#"<div><p v-for={o in list}><span>{o + 'foo'}</span></p></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(), _createElementBlock("div", null, [(_openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (o) => (_openBlock(), _createElementBlock("p", null, [_createElementVNode("span", null, [_normalizeVNode(() => o + "foo")])]))), 256))]);
  "#);
}

#[test]
fn should_not_cache_expressions_that_refer_scope_variables_v_slot() {
  let code = transform(
    r#"<Comp v-slot={{ foo }}>{foo}</Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  _openBlock(), _createBlock(Comp, null, {
  	default: _withCtx(({ foo }) => [_normalizeVNode(() => foo)]),
  	_: 1
  });
  "#);
}

#[test]
fn should_not_cache_elements_with_cached_handlers() {
  let code = transform(
    r#"<div><div><div onClick={foo}/></div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("div", null, [_createElementVNode("div", { onClick: foo }, null, 8, _hoisted_1)])]);
  "#);
}

#[test]
fn should_not_cache_elements_with_cached_handlers_with_other_bindings() {
  let code = transform(
    r#"<div><div><div class={{}} onClick={foo}/></div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, normalizeClass as _normalizeClass, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createElementBlock("div", null, [_createElementVNode("div", null, [_createElementVNode("div", {
  	class: _normalizeClass({}),
  	onClick: foo
  }, null, 8, _hoisted_1)])]);
  "#);
}

#[test]
fn should_cache_keyed_template_v_for_with_plain_element_child() {
  let code = transform(
    r#"<div><template v-for={item in items} key={item}><span/></template></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [(_openBlock(true), _createElementBlock(_Fragment, null, _renderList(items, (item) => (_openBlock(), _createBlock(_Fragment, { key: item }, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))], 64))), 128))]);
  })();
  "#);
}

#[test]
fn should_not_cache_svg_with_directives() {
  let code = transform(
    r#"<div><svg v-foo><path d="M2,3H5.5L12"/></svg></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _directive_foo = _resolveDirective("foo");
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _withDirectives(_createElementVNode("svg", null, [_createElementVNode("path", { d: "M2,3H5.5L12" })], -1), [[_directive_foo]]))]);
  })();
  "#);
}

#[test]
fn clone_hoisted_array_children_in_v_for_hmr_mode() {
  let code = transform(
    r#"<div><div v-for={i in 1}><span class="hi"></span></div></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [(_openBlock(), _createElementBlock(_Fragment, null, _renderList(1, (i) => _cache[0] || (_cache[0] = _createElementVNode("div", null, [_createElementVNode("span", { class: "hi" })], -1))), 64))]);
  })();
  "#);
}
