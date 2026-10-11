pub use common::options::TransformOptions;
use napi::{
  Either, Env,
  bindgen_prelude::{Function, Object},
};
use napi_derive::napi;
use oxc_codegen::{Codegen, CodegenReturn};
use oxc_diagnostics::OxcDiagnostic;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{SourceType, Span};
use std::path::{Path, PathBuf};

use common::{
  error::{create_compiler_error, create_compiler_warning},
  options::Hmr,
};

use crate::transform::Transform;

mod hmr_or_ssr;
mod transform;

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default)]
pub struct CompilerOptions {
  pub on_error: Option<Function<'static, Object<'static>, ()>>,
  pub on_warn: Option<Function<'static, Object<'static>, ()>>,
  /// Generate source map?
  /// @default false
  pub source_map: Option<bool>,
  /// Filename for source map generation.
  /// Also used for self-recursive reference in templates
  /// @default 'index.jsx'
  pub filename: Option<String>,
  /// When enabled, JSX within `defineVaporComponent` is transformed to Vapor DOM,
  /// while all other JSX is transformed to Virtual DOM.
  pub interop: Option<bool>,
  /// Enabled HMR support.
  /// - `true`/`false`: a boolean to simply enable/disable HMR. When `true`, HMR
  ///   is enabled with default configuration.
  /// - `Hmr`: an object to enable HMR with custom configuration.
  /// @default false
  pub hmr: Option<Either<bool, Hmr>>,
  /// Enabled SSR support.
  /// @default false
  pub ssr: Option<bool>,
  /// Whether to enable compiler optimizations, including:
  /// - **Slots**: Detect if slots are stable for more efficient updates.
  /// - **Cache**: Cache event handler to avoid recreating closures on each render.
  /// - **Block**: Enable block tree optimizations.
  ///
  /// Note: this option is only used in interop mode.
  /// @default true
  pub optimize: Option<bool>,
  /// Customize where to import runtime helpers from vue-jsx-vapor.
  /// If not specified, defaults to the virtual module path (e.g., `/vue-jsx-vapor/vapor`).
  pub runtime_module_name: Option<String>,
  /// Whether to merge props when using JSXSpreadAttribute.
  /// - `true`: Use Vue's `mergeProps` behavior
  ///   - Event listeners (`onXxx`) are merged into an array
  ///   - `class` and `style` are normalized and merged
  ///   - Other props are overridden by later values
  /// - `false`: later props override earlier ones (like object spread)
  /// @default true
  pub merge_props: Option<bool>,
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
  let source = source.as_str();
  let ssr = options.ssr.unwrap_or(false);
  let CodegenReturn { code, map, .. } = transform(
    source,
    Some(TransformOptions {
      filename,
      source_map: options.source_map.unwrap_or(false),
      interop: options.interop.unwrap_or(false),
      hmr: options.hmr.unwrap_or(Either::A(false)),
      ssr,
      optimize: options.optimize.unwrap_or(true),
      runtime_module_name: options.runtime_module_name,
      merge_props: options.merge_props.unwrap_or(true),
      on_error: if let Some(on_error) = options.on_error {
        Box::new(move |message: String, span: Span| {
          let compiler_error = create_compiler_error(
            &env,
            message,
            filename,
            Some(line_column(source, span.start)),
          )
          .unwrap();
          on_error.call(compiler_error).unwrap();
        }) as Box<dyn Fn(String, Span)>
      } else {
        Box::new(|_: String, _: Span| {}) as Box<dyn Fn(String, Span)>
      },
      on_warn: if let Some(on_warn) = options.on_warn {
        Box::new(move |message: String, span: Span| {
          let warning = create_compiler_warning(
            &env,
            message,
            filename,
            Some(line_column(source, span.start)),
          )
          .unwrap();
          on_warn.call(warning).unwrap();
        }) as Box<dyn Fn(String, Span)>
      } else {
        Box::new(|_: String, _: Span| {}) as Box<dyn Fn(String, Span)>
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
  let parser_return = Parser::new(
    unsafe { &*(&options.allocator as *const _) },
    source,
    *options.source_type.borrow(),
  )
  .parse();
  for error in &parser_return.errors {
    options.on_error.as_ref()(error.message.to_string(), diagnostic_span(error));
  }
  let mut program = parser_return.program;
  if !parser_return.panicked {
    let program_ptr = &program as *const _;
    let semantic_return = SemanticBuilder::new()
      .with_check_syntax_error(true)
      .build(unsafe { &*program_ptr });
    for error in &semantic_return.errors {
      options.on_error.as_ref()(error.message.to_string(), diagnostic_span(error));
    }
    *options.semantic.borrow_mut() = semantic_return.semantic;
    Transform::new(unsafe { &*(&options as *const _) }).visit(&mut program);
  }
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

/// convert a byte offset into a 1-based line and a 0-based UTF-16 column,
/// which is what bundlers expect in error locations
fn line_column(source: &str, offset: u32) -> (u32, u32) {
  let mut offset = offset as usize;
  while offset > 0 && !source.is_char_boundary(offset) {
    offset -= 1;
  }
  let prefix = &source[..offset];
  let line = prefix.matches('\n').count() as u32 + 1;
  let column = match prefix.rfind('\n') {
    Some(i) => &prefix[i + 1..],
    None => prefix,
  }
  .chars()
  .map(|c| c.len_utf16() as u32)
  .sum();
  (line, column)
}

fn diagnostic_span(diagnostic: &OxcDiagnostic) -> Span {
  diagnostic
    .labels
    .as_ref()
    .and_then(|labels| labels.first())
    .map(|label| Span::new(label.offset() as u32, (label.offset() + label.len()) as u32))
    .unwrap_or_default()
}
