use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-for={item in items} key={item.id} onClick={() => remove(item)}>{item}</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  _delegateEvents("click");
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, delegateEvents as _delegateEvents, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _t0();
  		_n2.$evtclick = () => remove(_for_item0.value);
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value);
  		return _n2;
  	}, (item) => item.id);
  	return _n0;
  })();
  "#);
}

#[test]
fn key_only_binding_pattern() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
    >
      { row.id + row.id }
    </tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<tr> </tr>");
  (() => {
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.id + _for_item0.value.id);
  		return _n2;
  	}, (row) => row.id);
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern1() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      v-text={selected === row.id ? 'danger' : ''}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, setText as _setText, template as _template, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  const _t0 = _template("<tr> </tr>");
  (() => {
  	let _selector0_0;
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_selector0_0(() => {
  			_setText(_x2, _toDisplayString(selected === _for_item0.value.id ? "danger" : ""));
  		});
  		return _n2;
  	}, (row) => row.id, void 0, ({ createSelector }) => {
  		_selector0_0 = createSelector(() => selected);
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern2() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={selected === row.id ? 'danger' : ''}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<tr></tr>");
  (() => {
  	let _selector0_0;
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_selector0_0(() => {
  			_setClass(_n2, selected === _for_item0.value.id ? "danger" : "");
  		});
  		return _n2;
  	}, (row) => row.id, void 0, ({ createSelector }) => {
  		_selector0_0 = createSelector(() => selected);
  	});
  	return _n0;
  })();
  "#);
}

// Should not be optimized because row.label is not from parent scope
#[test]
fn selector_pattern3() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={row.label === row.id ? 'danger' : ''}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, renderEffect as _renderEffect, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<tr></tr>");
  (() => {
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_renderEffect(() => _setClass(_n2, _for_item0.value.label === _for_item0.value.id ? "danger" : ""));
  		return _n2;
  	}, (row) => row.id);
  	return _n0;
  })();
  "#);
}

#[test]
fn selector_pattern4() {
  let code = transform(
    "<tr
      v-for={row in rows}
      key={row.id}
      class={{ danger: row.id === selected }}
    ></tr>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, setClass as _setClass, template as _template } from "vue";
  const _t0 = _template("<tr></tr>");
  (() => {
  	let _selector0_0;
  	const _n0 = _createFor(() => rows, (_for_item0) => {
  		const _n2 = _t0();
  		_selector0_0(() => {
  			_setClass(_n2, { danger: _for_item0.value.id === selected });
  		});
  		return _n2;
  	}, (row) => row.id, void 0, ({ createSelector }) => {
  		_selector0_0 = createSelector(() => selected);
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn multi_effect() {
  let code = transform(
    "<div v-for={(item, index) in items} item={item} index={index} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		_renderEffect(() => {
  			_setProp(_n2, "item", _for_item0.value);
  			_setProp(_n2, "index", _for_key0.value);
  		});
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn nested_v_for() {
  let code = transform(
    "<div v-for={i in list}><span v-for={j in i}>{ j+i }</span></div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> </span>");
  const _t1 = _template("<div></div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n5 = _t1();
  		_setInsertionState(_n5, null, true);
  		const _n2 = _createFor(() => _for_item0.value, (_for_item1) => {
  			const _n4 = _t0();
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, () => _for_item1.value + _for_item0.value);
  			return _n4;
  		}, void 0, 1);
  		return _n5;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn object_value_key_and_index() {
  let code = transform(
    "<span v-for={(value, key, index) in items} key={id}>{ id }{ value }{ index }</span>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> </span>");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0, _for_index0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => id, () => _for_item0.value, () => _for_index0.value);
  		return _n2;
  	}, (value, key, index) => id);
  	return _n0;
  })();
  "#);
}

#[test]
fn object_de_structured_value() {
  let code = transform(
    "<span v-for={({ id, value }) in items} key={id}>{ id }{ value }</span>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> </span>");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.id, () => _for_item0.value.value);
  		return _n2;
  	}, ({ id, value }) => id);
  	return _n0;
  })();
  "#);
}

