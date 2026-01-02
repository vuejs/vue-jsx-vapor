pub use common::options::TransformOptions;
use napi::{
  Env,
  bindgen_prelude::{Function, Object},
};
use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenReturn, IndentChar};
use oxc_parser::Parser;
use oxc_span::{SourceType, Span};
use std::path::PathBuf;

use common::error::{ErrorCodes, create_compiler_error};

use crate::transform::Transform;

mod hmr_or_ssr;
mod transform;

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default)]
pub struct CompilerOptions {
  /**
   * Whether to compile components to createComponentWithFallback.
   * @default false
   */
  pub with_fallback: Option<bool>,
  /**
   * Separate option for end users to extend the native elements list
   */
  pub is_custom_element: Option<Function<'static, String, bool>>,
  pub on_error: Option<Function<'static, Object<'static>, ()>>,
  /**
   * Generate source map?
   * @default false
   */
  pub source_map: Option<bool>,
  /**
   * Filename for source map generation.
   * Also used for self-recursive reference in templates
   * @default 'index.jsx'
   */
  pub filename: Option<String>,
  /**
   * When enabled, JSX within `defineVaporComponent` is transformed to Vapor DOM,
   * while all other JSX is transformed to Virtual DOM.
   */
  pub interop: Option<bool>,
  /**
   * Enabled HMR support.
   * @default false
   */
  pub hmr: Option<bool>,
  /**
   * Enabled SSR support.
   * @default false
   */
  pub ssr: Option<bool>,
  /**
   * Whether the compiler should detect if the `patchFlag` for slots is stable.
   * This is only used in interop mode.
   * Note: This is not supported for slots within `CallExpression` (e.g. `map()`) or `ObjectExpression` | `FunctionExpression` slots.
   *       Please use v-for and v-slot directive instead.
   * @default false
   */
  pub optimize_slots: Option<bool>,
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct TransformReturn {
  pub code: String,
  pub map: Option<String>,
}

#[cfg(feature = "napi")]
#[napi]
pub fn _transform(env: Env, source: String, options: Option<CompilerOptions>) -> TransformReturn {
  use std::cell::RefCell;

  let options = options.unwrap_or_default();
  let filename = &options.filename.unwrap_or("index.jsx".to_string());
  let ssr = options.ssr.unwrap_or(false);
  let CodegenReturn { code, map, .. } = transform(
    &source,
    Some(TransformOptions {
      filename,
      source_type: SourceType::from_path(filename).unwrap(),
      source_map: options.source_map.unwrap_or(false),
      with_fallback: options.with_fallback.unwrap_or(false),
      interop: options.interop.unwrap_or(false),
      hmr: options.hmr.unwrap_or(false),
      ssr: RefCell::new(ssr),
      in_ssr: ssr,
      is_custom_element: if let Some(is_custom_element) = options.is_custom_element {
        Box::new(move |tag: String| is_custom_element.call(tag).unwrap())
          as Box<dyn Fn(String) -> bool>
      } else {
        Box::new(|_: String| false) as Box<dyn Fn(String) -> bool>
      },
      on_error: if let Some(on_error) = options.on_error {
        Box::new(move |code: ErrorCodes, span: Span| {
          let compiler_error = create_compiler_error(&env, code, span).unwrap();
          on_error.call(compiler_error).unwrap();
        }) as Box<dyn Fn(ErrorCodes, Span)>
      } else {
        Box::new(|_: ErrorCodes, _: Span| {}) as Box<dyn Fn(ErrorCodes, Span)>
      },
      ..Default::default()
    }),
  );
  TransformReturn {
    code,
    map: map.map(|m| m.to_json_string()),
  }
}

pub fn transform<'a>(source: &'a str, options: Option<TransformOptions<'a>>) -> CodegenReturn {
  use oxc_codegen::CodegenOptions;
  let options = options.unwrap_or_default();
  let filename = options.filename;
  let source_map = options.source_map;
  let source_type = options.source_type;
  let allocator = Allocator::default();
  let allocator = &allocator as *const Allocator;
  let mut program = Parser::new(unsafe { &*allocator }, source, source_type)
    .parse()
    .program;
  Transform::new(unsafe { &*allocator }, unsafe {
    &*(&options as *const TransformOptions)
  })
  .traverse(&mut program);
  Codegen::new()
    .with_options(CodegenOptions {
      source_map_path: if source_map {
        Some(PathBuf::from(&filename))
      } else {
        None
      },
      indent_width: 2,
      indent_char: IndentChar::Space,
      ..CodegenOptions::default()
    })
    .build(&program)
}
