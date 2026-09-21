use compiler::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn key() {
  let code = transform(
    "<div key={id} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>", 3);
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
  let code = transform(
    r#"<div v-once key={id} />"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#,
  );
}

#[test]
fn key_with_v_if() {
  let code = transform(
    "<div v-if={id} key={id} />",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>", 3);
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
fn key_with_template_v_if() {
  let code = transform(
    r#"<div>
      <template v-if={ok} key={a}><div /></template>
      <template v-else-if={foo} key={b}><div /></template>
      <template v-else key={c}><div /></template>
    </div>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createKeyedFragment as _createKeyedFragment, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<template></template>");
  const _t2 = _template("<template>");
  const _t3 = _template("<div>", 1);
  (() => {
  	const _n13 = _t3();
  	_setInsertionState(_n13);
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _createKeyedFragment(() => a, () => {
  			const _n4 = _t0();
  			return _n4;
  		});
  		return _n2;
  	}, () => _createIf(() => foo, () => {
  		const _n6 = _createKeyedFragment(() => b, () => {
  			const _n8 = _t0();
  			return _n8;
  		});
  		return _n6;
  	}, () => {
  		const _n10 = _createKeyedFragment(() => c, () => {
  			const _n12 = _t0();
  			return _n12;
  		});
  		return _n10;
  	}, 517), 261);
  	return _n13;
  })();
  "#);
}

#[test]
fn key_with_template_v_slot() {
  let code = transform(
    r#"<Comp><template v-slot:foo={({ x })} key={a}>{ x }</template></Comp>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  // same as vdom: the key on a <template> slot is ignored
  assert!(!code.contains("_createKeyedFragment("));
  assert!(code.contains("foo: _extend((_slotProps0) =>"));
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx/vapor";
  import { extend as _extend } from "vue";
  (() => {
  	const _n2 = _createComponent(Comp, null, { foo: _extend((_slotProps0) => {
  		const _n0 = _createNodes(() => _slotProps0.x);
  		return _n0;
  	}, { _: 1 }) }, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn key_with_anchor_insertion_in_middle() {
  let code = transform(
    "<div>
      <div></div>
      <div key={foo}></div>
      <div></div>
    </div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, createKeyedFragment as _createKeyedFragment, next as _next, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<div><div></div><!><div>", 1);
  (() => {
  	const _n4 = _t1();
  	const _n3 = _next(_child(_n4));
  	_setInsertionState(_n4, _n3);
  	const _n0 = _createKeyedFragment(() => foo, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n4;
  })();
  "#);
}

#[test]
fn key_in_component() {
  let code = transform(
    "<Comp><div key={key} /></Comp>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { createKeyedFragment as _createKeyedFragment, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n3 = _createComponent(Comp, null, () => {
  		const _n0 = _createKeyedFragment(() => key, () => {
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
fn static_key() {
  let code = transform(
    "<>
      <div key={1} />
      <Comp key={1} />
    </>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	_setBlockKey(_n0, 1);
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n1, 1);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn boolean_static_expression_key() {
  let code = transform(
    "<>
      <div key={true} />
      <Comp key={true} />
    </>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	_setBlockKey(_n0, true);
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n1, true);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn null_static_expression_key() {
  let code = transform(
    "<>
      <div key={null} />
      <Comp key={null} />
    </>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	_setBlockKey(_n0, null);
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n1, null);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn v_once_with_static_key() {
  let code = transform(
    r#"<>
      <div v-once key="foo" />
      <Comp v-once key="foo" />
    </>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	_setBlockKey(_n0, "foo");
  	const _n1 = _createComponent(Comp, null, null, null, true);
  	_setBlockKey(_n1, "foo");
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn key_without_value() {
  let code = transform(
    r#"<>
      <div key />
      <Comp key />
    </>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _t0();
  	_setBlockKey(_n0, true);
  	const _n1 = _createComponent(Comp);
  	_setBlockKey(_n1, true);
  	return [_n0, _n1];
  })();
  "#);
}

#[test]
fn nested_element_and_key() {
  let code = transform(
    r#"<div><span key="a"></span></div>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains("_setBlockKey("));
  assert!(code.contains("_child("));
  assert_snapshot!(code, @r#"
  import { child as _child, setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div><span>", 1);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _child(_n1);
  	_setBlockKey(_n0, "a");
  	return _n1;
  })();
  "#);
}

#[test]
fn slot_roots_with_key() {
  let code = transform(
    r#"<Foo><div key="a"></div><div key="b"></div></Foo>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(code.matches("_setBlockKey(").count(), 2);
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("<div>");
  (() => {
  	const _n2 = _createComponent(Foo, null, () => {
  		const _n0 = _t0();
  		const _n1 = _t1();
  		_setBlockKey(_n0, "a");
  		_setBlockKey(_n1, "b");
  		return [_n0, _n1];
  	}, true);
  	return _n2;
  })();
  "#);
}

#[test]
fn v_if_branch_root_with_key() {
  let code = transform(
    r#"<div v-if="ok" key="a"></div>"#,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains("_setBlockKey("));
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setBlockKey as _setBlockKey, template as _template } from "vue";
  const _t0 = _template("<div>", 1);
  (() => {
  	const _n0 = _createIf(() => "ok", () => {
  		const _n2 = _t0();
  		_setBlockKey(_n2, "a");
  		return _n2;
  	}, null, 17);
  	return _n0;
  })();
  "#);
}
