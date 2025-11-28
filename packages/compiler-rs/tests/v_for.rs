use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div v-for={item in items} key={item.id} onClick={() => remove(item)}>{item}</div>",
    None,
  )
  .code;
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}

#[test]
fn multi_effect() {
  let code = transform(
    "<div v-for={(item, index) in items} item={item} index={index} />",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn nested_v_for() {
  let code = transform(
    "<div v-for={i in list}><span v-for={j in i}>{ j+i }</span></div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn object_value_key_and_index() {
  let code = transform(
    "<span v-for={(value, key, index) in items} key={id}>{ id }{ value }{ index }</span>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn object_de_structured_value() {
  let code = transform(
    "<span v-for={({ id, value }) in items} key={id}>{ id }{ value }</span>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn object_de_structured_value_with_rest() {
  let code = transform(
    "<div v-for={(  { id, ...other }, index) in list} key={id}>{ id + other + index }</div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn array_de_structured_value() {
  let code = transform(
    "<div v-for={([id, other], index) in list} key={id}>{ id + other + index }</div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn array_de_structured_value_with_rest() {
  let code = transform("<div v-for={([id, [foo], {bar}, ...other], index) in list} key={id}>{ id + other + index + foo + bar }</div>", None).code;
  assert_snapshot!(code);
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
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}

#[test]
fn on_component() {
  let code = transform("<Comp v-for={item in list}>{item}</Comp>", None).code;
  assert_snapshot!(code);
}

#[test]
fn on_template_with_single_component_child() {
  let code = transform(
    "<template v-for={item in list}><Comp>{item}</Comp></template>",
    None,
  )
  .code;
  assert_snapshot!(code);
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
  assert_snapshot!(code);
}

#[test]
fn expression_object() {
  let code = transform(
    "<div v-for={(item, index) in Array.from({ length: count.value }).map((_, id) => ({ id }))} id={index}>
      {item}
    </div>", None).code;
  assert_snapshot!(code);
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
