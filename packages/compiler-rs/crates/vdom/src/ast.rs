use common::expression::jsx_attribute_value_to_expression;
use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{ArrayExpression, Expression, JSXAttribute, JSXChild, JSXElement};
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
  /// Some(true): v-for with single child
  /// Some(false):  v-for with multiple children
  /// None: not v-for
  pub v_for: Option<bool>,
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
    context: &TransformContext<'a>,
  ) -> Self {
    Self {
      condition: if dir.name.get_identifier().name == "v-else" {
        None
      } else {
        dir.value.as_mut().map(|value| {
          jsx_attribute_value_to_expression(value.take_in(context.allocator), context.allocator)
        })
      },
      node,
    }
  }
}

#[derive(Debug)]
pub struct TextCallNode<'a> {
  pub context_type: ConstantTypes,
  pub codegen_node: Expression<'a>,
}

#[derive(Debug)]
pub struct CacheExpression<'a> {
  pub index: usize,
  pub value: *mut Expression<'a>,
  pub need_pause_tracking: bool,
  pub in_v_once: bool,
  pub need_array_spread: bool,
}

impl<'a> CacheExpression<'a> {
  pub fn new(index: usize, value: *mut Expression<'a>, is_v_node: bool, in_v_once: bool) -> Self {
    Self {
      index,
      value,
      need_pause_tracking: is_v_node,
      in_v_once,
      need_array_spread: false,
    }
  }
}

pub struct ForNode<'a> {
  pub source: Option<Expression<'a>>,
  pub value: Option<Expression<'a>>,
  pub key: Option<Expression<'a>>,
  pub index: Option<Expression<'a>>,
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
