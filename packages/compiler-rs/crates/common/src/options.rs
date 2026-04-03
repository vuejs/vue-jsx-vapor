use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_semantic::Semantic;
use std::{
  cell::RefCell,
  collections::{BTreeSet, HashMap},
};

use indexmap::IndexMap;
use napi::Either;
use oxc_ast::ast::Expression;
use oxc_span::{SourceType, Span};

use crate::error::ErrorCodes;

pub struct RootJsx<'a> {
  pub node_ptr: *mut Expression<'a>,
  pub expression: Expression<'a>,
}

type OnEnterExpression<'a> =
  Box<dyn Fn(*mut Expression<'a>) -> Option<(*mut Expression<'a>, bool)> + 'a>;
type OnLeaveExpression<'a> = Box<dyn Fn(&Expression) + 'a>;
type CreateRootJSX<'a> = Box<dyn Fn(*mut Expression<'a>, bool) -> RootJsx<'a> + 'a>;

#[napi(object)]
pub struct Hmr {
  /**
   * The name of the function to be used for defining components.
   * This is useful when you have a custom defineComponent function.
   * @default ['defineComponent', 'defineVaporComponent']
   */
  pub define_component_name: Vec<String>,
}

#[derive(Debug)]
pub struct SlotScope<'a> {
  pub dynamic: bool,
  pub forwarded: bool,
  pub identifiers: Vec<&'a str>,
}

pub struct TransformOptions<'a> {
  pub allocator: Allocator,
  pub semantic: RefCell<Semantic<'a>>,
  pub templates: RefCell<Vec<(String, bool, i32)>>,
  pub helpers: RefCell<BTreeSet<&'a str>>,
  pub delegates: RefCell<BTreeSet<&'a str>>,
  pub hoists: RefCell<Vec<Expression<'a>>>,
  pub on_error: Box<dyn Fn(ErrorCodes, Span) + 'a>,
  pub create_root_jsx: RefCell<Option<CreateRootJSX<'a>>>,
  pub on_enter_expression: RefCell<Option<OnEnterExpression<'a>>>,
  pub on_leave_expression: RefCell<Option<OnLeaveExpression<'a>>>,
  pub source_map: bool,
  pub filename: &'a str,
  pub source_text: RefCell<&'a str>,
  pub source_type: RefCell<SourceType>,
  pub interop: bool,
  pub hmr: Either<bool, Hmr>,
  pub ssr: bool,
  pub in_v_for: RefCell<i32>,
  pub in_v_slot: RefCell<i32>,
  pub in_v_once: RefCell<bool>,
  pub in_vapor: RefCell<i32>,
  pub identifiers: RefCell<HashMap<&'a str, i32>>,
  pub slot_scopes: RefCell<IndexMap<Span, SlotScope<'a>>>,
  pub cache_index: RefCell<i32>,
  pub optimize: bool,
  pub runtime_module_name: Option<String>,
  pub scope_identifiers_map: RefCell<HashMap<Span, (bool, Vec<&'a str>)>>,
}

impl<'a> Default for TransformOptions<'a> {
  fn default() -> Self {
    Self {
      allocator: Allocator::default(),
      semantic: RefCell::new(Semantic::default()),
      source_text: RefCell::new(""),
      filename: "index.jsx",
      source_type: RefCell::new(SourceType::jsx()),
      templates: RefCell::new(vec![]),
      helpers: RefCell::new(BTreeSet::new()),
      delegates: RefCell::new(BTreeSet::new()),
      hoists: RefCell::new(vec![]),
      source_map: false,
      on_error: Box::new(|_, _| {}),
      interop: false,
      hmr: Either::A(false),
      ssr: false,
      create_root_jsx: RefCell::new(None),
      on_enter_expression: RefCell::new(None),
      on_leave_expression: RefCell::new(None),
      in_v_for: RefCell::new(0),
      in_v_slot: RefCell::new(0),
      in_v_once: RefCell::new(false),
      in_vapor: RefCell::new(0),
      identifiers: RefCell::new(HashMap::new()),
      slot_scopes: RefCell::new(IndexMap::new()),
      cache_index: RefCell::new(0),
      optimize: true,
      runtime_module_name: None,
      scope_identifiers_map: RefCell::new(HashMap::new()),
    }
  }
}

impl<'a> TransformOptions<'a> {
  pub fn helper(&self, name: &'a str) -> &'a str {
    self.helpers.borrow_mut().insert(&name[1..]);
    name
  }
  pub fn remove_identifiers(&self, ids: Vec<&'a str>) {
    let identifiers = &mut self.identifiers.borrow_mut();
    for id in ids {
      if let Some(v) = identifiers.get_mut(&id)
        && *v > 1
      {
        *v -= 1;
      } else {
        identifiers.remove(&id);
      }
    }
  }
}
