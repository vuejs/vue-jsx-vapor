use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn import_resolve_component() {
  let code = transform(
    r#"<foo-bar />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock, resolveComponent as _resolveComponent } from "vue";
  (() => {
  	const _component_foo_bar = _resolveComponent("foo-bar");
  	return _openBlock(), _createBlock(_component_foo_bar);
  })();
  "#);
}

#[test]
fn resolve_namespaced_component_from_setup_bindings() {
  let code = transform(
    r#"<Foo.Example/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createBlock(Foo.Example);
  "#);
}

#[test]
fn resolve_namespaced_component_from_setup_bindings_inline_const() {
  let code = transform(
    r#"<Foo.Example/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createBlock(Foo.Example);
  "#);
}

#[test]
fn static_props() {
  let code = transform(
    r#"<div id="foo" class="bar" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = {
  	id: "foo",
  	class: "bar"
  };
  _openBlock(), _createElementBlock("div", _hoisted_1);
  "#)
}

#[test]
fn props_children() {
  let code = transform(
    r#"<div id="foo"><span/></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { id: "foo" };
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", _hoisted_1, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]);
  })();
  "#)
}

#[test]
fn zero_placeholder_for_children_with_no_props() {
  let code = transform(
    r#"<div><span/></div>"#,
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
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]);
  })();
  "#)
}

#[test]
fn v_bind_obj() {
  let code = transform(
    r#"<div {...obj} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, guardReactiveProps as _guardReactiveProps, normalizeProps as _normalizeProps, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", _normalizeProps(_guardReactiveProps(obj)), null, 16);
  "#)
}

#[test]
fn v_bind_obj_after_static_prop() {
  let code = transform(
    r#"<div id="foo" {...obj} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", _mergeProps({ id: "foo" }, obj), null, 16);
  "#)
}

#[test]
fn v_bind_obj_before_static_prop() {
  let code = transform(
    r#"<div {...obj} id="foo" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", _mergeProps(obj, { id: "foo" }), null, 16);
  "#)
}

#[test]
fn v_bind_obj_between_static_prop() {
  let code = transform(
    r#"<div id="foo" {...obj} class="bar" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", _mergeProps({ id: "foo" }, obj, { class: "bar" }), null, 16);
  "#)
}

#[test]
fn v_on_obj() {
  let code = transform(
    r#"<div id="foo" v-on={obj} class="bar" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock, toHandlers as _toHandlers } from "vue";
  _openBlock(), _createElementBlock("div", _mergeProps({ id: "foo" }, _toHandlers(obj, true), { class: "bar" }), null, 16);
  "#)
}

#[test]
fn v_on_obj_on_component() {
  let code = transform(
    r#"<Foo id="foo" v-on={obj} class="bar" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createBlock as _createBlock, mergeProps as _mergeProps, openBlock as _openBlock, toHandlers as _toHandlers } from "vue";
  _openBlock(), _createBlock(Foo, _mergeProps({ id: "foo" }, _toHandlers(obj), { class: "bar" }), null, 16);
  "#)
}

#[test]
fn v_on_obj_and_v_bind_obj() {
  let code = transform(
    r#"<div id="foo" v-on={handlers} {...obj} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock, toHandlers as _toHandlers } from "vue";
  _openBlock(), _createElementBlock("div", _mergeProps({ id: "foo" }, _toHandlers(handlers, true), obj), null, 16);
  "#)
}

#[test]
fn should_handle_plain_template_as_normal_element() {
  let code = transform(
    r#"<template id="foo" />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { id: "foo" };
  _openBlock(), _createElementBlock("template", _hoisted_1);
  "#)
}

#[test]
fn should_handle_teleport_with_normal_children() {
  let code = transform(
    r#"<Teleport target="\#foo"><span /></Teleport>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Teleport, { target: "\\#foo" }, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]);
  })();
  "#)
}

#[test]
fn should_handle_suspense() {
  let code = transform(
    r#"<Suspense>
      <template v-slot:default>foo</template>
      <template v-slot:fallback>fallback</template>
    </Suspense>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(Suspense, null, {
  		default: _withCtx(() => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))]),
  		fallback: _withCtx(() => [_cache[1] || (_cache[1] = _normalizeVNode("fallback", -1))]),
  		_: 1
  	});
  })();
  "#)
}

#[test]
fn should_handle_keep_alive() {
  let code = transform(
    r#"<div>
      <KeepAlive>
        <span />
      </keepAlive>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [(_openBlock(), _createBlock(KeepAlive, null, [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))], 1024))]);
  })();
  "#)
}

