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
  const _t0 = _template("<div> </div>", true);
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
  const _t0 = _template("<div></div>");
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
  const _t0 = _template("<div><p> </p> <p> </p></div>", true);
  (() => {
  	const _n3 = _t0();
  	const _n0 = _child(_n3);
  	const _n1 = _next(_n0, 1);
  	const _n2 = _next(_n1, 2);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => first);
  	_setNodes(_n1, () => second, " 456 ", () => foo);
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
  const _t0 = _template("<div><div>x</div><div><span> </span></div><div><span> </span></div><div><span> </span></div></div>", true);
  (() => {
  	const _n3 = _t0();
  	const _p0 = _next(_child(_n3, 1));
  	const _p1 = _next(_p0, 2);
  	const _p2 = _next(_p1, 3);
  	const _n2 = _child(_p2);
  	const _n1 = _child(_p1);
  	const _n0 = _child(_p0);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => ({ msg }));
  	const _x1 = _txt(_n1);
  	_setNodes(_x1, () => ({ msg }));
  	const _x2 = _txt(_n2);
  	_setNodes(_x2, () => ({ msg }));
  	return _n3;
  })();
  "#);
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
  const _t0 = _template("<div><div>x</div><div>x</div><div> </div></div>", true);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _nthChild(_n1, 2, 2);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => msg);
  	return _n1;
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
  const _t0 = _template("<div></div>");
  const _t1 = _template("<div><div></div><!><div></div></div>", true);
  (() => {
  	const _n4 = _t1();
  	const _n3 = _next(_child(_n4, 1));
  	_setInsertionState(_n4, _n3, true);
  	const _n0 = _createIf(() => 1, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, true);
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
  const _t0 = _template("<div> </div>", true);
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
fn next_child_and_nthchild_should_be_above_the_set_insertion_state() {
  let code = transform(
    "<div>
      <div />
      <Comp />
      <div />
      <div v-if={true} />
      <div>
        <button disabled={foo} />
      </div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, next as _next, nthChild as _nthChild, renderEffect as _renderEffect, setInsertionState as _setInsertionState, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("<div><div></div><!><!><div></div><div><button></button></div></div>", true);
  (() => {
  	const _n6 = _t1();
  	const _n5 = _next(_child(_n6, 1));
  	const _n7 = _nthChild(_n6, 3, 3);
  	const _p0 = _next(_n7, 4);
  	const _n4 = _child(_p0);
  	_setInsertionState(_n6, _n5);
  	const _n0 = _createComponent(Comp);
  	_setInsertionState(_n6, _n7, true);
  	const _n1 = _createIf(() => true, () => {
  		const _n3 = _t0();
  		return _n3;
  	}, null, true);
  	_renderEffect(() => _setProp(_n4, "disabled", foo));
  	return _n6;
  })();
  "#);
}