#[test]
fn object_de_structured_value_with_rest() {
  let code = transform(
    "<div v-for={(  { id, ...other }, index) in list} key={id}>{ id + other + index }</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, getRestElement as _getRestElement, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.id + _getRestElement(_for_item0.value, ["id"]) + _for_key0.value);
  		return _n2;
  	}, ({ id, ...other }, index) => id);
  	return _n0;
  })();
  "#);
}

#[test]
fn array_de_structured_value() {
  let code = transform(
    "<div v-for={([id, other], index) in list} key={id}>{ id + other + index }</div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value[0] + _for_item0.value[1] + _for_key0.value);
  		return _n2;
  	}, ([id, other], index) => id);
  	return _n0;
  })();
  "#);
}

#[test]
fn array_de_structured_value_with_rest() {
  let code = transform("<div v-for={([id, [foo], {bar}, ...other], index) in list} key={id}>{ id + other + index + foo + bar }</div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value[0] + _for_item0.value.slice(3) + _for_key0.value + _for_item0.value[1][0] + _for_item0.value[2].bar);
  		return _n2;
  	}, ([id, [foo], {bar}, ...other], index) => id);
  	return _n0;
  })();
  "#);
}

#[test]
fn aliases_with_complex_expressions() {
  let code = transform(
    "<div v-for={({ foo, baz: [qux] }) in list}>
      { foo + baz + qux }
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value.foo + baz + _for_item0.value.baz[0]);
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn fast_remove_flag() {
  let code = transform(
    "<div>
      <span v-for={j in i}>{ j+i }</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> </span>");
  const _t1 = _template("<div></div>", true);
  (() => {
  	const _n3 = _t1();
  	_setInsertionState(_n3, null, true);
  	const _n0 = _createFor(() => i, (_for_item0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value + i);
  		return _n2;
  	}, void 0, 1);
  	return _n3;
  })();
  "#);
}

#[test]
fn on_component() {
  let code = transform("<Comp v-for={item in list}>{item}</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template(" ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n3 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  			const _n2 = _t0();
  			_setNodes(_n2, () => _for_item0.value);
  			return _n2;
  		}) });
  		return _n3;
  	}, void 0, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn on_template_with_single_component_child() {
  let code = transform(
    "<template v-for={item in list}><Comp>{item}</Comp></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template(" ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n3 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  			const _n2 = _t0();
  			_setNodes(_n2, () => _for_item0.value);
  			return _n2;
  		}) });
  		return _n3;
  	}, void 0, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn identifiers() {
  let code = transform(
    "<div v-for={(item, index) in items} id={index}>
    { ((item) => {
      let index = 1
      return [item, index]
    })(item) }
    { (() => {
      switch (item) {
        case index: {
          let item = ''
          return `${[item, index]}`;
        }
      }
    })() }
  </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => ((item) => {
  			let index = 1;
  			return [item, index];
  		})(_for_item0.value), () => (() => {
  			switch (_for_item0.value) {
  				case _for_key0.value: {
  					let item = "";
  					return `${[item, _for_key0.value]}`;
  				}
  			}
  		})());
  		_renderEffect(() => _setProp(_n2, "id", _for_key0.value));
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn expression_object() {
  let code = transform(
    "<div v-for={(item, index) in Array.from({ length: count.value }).map((_, id) => ({ id }))} id={index}>
      {item}
    </div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
  (() => {
  	const _n0 = _createFor(() => Array.from({ length: count.value }).map((_, id) => ({ id })), (_for_item0, _for_key0) => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => _for_item0.value);
  		_renderEffect(() => _setProp(_n2, "id", _for_key0.value));
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_for_with_slotlet() {
  let code = transform(
    r#"<template v-for={item in items}><slot/></template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createSlot("default");
  		return _n2;
  	}, void 0, 2);
  	return _n0;
  })();
  "#)
}

#[test]
fn v_for_on_slotlet() {
  let code = transform(r#"<slot v-for={item in items}></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createFor(() => items, (_for_item0) => {
  		const _n2 = _createSlot("default");
  		return _n2;
  	}, void 0, 2);
  	return _n0;
  })();
  "#)
}

#[test]
fn should_raise_error_if_has_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-for />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VForNoExpression));
}

#[test]
fn should_raise_error_if_malformed_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-for={foo} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VForMalformedExpression));
}