#[test]
fn should_handle_base_transition() {
  let code = transform(
    r#"<BaseTransition>
        <span />
      </BaseTransition>"#,
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
  	return _openBlock(), _createBlock(BaseTransition, null, {
  		default: _withCtx(() => [_cache[0] || (_cache[0] = _createElementVNode("span", null, null, -1))]),
  		_: 1
  	});
  })();
  "#)
}

#[test]
fn directive_transforms() {
  let code = transform(
    r#"<div v-foo:bar={hello} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
  	const _directive_foo = _resolveDirective("foo");
  	return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[
  		_directive_foo,
  		hello,
  		bar
  	]]);
  })();
  "#)
}

#[test]
fn runtime_directives() {
  let code = transform(
    r#"<div v-foo v-bar="x" v-baz:$arg$_mod_mad={y} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
  	const _directive_foo = _resolveDirective("foo");
  	const _directive_bar = _resolveDirective("bar");
  	const _directive_baz = _resolveDirective("baz");
  	return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [
  		[_directive_foo],
  		[_directive_bar, "x"],
  		[
  			_directive_baz,
  			y,
  			arg,
  			{
  				mod: true,
  				mad: true
  			}
  		]
  	]);
  })();
  "#)
}

#[test]
fn props_merging_event_handlers() {
  let code = transform(
    r#"<div onClick_foo={a} onClick_bar={b} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  _openBlock(), _createElementBlock("div", { onClick: [a, b] }, null, 8, _hoisted_1);
  "#)
}

#[test]
fn props_merging_style() {
  let code = transform(
    r#"<div style="color: green" style={{ color: 'red' }} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, normalizeStyle as _normalizeStyle, openBlock as _openBlock } from "vue";
  const _hoisted_1 = { style: _normalizeStyle(["color: green", { color: "red" }]) };
  _openBlock(), _createElementBlock("div", _hoisted_1);
  "#)
}

#[test]
fn props_merging_class() {
  let code = transform(
    r#"<div class="foo" class={{ bar: isBar }} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, normalizeClass as _normalizeClass, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("div", { class: _normalizeClass(["foo", { bar: isBar }]) }, null, 2);
  "#)
}

mod patch_flag_analysis {
  use compiler_rs::{TransformOptions, transform};
  use insta::assert_snapshot;

  #[test]
  fn text() {
    let code = transform(
      r#"<div>foo</div>"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    _openBlock(), _createElementBlock("div", null, "foo");
    "#)
  }

  #[test]
  fn class() {
    let code = transform(
      r#"<div class={foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, normalizeClass as _normalizeClass, openBlock as _openBlock } from "vue";
    _openBlock(), _createElementBlock("div", { class: _normalizeClass(foo) }, null, 2);
    "#)
  }

  #[test]
  fn style() {
    let code = transform(
      r#"<div style={foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, normalizeStyle as _normalizeStyle, openBlock as _openBlock } from "vue";
    _openBlock(), _createElementBlock("div", { style: _normalizeStyle(foo) }, null, 4);
    "#)
  }

  #[test]
  fn props() {
    let code = transform(
      r#"<div id="foo" foo={bar} baz={qux} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    const _hoisted_1 = ["foo", "baz"];
    _openBlock(), _createElementBlock("div", {
    	id: "foo",
    	foo: bar,
    	baz: qux
    }, null, 8, _hoisted_1);
    "#)
  }

  #[test]
  fn class_style_props() {
    let code = transform(
      r#"<div id="foo" class={cls} style={styl} foo={bar} baz={qux} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, normalizeClass as _normalizeClass, normalizeStyle as _normalizeStyle, openBlock as _openBlock } from "vue";
    const _hoisted_1 = ["foo", "baz"];
    _openBlock(), _createElementBlock("div", {
    	id: "foo",
    	class: _normalizeClass(cls),
    	style: _normalizeStyle(styl),
    	foo: bar,
    	baz: qux
    }, null, 14, _hoisted_1);
    "#)
  }

  #[test]
  fn props_on_component() {
    // should treat `class` and `style` as PROPS
    let code = transform(
      r#"<Foo id={foo} class={cls} style={styl} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createBlock as _createBlock, normalizeClass as _normalizeClass, normalizeStyle as _normalizeStyle, openBlock as _openBlock } from "vue";
    _openBlock(), _createBlock(Foo, {
    	id: foo,
    	class: _normalizeClass(cls),
    	style: _normalizeStyle(styl)
    }, null, 8, [
    	"id",
    	"class",
    	"style"
    ]);
    "#)
  }

  #[test]
  fn full_props_v_bind() {
    let code = transform(
      r#"<div {...foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, guardReactiveProps as _guardReactiveProps, normalizeProps as _normalizeProps, openBlock as _openBlock } from "vue";
    _openBlock(), _createElementBlock("div", _normalizeProps(_guardReactiveProps(foo)), null, 16);
    "#)
  }

  #[test]
  fn full_props_with_others() {
    let code = transform(
      r#"<div id="foo" {...foo} class={cls} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock } from "vue";
    _openBlock(), _createElementBlock("div", _mergeProps({ id: "foo" }, foo, { class: cls }), null, 16);
    "#)
  }

  #[test]
  fn need_patch_static_ref() {
    let code = transform(
      r#"<div ref="foo" />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    const _hoisted_1 = { ref: "foo" };
    _openBlock(), _createElementBlock("div", _hoisted_1, null, 512);
    "#)
  }

  #[test]
  fn need_patch_dynamic_ref() {
    let code = transform(
      r#"<div ref={foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    _openBlock(), _createElementBlock("div", { ref: foo }, null, 512);
    "#)
  }

  #[test]
  fn need_patch_custom_directives() {
    let code = transform(
      r#"<div v-foo />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
    (() => {
    	const _directive_foo = _resolveDirective("foo");
    	return _withDirectives((_openBlock(), _createElementBlock("div", null, null, 512)), [[_directive_foo]]);
    })();
    "#)
  }

  #[test]
  fn need_patch_vnode_hooks() {
    let code = transform(
      r#"<div onVue:updated={foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    const _hoisted_1 = ["onVnodeUpdated"];
    _openBlock(), _createElementBlock("div", { onVnodeUpdated: foo }, null, 8, _hoisted_1);
    "#)
  }

  #[test]
  fn need_hydration_for_v_on() {
    let code = transform(
      r#"<div onKeyup={foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    const _hoisted_1 = ["onKeyup"];
    _openBlock(), _createElementBlock("div", { onKeyup: foo }, null, 40, _hoisted_1);
    "#)
  }

  #[test]
  fn need_hydration_for_v_bind_prop() {
    let code = transform(
      r#"<div id_prop={id} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    const _hoisted_1 = [".id"];
    _openBlock(), _createElementBlock("div", { ".id": id }, null, 40, _hoisted_1);
    "#)
  }

  #[test]
  fn should_not_have_props_patchflag_for_constant_v_on_handlers() {
    let code = transform(
      r#"<div onKeydown={foo} />"#,
      Some(TransformOptions {
        interop: true,
        ..Default::default()
      }),
    )
    .code;
    assert_snapshot!(code, @r#"
    import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
    const _hoisted_1 = ["onKeydown"];
    _openBlock(), _createElementBlock("div", { onKeydown: foo }, null, 40, _hoisted_1);
    "#)
  }
}

#[test]
fn custom_element() {
  let code = transform(
    r#"<my-custom-element>foo</my-custom-element>"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  _openBlock(), _createElementBlock("my-custom-element", null, "foo");
  "#)
}

#[test]
fn custom_element_with_v_model() {
  let code = transform(
    r#"<my-custom-element v-model={foo}></my-custom-element>"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["modelValue", "onUpdate:modelValue"];
  _openBlock(), _createElementBlock("my-custom-element", {
  	modelValue: foo,
  	"onUpdate:modelValue": ($event) => foo = $event
  }, null, 8, _hoisted_1);
  "#)
}

#[test]
fn custom_element_with_v_on() {
  let code = transform(
    r#"<my-custom-element onFoo={foo}></my-custom-element>"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onFoo"];
  _openBlock(), _createElementBlock("my-custom-element", { onFoo: foo }, null, 40, _hoisted_1);
  "#)
}

#[test]
fn custom_element_with_v_html() {
  let code = transform(
    r#"<my-custom-element v-html={foo}></my-custom-element>"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["innerHTML"];
  _openBlock(), _createElementBlock("my-custom-element", { innerHTML: foo }, null, 8, _hoisted_1);
  "#)
}

#[test]
fn custom_element_with_v_text() {
  let code = transform(
    r#"<my-custom-element v-text={foo}></my-custom-element>"#,
    Some(TransformOptions {
      interop: true,
      is_custom_element: Box::new(|tag| tag == "my-custom-element"),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, toDisplayString as _toDisplayString } from "vue";
  const _hoisted_1 = ["textContent"];
  _openBlock(), _createElementBlock("my-custom-element", { textContent: _toDisplayString(foo) }, null, 8, _hoisted_1);
  "#)
}

#[test]
fn svg_should_be_forced_into_blocks() {
  let code = transform(
    r#"<div><svg/></div>"#,
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
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("svg", null, null, -1))]);
  })();
  "#)
}

#[test]
fn math_should_be_forced_into_blocks() {
  let code = transform(
    r#"<div><math/></div>"#,
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
  	return _openBlock(), _createElementBlock("div", null, [_cache[0] || (_cache[0] = _createElementVNode("math", null, null, -1))]);
  })();
  "#)
}

#[test]
fn force_block_for_runtime_custom_directive_with_children() {
  let code = transform(
    r#"<div v-foo>hello</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, resolveDirective as _resolveDirective, withDirectives as _withDirectives } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	const _directive_foo = _resolveDirective("foo");
  	return _withDirectives((_openBlock(), _createElementBlock("div", null, _cache[0] || (_cache[0] = _normalizeVNode("hello", -1)))), [[_directive_foo]]);
  })();
  "#)
}

#[test]
fn element_with_dynamic_prop_should_be_forced_into_blocks() {
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
  "#)
}

#[test]
fn ref_for_marker_on_static_ref() {
  let code = transform(
    r#"<div v-for={i in l} ref="x"/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  const _hoisted_1 = {
  	ref_for: true,
  	ref: "x"
  };
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(l, (i) => (_openBlock(), _createElementBlock("div", _hoisted_1, null, 512))), 256);
  "#)
}

#[test]
fn ref_for_marker_on_dynamic_ref() {
  let code = transform(
    r#"<div v-for={i in l} ref={x}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(l, (i) => (_openBlock(), _createElementBlock("div", {
  	ref_for: true,
  	ref: x
  }, null, 512))), 256);
  "#)
}

#[test]
fn ref_for_marker_on_v_bind() {
  let code = transform(
    r#"<div v-for={i in l} {...x}/>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, mergeProps as _mergeProps, openBlock as _openBlock, renderList as _renderList } from "vue";
  _openBlock(true), _createElementBlock(_Fragment, null, _renderList(l, (i) => (_openBlock(), _createElementBlock("div", _mergeProps({ ref_for: true }, x), null, 16))), 256);
  "#)
}

#[test]
fn keep_alive() {
  let code = transform(
    r#"<div><KeepAlive include={foo}>foo</KeepAlive></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createElementBlock("div", null, [(_openBlock(), _createBlock(KeepAlive, { include: foo }, _cache[0] || (_cache[0] = _normalizeVNode("foo", -1)), 1032, ["include"]))]);
  })();
  "#)
}

#[test]
fn fragment_in_fragment() {
  let code = transform(
    r#"<>foo<>bar</>baz</>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
  	const _cache = _createVNodeCache(0);
  	return _openBlock(), _createBlock(_Fragment, null, [
  		_cache[1] || (_cache[1] = _normalizeVNode("foo", -1)),
  		_createVNode(_Fragment, null, [_cache[0] || (_cache[0] = _normalizeVNode("bar", -1))], 64),
  		_cache[2] || (_cache[2] = _normalizeVNode("baz", -1))
  	], 64);
  })();
  "#)
}
