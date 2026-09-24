use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions, patch_flag::VaporVForFlags};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-for={item in items} key={item.id} onClick={() => remove(item)}>{item}</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, on as _on, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _t0();
  		_on(_n2, "click", () => remove(_for_item0.value));
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value);
  		return _n2;
  	}, (item) => item.id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn key_only_binding_pattern() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
    >
      { row.id + row.id }
    </tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<tr> ");
  (() => {
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.id + _for_item0.value.id);
  		return _n2;
  	}, (row) => row.id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn key_only_binding_pattern2() {
  let code = transform(
    r#"<tr
      v-for={row in rows}
      key={row.id}
      class={row.id === state.selected ? 'danger' : ''}
    ></tr>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSelector as _createSelector, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<tr>");
  (() => {
  	const _selector0 = _createSelector(() => state.selected);
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_selector0(_for_item0.value.id, () => {
  			_setClassName(_n2, _for_item0.value.id === state.selected ? 1 : 0, "danger");
  		});
  		return _n2;
  	}, (row) => row.id, 8);
  	_n0.onReset(_selector0.reset);
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern1() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      v-text={selected === row.id ? 'danger' : ''}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSelector as _createSelector, setText as _setText, template as _template, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  const _t0 = _template("<tr> ");
  (() => {
  	const _selector0 = _createSelector(() => selected);
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_selector0(_for_item0.value.id, () => {
  			_setText(_x2, _toDisplayString(selected === _for_item0.value.id ? "danger" : ""));
  		});
  		return _n2;
  	}, (row) => row.id, 8);
  	_n0.onReset(_selector0.reset);
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern2() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={selected === row.id ? 'danger' : ''}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSelector as _createSelector, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<tr>");
  (() => {
  	const _selector0 = _createSelector(() => selected);
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_selector0(_for_item0.value.id, () => {
  			_setClassName(_n2, selected === _for_item0.value.id ? 1 : 0, "danger");
  		});
  		return _n2;
  	}, (row) => row.id, 8);
  	_n0.onReset(_selector0.reset);
  	return _n0;
  })();
  "#);
}

