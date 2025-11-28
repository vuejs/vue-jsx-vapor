use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<Comp v-slots={{ default: ({ foo })=> <>{ foo + bar }</> }}></Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn nested() {
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
  assert_snapshot!(code);
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
