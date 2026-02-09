use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_semantic::Semantic;
use std::{
  cell::RefCell,
  collections::{BTreeSet, HashMap},
};

use indexmap::IndexMap;
use napi::Either;
use oxc_ast::ast::{CallExpression, Expression};
use oxc_span::{SourceType, Span};

use crate::error::ErrorCodes;

pub struct RootJsx<'a> {
  pub node: Expression<'a>,
  pub node_ref: *mut Expression<'a>,
  pub vdom: bool,
}

type OnExitProgram<'a> = Box<dyn Fn(Vec<RootJsx<'a>>) + 'a>;
type OnEnterExpression<'a> =
  Box<dyn Fn(*mut Expression<'a>) -> Option<(*mut Expression<'a>, bool)> + 'a>;
type OnLeaveExpression<'a> = Box<dyn Fn(&CallExpression) + 'a>;

#[napi(object)]
pub struct Hmr {
  /**
   * The name of the function to be used for defining components.
   * This is useful when you have a custom defineComponent function.
   * @default ['defineComponent', 'defineVaporComponent']
   */
  pub define_component_name: Vec<String>,
}

pub struct SlotScope {
  pub seen: i32,
  pub identifiers: Vec<String>,
  pub forwarded: bool,
}

pub struct TransformOptions<'a> {
  pub allocator: Allocator,
  pub semantic: RefCell<Semantic<'a>>,
  pub templates: RefCell<Vec<(String, bool, i32)>>,
  pub helpers: RefCell<BTreeSet<String>>,
  pub delegates: RefCell<BTreeSet<String>>,
  pub hoists: RefCell<Vec<Expression<'a>>>,
  pub is_custom_element: Box<dyn Fn(String) -> bool + 'a>,
  pub on_error: Box<dyn Fn(ErrorCodes, Span) + 'a>,
  pub on_exit_program: RefCell<Option<OnExitProgram<'a>>>,
  pub on_enter_expression: RefCell<Option<OnEnterExpression<'a>>>,
  pub on_leave_expression: RefCell<Option<OnLeaveExpression<'a>>>,
  pub source_map: bool,
  pub filename: &'a str,
  pub source_text: RefCell<&'a str>,
  pub source_type: SourceType,
  pub interop: bool,
  pub hmr: Either<bool, Hmr>,
  pub ssr: bool,
  pub in_v_for: RefCell<i32>,
  pub in_v_slot: RefCell<i32>,
  pub in_v_once: RefCell<bool>,
  pub in_vapor: RefCell<i32>,
  pub identifiers: RefCell<HashMap<String, i32>>,
  pub slot_scopes: RefCell<IndexMap<Span, SlotScope>>,
  pub cache_index: RefCell<i32>,
  pub optimize_slots: bool,
  pub runtime_module_name: Option<String>,
}

impl<'a> Default for TransformOptions<'a> {
  fn default() -> Self {
    Self {
      allocator: Allocator::default(),
      semantic: RefCell::new(Semantic::default()),
      source_text: RefCell::new(""),
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
      ssr: false,
      on_exit_program: RefCell::new(None),
      on_enter_expression: RefCell::new(None),
      on_leave_expression: RefCell::new(None),
      in_v_for: RefCell::new(0),
      in_v_slot: RefCell::new(0),
      in_v_once: RefCell::new(false),
      in_vapor: RefCell::new(0),
      identifiers: RefCell::new(HashMap::new()),
      slot_scopes: RefCell::new(IndexMap::new()),
      cache_index: RefCell::new(0),
      optimize_slots: false,
      runtime_module_name: None,
    }
  }
}
