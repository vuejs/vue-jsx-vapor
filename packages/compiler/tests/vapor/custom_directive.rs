use compiler::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-example></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[_directive_example]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn binding_value() {
  let code = transform(
    "<div v-example={msg}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[_directive_example, () => msg]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn object_literal_binding_value() {
  let code = transform(
    "<div v-example={{ value: msg, other: 1 }}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains("() => ({"));
  assert!(code.contains("value: msg"));
  assert!(code.contains("other: 1"));
  assert_snapshot!(code, @r#"
	import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
	const _t0 = _template("<div>", 1);
	(() => {
		const _directive_example = _resolveDirective("example");
		const _n0 = _t0();
		_withVaporDirectives(_n0, [[_directive_example, () => ({
			value: msg,
			other: 1
		})]]);
		return _n0;
	})();
	"#);
}

#[test]
fn static_parameters() {
  let code = transform(
    "<div v-example:foo={msg}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		() => "foo"
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn modifiers() {
  let code = transform(
    "<div v-example_bar={msg}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		void 0,
  		{ bar: true }
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn modifiers_with_binding() {
  let code = transform(
    "<div v-example_foo-bar></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		void 0,
  		void 0,
  		{ "foo-bar": true }
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn static_argument_and_modifiers() {
  let code = transform(
    "<div v-example:foo_bar={msg}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		() => "foo",
  		{ bar: true }
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_argument() {
  let code = transform(
    "<div v-example:$foo$={msg}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		() => foo
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn component() {
  let code = transform(
    "<Comp v-test>
      <div v-if={true}>
        <Bar v-hello_world />
      </div>
    </Comp>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { createIf as _createIf, extend as _extend, resolveDirective as _resolveDirective, setInsertionState as _setInsertionState, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _directive_test = _resolveDirective("test");
  	const _directive_hello = _resolveDirective("hello");
  	const _n0 = _createComponent(Comp, null, _extend(() => {
  		const _n1 = _createIf(() => true, () => {
  			const _n4 = _t0();
  			_setInsertionState(_n4);
  			const _n3 = _createComponent(Bar);
  			_withVaporDirectives(_n3, [[
  				_directive_hello,
  				void 0,
  				void 0,
  				{ world: true }
  			]]);
  			return _n4;
  		}, null, 17);
  		return _n1;
  	}, { _: 1 }), true);
  	_withVaporDirectives(_n0, [[_directive_test]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn is_not_directive() {
  let code = transform(
    "<div vExample={msg}></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _t0();
  	_renderEffect(() => _setProp(_n0, "vExample", msg));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_resolve_directive() {
  let code = transform(
    "() => {
      const vExample = () => {}
      return <div v-example></div>
    }",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  () => {
  	const vExample = () => {};
  	return (() => {
  		const _n0 = _t0();
  		_withVaporDirectives(_n0, [[vExample]]);
  		return _n0;
  	})();
  };
  "#);
}

#[test]
fn array_args() {
  let code = transform(
    "<div v-example={[foo, bar, ['modify1', 'modify2']]} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => foo,
  		() => bar,
  		{
  			modify1: true,
  			modify2: true
  		}
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn array_args_with_modifiers() {
  let code = transform(
    "<div v-example={[foo, ['modify1', 'modify2']]} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => foo,
  		void 0,
  		{
  			modify1: true,
  			modify2: true
  		}
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn array_args_with_arg() {
  let code = transform(
    "<div v-example:foo={[foo, bar, ['modify1', 'modify2']]} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => foo,
  		() => "foo",
  		{
  			modify1: true,
  			modify2: true
  		}
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn applies_custom_directives_after_props_children_and_v_model() {
  let code = transform(
    "<div v-dir id={foo}>{ bar }<span v-if={ok} /><Comp /><input v-model={text} /></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains(
    "_applyTextModel(_n6, () => text, (_value) => text = _value);\n\t_withVaporDirectives(_n0, [[_directive_dir]]);\n\treturn _n0;"
  ));
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx/vapor";
  import { applyTextModel as _applyTextModel, child as _child, createIf as _createIf, next as _next, renderEffect as _renderEffect, resolveDirective as _resolveDirective, setInsertionState as _setInsertionState, setProp as _setProp, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<span>", 2);
  const _t1 = _template("<div> <!><!><input>", 1);
  (() => {
  	const _directive_dir = _resolveDirective("dir");
  	const _n0 = _t1();
  	const _n1 = _child(_n0, true);
  	const _n7 = _next(_n1);
  	const _n8 = _next(_n7);
  	const _n6 = _next(_n8);
  	_setNodes(_n1, () => bar);
  	_renderEffect(() => _setProp(_n0, "id", foo));
  	_setInsertionState(_n0, _n7);
  	const _n2 = _createIf(() => ok, () => {
  		const _n4 = _t0();
  		return _n4;
  	}, null, 33);
  	_setInsertionState(_n0, _n8);
  	const _n5 = _createComponent(Comp);
  	_applyTextModel(_n6, () => text, (_value) => text = _value);
  	_withVaporDirectives(_n0, [[_directive_dir]]);
  	return _n0;
  })();
  "#);
}
