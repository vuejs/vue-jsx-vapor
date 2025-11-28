use compiler_rs::transform::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div id={id}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn no_expression() {
  let code = transform("<div id />", None).code;
  assert_snapshot!(code);
}

#[test]
fn camel_modifier() {
  let code = transform("<div foo-bar_camel={id}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn camel_modifier_with_no_expression() {
  let code = transform("<div foo-bar_camel />", None).code;
  assert_snapshot!(code);
}

#[test]
fn prop_modifier() {
  let code = transform("<div fooBar_prop={id}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn prop_modifier_with_no_expression() {
  let code = transform("<div fooBar_prop />", None).code;
  assert_snapshot!(code);
}

#[test]
fn attr_modifier() {
  let code = transform("<div foo-bar_attr={id}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn attr_modifier_with_no_expression() {
  let code = transform("<div foo-bar_attr />", None).code;
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}
