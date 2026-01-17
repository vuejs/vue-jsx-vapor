use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn key() {
  let code = transform("<div key={id} />", None).code;
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const n0 = _createKeyedFragment(() => id, () => {
  		const n2 = t0();
  		return n2;
  	});
  	return n0;
  })();
  "#);
}

#[test]
fn key_with_v_once() {
  let code = transform(r#"<div v-once key={id} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n0 = t0();
  	return n0;
  })();
  "#,
  );
}

#[test]
fn key_with_v_if() {
  let code = transform("<div v-if={id} key={id} />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const n0 = _createIf(() => id, () => {
  		const n2 = _createKeyedFragment(() => id, () => {
  			const n4 = t0();
  			return n4;
  		});
  		return n2;
  	});
  	return n0;
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
  const t0 = _template("<div></div>");
  const t1 = _template("<div><div></div><!><div></div></div>", true);
  (() => {
  	const n4 = t1();
  	const n3 = _next(_child(n4, 1));
  	_setInsertionState(n4, n3, true);
  	const n0 = _createKeyedFragment(() => 1, () => {
  		const n2 = t0();
  		return n2;
  	});
  	return n4;
  })();
  "#);
}
