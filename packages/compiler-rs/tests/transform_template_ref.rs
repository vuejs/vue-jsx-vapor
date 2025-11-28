use compiler_rs::transform::transform;
use insta::assert_snapshot;

#[test]
fn static_ref() {
  let code = transform("<div ref=\"foo\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_ref() {
  let code = transform("<div ref={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn function_ref() {
  let code = transform(
    "<Comp v-slot={{baz}}>
      <div ref={bar => {
        foo.value = bar
        ;({ baz, bar: baz } = bar)
        console.log(foo.value, baz)
      }} />
  </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn ref_v_if() {
  let code = transform("<div ref={foo} v-if={true} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn ref_v_for() {
  let code = transform("<div ref={foo} v-for={item in [1,2,3]} />", None).code;
  assert_snapshot!(code);
}
