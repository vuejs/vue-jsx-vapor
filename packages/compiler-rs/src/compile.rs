use std::cell::RefCell;
use std::{collections::BTreeSet, path::PathBuf};

use crate::transform::{TransformContext, TransformOptions};

use napi::{
  Env,
  bindgen_prelude::{Function, Object},
};
use napi_derive::napi;
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::ast::{ExpressionStatement, Program, Statement};
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn, IndentChar};
use oxc_parser::{ParseOptions, Parser};
use oxc_span::{SPAN, SourceType};

#[cfg_attr(feature = "napi", napi)]
pub type Template = (String, bool);

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
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug)]
pub struct CompileCodegenResult {
  pub helpers: BTreeSet<String>,
  pub templates: Vec<Template>,
  pub delegates: BTreeSet<String>,
  pub code: String,
}

#[cfg(feature = "napi")]
#[napi]
pub fn _compile(
  env: Env,
  source: String,
  options: Option<CompilerOptions>,
) -> CompileCodegenResult {
  use crate::utils::error::ErrorCodes;
  let options = options.unwrap_or_default();
  let filename = &options.filename.unwrap_or("index.jsx".to_string());
  compile(
    &source,
    Some(TransformOptions {
      filename,
      source_type: SourceType::from_path(filename).unwrap(),
      templates: RefCell::new(vec![]),
      helpers: RefCell::new(BTreeSet::new()),
      delegates: RefCell::new(BTreeSet::new()),
      source_map: options.source_map.unwrap_or(false),
      with_fallback: options.with_fallback.unwrap_or(false),
      interop: options.interop.unwrap_or(false),
      hmr: options.hmr.unwrap_or(false),
      ssr: options.ssr.unwrap_or(false),
      is_custom_element: if let Some(is_custom_element) = options.is_custom_element {
        Box::new(move |tag: String| is_custom_element.call(tag).unwrap())
          as Box<dyn Fn(String) -> bool>
      } else {
        Box::new(|_: String| false) as Box<dyn Fn(String) -> bool>
      },
      on_error: if let Some(on_error) = options.on_error {
        use oxc_span::Span;

        use crate::utils::error::create_compiler_error;

        Box::new(move |code: ErrorCodes, span: Span| {
          let compiler_error = create_compiler_error(&env, code, span).unwrap();
          on_error.call(compiler_error).unwrap();
        }) as Box<dyn Fn(ErrorCodes, Span)>
      } else {
        use oxc_span::Span;

        Box::new(|_: ErrorCodes, _: Span| {}) as Box<dyn Fn(ErrorCodes, Span)>
      },
    }),
  )
}

pub fn compile(source: &str, options: Option<TransformOptions>) -> CompileCodegenResult {
  let options = options.unwrap_or_default();
  let allocator = Allocator::default();
  let mut root = Parser::new(&allocator, source, options.source_type)
    .with_options(ParseOptions {
      parse_regular_expression: true,
      ..ParseOptions::default()
    })
    .parse();
  let Statement::ExpressionStatement(stmt) = root.program.body.get_mut(0).unwrap() else {
    panic!("Expected ExpressionStatement");
  };

  let filename = options.filename;
  let source_map = options.source_map;

  let context = TransformContext::new(&allocator, &options);
  let expression = context.transform(stmt.expression.take_in(&allocator), source);
  let program = Program {
    span: SPAN,
    source_text: source,
    comments: oxc_allocator::Vec::new_in(&allocator),
    hashbang: None,
    directives: oxc_allocator::Vec::new_in(&allocator),
    body: oxc_allocator::Vec::from_array_in(
      [Statement::ExpressionStatement(oxc_allocator::Box::new_in(
        ExpressionStatement {
          span: SPAN,
          expression,
        },
        &allocator,
      ))],
      &allocator,
    ),
    scope_id: Default::default(),
    source_type: options.source_type,
  };
  let CodegenReturn { code, .. } = Codegen::new()
    .with_options(CodegenOptions {
      source_map_path: if source_map {
        Some(PathBuf::from(filename))
      } else {
        None
      },
      indent_width: 2,
      indent_char: IndentChar::Space,
      ..CodegenOptions::default()
    })
    .build(&program);

  CompileCodegenResult {
    code,
    delegates: context.options.delegates.take(),
    helpers: context.options.helpers.take(),
    templates: context.options.templates.take(),
  }
}
