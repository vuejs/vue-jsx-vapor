use std::cell::RefCell;

use common::options::TransformOptions;
use compiler::transform;

// vapor has no `v-memo`; the directive is dropped from the output and upstream
// warns about it, so the same warning is emitted here.
const MESSAGE: &str = "v-memo is not supported in Vapor mode and will be ignored.";

fn warnings_for(source: &str) -> Vec<(String, usize)> {
  let warnings: RefCell<Vec<(String, usize)>> = RefCell::new(vec![]);
  transform(
    source,
    Some(TransformOptions {
      vapor: true,
      on_warn: Box::new(|message, span| {
        warnings.borrow_mut().push((message, span.start as usize));
      }),
      ..Default::default()
    }),
  );
  warnings.into_inner()
}

#[test]
fn warns_wherever_v_memo_appears() {
  let cases = [
    "<div v-memo={[foo]}>{bar}</div>",
    "<Comp v-memo={[foo]} value={bar} />",
    "<div v-if={ok} v-for={item in items} v-memo={[foo]}>{item}</div>",
    "<template v-if={ok} v-memo={[foo]}><div>{bar}</div></template>",
    "<Comp><template v-slot:default v-memo={[foo]}>{bar}</template></Comp>",
    "<slot v-memo={[foo]}>fallback</slot>",
  ];

  for source in cases {
    let warnings = warnings_for(source);
    // a structural directive makes the element visit twice, so the warning
    // must still be deduped by directive
    assert_eq!(warnings.len(), 1, "{source}: {warnings:?}");
    let (message, start) = &warnings[0];
    assert_eq!(message, MESSAGE, "{source}");
    // the span points at the directive, not the whole element
    assert!(source[*start..].starts_with("v-memo"), "{source} @ {start}");
  }
}

// the warning is the only effect: the directive itself is ignored either way
#[test]
fn v_memo_does_not_change_the_generated_code() {
  for source in [
    "<div v-memo={[foo]}>{bar}</div>",
    "<Comp v-memo={[foo]} value={bar} />",
    "<div v-if={ok} v-for={item in items} v-memo={[foo]}>{item}</div>",
    "<template v-if={ok} v-memo={[foo]}><div>{bar}</div></template>",
    "<Comp><template v-slot:default v-memo={[foo]}>{bar}</template></Comp>",
    "<slot v-memo={[foo]}>fallback</slot>",
  ] {
    let with = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    let without = transform(
      &source.replace(" v-memo={[foo]}", ""),
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(with, without, "{source}");
  }
}

#[test]
fn stays_quiet_without_v_memo() {
  assert!(warnings_for("<div>{bar}</div>").is_empty());
  assert!(warnings_for("<Comp value={bar} />").is_empty());
}
