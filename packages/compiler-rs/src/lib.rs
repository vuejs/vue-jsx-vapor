pub use common::options::TransformOptions;
use napi::{
  Either, Env,
  bindgen_prelude::{Function, Object},
};
use napi_derive::napi;
use oxc_codegen::{Codegen, CodegenReturn};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{SourceType, Span};
use std::path::{Path, PathBuf};

use common::{
  error::{ErrorCodes, create_compiler_error},
  options::Hmr,
};

use crate::transform::Transform;

mod hmr_or_ssr;
mod transform;

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default)]
pub struct CompilerOptions {
  /**
   * Separate option for end users to extend the native elements list
   */
  #[cfg_attr(feature = "napi", napi(ts_type = "(arg: string) => boolean"))]
  pub is_custom_element: Option<Function<'static, &'static str, bool>>,
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
   * - `true`/`false`: a boolean to simply enable/disable HMR. When `true`, HMR
   *   is enabled with default configuration.
   * - `Hmr`: an object to enable HMR with custom configuration.
   * @default false
   */
  pub hmr: Option<Either<bool, Hmr>>,
  /**
   * Enabled SSR support.
   * @default false
   */
  pub ssr: Option<bool>,
  /**
   * Whether to enable compiler optimizations, including:
   * - **Slots**: Detect if slots are stable for more efficient updates.
   * - **Cache**: Cache event handler to avoid recreating closures on each render.
   * - **Block**: Enable block tree optimizations.
   *
   * Note: this option is only used in interop mode.
   * @default true
   */
  pub optimize: Option<bool>,
  /**
   * Customize where to import runtime helpers from vue-jsx-vapor.
   * If not specified, defaults to the virtual module path (e.g., `/vue-jsx-vapor/vapor`).
   */
  pub runtime_module_name: Option<String>,
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
  let options = options.unwrap_or_default();
  let filename = &options.filename.unwrap_or("index.jsx".to_string());
  let ssr = options.ssr.unwrap_or(false);
  let CodegenReturn { code, map, .. } = transform(
    &source,
    Some(TransformOptions {
      filename,
      source_map: options.source_map.unwrap_or(false),
      interop: options.interop.unwrap_or(false),
      hmr: options.hmr.unwrap_or(Either::A(false)),
      ssr,
      optimize: options.optimize.unwrap_or(true),
      runtime_module_name: options.runtime_module_name,
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
  *options.source_text.borrow_mut() = source;
  *options.source_type.borrow_mut() = {
    if let Some(ext) = Path::new(options.filename)
      .extension()
      .and_then(std::ffi::OsStr::to_str)
      && let Some(ext) = ext.split("?").next()
    {
      SourceType::from_extension(ext).unwrap()
    } else {
      SourceType::from_path(options.filename).unwrap()
    }
  };
  let mut program = Parser::new(
    unsafe { &*(&options.allocator as *const _) },
    source,
    *options.source_type.borrow(),
  )
  .parse()
  .program;
  let program_ptr = &program as *const _;
  *options.semantic.borrow_mut() = SemanticBuilder::new()
    .build(unsafe { &*program_ptr })
    .semantic;
  Transform::new(unsafe { &*(&options as *const _) }).visit(&mut program);
  Codegen::new()
    .with_options(CodegenOptions {
      source_map_path: if options.source_map {
        Some(PathBuf::from(&options.filename))
      } else {
        None
      },
      ..CodegenOptions::default()
    })
    .build(&program)
}
