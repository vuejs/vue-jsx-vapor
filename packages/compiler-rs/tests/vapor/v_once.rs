use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-once>
      { msg }
      <span class={clz} />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<div> <span></span></div>", true);
  (() => {
  	const _n2 = _t0();
  	const _n0 = _child(_n2, 0);
  	const _n1 = _next(_n0, 1);
  	_setNodes(_n0, msg);
  	_setClass(_n1, clz);
  	return _n2;
  })();
  "#);
}

#[test]
fn as_root_node() {
  let code = transform("<div id={foo} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n0 = _t0();
  	_setProp(_n0, "id", foo);
  	return _n0;
  })();
  "#);
}

#[test]
fn on_nested_plain_element() {
  let code = transform("<div><div id={foo} v-once /></div>", None).code;
  assert_snapshot!(code, @r#"
  import { child as _child, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div><div></div></div>", true);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _child(_n1, 0);
  	_setProp(_n0, "id", foo);
  	return _n1;
  })();
  "#);
}

#[test]
fn on_component() {
  let code = transform("<div><Comp id={foo} v-once /></div>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div></div>", true);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1, null, true);
  	const _n0 = _createComponent(Comp, { id: () => foo }, null, null, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn inside_v_once() {
  let code = transform("<div v-once><div v-once/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div></div></div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn with_v_if() {
  let code = transform("<div v-if={expr} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  (() => {
  	const _n0 = _createIf(() => expr, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn with_v_if_else() {
  let code = transform("<><div v-if={expr} v-once /><p v-else/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("<p></p>");
  (() => {
  	const _n0 = _createIf(() => expr, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn with_conditional_expression() {
  let code = transform(
    "<div v-once>{ok? <span>{msg}</span> : <div>fail</div> }</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> </span>");
  const _t1 = _template("<div>fail</div>");
  const _t2 = _template("<div></div>", true);
  (() => {
  	const _n5 = _t2();
  	_setInsertionState(_n5, null, true);
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, msg);
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, true);
  	return _n5;
  })();
  "#);
}

#[test]
fn with_v_for() {
  let code = transform("<div v-for={i in list} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _t0();
  		return _n2;
  	}, void 0, 4);
  	return _n0;
  })();
  "#);
}

#[test]
fn execution_order() {
  let code = transform(
    "<div>
      <span v-once>{ foo }</span>
      { bar }<br/>
      { baz }
      <div foo={true}>{foo}</div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, nthChild as _nthChild, setProp as _setProp, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><span> </span> <br> <div> </div></div>", true);
  (() => {
  	const _n4 = _t0();
  	const _n0 = _child(_n4, 0);
  	const _n1 = _next(_n0, 1);
  	const _n2 = _nthChild(_n4, 3, 3);
  	const _n3 = _next(_n2, 4);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, foo);
  	_setNodes(_n1, () => bar);
  	_setNodes(_n2, () => baz);
  	_setProp(_n3, "foo", true);
  	const _x3 = _txt(_n3);
  	_setNodes(_x3, () => foo);
  	return _n4;
  })();
  "#);
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-show />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VShowNoExpression));
}
