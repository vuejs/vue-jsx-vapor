use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div v-example></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[vExample]]);
    return n0;
  })();
  "#);
}

#[test]
fn binding_value() {
  let code = transform("<div v-example={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[vExample, () => msg]]);
    return n0;
  })();
  "#);
}

#[test]
fn static_parameters() {
  let code = transform("<div v-example:foo={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[
      vExample,
      () => msg,
      "foo"
    ]]);
    return n0;
  })();
  "#);
}

#[test]
fn modifiers() {
  let code = transform("<div v-example_bar={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[
      vExample,
      () => msg,
      void 0,
      { bar: true }
    ]]);
    return n0;
  })();
  "#);
}

#[test]
fn modifiers_with_binding() {
  let code = transform("<div v-example_foo-bar></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[
      vExample,
      void 0,
      void 0,
      { "foo-bar": true }
    ]]);
    return n0;
  })();
  "#);
}

#[test]
fn static_argument_and_modifiers() {
  let code = transform("<div v-example:foo_bar={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[
      vExample,
      () => msg,
      "foo",
      { bar: true }
    ]]);
    return n0;
  })();
  "#);
}

#[test]
fn dynamic_argument() {
  let code = transform("<div v-example:$foo$={msg}></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
    const n0 = t0();
    _withVaporDirectives(n0, [[
      vExample,
      () => msg,
      foo
    ]]);
    return n0;
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
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template, withVaporDirectives as _withVaporDirectives } from "vue";
  const t0 = _template("<div></div>");
  (() => {
    const n0 = _createComponent(Comp, null, { default: () => {
      const n2 = _createIf(() => true, () => {
        const n5 = t0();
        _setInsertionState(n5);
        const n4 = _createComponent(Bar);
        _withVaporDirectives(n4, [[
          vHello,
          void 0,
          void 0,
          { world: true }
        ]]);
        return n5;
      }, null, true);
      return n2;
    } }, true);
    _withVaporDirectives(n0, [[vTest]]);
    return n0;
  })();
  "#);
}
