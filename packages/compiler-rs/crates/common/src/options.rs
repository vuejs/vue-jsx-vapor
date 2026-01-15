use napi_derive::napi;
use std::{
  cell::RefCell,
  collections::{BTreeSet, HashMap},
};

use indexmap::IndexMap;
use napi::Either;
use oxc_ast::{AstKind, ast::Expression};
use oxc_span::{SourceType, Span};

use crate::error::ErrorCodes;

pub struct RootJsx<'a> {
  pub node: Expression<'a>,
  pub node_ref: *mut Expression<'a>,
  pub vdom: bool,
}

type OnExitProgram<'a> = Box<dyn Fn(Vec<RootJsx<'a>>, &'a str) + 'a>;
type OnEnterExpression<'a> =
  Box<dyn Fn(*mut Expression<'a>, &Vec<AstKind>) -> Option<(*mut Expression<'a>, bool)> + 'a>;

#[napi(object)]
pub struct Hmr {
  /**
   * The name of the function to be used for defining components.
   * This is useful when you have a custom defineComponent function.
   * @default ['defineComponent', 'defineVaporComponent']
   */
  pub define_component_name: Vec<String>,
}

pub struct TransformOptions<'a> {
  pub templates: RefCell<Vec<(String, bool)>>,
  pub helpers: RefCell<BTreeSet<String>>,
  pub delegates: RefCell<BTreeSet<String>>,
  pub hoists: RefCell<Vec<Expression<'a>>>,
  pub is_custom_element: Box<dyn Fn(String) -> bool + 'a>,
  pub on_error: Box<dyn Fn(ErrorCodes, Span) + 'a>,
  pub on_exit_program: RefCell<Option<OnExitProgram<'a>>>,
  pub on_enter_expression: RefCell<Option<OnEnterExpression<'a>>>,
  pub source_map: bool,
  pub filename: &'a str,
  pub source_type: SourceType,
  pub interop: bool,
  pub hmr: Either<bool, Hmr>,
  pub ssr: RefCell<bool>,
  pub in_v_for: RefCell<i32>,
  pub in_v_slot: RefCell<i32>,
  pub in_v_once: RefCell<bool>,
  pub identifiers: RefCell<HashMap<String, i32>>,
  pub slot_identifiers: RefCell<IndexMap<Span, (i32, Vec<String>)>>,
  pub cache_index: RefCell<i32>,
  /**
   * Indicates whether the compiler generates code for SSR,
   * it is always true when generating code for SSR,
   * regardless of whether we are generating code for SSR's fallback branch,
   * this means that when the compiler generates code for SSR's fallback branch:
   *  - context.ssr = false
   *  - context.inSSR = true
   */
  pub in_ssr: bool,
  pub optimize_slots: bool,
}

impl<'a> Default for TransformOptions<'a> {
  fn default() -> Self {
    Self {
      filename: "index.jsx",
      source_type: SourceType::jsx(),
      templates: RefCell::new(vec![]),
      helpers: RefCell::new(BTreeSet::new()),
      delegates: RefCell::new(BTreeSet::new()),
      hoists: RefCell::new(vec![]),
      source_map: false,
      is_custom_element: Box::new(|_| false),
      on_error: Box::new(|_, _| {}),
      interop: false,
      hmr: Either::A(false),
      ssr: RefCell::new(false),
      on_exit_program: RefCell::new(None),
      on_enter_expression: RefCell::new(None),
      in_v_for: RefCell::new(0),
      in_v_slot: RefCell::new(0),
      in_v_once: RefCell::new(false),
      identifiers: RefCell::new(HashMap::new()),
      slot_identifiers: RefCell::new(IndexMap::new()),
      cache_index: RefCell::new(0),
      in_ssr: false,
      optimize_slots: false,
    }
  }
}