// Should not be optimized because row.label is not from parent scope
#[test]
fn selector_pattern3() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={row.label === row.id ? 'danger' : ''}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<tr>");
  (() => {
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_renderEffect(() => _setClassName(_n2, _for_item0.value.label === _for_item0.value.id ? 1 : 0, "danger"));
  		return _n2;
  	}, (row) => row.id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern4() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={{ danger: row.id === selected }}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSelector as _createSelector, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<tr>");
  (() => {
  	const _selector0 = _createSelector(() => selected);
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_selector0(_for_item0.value.id, () => {
  			_setClassName(_n2, _for_item0.value.id === selected ? 1 : 0, "danger");
  		});
  		return _n2;
  	}, (row) => row.id, 8);
  	_n0.onReset(_selector0.reset);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_selector_pattern() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={{ danger: row.id === selected ? danger : null }}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<tr>");
  (() => {
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_renderEffect(() => _setClassName(_n2, (_for_item0.value.id === selected ? danger : null) ? 1 : 0, "danger"));
  		return _n2;
  	}, (row) => row.id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn multiple_selector_patterns_on_one_v_for() {
  let code = transform(
    r#"<tr
        v-for={row in rows}
        key={row.id}
        class={selected === row.id ? 'a' : ''}
        title={active === row.id ? 'b' : ''}
      ></tr>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSelector as _createSelector, setClassName as _setClassName, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<tr>");
  (() => {
  	const _selector0_0 = _createSelector(() => selected);
  	const _selector0_1 = _createSelector(() => active);
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_selector0_0(_for_item0.value.id, () => {
  			_setClassName(_n2, selected === _for_item0.value.id ? 1 : 0, "a");
  		});
  		_selector0_1(_for_item0.value.id, () => {
  			_setProp(_n2, "title", active === _for_item0.value.id ? "b" : "");
  		});
  		return _n2;
  	}, (row) => row.id, 8);
  	_n0.onReset(_selector0_0.reset);
  	_n0.onReset(_selector0_1.reset);
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern_requires_the_key_itself_on_one_side() {
  let compile_class = |key: &str, cond: &str| {
    transform(
      &format!(
        r#"<li v-for={{(item, i) in items}} key={{{key}}} class={{{{ active: {cond} }}}}></li>"#
      ),
      None,
    )
    .code
  };

  // the selector only re-runs rows whose key equals the old/new value
  assert!(!compile_class("i", "i + 1 === page").contains("_createSelector"));
  assert!(!compile_class("i", "page === i * 2").contains("_createSelector"));
  assert!(!compile_class("item.id", "item.id + 1 === page").contains("_createSelector"));
  assert!(
    compile_class("i", "i === page - 1")
      .contains("const _selector0 = _createSelector(() => page - 1);")
  );
}

#[test]
fn multi_effect() {
  let code = transform(
    "<div v-for={(item, index) in items} item={item} index={index} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		_renderEffect(() => {
  			_setProp(_n2, "item", _for_item0.value);
  			_setProp(_n2, "index", _for_key0.value);
  		});
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn multi_class_name_helper_with_repeated_v_for_value() {
  let code = transform(
    r#"<div v-for={todo in todos} key={todo.id} class={{ completed: todo.completed, editing: todo === editedTodo }} />"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, renderEffect as _renderEffect, setClassName as _setClassName, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createFor(() => todos, (_for_item0) => {
  		const _n2 = _t0();
  		_renderEffect(() => _setClassName(_n2, (_for_item0.value.completed ? 1 : 0) | (_for_item0.value === editedTodo ? 2 : 0), [" completed", " editing"]));
  		return _n2;
  	}, (todo) => todo.id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn nested_v_for() {
  let code = transform(
    "<div v-for={i in list}><span v-for={j in i}>{ j+i }</span></div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  const _t1 = _template("<div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n5 = _t1();
  		_setInsertionState(_n5);
  		const _n2 = _createFor(() => _for_item0.value, (_for_item1) => {
  			const _n4 = _t0();
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, () => _for_item1.value + _for_item0.value);
  			return _n4;
  		}, void 0, 9);
  		return _n5;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn object_value_key_and_index() {
  let code = transform(
    "<span v-for={(value, key, index) in items} key={id}>{ id }{ value }{ index }</span>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0, _for_index0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => id, () => _for_item0.value, () => _for_index0.value);
  		return _n2;
  	}, (value, key, index) => id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn object_de_structured_value() {
  let code = transform(
    "<span v-for={({ id, value }) in items} key={id}>{ id }{ value }</span>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.id, () => _for_item0.value.value);
  		return _n2;
  	}, ({ id, value }) => id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn object_de_structured_value_with_rest() {
  let code = transform(
    "<div v-for={(  { id, ...other }, index) in list} key={id}>{ id + other + index }</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, getRestElement as _getRestElement, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.id + _getRestElement(_for_item0.value, ["id"]) + _for_key0.value);
  		return _n2;
  	}, ({ id, ...other }, index) => id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn array_de_structured_value() {
  let code = transform(
    "<div v-for={([id, other], index) in list} key={id}>{ id + other + index }</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value[0] + _for_item0.value[1] + _for_key0.value);
  		return _n2;
  	}, ([id, other], index) => id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn array_de_structured_value_with_rest() {
  let code = transform("<div v-for={([id, [foo], {bar}, ...other], index) in list} key={id}>{ id + other + index + foo + bar }</div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value[0] + _for_item0.value.slice(3) + _for_key0.value + _for_item0.value[1][0] + _for_item0.value[2].bar);
  		return _n2;
  	}, ([id, [foo], {bar}, ...other], index) => id, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn aliases_with_complex_expressions() {
  let code = transform(
    "<div v-for={({ foo, baz: [qux] }) in list}>
      { foo + baz + qux }
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.foo + baz + _for_item0.value.baz[0]);
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn fast_remove_flag() {
  let code = transform(
    "<div>
      <span v-for={j in i}>{ j+i }</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  const _t1 = _template("<div>", 1);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3);
  	const _n0 = _createFor(() => i, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value + i);
  		return _n2;
  	}, void 0, 9);
  	return _n3;
  })();
  "#);
}

#[test]
fn on_component() {
  let code = transform("<Comp v-for={item in list}>{item}</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent, normalizeVaporSlots as _normalizeVaporSlots } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor } from "vue";
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createComponent(Comp, null, { $: [() => _normalizeVaporSlots(_for_item0.value)] });
  		return _n2;
  	}, void 0, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_on_slot_outlet_marks_fragment_block() {
  let code = transform(
    "<slot v-for={item in list} name={item.name} key={item.id} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createSlot(() => _for_item0.value.name);
  		return _n2;
  	}, (item) => item.id, 16);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_single_node_flag_is_not_set_for_fragment_item_blocks() {
  let code = transform(
    "<template v-for={item in list}><div>{ item }</div><span>{ item }</span></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  const _t1 = _template("<span> ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _t0();
  		const _n3 = _t1();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value);
  		const _x3 = _txt(_n3);
  		_setNodes(_x3, () => _for_item0.value);
  		return [_n2, _n3];
  	}, void 0, 64);
  	return _n0;
  })();
  "#);
}

#[test]
fn on_template_with_single_component_child() {
  let code = transform(
    "<template v-for={item in list}><Comp>{item}</Comp></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent, normalizeVaporSlots as _normalizeVaporSlots } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor } from "vue";
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createComponent(Comp, null, { $: [() => _normalizeVaporSlots(_for_item0.value)] });
  		return _n2;
  	}, void 0, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_on_template_with_element_and_component_v_if_branches() {
  let code = transform(
    "<template v-for={item in items}>
      <div v-if={item.id===1}>hi</div>
      <Comp v-else></Comp>
    </template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>hi");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createIf(() => _for_item0.value.id === 1, () => {
  			const _n4 = _t0();
  			return _n4;
  		}, () => {
  			const _n6 = _createComponent(Comp);
  			return _n6;
  		}, 266);
  		return _n2;
  	}, void 0, 80);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_on_template_with_nested_v_for_child_marks_fragment_block() {
  let code = transform(
    "<template v-for={row in rows}><div v-for={item in row}>{ item }</div></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _createFor(() => _for_item0.value, (_for_item1) => {
  			const _n4 = _t0();
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, () => _for_item1.value);
  			return _n4;
  		}, void 0, 8);
  		return _n2;
  	}, void 0, 80);
  	return _n0;
  })();
  "#);
}

