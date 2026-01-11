use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn implicit_default_slot() {
  let code = transform("<Comp><div/></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
    const n1 = _createComponent(Comp, null, { default: () => {
      const n0 = t0();
      return n0;
    } }, true);
    return n1;
  })();
  "#);
}

#[test]
fn on_component_default_slot() {
  let code = transform("<Comp v-slot={scope}>{ scope.foo + bar }</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n1 = _createComponent(Comp, null, { default: (scope) => {
      const n0 = _createNodes(() => scope.foo + bar);
      return n0;
    } }, true);
    return n1;
  })();
  "#);

  assert!(code.contains("default: (scope) =>"));
  assert!(code.contains("scope.foo + bar"));
}

#[test]
fn on_component_named_slot() {
  let code = transform(
    "<Comp v-slot:named={({ foo })}>{{ foo }}{{ foo: foo }}</Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n2 = _createComponent(Comp, null, { named: (_slotProps0) => {
      const n0 = _createNodes(() => ({ foo: _slotProps0.foo }), () => ({ foo: _slotProps0.foo }));
      return n0;
    } }, true);
    return n2;
  })();
  "#);

  assert!(code.contains("named: (_slotProps0) =>"));
  assert!(code.contains("{ foo: _slotProps0.foo }"));
}

#[test]
fn on_component_named_slot_multiple() {
  let code = transform(
    "<Comp>
      <template v-slot:left>
        foo
      </template>
      <template v-slot:right>
        <Comp v-slot:left>foo</Comp>
      </template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("foo");
  (() => {
    const n10 = _createComponent(Comp, null, {
      left: () => {
        const n1 = t0();
        return n1;
      },
      right: () => {
        const n6 = _createComponent(Comp, null, { left: () => {
          const n5 = t0();
          return n5;
        } });
        return n6;
      }
    }, true);
    return n10;
  })();
  "#);
}

#[test]
fn on_component_dynamically_named_slot() {
  let code = transform("<Comp v-slot:$named$={{ foo }}>{ foo + bar }</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n1 = _createComponent(Comp, null, { $: [() => ({
      name: named,
      fn: (_slotProps0) => {
        const n0 = _createNodes(() => _slotProps0.foo + bar);
        return n0;
      }
    })] }, true);
    return n1;
  })();
  "#);

  assert!(code.contains("fn: (_slotProps0) =>"));
  assert!(code.contains("_slotProps0.foo + bar"));
}

#[test]
fn named_slots_with_implicit_default_slot() {
  let code = transform(
    "<Comp>
      <template v-slot:one>foo</template>bar<span/>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("foo");
  const t1 = _template("bar");
  const t2 = _template("<span></span>");
  (() => {
    const n6 = _createComponent(Comp, null, {
      one: () => {
        const n1 = t0();
        return n1;
      },
      default: () => {
        const n3 = t1();
        const n4 = t2();
        return [n3, n4];
      }
    }, true);
    return n6;
  })();
  "#);
}

#[test]
fn named_slots_with_comment() {
  let code = transform(
    "<Comp>
      {/* foo */}
      <template v-slot:one>foo</template>foo<span/>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("foo");
  const t1 = _template("<span></span>");
  (() => {
    const n8 = _createComponent(Comp, null, {
      one: () => {
        const n3 = t0();
        return n3;
      },
      default: () => {
        const n5 = t0();
        const n6 = t1();
        return [n5, n6];
      }
    }, true);
    return n8;
  })();
  "#);
}

#[test]
fn nested_slots_scoping() {
  let code = transform(
    "<Comp>
      <template v-slot:default={{ foo }}>
        <Inner v-slot={{ bar }}>
          { foo + bar + baz }
        </Inner>
        { foo + bar + baz }
      </template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n11 = _createComponent(Comp, null, { default: (_slotProps0) => {
      const n5 = _createComponent(Inner, null, { default: (_slotProps1) => {
        const n3 = _createNodes(() => _slotProps0.foo + _slotProps1.bar + baz);
        return n3;
      } });
      const n7 = _createNodes(() => _slotProps0.foo + bar + baz);
      return [n5, n7];
    } }, true);
    return n11;
  })();
  "#);
}

#[test]
fn dynamic_slots_name() {
  let code = transform(
    "<Comp>
      <template v-slot:$name$>{foo}</template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n4 = _createComponent(Comp, null, { $: [() => ({
      name,
      fn: () => {
        const n1 = _createNodes(() => foo);
        return n1;
      }
    })] }, true);
    return n4;
  })();
  "#);
}

#[test]
fn dynamic_slots_name_with_v_for() {
  let code = transform(
    "<Comp>
      <template v-for={item in list} v-slot:$item$={{ bar }}>{ bar }</template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createForSlots as _createForSlots } from "vue";
  (() => {
    const n4 = _createComponent(Comp, null, { $: [() => _createForSlots(list, (item) => ({
      name: item,
      fn: (_slotProps0) => {
        const n1 = _createNodes(() => _slotProps0.bar);
        return n1;
      }
    }))] }, true);
    return n4;
  })();
  "#);
}

#[test]
fn dynamic_slots_name_with_v_if_and_v_else_if() {
  let code = transform(
    "<Comp>
      <template v-if={condition} v-slot:condition>condition slot</template>
      <template v-else-if={anotherCondition} v-slot:condition={{ foo, bar }}>another condition</template>
      <template v-else-if={otherCondition} v-slot:condition>other condition</template>
      <template v-else v-slot:condition>else condition</template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("condition slot");
  const t1 = _template("another condition");
  const t2 = _template("other condition");
  const t3 = _template("else condition");
  (() => {
    const n13 = _createComponent(Comp, null, { $: [() => condition ? {
      name: "condition",
      fn: () => {
        const n1 = t0();
        return n1;
      }
    } : anotherCondition ? {
      name: "condition",
      fn: (_slotProps0) => {
        const n4 = t1();
        return n4;
      }
    } : otherCondition ? {
      name: "condition",
      fn: () => {
        const n7 = t2();
        return n7;
      }
    } : {
      name: "condition",
      fn: () => {
        const n10 = t3();
        return n10;
      }
    }] }, true);
    return n13;
  })();
  "#);
}

#[test]
fn quote_slot_name() {
  let code = transform(
    "<Comp>
      <template v-slot:nav-bar-title-before></template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n3 = _createComponent(Comp, null, { "nav-bar-title-before": () => {
      return null;
    } }, true);
    return n3;
  })();
  "#);
}

#[test]
fn nested_component_slot() {
  let code = transform("<A><B/></A>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
    const n1 = _createComponent(A, null, { default: () => {
      const n0 = _createComponent(B);
      return n0;
    } }, true);
    return n1;
  })();
  "#);
}

#[test]
fn error_on_extraneous_children_with_named_default_slot() {
  let error = RefCell::new(None);
  transform(
    "<Comp>
      <template v-slot:default>foo</template>bar
    </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(
    *error.borrow(),
    Some(ErrorCodes::VSlotExtraneousDefaultSlotChildren)
  );
}

#[test]
fn error_on_duplicated_slot_names() {
  let error = RefCell::new(None);
  transform(
    "<Comp>
      <template v-slot:foo></template>
      <template v-slot:foo></template>
    </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotDuplicateSlotNames));
}

#[test]
fn error_on_invalid_mixed_slot_usage() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slot={foo}>
      <template v-slot:foo></template>
    </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMixedSlotUsage));
}

#[test]
fn error_on_v_slot_usage_on_plain_elements() {
  let error = RefCell::new(None);
  transform(
    "<div v-slot/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMisplaced));
}
