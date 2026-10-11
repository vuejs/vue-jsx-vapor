use std::cell::RefCell;

use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;
use oxc_span::Span;

#[test]
fn parse_errors_are_reported() {
  let errors = RefCell::new(Vec::new());
  transform(
    "const _c = () => <div><b></div>",
    Some(TransformOptions {
      on_error: Box::new(|message, span| {
        errors.borrow_mut().push((message, span));
      }),
      ..Default::default()
    }),
  );
  let errors = errors.into_inner();
  assert_eq!(errors.len(), 2);
  assert_snapshot!(errors[0].0, @"Expected corresponding JSX closing tag for 'b'.");
  assert_ne!(errors[0].1, Span::default());
}

#[test]
fn semantic_errors_are_reported() {
  let errors = RefCell::new(Vec::new());
  transform(
    "const a = 1; const a = 2;",
    Some(TransformOptions {
      on_error: Box::new(|message, span| {
        errors.borrow_mut().push((message, span));
      }),
      ..Default::default()
    }),
  );
  let errors = errors.into_inner();
  assert_eq!(errors.len(), 1);
  assert_snapshot!(errors[0].0, @"Identifier `a` has already been declared");
  assert_ne!(errors[0].1, Span::default());
}

#[test]
fn semantic_syntax_errors_are_reported() {
  let errors = RefCell::new(Vec::new());
  transform(
    "export { missing };",
    Some(TransformOptions {
      on_error: Box::new(|message, span| {
        errors.borrow_mut().push((message, span));
      }),
      ..Default::default()
    }),
  );
  assert_eq!(errors.into_inner().len(), 1);
}

#[test]
fn no_diagnostics_on_valid_source() {
  let errors = RefCell::new(Vec::new());
  let code = transform(
    "const _c = () => <div>{foo}</div>",
    Some(TransformOptions {
      on_error: Box::new(|message, span| {
        errors.borrow_mut().push((message, span));
      }),
      ..Default::default()
    }),
  )
  .code;
  assert!(errors.into_inner().is_empty());
  assert!(!code.is_empty());
}

#[test]
fn validation_errors_are_reported() {
  let errors = RefCell::new(Vec::new());
  transform(
    "const _c = () => <input v-model={a + b} />",
    Some(TransformOptions {
      on_error: Box::new(|message, span| {
        errors.borrow_mut().push((message, span));
      }),
      ..Default::default()
    }),
  );
  let errors = errors.into_inner();
  assert_eq!(errors.len(), 1);
  assert_snapshot!(
    errors[0].0,
    @"v-model value must be a valid JavaScript member expression."
  );
  assert_ne!(errors[0].1, Span::default());
}