#[test]
// mirrors compiler-ssr for a vapor component: only Transition still renders
// its children without nested fragment markers
fn v_for_on_template_under_a_transition_group_has_wrapped_rows() {
  let rows = "<template v-for={item in items}><li>{item}</li><li>b</li></template>";
  let slot_root = VaporVForFlags::SlotRoot as i32;
  let wrapped_rows = slot_root | VaporVForFlags::WrappedRows as i32;

  let code = transform(
    &format!("<TransitionGroup tag=\"ul\">{rows}</TransitionGroup>"),
    None,
  )
  .code;
  assert!(code.contains(&format!("void 0, {wrapped_rows}")), "{code}");

  let code = transform(&format!("<Transition>{rows}</Transition>"), None).code;
  assert!(code.contains(&format!("void 0, {slot_root}")), "{code}");
  assert!(!code.contains(&format!("void 0, {wrapped_rows}")), "{code}");
}

#[test]
fn v_for_on_template_with_keyed_child_marks_fragment_block() {
  let code = transform(
    "<template v-for={item in items}><div key={item.id}>{ item.text }</div></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createKeyedFragment as _createKeyedFragment, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createKeyedFragment(() => _for_item0.value.id, () => {
  			const _n4 = _t0();
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, () => _for_item0.value.text);
  			return _n4;
  		});
  		return _n2;
  	}, void 0, 16);
  	return _n0;
  })();
  "#);
}

