use std::cell::RefCell;

use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn parse_errors_are_reported() {
  let diagnostics = RefCell::new(Vec::new());
  transform(
    "const _c = () => <div><b></div>",
    Some(TransformOptions {
      on_diagnostic: Box::new(|diagnostic| {
        diagnostics.borrow_mut().push((
          diagnostic.message.to_string(),
          diagnostic
            .labels
            .as_ref()
            .and_then(|labels| labels.first().map(|label| (label.offset(), label.len()))),
        ));
      }),
      ..Default::default()
    }),
  );
  let diagnostics = diagnostics.into_inner();
  assert_eq!(diagnostics.len(), 2);
  let (message, label) = &diagnostics[0];
  assert_snapshot!(message, @"Expected corresponding JSX closing tag for 'b'.");
  assert!(label.is_some());
}

#[test]
fn semantic_errors_are_reported() {
  let diagnostics = RefCell::new(Vec::new());
  transform(
    "const a = 1; const a = 2;",
    Some(TransformOptions {
      on_diagnostic: Box::new(|diagnostic| {
        diagnostics
          .borrow_mut()
          .push(diagnostic.message.to_string());
      }),
      ..Default::default()
    }),
  );
  assert_eq!(diagnostics.into_inner().len(), 1);
}

#[test]
fn semantic_syntax_errors_are_reported() {
  let diagnostics = RefCell::new(Vec::new());
  transform(
    "export { missing };",
    Some(TransformOptions {
      on_diagnostic: Box::new(|diagnostic| {
        diagnostics
          .borrow_mut()
          .push(diagnostic.message.to_string());
      }),
      ..Default::default()
    }),
  );
  assert_eq!(diagnostics.into_inner().len(), 1);
}

#[test]
fn no_diagnostics_on_valid_source() {
  let diagnostics = RefCell::new(Vec::new());
  let code = transform(
    "const _c = () => <div>{foo}</div>",
    Some(TransformOptions {
      on_diagnostic: Box::new(|diagnostic| {
        diagnostics
          .borrow_mut()
          .push(diagnostic.message.to_string());
      }),
      ..Default::default()
    }),
  )
  .code;
  assert!(diagnostics.into_inner().is_empty());
  assert!(!code.is_empty());
}
