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
  const _t0 = _template("<h1>foo</h1>");
  (() => {
  	const _n1 = _createComponent(VaporTransition, { persisted: () => true }, { default: () => {
  		const _n0 = _t0();
  		_applyVShow(_n0, () => show);
  		return _n0;
  	} }, true);
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
  import { applyVShow as _applyVShow, template as _template } from "vue";
  const _t0 = _template("<h1>foo</h1>");
  (() => {
  	const deferredApplyVShows = [];
  	const _n1 = _createComponent(VaporTransition, {
  		appear: () => true,
  		onAppear: () => () => {},
  		persisted: () => true
  	}, { default: () => {
  		const _n0 = _t0();
  		deferredApplyVShows.push(() => _applyVShow(_n0, () => show));
  		return _n0;
  	} }, true);
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
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<h1>foo</h1>");
  (() => {
  	const _n3 = _createComponent(VaporTransition, null, { default: () => {
  		const _n0 = _createIf(() => show, () => {
  			const _n2 = _t0();
  			return _n2;
  		});
  		return _n0;
  	} }, true);
  	return _n3;
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
  const _t0 = _template("<h1>foo</h1>");
  (() => {
  	const _n3 = _createComponent(VaporTransition, null, { default: () => {
  		const _n0 = _createKeyedFragment(() => foo, () => {
  			const _n2 = _t0();
  			return _n2;
  		});
  		return _n0;
  	} }, true);
  	return _n3;
  })();
  "#);
}
