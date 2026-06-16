use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler::transform;
use insta::assert_snapshot;

#[test]
fn default_slot_outlet() {
  let code = transform(
    r#"<slot>
    </slot>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot();
  	return _n0;
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet() {
  let code = transform(r#"<slot name="foo" />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("foo");
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet() {
  let code = transform(r#"<slot name={foo} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot(() => foo);
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_props() {
  let code = transform(r#"<slot foo="bar" baz={qux} foo-bar={foo-bar} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("default", {
  		foo: "bar",
  		baz: () => qux,
  		"foo-bar": () => foo - bar
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn statically_named_slot_outlet_with_props() {
  let code = transform(r#"<slot name="foo" foo="bar" baz={qux} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot("foo", {
  		foo: "bar",
  		baz: () => qux
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn dynamically_named_slot_outlet_with_props() {
  let code = transform(r#"<slot name={foo} foo="bar" baz={qux} />"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createSlot(() => foo, {
  		foo: "bar",
  		baz: () => qux
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_fallback() {
  let code = transform(r#"<slot><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("default", null, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn named_slot_outlet_with_fallback() {
  let code = transform(r#"<slot name="foo"><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("foo", null, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn default_slot_outlet_with_props_and_fallback() {
  let code = transform(r#"<slot foo={bar}><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("default", { foo: () => bar }, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn named_slot_outlet_with_props_and_fallback() {
  let code = transform(r#"<slot name="foo" foo={bar}><div /></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n0 = _createSlot("foo", { foo: () => bar }, () => {
  		const _n1 = _t0();
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn slots_component() {
  let code = transform(r#"<slots.foo foo={bar}><div /></slots.foo>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n1 = _createComponent(slots.foo, { foo: () => bar }, () => {
  		const _n0 = _t0();
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn dollor_slots_component() {
  let code = transform(r#"<$slots.foo foo={bar}><div /></$slots.foo>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  (() => {
  	const _n1 = _createComponent($slots.foo, { foo: () => bar }, () => {
  		const _n0 = _t0();
  		return _n0;
  	}, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn root_v_if_fallbck() {
  let code = transform(r#"<slot><span v-if={ok}/></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createSlot as _createSlot, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  (() => {
  	const _n0 = _createSlot("default", null, () => {
  		const _n1 = _createIf(() => ok, () => {
  			const _n3 = _t0();
  			return _n3;
  		}, null, 161);
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn nested_root_v_for_fallbck() {
  let code = transform(
    r#"<slot><template v-if={ok}><span v-for={item in items}>{ item }</span></template></slot>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createIf as _createIf, createSlot as _createSlot, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  (() => {
  	const _n0 = _createSlot("default", null, () => {
  		const _n1 = _createIf(() => ok, () => {
  			const _n3 = _createFor(() => items, (_for_item0) => {
  				const _n5 = _t0();
  				const _x5 = _txt(_n5);
  				_setNodes(_x5, () => _for_item0.value);
  				return _n5;
  			}, void 0, 40);
  			return _n3;
  		}, null, 129);
  		return _n1;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn does_not_mark_non_root_fallback_v_if_as_slot_root() {
  let code = transform(r#"<div><span v-if={ok}/></div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  const _t1 = _template("<div>", 1);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, null, 0);
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	return _n3;
  })();
  "#);
}

#[test]
fn error_on_unexpected_cunstom_directive_on_slot() {
  let error = RefCell::new(None);
  transform(
    r#"<slot v-foo />"#,
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  )
  .code;
  assert_eq!(
    *error.borrow(),
    Some(ErrorCodes::VSlotUnexpectedDirectiveOnSlotOutlet)
  );
}
