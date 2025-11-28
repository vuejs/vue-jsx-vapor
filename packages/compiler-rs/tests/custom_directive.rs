use compiler_rs::transform::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div v-example></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn binding_value() {
  let code = transform("<div v-example={msg}></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn static_parameters() {
  let code = transform("<div v-example:foo={msg}></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn modifiers() {
  let code = transform("<div v-example_bar={msg}></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn modifiers_with_binding() {
  let code = transform("<div v-example_foo-bar></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn static_argument_and_modifiers() {
  let code = transform("<div v-example:foo_bar={msg}></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_argument() {
  let code = transform("<div v-example:$foo$={msg}></div>", None).code;
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}