#[test]
fn identifiers() {
  let code = transform(
    "let item = ''
    ;<div v-for={(item, index) in items} id={index}>
    { ((item) => {
      let index = 1
      return [item, index]
    })(item) }
    { (() => {
      switch (item) {
        case index: {
          let item = ''
          return `${[item, index]}`;
        }
      }
    })() }
  </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  let item = "";
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => ((item) => {
  			let index = 1;
  			return [item, index];
  		})(_for_item0.value), () => (() => {
  			switch (_for_item0.value) {
  				case _for_key0.value: {
  					let item = "";
  					return `${[item, _for_key0.value]}`;
  				}
  			}
  		})());
  		_renderEffect(() => _setProp(_n2, "id", _for_key0.value));
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn expression_object() {
  let code = transform(
    "<div v-for={(item, index) in Array.from({ length: count.value }).map((_, id) => ({ id }))} id={index}>
      {item}
    </div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => Array.from({ length: count.value }).map((_, id) => ({ id })), (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value);
  		_renderEffect(() => _setProp(_n2, "id", _for_key0.value));
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_for_with_slotlet() {
  let code = transform(
    r#"<template v-for={item in items}><slot/></template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createSlot();
  		return _n2;
  	}, void 0, 16);
  	return _n0;
  })();
  "#)
}

#[test]
fn v_for_on_slotlet() {
  let code = transform(r#"<slot v-for={item in items}></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createSlot("default");
  		return _n2;
  	}, void 0, 16);
  	return _n0;
  })();
  "#)
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-for />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VForNoExpression));
}

#[test]
fn should_raise_error_if_malformed_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-for={foo} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VForMalformedExpression));
}

