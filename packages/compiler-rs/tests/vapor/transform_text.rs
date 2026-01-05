use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn static_template() {
  let code = transform(
    "<div>
      <div>hello</div>
      <input />
      <span />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn interpolation() {
  let code = transform("<>{ 1 }{ 2 }{a +b +       c }</>", None).code;
  assert_snapshot!(code);
}

#[test]
fn on_consecutive_text() {
  let code = transform("<>{ \"hello world\" }</>", None).code;
  assert_snapshot!(code);
}

#[test]
fn consecutive_text() {
  let code = transform("<>{ msg }  <div/></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn escapes_raw_static_text_when_generating_the_template_string() {
  let code = transform("<code>&lt;script&gt;</code>", None).code;
  assert_snapshot!(code);
}

#[test]
fn text_like() {
  let code = transform("<div>{ (2) }{`foo${1}`}{1}{1n}</div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn conditional_expression() {
  let code = transform(
    "<>{ok? (<span>{msg}</span>) : fail ? (<div>fail</div>)  : null }</>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn multiple_conditional() {
  let code = transform("<>{ok? ok : fail} {foo ? foo : <span />}</>", None).code;
  assert_snapshot!(code);
}

#[test]
fn logical_expression() {
  let code = transform("<>{ok && (<div>{msg}</div>)}</>", None).code;
  assert_snapshot!(code);
}

#[test]
fn logical_expression_or() {
  let code = transform(r#"<div>{foo || <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  const t1 = _template("<div></div>", true);
  (() => {
    const n5 = t1();
    _setInsertionState(n5);
    const n0 = _createIf(() => foo, () => {
      const n2 = _createNodes(() => foo);
      return n2;
    }, () => {
      const n4 = t0();
      const x4 = _child(n4);
      _setNodes(x4, () => foo);
      return n4;
    });
    return n5;
  })();
  "#)
}

#[test]
fn logical_expression_coalesce() {
  let code = transform(r#"<div>{foo ?? <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  const t1 = _template("<div></div>", true);
  (() => {
    const n5 = t1();
    _setInsertionState(n5);
    const n0 = _createIf(() => foo == null, () => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => foo);
      return n2;
    }, () => {
      const n4 = _createNodes(() => foo);
      return n4;
    });
    return n5;
  })();
  "#)
}

#[test]
fn expression_map() {
  let code = transform(
    "<>{Array.from({ length: count.value }).map((_, index) => {
      if (index > 1) {
        return <div>1</div>
      } else {
        return [<span>({index}) lt 1</span>, <br />]
      }
    })}</>",
    None,
  )
  .code;
  assert_snapshot!(code);
}
