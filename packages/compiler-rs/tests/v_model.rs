use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<input v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn modifiers_number() {
  let code = transform("<input v-model_number={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn modifiers_trim() {
  let code = transform("<input v-model_trim={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn modifiers_lazy() {
  let code = transform("<input v-model_lazy={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_input_text() {
  let code = transform("<input type=\"text\" v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_input_radio() {
  let code = transform("<input type=\"radio\" v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_input_checkbox() {
  let code = transform("<input type=\"checkbox\" v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_select() {
  let code = transform("<select v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_textarea() {
  let code = transform("<textarea v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_input_dynamic_type() {
  let code = transform("<input type={foo} v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_dynamic_props() {
  let code = transform("<input {...obj} v-model={model} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_member_expression() {
  let code = transform("<input v-model={setupRef.child} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn should_support_member_expression_with_inline() {
  let code = transform("<><input v-model={setupRef.child} /><input v-model={setupLet.child} /><input v-model={setupMaybeRef.child} /></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn errors_invalid_element() {
  let error = RefCell::new(None);
  transform(
    "<span v-model={model} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnInvalidElement));
}

#[test]
fn errors_plain_elements_with_argument() {
  let error = RefCell::new(None);
  transform(
    "<input v-model:value={model} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelArgOnElement));
}

#[test]
fn errors_allow_usage_on_custom_element() {
  let error = RefCell::new(None);
  transform(
    "<my-input v-model={model} />",
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag.starts_with("my-")),
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), None);
}

#[test]
fn errors_if_used_file_input_element() {
  let error = RefCell::new(None);
  transform(
    "<input type=\"file\" v-model={test} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnFileInputElement));
}

#[test]
fn errors_on_dynamic_value_binding_alongside_v_model() {
  let error = RefCell::new(None);
  transform(
    "<input v-model={test} value={test} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelUnnecessaryValue));
}

#[test]
fn errors_should_not_error_on_static_value_binding_alongside_v_model() {
  let error = RefCell::new(None);
  transform(
    "<input v-model={test} value=\"test\" />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), None);
}

#[test]
fn errors_empty_expression() {
  let error = RefCell::new(None);
  transform(
    "<span v-model=\"\" />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelMalformedExpression));
}

#[test]
fn errors_mal_formed_expression() {
  let error = RefCell::new(None);
  transform(
    "<span v-model={a + b} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelMalformedExpression));
}

#[test]
fn component() {
  let code = transform("<Comp v-model={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_with_arguments() {
  let code = transform("<Comp v-model:bar={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_with_dynamic_arguments() {
  let code = transform("<Comp v-model:$arg$={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_with_dynamic_arguments_with_v_for() {
  let code = transform("<Comp v-for={{arg} in list} v-model:$arg$={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_should_generate_model_value_modifiers() {
  let code = transform("<Comp v-model_trim_bar-baz={foo} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_with_arguments_should_generate_model_modifiers() {
  let code = transform(
    "<Comp v-model:foo_trim={foo} v-model:bar_number={bar} />",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn component_with_dynamic_arguments_should_generate_model_modifiers() {
  let code = transform(
    "<Comp v-model:$foo$_trim={foo} v-model:$bar_value$_number={bar} />",
    None,
  )
  .code;
  assert_snapshot!(code);
}
