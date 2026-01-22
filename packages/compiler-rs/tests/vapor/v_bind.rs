use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div id={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_renderEffect(() => _setProp(n0, "id", id));
  	return n0;
  })();
  "#);
}

#[test]
fn no_expression() {
  let code = transform("<div id />", None).code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_setProp(n0, "id", true);
  	return n0;
  })();
  "#);
}

#[test]
fn empty_expression() {
  let code = transform("<div foo={}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	return n0;
  })();
  "#);
}

#[test]
fn camel_modifier() {
  let code = transform("<div foo-bar_camel={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_renderEffect(() => _setProp(n0, "fooBar", id));
  	return n0;
  })();
  "#);
}

#[test]
fn camel_modifier_with_no_expression() {
  let code = transform("<div foo-bar_camel />", None).code;
  assert_snapshot!(code, @r#"
  import { setAttr as _setAttr, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_setAttr(n0, "foo-bar", true);
  	return n0;
  })();
  "#);
}

#[test]
fn prop_modifier() {
  let code = transform("<div fooBar_prop={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setDOMProp as _setDOMProp, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_renderEffect(() => _setDOMProp(n0, "fooBar", id));
  	return n0;
  })();
  "#);
}

#[test]
fn prop_modifier_with_no_expression() {
  let code = transform("<div fooBar_prop />", None).code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_setProp(n0, "fooBar", true);
  	return n0;
  })();
  "#);
}

#[test]
fn attr_modifier() {
  let code = transform("<div foo-bar_attr={id}/>", None).code;
  assert_snapshot!(code, @r#"
  import { renderEffect as _renderEffect, setAttr as _setAttr, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_renderEffect(() => _setAttr(n0, "foo-bar", id));
  	return n0;
  })();
  "#);
}

#[test]
fn attr_modifier_with_no_expression() {
  let code = transform("<div foo-bar_attr />", None).code;
  assert_snapshot!(code, @r#"
  import { setAttr as _setAttr, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	_setAttr(n0, "foo-bar", true);
  	return n0;
  })();
  "#);
}

#[test]
fn with_constant_value() {
  let code = transform(
    "<div
      a={void 0}
      b={1 > 2}
      c={1 + 2}
      d={1 ? 2 : 3}
      e={(2)}
      f={`foo${1}`}
      g={1}
      h={'1'}
      i={true}
      j={null}
      l={{ foo: 1 }}
      n={{ ...{ foo: 1 } }}
      o={[1, , 3]}
      p={[1, ...[2, 3]]}
      q={[1, 2]}
      r={/\\s+/}
    />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div e=\"2\" f=\"foo1\" g=\"1\" h=\"1\"></div>", true);
  (() => {
  	const n0 = t0();
  	_setProp(n0, "a", void 0);
  	_setProp(n0, "b", 1 > 2);
  	_setProp(n0, "c", 1 + 2);
  	_setProp(n0, "d", 1 ? 2 : 3);
  	_setProp(n0, "i", true);
  	_setProp(n0, "j", null);
  	_setProp(n0, "l", { foo: 1 });
  	_setProp(n0, "n", { ...{ foo: 1 } });
  	_setProp(n0, "o", [
  		1,
  		,
  		3
  	]);
  	_setProp(n0, "p", [1, ...[2, 3]]);
  	_setProp(n0, "q", [1, 2]);
  	_setProp(n0, "r", /\s+/);
  	return n0;
  })();
  "#);
}

#[test]
fn number_value() {
  let code = transform(
    "<>
      <div depth={0} />
      <Comp depth={0} />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("<div depth=\"0\"></div>");
  (() => {
  	const n0 = t0();
  	const n1 = _createComponent(Comp, { depth: () => 0 });
  	return [n0, n1];
  })();
  "#);
}
