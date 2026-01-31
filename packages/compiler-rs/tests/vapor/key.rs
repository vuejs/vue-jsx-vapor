use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn key() {
  let code = transform("<div key={id} />", None).code;
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createKeyedFragment(() => id, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn key_with_v_once() {
  let code = transform(r#"<div v-once key={id} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#,
  );
}

#[test]
fn key_with_v_if() {
  let code = transform("<div v-if={id} key={id} />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createIf(() => id, () => {
  		const _n2 = _createKeyedFragment(() => id, () => {
  			const _n4 = _t0();
  			return _n4;
  		});
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn key_with_anchor_insertion_in_middle() {
  let code = transform(
    "<div>
      <div></div>
      <div key={1}></div>
      <div></div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, createKeyedFragment as _createKeyedFragment, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>");
  const _t1 = _template("<div><div></div><!><div>", true);
  (() => {
  	const _n4 = _t1();
  	const _n3 = _next(_child(_n4, 1));
  	_setInsertionState(_n4, _n3, true);
  	const _n0 = _createKeyedFragment(() => 1, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n4;
  })();
  "#);
}
