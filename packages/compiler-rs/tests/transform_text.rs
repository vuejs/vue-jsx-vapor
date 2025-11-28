use compiler_rs::transform::transform;
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
  let code = transform("<>{ msg }</>", None).code;
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
fn expression_conditional() {
  let code = transform(
    "<>{ok? (<span>{msg}</span>) : fail ? (<div>fail</div>)  : null }</>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn expression_logical() {
  let code = transform("<>{ok && (<div>{msg}</div>)}</>", None).code;
  assert_snapshot!(code);
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
