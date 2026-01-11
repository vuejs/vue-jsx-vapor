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
  import { child as _child, createFor as _createFor, delegateEvents as _delegateEvents, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => items, (_for_item0) => {
      const n2 = t0();
      n2.$evtclick = () => remove(_for_item0.value);
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value);
      return n2;
    }, (item) => item.id);
    return n0;
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
  import { child as _child, createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<tr> </tr>");
  (() => {
    const n0 = _createFor(() => rows, (_for_item0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value.id + _for_item0.value.id);
      return n2;
    }, (row) => row.id);
    return n0;
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
  import { child as _child, createFor as _createFor, setText as _setText, template as _template, toDisplayString as _toDisplayString } from "vue";
  const t0 = _template("<tr> </tr>");
  (() => {
    let _selector0_0;
    const n0 = _createFor(() => rows, (_for_item0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _selector0_0(() => {
        _setText(x2, _toDisplayString(selected === _for_item0.value.id ? "danger" : ""));
      });
      return n2;
    }, (row) => row.id, void 0, ({ createSelector }) => {
      _selector0_0 = createSelector(() => selected);
    });
    return n0;
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
  const t0 = _template("<tr></tr>");
  (() => {
    let _selector0_0;
    const n0 = _createFor(() => rows, (_for_item0) => {
      const n2 = t0();
      _selector0_0(() => {
        _setClass(n2, selected === _for_item0.value.id ? "danger" : "");
      });
      return n2;
    }, (row) => row.id, void 0, ({ createSelector }) => {
      _selector0_0 = createSelector(() => selected);
    });
    return n0;
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
  const t0 = _template("<tr></tr>");
  (() => {
    const n0 = _createFor(() => rows, (_for_item0) => {
      const n2 = t0();
      _renderEffect(() => _setClass(n2, _for_item0.value.label === _for_item0.value.id ? "danger" : ""));
      return n2;
    }, (row) => row.id);
    return n0;
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
  const t0 = _template("<tr></tr>");
  (() => {
    let _selector0_0;
    const n0 = _createFor(() => rows, (_for_item0) => {
      const n2 = t0();
      _selector0_0(() => {
        _setClass(n2, { danger: _for_item0.value.id === selected });
      });
      return n2;
    }, (row) => row.id, void 0, ({ createSelector }) => {
      _selector0_0 = createSelector(() => selected);
    });
    return n0;
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
  const t0 = _template("<div></div>");
  (() => {
    const n0 = _createFor(() => items, (_for_item0, _for_key0) => {
      const n2 = t0();
      _renderEffect(() => {
        _setProp(n2, "item", _for_item0.value);
        _setProp(n2, "index", _for_key0.value);
      });
      return n2;
    });
    return n0;
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
  import { child as _child, createFor as _createFor, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<span> </span>");
  const t1 = _template("<div></div>");
  (() => {
    const n0 = _createFor(() => list, (_for_item0) => {
      const n5 = t1();
      _setInsertionState(n5);
      const n2 = _createFor(() => _for_item0.value, (_for_item1) => {
        const n4 = t0();
        const x4 = _child(n4);
        _setNodes(x4, () => _for_item1.value + _for_item0.value);
        return n4;
      }, void 0, 1);
      return n5;
    });
    return n0;
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
  import { child as _child, createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<span> </span>");
  (() => {
    const n0 = _createFor(() => items, (_for_item0, _for_key0, _for_index0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => id, () => _for_item0.value, () => _for_index0.value);
      return n2;
    }, (value, key, index) => id);
    return n0;
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
  import { child as _child, createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<span> </span>");
  (() => {
    const n0 = _createFor(() => items, (_for_item0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value.id, () => _for_item0.value.value);
      return n2;
    }, ({ id, value }) => id);
    return n0;
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
  import { child as _child, createFor as _createFor, getRestElement as _getRestElement, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => list, (_for_item0, _for_key0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value.id + _getRestElement(_for_item0.value, ["id"]) + _for_key0.value);
      return n2;
    }, ({ id, ...other }, index) => id);
    return n0;
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
  import { child as _child, createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => list, (_for_item0, _for_key0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value[0] + _for_item0.value[1] + _for_key0.value);
      return n2;
    }, ([id, other], index) => id);
    return n0;
  })();
  "#);
}

#[test]
fn array_de_structured_value_with_rest() {
  let code = transform("<div v-for={([id, [foo], {bar}, ...other], index) in list} key={id}>{ id + other + index + foo + bar }</div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => list, (_for_item0, _for_key0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value[0] + _for_item0.value.slice(3) + _for_key0.value + _for_item0.value[1][0] + _for_item0.value[2].bar);
      return n2;
    }, ([id, [foo], {bar}, ...other], index) => id);
    return n0;
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
  import { child as _child, createFor as _createFor, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => list, (_for_item0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value.foo + baz + _for_item0.value.baz[0]);
      return n2;
    });
    return n0;
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
  import { child as _child, createFor as _createFor, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<span> </span>");
  const t1 = _template("<div></div>", true);
  (() => {
    const n3 = t1();
    _setInsertionState(n3);
    const n0 = _createFor(() => i, (_for_item0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value + i);
      return n2;
    }, void 0, 1);
    return n3;
  })();
  "#);
}

#[test]
fn on_component() {
  let code = transform("<Comp v-for={item in list}>{item}</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, template as _template } from "vue";
  const t0 = _template(" ");
  (() => {
    const n0 = _createFor(() => list, (_for_item0) => {
      const n3 = _createComponent(Comp, null, { default: () => {
        const n2 = t0();
        _setNodes(n2, () => _for_item0.value);
        return n2;
      } });
      return n3;
    }, void 0, 2);
    return n0;
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
  import { createFor as _createFor, template as _template } from "vue";
  const t0 = _template(" ");
  (() => {
    const n0 = _createFor(() => list, (_for_item0) => {
      const n3 = _createComponent(Comp, null, { default: () => {
        const n2 = t0();
        _setNodes(n2, () => _for_item0.value);
        return n2;
      } });
      return n3;
    }, void 0, 2);
    return n0;
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
  import { child as _child, createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => items, (_for_item0, _for_key0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => ((item) => {
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
      _renderEffect(() => _setProp(n2, "id", _for_key0.value));
      return n2;
    });
    return n0;
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
  import { child as _child, createFor as _createFor, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
    const n0 = _createFor(() => Array.from({ length: count.value }).map((_, id) => ({ id })), (_for_item0, _for_key0) => {
      const n2 = t0();
      const x2 = _child(n2);
      _setNodes(x2, () => _for_item0.value);
      _renderEffect(() => _setProp(n2, "id", _for_key0.value));
      return n2;
    });
    return n0;
  })();
  "#);
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
