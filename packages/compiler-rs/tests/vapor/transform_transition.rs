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
    const n3 = _createComponent(VaporTransition, { persisted: () => true }, { default: () => {
      const n1 = t0();
      _applyVShow(n1, () => show);
      return n1;
    } }, true);
    return n3;
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
    const n3 = _createComponent(VaporTransition, {
      appear: () => true,
      onAppear: () => () => {},
      persisted: () => true
    }, { default: () => {
      const n1 = t0();
      deferredApplyVShows.push(() => _applyVShow(n1, () => show));
      return n1;
    } }, true);
    deferredApplyVShows.forEach((fn) => fn());
    return n3;
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
    const n5 = _createComponent(VaporTransition, null, { default: () => {
      const n1 = _createIf(() => show, () => {
        const n3 = t0();
        return n3;
      });
      return n1;
    } }, true);
    return n5;
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
  import { template as _template } from "vue";
  const t0 = _template("<h1>foo</h1>");
  (() => {
    const n3 = _createComponent(VaporTransition, null, { default: () => {
      const n1 = t0();
      return n1;
    } }, true);
    return n3;
  })();
  "#);
}
