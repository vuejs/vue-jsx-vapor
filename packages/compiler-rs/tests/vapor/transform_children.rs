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
  import { child as _child, template as _template } from "vue";
  const t0 = _template("<div> </div>", true);
  (() => {
  	const n0 = t0();
  	const x0 = _child(n0);
  	_setNodes(x0, () => foo, " ", () => bar);
  	return n0;
  })();
  "#);
}

#[test]
fn comments() {
  let code = transform("<>{/*foo*/}<div>{/*bar*/}</div></>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const n1 = t0();
  	return n1;
  })();
  "#);
}

#[test]
fn fragment() {
  let code = transform("<>{foo}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const n0 = _createNodes(() => foo);
  	return n0;
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
  import { child as _child, next as _next, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div><p> </p> <p> </p></div>", true);
  (() => {
  	const n3 = t0();
  	const n0 = _child(n3);
  	const n1 = _next(n0);
  	const n2 = _next(n1);
  	const x0 = _child(n0);
  	_setNodes(x0, () => first);
  	_setNodes(n1, "123 ", () => second, " 456 ", () => foo);
  	const x2 = _child(n2);
  	_setNodes(x2, () => forth);
  	_renderEffect(() => _setProp(n3, "id", id));
  	return n3;
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
  import { child as _child, next as _next, template as _template } from "vue";
  const t0 = _template("<div><div>x</div><div><span> </span></div><div><span> </span></div><div><span> </span></div></div>", true);
  (() => {
  	const n3 = t0();
  	const p0 = _next(_child(n3));
  	const p1 = _next(p0);
  	const p2 = _next(p1);
  	const n2 = _child(p2);
  	const n1 = _child(p1);
  	const n0 = _child(p0);
  	const x0 = _child(n0);
  	_setNodes(x0, () => ({ msg }));
  	const x1 = _child(n1);
  	_setNodes(x1, () => ({ msg }));
  	const x2 = _child(n2);
  	_setNodes(x2, () => ({ msg }));
  	return n3;
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
  import { child as _child, nthChild as _nthChild, template as _template } from "vue";
  const t0 = _template("<div><div>x</div><div>x</div><div> </div></div>", true);
  (() => {
  	const n1 = t0();
  	const n0 = _nthChild(n1, 2);
  	const x0 = _child(n0);
  	_setNodes(x0, () => msg);
  	return n1;
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
  const t0 = _template("<div></div>");
  const t1 = _template("<div><div></div><!><div></div></div>", true);
  (() => {
  	const n4 = t1();
  	const n3 = _next(_child(n4));
  	_setInsertionState(n4, n3);
  	const n0 = _createIf(() => 1, () => {
  		const n2 = t0();
  		return n2;
  	}, null, true);
  	return n4;
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
  import { child as _child, template as _template } from "vue";
  const t0 = _template("<div> </div>", true);
  (() => {
  	const n0 = t0();
  	const x0 = _child(n0);
  	_setNodes(x0, () => (() => {
  		const n0 = _createComponent(Comp, null, null, true);
  		return n0;
  	})());
  	return n0;
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
  const t0 = _template("<div></div>");
  const t1 = _template("<div><div></div><!><div></div><!><div><button></button></div></div>", true);
  (() => {
  	const n6 = t1();
  	const n5 = _next(_child(n6));
  	const n7 = _nthChild(n6, 3);
  	const p0 = _next(n7);
  	const n4 = _child(p0);
  	_setInsertionState(n6, n5);
  	const n0 = _createComponent(Comp);
  	_setInsertionState(n6, n7);
  	const n1 = _createIf(() => true, () => {
  		const n3 = t0();
  		return n3;
  	}, null, true);
  	_renderEffect(() => _setProp(n4, "disabled", foo));
  	return n6;
  })();
  "#);
}
