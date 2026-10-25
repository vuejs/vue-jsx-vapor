use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<VaporTransition>
      <h1 v-show={show}>foo</h1>
    </VaporTransition>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { applyVShow as _applyVShow, template as _template } from "vue";
  const _t0 = _template("<h1>foo", 1);
  (() => {
  	const _n1 = _createComponent(VaporTransition, { persisted: true }, () => {
  		const _n0 = _t0();
  		_applyVShow(_n0, () => show);
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

// vuejs/core#15274
#[test]
fn propagates_component_root_to_transition_child_with_dynamic_class() {
  let code = transform(
    r#"<VaporTransition>
      <div class={["box", data.value.internalClass]}>child</div>
    </VaporTransition>"#,
    None,
  )
  .code;

  assert!(code.contains(r#"const _t0 = _template("<div>child", 1);"#));
  assert!(code.contains(r#"_setClass(_n0, ["box", data.value.internalClass])"#));
}

#[test]
fn does_not_propagate_component_root_through_nested_transition() {
  let code = transform(
    r#"<section>
      <VaporTransition>
        <div>child</div>
      </VaporTransition>
    </section>"#,
    None,
  )
  .code;

  assert!(!code.contains(r#"_template("<div>child", 1)"#));
}

#[test]
fn v_show_with_appear() {
  let code = transform(
    r#"<VaporTransition appear onAppear={() => {}}>
      <h1 v-show={show}>foo</h1>
    </VaporTransition>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { applyVShow as _applyVShow, template as _template } from "vue";
  const _t0 = _template("<h1>foo", 1);
  (() => {
  	const _n1 = _createComponent(VaporTransition, {
  		appear: true,
  		onAppear: () => () => {},
  		persisted: true
  	}, () => {
  		const _n0 = _t0();
  		_applyVShow(_n0, () => show);
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn work_with_v_if() {
  let code = transform(
    r#"<VaporTransition>
      <h1 v-if={show}>foo</h1>
    </VaporTransition>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<h1>foo", 3);
  (() => {
  	const _n3 = _createComponent(VaporTransition, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, null, 129);
  		return _n0;
  	}, { _: 1 }), true);
  	return _n3;
  })();
  "#);
}

#[test]
fn work_with_v_if_v_else() {
  let code = transform(
    r#"<VaporTransition>
      <h1 v-if={condition == 1}>1</h1>
      <h2 v-else-if={condition == 2}>2</h2>
      <h3 v-else>3</h3>
    </VaporTransition>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<h1>1", 3);
  const _t1 = _template("<h2>2", 3);
  const _t2 = _template("<h3>3", 3);
  (() => {
  	const _n9 = _createComponent(VaporTransition, null, _extend(() => {
  		const _n0 = _createIf(() => condition == 1, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, () => _createIf(() => condition == 2, () => {
  			const _n4 = _t1();
  			return _n4;
  		}, () => {
  			const _n7 = _t2();
  			return _n7;
  		}, 645), 389);
  		return _n0;
  	}, { _: 1 }), true);
  	return _n9;
  })();
  "#);
}

#[test]
fn work_with_condition_expression() {
  let code = transform(
    r#"<VaporTransition>
      { condition == 1
        ? <h1>1</h1>
        : condition == 2
          ? <h2>2</h1>
          : <h3>3</h1> }
    </VaporTransition>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<h1>1", 3);
  const _t1 = _template("<h2>2", 3);
  const _t2 = _template("<h3>3", 3);
  (() => {
  	const _n7 = _createComponent(VaporTransition, null, _extend(() => {
  		const _n0 = _createIf(() => condition == 1, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, () => _createIf(() => condition == 2, () => {
  			const _n4 = _t1();
  			return _n4;
  		}, () => {
  			const _n6 = _t2();
  			return _n6;
  		}, 641), 389);
  		return _n0;
  	}, { _: 1 }), true);
  	return _n7;
  })();
  "#);
}

#[test]
fn transition_work_with_dynamic_keyed_children() {
  let code = transform(
    "<VaporTransition>
      <h1 key={foo}>foo</h1>
    </VaporTransition>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<h1>foo", 3);
  (() => {
  	const _n3 = _createComponent(VaporTransition, null, () => {
  		const _n0 = _createKeyedFragment(() => foo, () => {
  			const _n2 = _t0();
  			return _n2;
  		});
  		return _n0;
  	}, true);
  	return _n3;
  })();
  "#);
}

#[test]
fn does_not_inject_persisted_when_v_if_owns_a_v_show_child() {
  let code = transform(
    "<Transition>
      <h1 v-if={show} v-show={true} />
    </Transition>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { applyVShow as _applyVShow, createIf as _createIf, extend as _extend, template as _template } from "vue";
  const _t0 = _template("<h1>", 1);
  (() => {
  	const _n3 = _createComponent(Transition, null, _extend(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			_applyVShow(_n2, () => true);
  			return _n2;
  		}, null, 129);
  		return _n0;
  	}, { _: 1 }), true);
  	return _n3;
  })();
  "#);
}

// upstream: `child of a non-root transition is a root template`. The child
// inherits the transition's fallthrough attrs, so its template is a root one
// (class / style layered, not overwritten).
#[test]
fn child_of_a_non_root_transition_is_a_root_template() {
  let code = transform(
    "<div><Transition><p class=\"y\">z</p></Transition></div>",
    None,
  )
  .code;

  assert!(code.contains(r#"const _t0 = _template("<p class=y>z", 3);"#));
  assert!(code.contains(r#"const _t1 = _template("<div>", 1);"#));
}

// upstream: `child of a transition under a v-for is a root template`. A v-for
// only cuts the chain between a host and its root; the transition's
// fallthrough reaches its child wherever the transition sits.
#[test]
fn child_of_a_transition_under_a_v_for_is_a_root_template() {
  let code = transform(
    "<div v-for={i in l}>\
      <Transition>\
        <p v-if={ok} class=\"y\">y</p>\
        <p v-else class=\"n\">n</p>\
      </Transition>\
    </div>",
    None,
  )
  .code;

  assert!(code.contains(r#"const _t0 = _template("<p class=y>y", 1);"#));
  assert!(code.contains(r#"const _t1 = _template("<p class=n>n", 1);"#));
}
