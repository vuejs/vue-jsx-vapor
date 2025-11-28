use std::{collections::HashMap, sync::LazyLock};

use napi::{Env, Error, Result, bindgen_prelude::Object};
use napi_derive::napi;
use oxc_span::Span;

#[cfg_attr(feature = "napi", napi)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrorCodes {
  VIfNoExpression = 28,
  VElseNoAdjacentIf = 30,
  VForNoExpression = 31,
  VForMalformedExpression = 32,
  VOnNoExpression = 35,
  VSlotMixedSlotUsage = 37,
  VSlotDuplicateSlotNames = 38,
  VSlotExtraneousDefaultSlotChildren = 39,
  VSlotMisplaced = 40,
  VModelNoExpression = 41,
  VModelMalformedExpression = 42,
  VHtmlNoExpression = 53,
  VHtmlWithChildren = 54,
  VTextNoExpression = 55,
  VTextWithChildren = 56,
  VModelOnInvalidElement = 57,
  VModelArgOnElement = 58,
  VModelOnFileInputElement = 59,
  VModelUnnecessaryValue = 60,
  VShowNoExpression = 61,
  VSlotsNoExpression = 62,
}

pub static ERROR_MESSAGES: LazyLock<HashMap<ErrorCodes, &str>> = LazyLock::new(|| {
  HashMap::from([
    (
      ErrorCodes::VIfNoExpression,
      "v-if/v-else-if is missing expression.",
    ),
    (
      ErrorCodes::VElseNoAdjacentIf,
      "v-else/v-else-if has no adjacent v-if or v-else-if.",
    ),
    (ErrorCodes::VForNoExpression, "v-for is missing expression."),
    (
      ErrorCodes::VForMalformedExpression,
      "v-for has invalid expression.",
    ),
    (ErrorCodes::VOnNoExpression, "v-on is missing expression."),
    (
      ErrorCodes::VSlotMixedSlotUsage,
      "Mixed v-slot usage on both the component and nested <template>. When there are multiple named slots, all slots should use <template> syntax to avoid scope ambiguity.",
    ),
    (
      ErrorCodes::VSlotDuplicateSlotNames,
      "Duplicate slot names found.",
    ),
    (
      ErrorCodes::VSlotExtraneousDefaultSlotChildren,
      "Extraneous children found when component already has explicitly named default slot. These children will be ignored.",
    ),
    (
      ErrorCodes::VModelNoExpression,
      "v-model is missing expression.",
    ),
    (
      ErrorCodes::VModelMalformedExpression,
      "v-model value must be a valid JavaScript member expression.",
    ),
    (
      ErrorCodes::VSlotMisplaced,
      "v-slot can only be used on components or <template> tags.",
    ),
    (
      ErrorCodes::VSlotsNoExpression,
      "v-slots is missing expression.",
    ),
    (
      ErrorCodes::VHtmlNoExpression,
      "v-html is missing expression.",
    ),
    (
      ErrorCodes::VHtmlWithChildren,
      "v-html will override element children.",
    ),
    (
      ErrorCodes::VTextNoExpression,
      "v-text is missing expression.",
    ),
    (
      ErrorCodes::VTextWithChildren,
      "v-text will override element children.",
    ),
    (
      ErrorCodes::VModelArgOnElement,
      "v-model argument is not supported on plain elements.",
    ),
    (
      ErrorCodes::VModelOnInvalidElement,
      "v-model can only be used on <input>, <textarea> and <select> elements.",
    ),
    (
      ErrorCodes::VModelOnFileInputElement,
      "v-model cannot be used on file inputs since they are read-only. Use a v-on:change listener instead.",
    ),
    (
      ErrorCodes::VModelUnnecessaryValue,
      "Unnecessary value binding used alongside v-model. It will interfere with v-model's behavior.",
    ),
    (
      ErrorCodes::VShowNoExpression,
      "v-show is missing expression.",
    ),
  ])
});

#[napi(object, js_name = "CompilerError extends SyntaxError")]
pub struct CompilerError {
  pub code: i32,
  pub loc: Option<(u32, u32)>,
}

pub fn create_compiler_error<'a>(env: &'a Env, code: ErrorCodes, loc: Span) -> Result<Object<'a>> {
  let msg = ERROR_MESSAGES.get(&code).unwrap().to_string();
  let mut error = env.create_error(Error::from_reason(&msg))?;
  error.set("code", code as i32)?;
  error.set("loc", (loc.start, loc.end))?;
  Ok(error)
}