// aliases are parsed as expressions here, so a default value or a destructuring
// pattern is part of the alias source and the bound name has to come off the ast.
// Upstream also covers ts type annotations, which this parser cannot express.
#[test]
fn v_for_alias_with_default_value() {
  let code = transform(
    "<div v-for={(item, index = 0) in items}>{{ index }}</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, getDefaultValue as _getDefaultValue, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => ({ index: _getDefaultValue(_for_key0.value, () => 0) }));
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_alias_with_default_value_on_key_and_index() {
  let code = transform(
    "<div v-for={(item = 'x', key, index = 99) in items}>{{ item }}{{ index }}</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, getDefaultValue as _getDefaultValue, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0, _for_index0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => ({ item: _getDefaultValue(_for_item0.value, () => "x") }), () => ({ index: _getDefaultValue(_for_index0.value, () => 99) }));
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_alias_defaults_are_kept_in_the_key_function_params() {
  let code = transform(
    "<div v-for={(item, key, index = 99) in items} key={index}>{{ item }}</div>",
    None,
  )
  .code;
  assert!(code.contains("(item, key, index = 99) => index"), "{code}");
}

#[test]
fn v_for_destructured_key_alias() {
  let code = transform(
    "<div v-for={(item, { k }) in items}>{{ item }}{{ k }}</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => ({ item: _for_item0.value }), () => ({ k: _for_key0.value.k }));
  		return _n2;
  	}, void 0, 8);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_for_array_destructured_key_alias() {
  let code = transform(
    "<div v-for={(item, [c, d]) in items}>{{ item }}{{ c }}{{ d }}</div>",
    None,
  )
  .code;
  assert!(code.contains("() => ({ c: _for_key0.value[0] })"), "{code}");
  assert!(code.contains("() => ({ d: _for_key0.value[1] })"), "{code}");
}

#[test]
fn v_for_destructured_key_alias_in_the_key_function_params() {
  let code = transform(
    "<div v-for={(item, { k }) in items} key={item}>{{ item }}{{ k }}</div>",
    None,
  )
  .code;
  assert!(code.contains("}, (item, { k }) => item,"), "{code}");
}

#[test]
fn v_for_object_destructured_key_alias_with_default() {
  let code = transform(
    "<div v-for={(item, { ku: kk = 1 }) in items}>{{ kk }}</div>",
    None,
  )
  .code;
  assert!(
    code.contains("() => ({ kk: _getDefaultValue(_for_key0.value.ku, () => 1) })"),
    "{code}"
  );
}

#[test]
fn v_for_object_destructured_value_alias_with_default() {
  let code = transform(
    "<div v-for={({ foo: bar = 1 }) in items}>{{ bar }}</div>",
    None,
  )
  .code;
  assert!(
    code.contains("() => ({ bar: _getDefaultValue(_for_item0.value.foo, () => 1) })"),
    "{code}"
  );
}

#[test]
fn v_for_object_destructured_index_alias_with_default() {
  let code = transform(
    "<div v-for={(item, key, { idx: i = 1 }) in items}>{{ i }}</div>",
    None,
  )
  .code;
  assert!(
    code.contains("() => ({ i: _getDefaultValue(_for_index0.value.idx, () => 1) })"),
    "{code}"
  );
}

// a shorthand default (`{ foo = 1 }`) is not expressible: the alias is parsed as
// an expression, and oxc rejects a shorthand assignment in an object literal.
#[test]
fn v_for_object_destructured_alias_shorthand_is_not_supported() {
  let code = transform("<div v-for={({ foo = 1 }) in items}>{{ foo }}</div>", None).code;
  assert!(!code.contains("_getDefaultValue"), "{code}");
}

// upstream: `processes key callback defaults in %s`. The ts-annotation variant
// and the `_ctx`/inline variants are not expressible here.
#[test]
fn key_callback_defaults_are_kept_in_the_key_function_params() {
  let code = transform(
    "<div v-for={(item = fallback, i) in items} key={i} />",
    None,
  )
  .code;
  assert!(code.contains("}, (item = fallback, i) => i,"), "{code}");
}

// upstream: `preserves callback locals while resolving defaults from an outer loop`
#[test]
fn preserves_callback_locals_while_resolving_defaults_from_an_outer_loop() {
  let code = transform(
    "<div v-for={(row, item) in rows}><span v-for={(item = row.fallback, key, index = item.id) in row.items} key={index} /></div>",
    None,
  )
  .code;
  assert!(
    code.contains("(item = _for_item0.value.fallback, key, index = item.id) => index"),
    "{code}"
  );
}

// the aliases are resolved through the ast, so a callback parameter wins over an
// outer loop alias of the same name instead of resolving to it.
#[test]
fn callback_params_shadow_an_outer_alias_of_the_same_name() {
  let code = transform(
    "<div v-for={(item) in a}><span v-for={(item) in b} key={item}>{item}</span></div>",
    None,
  )
  .code;
  assert!(code.contains("}, (item) => item,"), "{code}");
  assert!(
    code.contains("_setNodes(_x4, () => _for_item1.value)"),
    "{code}"
  );
}

// the default of an alias must not leak into the body: the bound name has to
// come off the value alias, not the default expression referencing it.
#[test]
fn body_of_an_alias_with_a_default_reads_the_bound_name() {
  let code = transform(
    "<div v-for={(row, item) in rows}><span v-for={(item = row.fallback, key, index = item.id) in row.items} key={index}>{item.name}</span></div>",
    None,
  )
  .code;
  assert!(
    code.contains("_setNodes(_x4, () => _getDefaultValue(_for_item1.value, () => _for_item0.value.fallback).name)"),
    "{code}"
  );
}

// a default referring to the alias it is bound to must not be resolved again.
#[test]
fn self_referencing_callback_default_does_not_recurse() {
  let code = transform(
    "<div v-for={(item = item.fallback) in list} key={item}>{item}</div>",
    None,
  )
  .code;
  assert!(code.contains("(item = item.fallback) => item,"), "{code}");
  assert!(
    code.contains("_setNodes(_x2, () => _getDefaultValue(_for_item0.value, () => item.fallback))"),
    "{code}"
  );
}
