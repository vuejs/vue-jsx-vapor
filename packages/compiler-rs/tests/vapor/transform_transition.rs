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
  import { applyVShow as _applyVShow, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<h1>foo");
  (() => {
  	const _n1 = _createComponent(VaporTransition, { persisted: () => true }, { default: _withVaporCtx(() => {
  		const _n0 = _t0();
  		_applyVShow(_n0, () => show);
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);
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
  import { applyVShow as _applyVShow, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<h1>foo");
  (() => {
  	const deferredApplyVShows = [];
  	const _n1 = _createComponent(VaporTransition, {
  		appear: () => true,
  		onAppear: () => () => {},
  		persisted: () => true
  	}, { default: _withVaporCtx(() => {
  		const _n0 = _t0();
  		deferredApplyVShows.push(() => _applyVShow(_n0, () => show));
  		return _n0;
  	}) }, true);
  	deferredApplyVShows.forEach((fn) => fn());
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
  import { createIf as _createIf, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<h1>foo");
  (() => {
  	const _n3 = _createComponent(VaporTransition, null, { default: _withVaporCtx(() => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		});
  		return _n0;
  	}) }, true);
  	return _n3;
  })();
  "#);
}

#[test]
fn work_with_v_if_v_else() {
  let code = transform(
    r#"<VaporTransition>
      <h1 v-if={condition == 1}>1</h1>
      <h2 v-else-if={condition == 2}>2</h1>
      <h3 v-else>3</h1>
    </VaporTransition>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<h1>1");
  const _t1 = _template("<h2>2");
  const _t2 = _template("<h3>3");
  (() => {
  	const _n9 = _createComponent(VaporTransition, null, { default: _withVaporCtx(() => {
  		const _n0 = _createIf(() => condition == 1, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, () => _createIf(() => condition == 2, () => {
  			const _n4 = _t1();
  			return _n4;
  		}, () => {
  			const _n7 = _t2();
  			return _n7;
  		}, false, 1), false, 0);
  		return _n0;
  	}) }, true);
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
  import { createIf as _createIf, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<h1>1");
  const _t1 = _template("<h2>2");
  const _t2 = _template("<h3>3");
  (() => {
  	const _n7 = _createComponent(VaporTransition, null, { default: _withVaporCtx(() => {
  		const _n0 = _createIf(() => condition == 1, () => {
  			const _n2 = _t0();
  			return _n2;
  		}, () => _createIf(() => condition == 2, () => {
  			const _n4 = _t1();
  			return _n4;
  		}, () => {
  			const _n6 = _t2();
  			return _n6;
  		}, false, 1), false, 0);
  		return _n0;
  	}) }, true);
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
  import { createKeyedFragment as _createKeyedFragment, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("<h1>foo");
  (() => {
  	const _n3 = _createComponent(VaporTransition, null, { default: _withVaporCtx(() => {
  		const _n0 = _createKeyedFragment(() => foo, () => {
  			const _n2 = _t0();
  			return _n2;
  		});
  		return _n0;
  	}) }, true);
  	return _n3;
  })();
  "#);
}
