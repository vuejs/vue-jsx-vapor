use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div v-example></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
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
  let code = transform("<div v-example={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[_directive_example, () => msg]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn static_parameters() {
  let code = transform("<div v-example:foo={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		"foo"
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn modifiers() {
  let code = transform("<div v-example_bar={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
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
  let code = transform("<div v-example_foo-bar></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
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
  let code = transform("<div v-example:foo_bar={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		"foo",
  		{ bar: true }
  	]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamic_argument() {
  let code = transform("<div v-example:$foo$={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { resolveDirective as _resolveDirective, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _directive_example = _resolveDirective("example");
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		_directive_example,
  		() => msg,
  		foo
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, resolveDirective as _resolveDirective, setInsertionState as _setInsertionState, template as _template, withVaporCtx as _withVaporCtx, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>");
  (() => {
  	const _directive_test = _resolveDirective("test");
  	const _directive_hello = _resolveDirective("hello");
  	const _n0 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n1 = _createIf(() => true, () => {
  			const _n4 = _t0();
  			_setInsertionState(_n4, null, true);
  			const _n3 = _createComponent(Bar);
  			_withVaporDirectives(_n3, [[
  				_directive_hello,
  				void 0,
  				void 0,
  				{ world: true }
  			]]);
  			return _n4;
  		}, null, true);
  		return _n1;
  	}) }, true);
  	_withVaporDirectives(_n0, [[_directive_test]]);
  	return _n0;
  })();
  "#);
}

#[test]
fn none_resolve_directive() {
  let code = transform("<div vExample={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_withVaporDirectives(_n0, [[
  		vExample,
  		() => msg,
  		"vExample"
  	]]);
  	return _n0;
  })();
  "#);
}
