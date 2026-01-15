use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<Comp v-slots={{ default: ({ foo })=> <>{ foo + bar }</> }}></Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const n0 = _createComponent(Comp, null, { $: [{ default: ({ foo }) => (() => {
  		const n0 = _createNodes(() => foo + bar);
  		return n0;
  	})() }] }, true);
  	return n0;
  })();
  "#);
}

#[test]
fn function_expression_children() {
  let code = transform(
    r#"<Comp>
      {() => <div />}
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("<div></div>", true);
  (() => {
  	const n2 = _createComponent(Comp, null, { $: [{ default: () => (() => {
  		const n0 = t0();
  		return n0;
  	})() }] }, true);
  	return n2;
  })();
  "#);
}

#[test]
fn object_expression_children() {
  let code = transform(
    r#"<Comp>
      {{ default: () => <>foo</> }}
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("foo");
  (() => {
  	const n2 = _createComponent(Comp, null, { $: [{ default: () => (() => {
  		const n0 = t0();
  		return n0;
  	})() }] }, true);
  	return n2;
  })();
  "#);
}

#[test]
fn object_expression_children_with_computed_property() {
  let code = transform(
    r#"<Comp>
      {{ [foo]: () => <>foo</> }}
    </Comp>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("foo");
  (() => {
  	const n2 = _createComponent(Comp, null, { $: [() => ({ [foo]: () => (() => {
  		const n0 = t0();
  		return n0;
  	})() })] }, true);
  	return n2;
  })();
  "#);
}

#[test]
fn v_slot_with_v_slots() {
  let code = transform(
    "<Comp v-slot={{ bar }}>
      <Comp bar={bar} v-slots={{
        bar,
        default: ({ foo })=> <>
          { foo + bar }
          {<Comp v-slot={{baz}}>{bar}{baz}</Comp>}
        </>
      }}>
      </Comp>{bar}
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const n5 = _createComponent(Comp, null, { default: (_slotProps0) => {
  		const n2 = _createComponent(Comp, { bar: () => _slotProps0.bar }, { $: [{
  			bar: _slotProps0.bar,
  			default: ({ foo }) => (() => {
  				const n1 = _createNodes(() => foo + bar, () => (() => {
  					const n2 = _createComponent(Comp, null, { default: (_slotProps0) => {
  						const n0 = _createNodes(() => bar, () => _slotProps0.baz);
  						return n0;
  					} }, true);
  					return n2;
  				})());
  				return n1;
  			})()
  		}] });
  		const n3 = _createNodes(() => _slotProps0.bar);
  		return [n2, n3];
  	} }, true);
  	return n5;
  })();
  "#);
}

#[test]
fn should_raise_error_if_not_component() {
  let error = RefCell::new(None);
  transform(
    "<div v-slots={obj}></div>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMisplaced));
}

#[test]
fn should_raise_error_if_has_children() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slots={obj}> </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMixedSlotUsage));
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slots></Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotsNoExpression));
}
