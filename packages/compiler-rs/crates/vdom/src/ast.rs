use napi::bindgen_prelude::Either3;
use oxc_ast::ast::{ArrayExpression, Expression, JSXAttribute, JSXChild};
use oxc_span::Span;

use crate::transform::TransformContext;

#[derive(Debug)]
pub enum NodeTypes<'a> {
  VNodeCall(VNodeCall<'a>),
  TextCallNode(Expression<'a>),
  CacheExpression(Expression<'a>),
}

pub type VNodeCallChildren<'a> =
  Either3<*mut JSXChild<'a>, *mut oxc_allocator::Vec<'a, JSXChild<'a>>, Expression<'a>>;

#[derive(Debug, Default)]
pub struct VNodeCall<'a> {
  pub tag: String,
  pub props: Option<Expression<'a>>,
  pub children: Option<VNodeCallChildren<'a>>,
  pub patch_flag: Option<i32>,
  pub dynamic_props: Option<Expression<'a>>,
  pub directives: Option<ArrayExpression<'a>>,
  pub is_block: bool,
  pub disable_tracking: bool,
  pub is_component: bool,
  pub v_for: Option<Vec<String>>,
  pub v_if: Option<Vec<IfBranchNode<'a>>>,
  pub loc: Span,
}

#[derive(Debug)]
pub struct IfBranchNode<'a> {
  pub condition: Option<Expression<'a>>,
  pub node: *mut JSXChild<'a>,
}

impl<'a> IfBranchNode<'a> {
  pub fn new(
    node: &mut JSXChild<'a>,
    dir: &mut JSXAttribute<'a>,
    context: &'a TransformContext<'a>,
  ) -> Self {
    Self {
      condition: if dir.name.get_identifier().name == "v-else" {
        None
      } else {
        dir
          .value
          .as_mut()
          .map(|value| context.jsx_attribute_value_to_expression(value))
      },
      node,
    }
  }
}

pub struct ForNode<'a> {
  pub source: Option<Expression<'a>>,
  pub value: Option<Expression<'a>>,
  pub key: Option<Expression<'a>>,
  pub index: Option<Expression<'a>>,
  pub identifiers: Vec<String>,
}

/**
 * Static types have several levels.
 * Higher levels implies lower levels. e.g. a node that can be stringified
 * can always be hoisted and skipped for patch.
 */
#[derive(Clone, Debug)]
pub enum ConstantTypes {
  NotConstant,
  CanSkipPatch,
  CanCache,
  CanStringify,
}

pub fn get_vnode_helper(ssr: bool, is_component: bool) -> String {
  String::from(if ssr || is_component {
    "createVNode"
  } else {
    "createElementVNode"
  })
}

pub fn get_vnode_block_helper(ssr: bool, is_component: bool) -> String {
  String::from(if ssr || is_component {
    "createBlock"
  } else {
    "createElementBlock"
  })
}
