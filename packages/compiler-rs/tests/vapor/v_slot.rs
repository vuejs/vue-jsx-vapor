use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn implicit_default_slot() {
  let code = transform("<Comp><div/></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<div></div>");
  (() => {
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _t0();
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn on_component_default_slot() {
  let code = transform("<Comp v-slot={scope}>{ scope.foo + bar }</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx((scope) => {
  		const _n0 = _createNodes(() => scope.foo + bar);
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);

  assert!(code.contains("default: _withVaporCtx((scope) =>"));
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
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { named: _withVaporCtx((_slotProps0) => {
  		const _n0 = _createNodes(() => ({ foo: _slotProps0.foo }), () => ({ foo: _slotProps0.foo }));
  		return _n0;
  	}) }, true);
  	return _n2;
  })();
  "#);

  assert!(code.contains("named: _withVaporCtx((_slotProps0) =>"));
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
  import { template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("foo");
  (() => {
  	const _n5 = _createComponent(Comp, null, {
  		left: _withVaporCtx(() => {
  			const _n0 = _t0();
  			return _n0;
  		}),
  		right: _withVaporCtx(() => {
  			const _n3 = _createComponent(Comp, null, { left: _withVaporCtx(() => {
  				const _n2 = _t0();
  				return _n2;
  			}) });
  			return _n3;
  		})
  	}, true);
  	return _n5;
  })();
  "#);
}

#[test]
fn on_component_dynamically_named_slot() {
  let code = transform("<Comp v-slot:$named$={{ foo }}>{ foo + bar }</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { $: [() => ({
  		name: named,
  		fn: _withVaporCtx((_slotProps0) => {
  			const _n0 = _createNodes(() => _slotProps0.foo + bar);
  			return _n0;
  		})
  	})] }, true);
  	return _n1;
  })();
  "#);

  assert!(code.contains("fn: _withVaporCtx((_slotProps0) =>"));
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
  import { template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("foo");
  const _t1 = _template("bar");
  const _t2 = _template("<span></span>");
  (() => {
  	const _n4 = _createComponent(Comp, null, {
  		one: _withVaporCtx(() => {
  			const _n0 = _t0();
  			return _n0;
  		}),
  		default: _withVaporCtx(() => {
  			const _n2 = _t1();
  			const _n3 = _t2();
  			return [_n2, _n3];
  		})
  	}, true);
  	return _n4;
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
  import { template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("foo");
  const _t1 = _template("<span></span>");
  (() => {
  	const _n4 = _createComponent(Comp, null, {
  		one: _withVaporCtx(() => {
  			const _n0 = _t0();
  			return _n0;
  		}),
  		default: _withVaporCtx(() => {
  			const _n2 = _t0();
  			const _n3 = _t1();
  			return [_n2, _n3];
  		})
  	}, true);
  	return _n4;
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
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n4 = _createComponent(Comp, null, { default: _withVaporCtx((_slotProps0) => {
  		const _n1 = _createComponent(Inner, null, { default: _withVaporCtx((_slotProps1) => {
  			const _n0 = _createNodes(() => _slotProps0.foo + _slotProps1.bar + baz);
  			return _n0;
  		}) });
  		const _n2 = _createNodes(() => _slotProps0.foo + bar + baz);
  		return [_n1, _n2];
  	}) }, true);
  	return _n4;
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
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [() => ({
  		name,
  		fn: _withVaporCtx(() => {
  			const _n0 = _createNodes(() => foo);
  			return _n0;
  		})
  	})] }, true);
  	return _n2;
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
  import { createForSlots as _createForSlots, withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [() => _createForSlots(list, (item) => ({
  		name: item,
  		fn: _withVaporCtx((_slotProps0) => {
  			const _n0 = _createNodes(() => _slotProps0.bar);
  			return _n0;
  		})
  	}))] }, true);
  	return _n2;
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
  import { template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("condition slot");
  const _t1 = _template("another condition");
  const _t2 = _template("other condition");
  const _t3 = _template("else condition");
  (() => {
  	const _n8 = _createComponent(Comp, null, { $: [() => condition ? {
  		name: "condition",
  		fn: _withVaporCtx(() => {
  			const _n0 = _t0();
  			return _n0;
  		})
  	} : anotherCondition ? {
  		name: "condition",
  		fn: _withVaporCtx((_slotProps0) => {
  			const _n2 = _t1();
  			return _n2;
  		})
  	} : otherCondition ? {
  		name: "condition",
  		fn: _withVaporCtx(() => {
  			const _n4 = _t2();
  			return _n4;
  		})
  	} : {
  		name: "condition",
  		fn: _withVaporCtx(() => {
  			const _n6 = _t3();
  			return _n6;
  		})
  	}] }, true);
  	return _n8;
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
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { "nav-bar-title-before": _withVaporCtx(() => {
  		return null;
  	}) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn nested_component_slot() {
  let code = transform("<A><B/></A>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(A, null, { default: _withVaporCtx(() => {
  		const _n0 = _createComponent(B);
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn slot_tag_only() {
  let code = transform(r#"<Comp><slot /></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _createSlot("default");
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn slot_tag_with_v_if() {
  let code = transform(r#"<Comp><slot v-if={ok} /></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, createSlot as _createSlot, withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n3 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _createIf(() => ok, () => {
  			const _n2 = _createSlot("default");
  			return _n2;
  		});
  		return _n0;
  	}) }, true);
  	return _n3;
  })();
  "#);
}

#[test]
fn slot_tag_with_v_for() {
  let code = transform(r#"<Comp><slot v-for={a in b} /></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createSlot as _createSlot, withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n3 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _createFor(() => b, (_for_item0) => {
  			const _n2 = _createSlot("default");
  			return _n2;
  		}, void 0, 2);
  		return _n0;
  	}) }, true);
  	return _n3;
  })();
  "#);
}

#[test]
fn slot_tag_with_template() {
  let code = transform(r#"<Comp><template><slot /></template></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<template></template>");
  (() => {
  	const _n2 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n1 = _t0();
  		const _n0 = _createSlot("default");
  		return [_n0, _n1];
  	}) }, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_tag_with_nested_component() {
  let code = transform(r#"<Comp><Comp><slot/></Comp></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n1 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  			const _n0 = _createSlot("default");
  			return _n0;
  		}) });
  		return _n1;
  	}) }, true);
  	return _n2;
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
