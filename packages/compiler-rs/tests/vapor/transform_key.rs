use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn key() {
  let code = transform("<div key={id} />", None).code;
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createKeyedFragment(() => id, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn key_with_v_once() {
  let code = transform(r#"<div v-once key={id} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#,
  );
}

#[test]
fn key_with_v_if() {
  let code = transform("<div v-if={id} key={id} />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createIf(() => id, () => {
  		const _n2 = _createKeyedFragment(() => id, () => {
  			const _n4 = _t0();
  			return _n4;
  		});
  		return _n2;
  	}, null, 1);
  	return _n0;
  })();
  "#);
}

#[test]
fn key_with_anchor_insertion_in_middle() {
  let code = transform(
    "<div>
      <div></div>
      <div key={foo}></div>
      <div></div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, createKeyedFragment as _createKeyedFragment, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><div></div><!><div>", true);
  (() => {
  	const _n4 = _t1();
  	const _n3 = _next(_child(_n4), 1);
  	_setInsertionState(_n4, _n3, 1, true);
  	const _n0 = _createKeyedFragment(() => foo, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n4;
  })();
  "#);
}

#[test]
fn key_in_component() {
  let code = transform("<Comp><div key={key} /></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createKeyedFragment as _createKeyedFragment, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n3 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _createKeyedFragment(() => key, () => {
  			const _n2 = _t0();
  			return _n2;
  		});
  		return _n0;
  	}) }, true);
  	return _n3;
  })();
  "#);
}

#[test]
fn static_key() {
  let code = transform(
    "<>
      <div key={1} />
      <Comp key={1} />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n0, 1);
  	_setBlockKey(_n1, 1);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn boolean_static_expression_key() {
  let code = transform(
    "<>
      <div key={true} />
      <Comp key={true} />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n0, true);
  	_setBlockKey(_n1, true);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn null_static_expression_key() {
  let code = transform(
    "<>
      <div key={null} />
      <Comp key={null} />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n0, null);
  	_setBlockKey(_n1, null);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn v_once_with_static_key() {
  let code = transform(
    r#"<>
      <div v-once key="foo" />
      <Comp v-once key="foo" />
    </>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createComponent(Comp, null, null, null, true);
  	_setBlockKey(_n0, "foo");
  	_setBlockKey(_n1, "foo");
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn key_without_value() {
  let code = transform(
    r#"<>
      <div key />
      <Comp key />
    </>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n0, true);
  	_setBlockKey(_n1, true);
  	return [_n0, _n1];
  })();
  "#);
}
