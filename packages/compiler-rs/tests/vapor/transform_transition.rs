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
  const t0 = _template("<h1>foo</h1>");
  (() => {
  	const n1 = _createComponent(VaporTransition, { persisted: () => true }, { default: () => {
  		const n0 = t0();
  		_applyVShow(n0, () => show);
  		return n0;
  	} }, true);
  	return n1;
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
  const t0 = _template("<h1>foo</h1>");
  (() => {
  	const deferredApplyVShows = [];
  	const n1 = _createComponent(VaporTransition, {
  		appear: () => true,
  		onAppear: () => () => {},
  		persisted: () => true
  	}, { default: () => {
  		const n0 = t0();
  		deferredApplyVShows.push(() => _applyVShow(n0, () => show));
  		return n0;
  	} }, true);
  	deferredApplyVShows.forEach((fn) => fn());
  	return n1;
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
  const t0 = _template("<h1>foo</h1>");
  (() => {
  	const n3 = _createComponent(VaporTransition, null, { default: () => {
  		const n0 = _createIf(() => show, () => {
  			const n2 = t0();
  			return n2;
  		});
  		return n0;
  	} }, true);
  	return n3;
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
  const t0 = _template("<h1>foo</h1>");
  (() => {
  	const n3 = _createComponent(VaporTransition, null, { default: () => {
  		const n0 = _createKeyedFragment(() => foo, () => {
  			const n2 = t0();
  			return n2;
  		});
  		return n0;
  	} }, true);
  	return n3;
  })();
  "#);
}
