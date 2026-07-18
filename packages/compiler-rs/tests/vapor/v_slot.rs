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
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n1 = _createComponent(Comp, null, () => {
  		const _n0 = _t0();
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn on_component_default_slot() {
  let code = transform("<Comp v-slot={scope}>{ scope.foo + bar }</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, _extend((scope) => {
  		const _n0 = _createNodes(() => scope.foo + bar);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n1;
  })();
  "#);

  assert!(!code.contains("_withVaporCtx((scope) =>"));
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
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { named: _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => ({ foo: _slotProps0.foo }), () => ({ foo: _slotProps0.foo }));
  		return _n0;
  	}, { _: 8 }) }, true);
  	return _n2;
  })();
  "#);

  assert!(!code.contains("named: _withVaporCtx((_slotProps0) =>"));
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
  const _t0 = _template("foo", 2);
  (() => {
  	const _n5 = _createComponent(Comp, null, {
  		left: () => {
  			const _n0 = _t0();
  			return _n0;
  		},
  		right: () => {
  			const _n3 = _createComponent(Comp, null, { left: () => {
  				const _n2 = _t0();
  				return _n2;
  			} });
  			return _n3;
  		}
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
  (() => {
  	const _n1 = _createComponent(Comp, null, { $: [{
  		name: named,
  		fn: (_slotProps0) => {
  			const _n0 = _createNodes(() => _slotProps0.foo + bar);
  			return _n0;
  		}
  	}] }, true);
  	return _n1;
  })();
  "#);

  assert!(!code.contains("fn: _withVaporCtx((_slotProps0) =>"));
  assert!(code.contains("_slotProps0.foo + bar"));
}

#[test]
fn nested_component_should_not_inherit_parent_slots() {
  let code = transform(
    "<Comp>
      <template v-slot:header></template>
      <Bar />
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, {
  		header: _extend(() => {
  			return [];
  		}, { _: 8 }),
  		default: () => {
  			const _n1 = _createComponent(Bar);
  			return _n1;
  		}
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_alias_uses_original_key() {
  let code = transform(
    r#"<Comp><template v-slot:default={{ msg: msg1 }}>{ msg1 }</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _slotProps0.msg);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_nested_destructuring() {
  let code = transform(
    r#"<Comp><template v-slot:default={{ foo: { bar: baz } }}>{ baz }</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _slotProps0.foo.bar);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_computed_key_destructuring() {
  let code = transform(
    r#"<Comp><template v-slot:default={{ [key.value]: val }}>{{ val }}</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => ({ val: _slotProps0[key.value] }));
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_rest_destructuring() {
  let code = transform(
    r#"<Comp><template v-slot:default={{ foo, ...rest }}>{ rest.bar }</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend, getRestElement as _getRestElement } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _getRestElement(_slotProps0, ["foo"]).bar);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_array_rest_destructuring() {
  let code = transform(
    r#"<Comp><template v-slot:default={{ arr: [first, ...rest] }}>{ rest[0] }</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _slotProps0.arr.slice(1)[0]);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_rest_with_computed_keys_preserved() {
  let code = transform(
    r#"<Comp><template v-slot:default={{ foo, [key]: val, ...rest }}>{ foo + rest.other }</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend, getRestElement as _getRestElement } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _slotProps0.foo + _getRestElement(_slotProps0, ["foo", key]).other);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_prop_assignment() {
  let code = transform(
    r#"<Comp v-slot={{ foo, bar }}>{foo++}{bar.value=1}</Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _slotProps0.foo++, () => _slotProps0.bar.value = 1);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
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
  const _t0 = _template("foo", 2);
  const _t1 = _template("bar", 2);
  const _t2 = _template("<span>", 2);
  (() => {
  	const _n4 = _createComponent(Comp, null, {
  		one: () => {
  			const _n0 = _t0();
  			return _n0;
  		},
  		default: () => {
  			const _n2 = _t1();
  			const _n3 = _t2();
  			return [_n2, _n3];
  		}
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
  import { template as _template } from "vue";
  const _t0 = _template("foo", 2);
  const _t1 = _template("<span>", 2);
  (() => {
  	const _n4 = _createComponent(Comp, null, {
  		one: () => {
  			const _n0 = _t0();
  			return _n0;
  		},
  		default: () => {
  			const _n2 = _t0();
  			const _n3 = _t1();
  			return [_n2, _n3];
  		}
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
  import { extend as _extend } from "vue";
  (() => {
  	const _n4 = _createComponent(Comp, null, (_slotProps0) => {
  		const _n1 = _createComponent(Inner, null, _extend((_slotProps1) => {
  			const _n0 = _createNodes(() => _slotProps0.foo + _slotProps1.bar + baz);
  			return _n0;
  		}, { _: 8 }));
  		const _n2 = _createNodes(() => _slotProps0.foo + bar + baz);
  		return [_n1, _n2];
  	}, true);
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
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [{
  		name,
  		fn: () => {
  			const _n0 = _createNodes(() => foo);
  			return _n0;
  		}
  	}] }, true);
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
  import { createForSlots as _createForSlots } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [_createForSlots(list, (item) => ({
  		name: item,
  		fn: (_slotProps0) => {
  			const _n0 = _createNodes(() => _slotProps0.bar);
  			return _n0;
  		}
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
  import { template as _template } from "vue";
  const _t0 = _template("condition slot", 2);
  const _t1 = _template("another condition", 2);
  const _t2 = _template("other condition", 2);
  const _t3 = _template("else condition", 2);
  (() => {
  	const _n8 = _createComponent(Comp, null, { $: [() => condition ? {
  		name: "condition",
  		fn: () => {
  			const _n0 = _t0();
  			return _n0;
  		}
  	} : anotherCondition ? {
  		name: "condition",
  		fn: (_slotProps0) => {
  			const _n2 = _t1();
  			return _n2;
  		}
  	} : otherCondition ? {
  		name: "condition",
  		fn: () => {
  			const _n4 = _t2();
  			return _n4;
  		}
  	} : {
  		name: "condition",
  		fn: () => {
  			const _n6 = _t3();
  			return _n6;
  		}
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
  import { extend as _extend } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { "nav-bar-title-before": _extend(() => {
  		return [];
  	}, { _: 8 }) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn nested_component_slot() {
  let code = transform("<A><B/></A>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n1 = _createComponent(A, null, () => {
  		const _n0 = _createComponent(B);
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn marks_root_v_if_slot_content_as_slot_root() {
  let code = transform("<Comp><span v-if={show}/></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  (() => {
  	const _n3 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, null, 161);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n3;
  })();
  "#);
}

#[test]
fn does_not_mark_non_root_v_if_slot_content_as_slot_root() {
  let code = transform("<Comp><div><span v-if={show}/></div></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  const _t1 = _template("<div>");
  (() => {
  	const _n4 = _createComponent(Comp, null, () => {
  		const _n3 = _t1();
  		_setInsertionState(_n3, null, 0);
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, null, 33);
  		return _n3;
  	}, true);
  	return _n4;
  })();
  "#);
}

#[test]
fn static_root_sibling_keeps_slot_content_stable() {
  let code = transform("<Comp><span/><div v-if={show}/></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<span></span>", 2);
  const _t1 = _template("<div>", 2);
  (() => {
  	const _n4 = _createComponent(Comp, null, () => {
  		const _n0 = _t0();
  		const _n1 = _createIf(() => show, () => {
  			const _n3 = _t1();
  			return _n3;
  		}, null, 161);
  		return [_n0, _n1];
  	}, true);
  	return _n4;
  })();
  "#);
}

#[test]
fn static_component_root_sibling_keeps_slot_content_stable() {
  let code = transform("<Comp><Foo/><Component is={view}/></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n2 = _createComponent(Comp, null, () => {
  		const _n0 = _createComponent(Foo);
  		const _n1 = _createComponent(Component, { is: () => view });
  		return [_n0, _n1];
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn all_dynamic_root_slot_content_is_non_stable() {
  let code = transform(
    "<Comp><div v-for={item in list}/><p v-if={ok}/></Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<p>", 2);
  (() => {
  	const _n6 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createFor(() => list, (_for_item0) => {
  			const _n2 = _t0();
  			return _n2;
  		}, void 0, 40);
  		const _n3 = _createIf(() => ok, () => {
  			const _n5 = _t1();
  			return _n5;
  		}, null, 161);
  		return [_n0, _n3];
  	}, { _: 8 }), true);
  	return _n6;
  })();
  "#);
}

#[test]
fn root_v_for_with_root_v_if_slot_content_is_non_stable() {
  let code = transform(
    "<Comp><div v-for={item in list}/><p v-if={ok}/></Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<p>", 2);
  (() => {
  	const _n6 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createFor(() => list, (_for_item0) => {
  			const _n2 = _t0();
  			return _n2;
  		}, void 0, 40);
  		const _n3 = _createIf(() => ok, () => {
  			const _n5 = _t1();
  			return _n5;
  		}, null, 161);
  		return [_n0, _n3];
  	}, { _: 8 }), true);
  	return _n6;
  })();
  "#);
}

#[test]
fn comment_with_dynamic_root_slot_content_is_non_stable() {
  let code = transform("<Comp>{/* foo */}<div v-if={show}/></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n3 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, null, 161);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n3;
  })();
  "#);
}

#[test]
fn marks_root_slot_outlet_fallbck_as_slot_root() {
  let code = transform("<Comp><slot><span v-if={show}/></slot></Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, createSlot as _createSlot, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  (() => {
  	const _n4 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createSlot("default", null, () => {
  			const _n1 = _createIf(() => show, () => {
  				const _n3 = _t0();
  				return _n3;
  			}, null, 161);
  			return _n1;
  		}, 4);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n4;
  })();
  "#);
}

#[test]
fn slot_tag_only() {
  let code = transform(r#"<Comp><slot /></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, extend as _extend } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createSlot("default", null, null, 4);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n1;
  })();
  "#);
}

#[test]
fn slot_tag_with_v_if() {
  let code = transform(r#"<Comp><slot v-if={ok} /></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, createSlot as _createSlot, extend as _extend } from "vue";
  (() => {
  	const _n3 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => ok, () => {
  			const _n2 = _createSlot("default", null, null, 4);
  			return _n2;
  		}, null, 129);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n3;
  })();
  "#);
}

#[test]
fn slot_tag_with_v_for() {
  let code = transform(r#"<Comp><slot v-for={a in b} /></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createSlot as _createSlot, extend as _extend } from "vue";
  (() => {
  	const _n3 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createFor(() => b, (_for_item0) => {
  			const _n2 = _createSlot("default", null, null, 4);
  			return _n2;
  		}, void 0, 48);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n3;
  })();
  "#);
}

#[test]
fn slot_tag_with_template() {
  let code = transform(r#"<Comp><template v-slot><slot /></template></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createSlot("default", null, null, 4);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_tag_with_nested_component() {
  let code = transform(r#"<Comp><Comp><slot/></Comp></Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, () => {
  		const _n1 = _createComponent(Comp, null, _extend(() => {
  			const _n0 = _createSlot("default", null, null, 4);
  			return _n0;
  		}, { _: 8 }));
  		return _n1;
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn default_slot_with_v_if_directive() {
  let code = transform(
    r#"<Comp><template v-slot v-if={show}></template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n1 = _createComponent(Comp, null, { $: [() => show ? {
  		name: "default",
  		fn: () => {
  			return [];
  		}
  	} : undefined] }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn default_slot_with_v_for_directive() {
  let code = transform(
    r#"<Comp><template v-slot v-for={item in list}>{item}</template></Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createForSlots as _createForSlots } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [_createForSlots(list, (item) => ({
  		name: "default",
  		fn: () => {
  			const _n0 = _createNodes(() => item);
  			return _n0;
  		}
  	}))] }, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_with_only_static_elements_is_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div>static content</div>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div>static content", 2);
  (() => {
  	const _n2 = _createComponent(Comp, null, () => {
  		const _n0 = _t0();
  		return _n0;
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_with_component_is_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <ChildComp />
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n2 = _createComponent(Comp, null, () => {
  		const _n0 = _createComponent(ChildComp);
  		return _n0;
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_with_slot_outlet_is_non_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <slot />
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createSlot as _createSlot, extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createSlot("default", null, null, 4);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn dynamic_slot_source_with_slot_outlet_keeps_dynamic_slot_function() {
  let code = transform(
    r#"<Comp>
      <template v-for={(_, name) in slots} v-slot:$name$>
        <slot name={name} />
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(code.contains("_createForSlots"));
  assert!(!code.contains("fn: _withVaporCtx(() =>"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createForSlots as _createForSlots, createSlot as _createSlot } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [_createForSlots(slots, (_, name) => ({
  		name,
  		fn: () => {
  			const _n0 = _createSlot(() => name, null, null, 4);
  			return _n0;
  		}
  	}))] }, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_with_component_inside_v_if_is_non_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div v-if={show}>
          <ChildComp />
        </div>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n5 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n3 = _t0();
  			_setInsertionState(_n3, null, 0);
  			const _n2 = _createComponent(ChildComp);
  			return _n3;
  		}, null, 129);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n5;
  })();
  "#);
}

#[test]
fn slot_with_component_inside_v_for_is_non_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div v-for={item in items}>
          <ChildComp />
        </div>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, extend as _extend, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n5 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createFor(() => items, (_for_item0) => {
  			const _n3 = _t0();
  			_setInsertionState(_n3, null, 0);
  			const _n2 = _createComponent(ChildComp);
  			return _n3;
  		}, void 0, 40);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n5;
  })();
  "#);
}

#[test]
fn slot_with_nested_v_if_containing_component_is_non_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div v-if={a}>
          <span v-if={b}>
            <ChildComp />
          </span>
        </div>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>");
  const _t1 = _template("<div>");
  (() => {
  	const _n8 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => a, () => {
  			const _n6 = _t1();
  			_setInsertionState(_n6, null, 0);
  			const _n2 = _createIf(() => b, () => {
  				const _n5 = _t0();
  				_setInsertionState(_n5, null, 0);
  				const _n4 = _createComponent(ChildComp);
  				return _n5;
  			});
  			return _n6;
  		}, null, 129);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n8;
  })();
  "#);
}

#[test]
fn slot_with_only_text_interpolation_is_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        {message}
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createNodes(() => message);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_with_v_if_but_no_component_is_non_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div v-if={show}>content</div>
        <span v-else>fallback</span>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<div>content", 2);
  const _t1 = _template("<span>fallback", 2);
  (() => {
  	const _n7 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, () => {
  			const _n4 = _t1();
  			return _n4;
  		}, 485);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n7;
  })();
  "#);
}

#[test]
fn slot_with_v_for_but_no_component_is_none_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div v-for={item in items}>{item}</div>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, extend as _extend, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n4 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createFor(() => items, (_for_item0) => {
  			const _n2 = _t0();
  			const _x2 = _txt(_n2);
  			_setNodes(_x2, () => _for_item0.value);
  			return _n2;
  		}, void 0, 40);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n4;
  })();
  "#);
}

#[test]
fn slot_with_custom_element_is_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <my-element></my-element>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createPlainElement as _createPlainElement } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, () => {
  		const _n0 = _createPlainElement("my-element");
  		return _n0;
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn slot_with_custom_element_inside_v_if_is_non_stable() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default>
        <div v-if={show}>
          <my-element></my-element>
        </div>
      </template>
    </Comp>"#,
    None,
  )
  .code;
  assert!(!code.contains("withVaporCtx"));
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, createPlainElement as _createPlainElement, extend as _extend, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n5 = _createComponent(Comp, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n3 = _t0();
  			_setInsertionState(_n3, null, 0);
  			const _n2 = _createPlainElement("my-element");
  			return _n3;
  		}, null, 129);
  		return _n0;
  	}, { _: 8 }), true);
  	return _n5;
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

#[test]
fn slot_v_else_missing_adjacent_v_if_should_report_compiler_error() {
  let error = RefCell::new(None);
  transform(
    "<>
      <Comp><template v-slot:foo v-else>foo</template></Comp>
      <Comp><template v-slot:foo v-else-if={ok}>foo</template></Comp>
    </>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn array_args() {
  let code = transform(
    "<Comp v-slot={[foo, bar]}>
      {foo}
    </Com>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n1 = _createComponent(Comp, null, { $: [{
  		name: bar,
  		fn: (foo) => {
  			const _n0 = _createNodes(() => foo);
  			return _n0;
  		}
  	}] }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn array_args_with_template() {
  let code = transform(
    "<Comp>
      <template v-slot={[foo, bar]}>
        {foo}
      </template>
    </Com>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n2 = _createComponent(Comp, null, { $: [{
  		name: bar,
  		fn: (foo) => {
  			const _n0 = _createNodes(() => foo);
  			return _n0;
  		}
  	}] }, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn array_args_with_arg() {
  let code = transform(
    "<Comp v-slot:foo={[foo, bar]}>
      {foo}
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { foo: _extend((foo) => {
  		const _n0 = _createNodes(() => foo);
  		return _n0;
  	}, { _: 8 }) }, true);
  	return _n1;
  })();
  "#);
}
