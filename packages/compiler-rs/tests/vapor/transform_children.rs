use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div>
      {foo} {bar}
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo, " ", () => bar);
  	return _n0;
  })();
  "#);
}

#[test]
fn comments() {
  let code = transform("<>{/*foo*/}<div>{/*bar*/}</div></>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn fragment() {
  let code = transform("<>{foo}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes(() => foo);
  	return _n0;
  })();
  "#);
}

#[test]
fn children_sibling_references() {
  let code = transform(
    "<div id={id}>
      <p>{ first }</p>
      123 { second } 456 {foo}
      <p>{ forth }</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, renderEffect as _renderEffect, setProp as _setProp, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><p> </p> <p> ", 1);
  (() => {
  	const _n3 = _t0();
  	const _n0 = _child(_n3);
  	const _n1 = _next(_n0, 1);
  	const _n2 = _next(_n1, 2);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => first);
  	_setNodes(_n1, "123 ", () => second, " 456 ", () => foo);
  	const _x2 = _txt(_n2);
  	_setNodes(_x2, () => forth);
  	_renderEffect(() => _setProp(_n3, "id", id));
  	return _n3;
  })();
  "#);
}

#[test]
fn efficient_traversal() {
  let code = transform(
    "<div>
      <div>x</div>
      <div><span>{{ msg }}</span></div>
      <div><span>{{ msg }}</span></div>
      <div><span>{{ msg }}</span></div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><div>x</div><div><span> </div><div><span> </div><div><span> ", 1);
  (() => {
  	const _n3 = _t0();
  	let _p0 = _next(_child(_n3), 1);
  	const _n0 = _child(_p0);
  	const _n1 = _child(_p0 = _next(_p0, 2));
  	const _n2 = _child(_p0 = _next(_p0, 3));
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => ({ msg }));
  	const _x1 = _txt(_n1);
  	_setNodes(_x1, () => ({ msg }));
  	const _x2 = _txt(_n2);
  	_setNodes(_x2, () => ({ msg }));
  	return _n3;
  })();
  "#);

  assert!(code.contains("let _p0 = _next(_child(_n3), 1)"));
  assert!(code.contains("const _n0 = _child(_p0)"));
  assert!(code.contains("const _n1 = _child(_p0 = _next(_p0, 2))"));
  assert!(code.contains("const _n2 = _child(_p0 = _next(_p0, 3))"));
  assert!(!code.contains("const _p1 = "));
  assert!(!code.contains("let _p1 = "));
}

#[test]
fn efficient_find() {
  let code = transform(
    "<div>
      <div>x</div>
      <div>x</div>
      <div>{ msg }</div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { nthChild as _nthChild, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><div>x</div><div>x</div><div> ", 1);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _nthChild(_n1, 2);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => msg);
  	return _n1;
  })();
  "#);
}

#[test]
fn inline_placeholder_when_branching_access_paths_share_one_parent_access() {
  let code = transform(
    "<div>
      <div>
        <section><span>{{ first }}</span></section>
        <section><span>{{ second }}</span></section>
      </div>
    </div>",
    None,
  )
  .code;

  assert!(code.contains("let _p0 = _child(_child("));
  assert!(code.contains("const _n0 = _child(_p0);"));
  assert!(code.contains("const _n1 = _child(_p0 = _next(_p0, 1));"));
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><div><section><span> </section><section><span> ", 1);
  (() => {
  	const _n2 = _t0();
  	let _p0 = _child(_child(_n2));
  	const _n0 = _child(_p0);
  	const _n1 = _child(_p0 = _next(_p0, 1));
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => ({ first }));
  	const _x1 = _txt(_n1);
  	_setNodes(_x1, () => ({ second }));
  	return _n2;
  })();
  "#);
}

#[test]
fn reuse_cursor_assignment_for_non_adjacent_following_access_path() {
  let code = transform(
    "<div>
      <div><span>{{ first }}</span></div>
      <i></i>
      <div><span>{{ second }}</span></div>
    </div>",
    None,
  )
  .code;

  assert!(code.contains("let _p0 = _child("));
  assert!(code.contains("const _n0 = _child(_p0);"));
  assert!(code.contains("const _n1 = _child(_p0 = _nthChild("));
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, nthChild as _nthChild, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><div><span> </div><i></i><div><span> ", 1);
  (() => {
  	const _n2 = _t0();
  	let _p0 = _child(_n2);
  	const _n0 = _child(_p0);
  	const _n1 = _child(_p0 = _nthChild(_n2, 2));
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => ({ first }));
  	const _x1 = _txt(_n1);
  	_setNodes(_x1, () => ({ second }));
  	return _n2;
  })();
  "#);
}

