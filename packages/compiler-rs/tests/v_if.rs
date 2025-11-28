use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div v-if={ok}>{msg}</div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn template() {
  let code = transform(
    "<template v-if={ok}><div/>hello<p v-text={msg}></p></template>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn dedupe_same_template() {
  let code = transform(
    "<><div v-if={ok}>hello</div><div v-if={ok}>hello</div></>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn component() {
  let code = transform("<Comp v-if={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn v_if_v_else() {
  let code = transform("<><div v-if={ok}/><p v-else/></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn v_if_v_if_else() {
  let code = transform("<><div v-if={ok}/><p v-else-if={orNot}/></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn v_if_v_else_if_v_else() {
  let code = transform(
    "<><div v-if={ok}/><p v-else-if={orNot}/><template v-else>fine</template></>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn v_if_v_if_or_v_elses() {
  let code = transform(
    "<div>
      <span v-if=\"foo\">foo</span>
      <span v-if=\"bar\">bar</span>
      <span v-else>baz</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn comment_between_branches() {
  let code = transform(
    "<>
      <div v-if={ok}/>
      {/* foo */}
      <p v-else-if={orNot}/>
      {/* bar */}
      <template v-else>fine{/* fine */}</template>
      <div v-text=\"text\" />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn v_on_with_v_if() {
  let code = transform(
    "<button v-on={{ click: clickEvent }} v-if={true}>w/ v-if</button>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn error_on_v_else_missing_adjacent_v_if() {
  let error = RefCell::new(None);
  transform(
    "<div v-else/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn error_on_v_else_if_missing_adjacent_v_if_or_v_else_if() {
  let error = RefCell::new(None);
  transform(
    "<div v-else-if={foo}/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn error_on_v_if_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-if/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VIfNoExpression));
}

// TODO codegen
