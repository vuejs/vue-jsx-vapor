use std::cell::RefCell;

use common::options::TransformOptions;
use compiler::transform;

// vapor has no `v-memo`; the directive is dropped from the output and upstream
// warns about it, so the same warning is emitted here.
const MESSAGE: &str = "v-memo is not supported in Vapor mode and will be ignored.";

fn warning_for(source: &str) -> Option<(String, usize)> {
  let warning: RefCell<Option<(String, usize)>> = RefCell::new(None);
  transform(
    source,
    Some(TransformOptions {
      vapor: true,
      on_warn: Box::new(|message, span| {
        *warning.borrow_mut() = Some((message, span.start as usize));
      }),
      ..Default::default()
    }),
  );
  warning.into_inner()
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
    let (message, start) = warning_for(source).unwrap_or_else(|| panic!("no warning for {source}"));
    assert_eq!(message, MESSAGE, "{source}");
    // the span points at the directive, not the whole element
    assert!(source[start..].starts_with("v-memo"), "{source} @ {start}");
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
  assert!(warning_for("<div>{bar}</div>").is_none());
  assert!(warning_for("<Comp value={bar} />").is_none());
}