#[test]
fn materialize_placeholder_when_inline_would_duplicate_parent_access() {
  let code = transform(
    "<div>
      <section>
        <div><span>{{ first }}</span></div>
        <i></i>
        <div><span>{{ second }}</span></div>
      </section>
    </div>",
    None,
  )
  .code;

  assert!(code.contains("let _p0 = _child("));
  assert!(code.contains("let _p1 = _child(_p0);"));
  assert!(code.contains("const _n0 = _child(_p1);"));
  assert!(code.contains("const _n1 = _child(_p1 = _nthChild(_p0, 2));"));
  assert!(!code.contains("_nthChild(_child("));
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, nthChild as _nthChild, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><section><div><span> </div><i></i><div><span> ", 1);
  (() => {
  	const _n2 = _t0();
  	let _p0 = _child(_n2);
  	let _p1 = _child(_p0);
  	const _n0 = _child(_p1);
  	const _n1 = _child(_p1 = _nthChild(_p0, 2));
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => ({ first }));
  	const _x1 = _txt(_n1);
  	_setNodes(_x1, () => ({ second }));
  	return _n2;
  })();
  "#);
}

#[test]
fn keep_nested_operation_parent_as_node_variable_before_sibling_lookup() {
  let code = transform(
    "<div>
      <section><Comp /></section>
      <section><span>{{ msg }}</span></section>
    </div>",
    None,
  )
  .code;

  assert!(code.contains("const _n1 = _child("));
  assert!(code.contains("const _n2 = _child(_next(_n1, 1));"));
  assert!(code.contains("_setInsertionState(_n1, null, 0);"));
  assert!(!code.contains("_p0 = _next"));
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, next as _next, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div><section></section><section><span> ", 1);
  (() => {
  	const _n3 = _t0();
  	const _n1 = _child(_n3);
  	const _n2 = _child(_next(_n1, 1));
  	_setInsertionState(_n1, null, 0);
  	const _n0 = _createComponent(Comp);
  	const _x2 = _txt(_n2);
  	_setNodes(_x2, () => ({ msg }));
  	return _n3;
  })();
  "#);
}

#[test]
fn anchor_insertion_in_middle() {
  let code = transform(
    "<div>
      <div></div>
      <div v-if={1}></div>
      <div></div>
    </div>",
    None,
  )
  .code;
  // ensure the insertion anchor is generated before the insertion statement
  assert_snapshot!(code, @r#"
  import { child as _child, createIf as _createIf, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<div><div></div><!><div>", 1);
  (() => {
  	const _n4 = _t1();
  	const _n3 = _next(_child(_n4), 1);
  	_setInsertionState(_n4, _n3, 1);
  	const _n0 = _createIf(() => 1, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 17);
  	return _n4;
  })();
  "#);
}

#[test]
fn jsx_component_in_jsx_expression_container() {
  let code = transform(
    "<div>
      {<Comp />}
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => (() => {
  		const _n0 = _createComponent(Comp, null, null, true);
  		return _n0;
  	})());
  	return _n0;
  })();
  "#);
}

#[test]
fn flushes_previous_effects_before_creating_child_component() {
  let code = transform(
    "<>
      <div>parent: { useId() }</div>
      <Child />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, "parent: ", () => useId());
  	const _n1 = _createComponent(Child);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn flushes_parent_props_before_creating_child_component() {
  let code = transform("<div id={useId()}><Child /></div>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { renderEffect as _renderEffect, setInsertionState as _setInsertionState, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n1 = _t0();
  	_renderEffect(() => _setProp(_n1, "id", useId()));
  	_setInsertionState(_n1, null, 0);
  	const _n0 = _createComponent(Child);
  	return _n1;
  })();
  "#);
}

#[test]
fn does_not_flush_later_v_for_effects_before_child_component() {
  let code = transform(
    "<div v-for={row in rows} key={row.id}>
      <span v-text={selected === row.id ? 'danger' : ''}></span>
      <Child />
      <span>{ useId() }</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, createFor as _createFor, next as _next, setInsertionState as _setInsertionState, setText as _setText, template as _template, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  const _t0 = _template("<div><span> </span><!><span> ");
  (() => {
  	const _selector0 = createSelector(() => selected);
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n6 = _t0();
  		const _n2 = _child(_n6);
  		const _n5 = _next(_n2, 1);
  		const _n4 = _next(_n5, 2);
  		const _x2 = _txt(_n2);
  		_setInsertionState(_n6, _n5, 1);
  		const _n3 = _createComponent(Child);
  		const _x4 = _txt(_n4);
  		_setNodes(_x4, () => useId());
  		_selector0(_for_item0.value.id, () => {
  			_setText(_x2, _toDisplayString(selected === _for_item0.value.id ? "danger" : ""));
  		});
  		return _n6;
  	}, (row) => row.id, 8);
  	n0.onReset(_selector0.reset);
  	return _n0;
  })();
  "#);
}
