use std::{cell::RefCell, collections::BTreeSet};

use oxc_ast::ast::Expression;
use oxc_span::{SourceType, Span};
use oxc_traverse::TraverseCtx;

use crate::error::ErrorCodes;

pub struct RootJsx<'a> {
  pub node: Expression<'a>,
  pub node_ref: *mut Expression<'a>,
  pub vdom: bool,
}

type OnExitProgram<'a> = Box<dyn Fn(Vec<RootJsx<'a>>, &'a str) + 'a>;
type OnEnterExpression<'a> = Box<
  dyn Fn(*mut Expression<'a>, &mut TraverseCtx<'a, ()>) -> Option<(*mut Expression<'a>, bool)> + 'a,
>;

pub struct TransformOptions<'a> {
  pub templates: RefCell<Vec<(String, bool)>>,
  pub helpers: RefCell<BTreeSet<String>>,
  pub delegates: RefCell<BTreeSet<String>>,
  pub with_fallback: bool,
  pub is_custom_element: Box<dyn Fn(String) -> bool + 'a>,
  pub on_error: Box<dyn Fn(ErrorCodes, Span) + 'a>,
  pub on_exit_program: RefCell<Option<OnExitProgram<'a>>>,
  pub on_enter_expression: RefCell<Option<OnEnterExpression<'a>>>,
  pub source_map: bool,
  pub filename: &'a str,
  pub source_type: SourceType,
  pub interop: bool,
  pub hmr: bool,
  pub ssr: bool,
  pub in_v_for: RefCell<i32>,
  pub in_v_once: RefCell<bool>,
}

impl<'a> Default for TransformOptions<'a> {
  fn default() -> Self {
    Self {
      filename: "index.jsx",
      source_type: SourceType::jsx(),
      templates: RefCell::new(vec![]),
      helpers: RefCell::new(BTreeSet::new()),
      delegates: RefCell::new(BTreeSet::new()),
      source_map: false,
      with_fallback: false,
      is_custom_element: Box::new(|_| false),
      on_error: Box::new(|_, _| {}),
      interop: false,
      hmr: false,
      ssr: false,
      on_exit_program: RefCell::new(None),
      on_enter_expression: RefCell::new(None),
      in_v_for: RefCell::new(0),
      in_v_once: RefCell::new(false),
    }
  }
}
