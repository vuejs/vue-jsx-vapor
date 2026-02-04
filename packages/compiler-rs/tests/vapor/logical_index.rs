use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn child_nth_child_next_with_logical_index() {
  let code = transform(
    r#"
    <div>
      <div />
      <Comp />
      <div />
      <div v-if="true" />
    </div>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><div></div><!><div></div>", true);
  (() => {
  	const _n5 = _t1();
  	const _n4 = _next(_child(_n5), 1);
  	_setInsertionState(_n5, _n4, 1);
  	const _n0 = _createComponent(Comp);
  	_setInsertionState(_n5, null, 3, true);
  	const _n1 = _createIf(() => "true", () => {
  		const _n3 = _t0();
  		return _n3;
  	});
  	return _n5;
  })();
  "#);
}

#[test]
fn nth_child_with_logical_index() {
  let code = transform(
    r#"<div>
      <div />
      <Comp />
      <div />
      <div v-if={true} />
      <div>
        <button disabled={foo} />
      </div>
    </div>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, next as _next, nthChild as _nthChild, renderEffect as _renderEffect, setInsertionState as _setInsertionState, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><div></div><!><div></div><!><div><button>", true);
  (() => {
  	const _n6 = _t1();
  	const _n5 = _next(_child(_n6), 1);
  	const _n7 = _nthChild(_n6, 3, 3);
  	const _p0 = _next(_n7, 4);
  	const _n4 = _child(_p0);
  	_setInsertionState(_n6, _n5, 1);
  	const _n0 = _createComponent(Comp);
  	_setInsertionState(_n6, _n7, 3, true);
  	const _n1 = _createIf(() => true, () => {
  		const _n3 = _t0();
  		return _n3;
  	}, null, true);
  	_renderEffect(() => _setProp(_n4, "disabled", foo));
  	return _n6;
  })();
  "#,
  );
}

#[test]
fn child_with_logical_index_when_prepend_exists_and_insert_anchor_needed() {
  let code = transform(
    "<div>
      <Comp1 />
      <div />
      <Comp2 />
      <span />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><div></div><!><span>", true);
  (() => {
  	const _n2 = _t0();
  	const _n3 = _next(_child(_n2), 2);
  	_setInsertionState(_n2, 0, 0);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n2, _n3, 2, true);
  	const _n1 = _createComponent(Comp2);
  	return _n2;
  })();
  "#);
}

#[test]
fn multiple_prepends_affect_logical_index() {
  let code = transform(
    "<div>
      <Comp1 />
      <Comp2 />
      <div />
      <Comp3 />
      <span />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><div></div><!><span>", true);
  (() => {
  	const _n3 = _t0();
  	const _n4 = _next(_child(_n3), 3);
  	_setInsertionState(_n3, 0, 0);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n3, 0, 1);
  	const _n1 = _createComponent(Comp2);
  	_setInsertionState(_n3, _n4, 3, true);
  	const _n2 = _createComponent(Comp3);
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_single_component_prepend() {
  let code = transform(
    "<div>
      <Comp />
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A", true);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1, 0, 0, true);
  	const _n0 = _createComponent(Comp);
  	return _n1;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_multiple_consecutive_prepend() {
  let code = transform(
    "<div>
      <Comp1 />
      <Comp2 />
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A", true);
  (() => {
  	const _n2 = _t0();
  	_setInsertionState(_n2, 0, 0);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n2, 0, 1, true);
  	const _n1 = _createComponent(Comp2);
  	return _n2;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_single_component_insert_in_middle() {
  let code = transform(
    "<div>
      <span>A</span>
      <Comp />
      <p>B</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span><!><p>B", true);
  (() => {
  	const _n2 = _t0();
  	const _n1 = _next(_child(_n2), 1);
  	_setInsertionState(_n2, _n1, 1, true);
  	const _n0 = _createComponent(Comp);
  	return _n2;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_multiple_consecutive_insert_in_middle() {
  let code = transform(
    "<div>
      <span>A</span>
      <Comp1 />
      <Comp2 />
      <p>B</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span><!><p>B", true);
  (() => {
  	const _n3 = _t0();
  	const _n2 = _next(_child(_n3), 1);
  	_setInsertionState(_n3, _n2, 1);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n3, _n2, 2, true);
  	const _n1 = _createComponent(Comp2);
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_single_component_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <Comp />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span>", true);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1, null, 1, true);
  	const _n0 = _createComponent(Comp);
  	return _n1;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_multiple_consecutive_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <Comp1 />
      <Comp2 />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span>", true);
  (() => {
  	const _n2 = _t0();
  	_setInsertionState(_n2, null, 1);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n2, null, 2, true);
  	const _n1 = _createComponent(Comp2);
  	return _n2;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_only_component_append_with_logical_index_0() {
  let code = transform(
    "<div>
      <Comp />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1, null, 0, true);
  	const _n0 = _createComponent(Comp);
  	return _n1;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_mixed_scenarios_prepend_and_append() {
  let code = transform(
    "<div>
      <Comp1 />
      <span>A</span>
      <Comp2 />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span>", true);
  (() => {
  	const _n2 = _t0();
  	_setInsertionState(_n2, 0, 0);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n2, null, 2, true);
  	const _n1 = _createComponent(Comp2);
  	return _n2;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_mixed_scenarios_prepend_and_insert_and_append() {
  let code = transform(
    "<div>
      <Comp1 />
      <span>A</span>
      <Comp2 />
      <p>B</p>
      <Comp3 />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span><!><p>B</p>", true);
  (() => {
  	const _n3 = _t0();
  	const _n4 = _next(_child(_n3), 2);
  	_setInsertionState(_n3, 0, 0);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n3, _n4, 2);
  	const _n1 = _createComponent(Comp2);
  	_setInsertionState(_n3, null, 4, true);
  	const _n2 = _createComponent(Comp3);
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_prepend() {
  let code = transform(
    "<div>
      <div v-if={show} />
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, 0, 0, true);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_insert() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-if={show} />
      <p>B</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, createIf as _createIf, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A</span><!><p>B", true);
  (() => {
  	const _n4 = _t1();
  	const _n3 = _next(_child(_n4), 1);
  	_setInsertionState(_n4, _n3, 1, true);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n4;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-if={show} />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A</span>", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, null, 1, true);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_for_prepend() {
  let code = transform(
    "<div>
      <div v-for={i in list} key={i} />
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, 0, 0, true);
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _t0();
  		return _n2;
  	}, (i) => i);
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_for_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-for={i in list} key={i} />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A</span>", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, null, 1, true);
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _t0();
  		return _n2;
  	}, (i) => i, 1);
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_key_prepend() {
  let code = transform(
    "<div>
      <div key={i} />
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, 0, 0, true);
  	const _n0 = _createKeyedFragment(() => i, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_key_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <div key={i} />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><span>A</span>", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, null, 1, true);
  	const _n0 = _createKeyedFragment(() => i, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n3;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_slot_prepend() {
  let code = transform(
    "<div>
      <slot />
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A", true);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1, 0, 0, true);
  	const _n0 = _createSlot("default");
  	return _n1;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_slot_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <slot />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><span>A</span>", true);
  (() => {
  	const _n1 = _t0();
  	_setInsertionState(_n1, null, 1, true);
  	const _n0 = _createSlot("default");
  	return _n1;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_with_v_else_should_share_same_logical_index() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-if={show}>if</div>
      <div v-else>else</div>
      <p>B</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, createIf as _createIf, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else");
  const _t2 = _template("<div><span>A</span><!><p>B", true);
  (() => {
  	const _n6 = _t2();
  	const _n5 = _next(_child(_n6), 1);
  	_setInsertionState(_n6, _n5, 1, true);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, false, 0);
  	return _n6;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_with_v_else_if_and_v_else_should_share_same_logical_index() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-if={a}>if</div>
      <div v-else-if={b}>else-if</div>
      <div v-else>else</div>
      <p>B</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, createIf as _createIf, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else-if");
  const _t2 = _template("<div>else");
  const _t3 = _template("<div><span>A</span><!><p>B", true);
  (() => {
  	const _n8 = _t3();
  	const _n7 = _next(_child(_n8), 1);
  	_setInsertionState(_n8, _n7, 1, true);
  	const _n0 = _createIf(() => a, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => _createIf(() => b, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, () => {
  		const _n6 = _t2();
  		return _n6;
  	}, false, 1), false, 0);
  	return _n8;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_and_v_else_prepend() {
  let code = transform(
    "<div>
      <div v-if={show}>if</div>
      <div v-else>else</div>
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else");
  const _t2 = _template("<div><span>A", true);
  (() => {
  	const _n5 = _t2();
  	_setInsertionState(_n5, 0, 0, true);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, false, 0);
  	return _n5;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_and_v_else_append() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-if={show}>if</div>
      <div v-else>else</div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else");
  const _t2 = _template("<div><span>A</span>", true);
  (() => {
  	const _n5 = _t2();
  	_setInsertionState(_n5, null, 1, true);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, false, 0);
  	return _n5;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_v_if_and_v_else_followed_by_component() {
  let code = transform(
    "<div>
      <span>A</span>
      <div v-if={show}>if</div>
      <div v-else>else</div>
      <Comp />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else");
  const _t2 = _template("<div><span>A</span>", true);
  (() => {
  	const _n6 = _t2();
  	_setInsertionState(_n6, null, 1);
  	const _n0 = _createIf(() => show, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, false, 0);
  	_setInsertionState(_n6, null, 2, true);
  	const _n5 = _createComponent(Comp);
  	return _n6;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_component_followed_by_v_if_v_else() {
  let code = transform(
    "<div>
      <Comp />
      <div v-if={show}>if</div>
      <div v-else>else</div>
      <span>A</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else");
  const _t2 = _template("<div><span>A", true);
  (() => {
  	const _n6 = _t2();
  	_setInsertionState(_n6, 0, 0);
  	const _n0 = _createComponent(Comp);
  	_setInsertionState(_n6, 0, 1, true);
  	const _n1 = _createIf(() => show, () => {
  		const _n3 = _t0();
  		return _n3;
  	}, () => {
  		const _n5 = _t1();
  		return _n5;
  	}, false, 0);
  	return _n6;
  })();
  "#);
}

#[test]
fn set_insertion_state_scenarios_component_and_v_if_v_else_and_component() {
  let code = transform(
    "<div>
      <Comp1 />
      <div v-if={show}>if</div>
      <div v-else>else</div>
      <Comp2 />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>if");
  const _t1 = _template("<div>else");
  const _t2 = _template("<div>", true);
  (() => {
  	const _n7 = _t2();
  	_setInsertionState(_n7, null, 0);
  	const _n0 = _createComponent(Comp1);
  	_setInsertionState(_n7, null, 1);
  	const _n1 = _createIf(() => show, () => {
  		const _n3 = _t0();
  		return _n3;
  	}, () => {
  		const _n5 = _t1();
  		return _n5;
  	}, false, 0);
  	_setInsertionState(_n7, null, 2, true);
  	const _n6 = _createComponent(Comp2);
  	return _n7;
  })();
  "#);
}
