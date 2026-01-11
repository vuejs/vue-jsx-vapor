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
  const t0 = _template("<div> <span></span></div>", true);
  (() => {
    const n2 = t0();
    const n0 = _child(n2);
    const n1 = _next(n0);
    _setNodes(n0, msg);
    _setClass(n1, clz);
    return n2;
  })();
  "#);
}

#[test]
fn as_root_node() {
  let code = transform("<div id={foo} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _setProp(n0, "id", foo);
    return n0;
  })();
  "#);
}

#[test]
fn on_nested_plain_element() {
  let code = transform("<div><div id={foo} v-once /></div>", None).code;
  assert_snapshot!(code, @r#"
  import { child as _child, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div><div></div></div>", true);
  (() => {
    const n1 = t0();
    const n0 = _child(n1);
    _setProp(n0, "id", foo);
    return n1;
  })();
  "#);
}

#[test]
fn on_component() {
  let code = transform("<div><Comp id={foo} v-once /></div>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n1 = t0();
    _setInsertionState(n1);
    const n0 = _createComponent(Comp, { id: () => foo }, null, null, true);
    return n1;
  })();
  "#);
}

#[test]
fn inside_v_once() {
  let code = transform("<div v-once><div v-once/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<div><div></div></div>", true);
  (() => {
    const n0 = t0();
    return n0;
  })();
  "#);
}

#[test]
fn with_v_if() {
  let code = transform("<div v-if={expr} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
    const n0 = _createIf(() => expr, () => {
      const n2 = t0();
      return n2;
    }, null, true);
    return n0;
  })();
  "#);
}

#[test]
fn with_v_if_else() {
  let code = transform("<><div v-if={expr} v-once /><p v-else/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const t0 = _template("<div></div>");
  const t1 = _template("<p></p>");
  (() => {
    const n0 = _createIf(() => expr, () => {
      const n2 = t0();
      return n2;
    }, () => {
      const n4 = t1();
      return n4;
    }, true);
    return n0;
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
  import { child as _child, createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<span> </span>");
  const t1 = _template("<div>fail</div>");
  const t2 = _template("<div></div>", true);
  (() => {
    const n5 = t2();
    _setInsertionState(n5);
    const n0 = _createIf(() => ok, () => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, msg);
      return n2;
    }, () => {
      const n4 = t1();
      return n4;
    }, true);
    return n5;
  })();
  "#);
}

#[test]
fn with_v_for() {
  let code = transform("<div v-for={i in list} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
    const n0 = _createFor(() => list, (_for_item0) => {
      const n2 = t0();
      return n2;
    }, void 0, 4);
    return n0;
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
  import { child as _child, next as _next, nthChild as _nthChild, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div><span> </span> <br> <div> </div></div>", true);
  (() => {
    const n4 = t0();
    const n0 = _child(n4);
    const n1 = _next(n0);
    const n2 = _nthChild(n4, 3);
    const n3 = _next(n2);
    const x0 = _child(n0);
    _setNodes(x0, foo);
    _setNodes(n1, () => bar);
    _setNodes(n2, () => baz);
    _setProp(n3, "foo", true);
    const x3 = _child(n3);
    _setNodes(x3, () => foo);
    return n4;
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
