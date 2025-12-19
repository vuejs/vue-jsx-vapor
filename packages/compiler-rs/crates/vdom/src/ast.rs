use common::text::is_empty_text;
use napi::bindgen_prelude::Either3;
use oxc_allocator::{Allocator, FromIn, TakeIn};
use oxc_ast::ast::{
  ArrayExpression, Expression, JSXAttribute, JSXChild, JSXClosingFragment, JSXFragment,
  JSXOpeningFragment,
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::transform::TransformContext;

#[derive(Debug)]
pub struct RootNode;
impl<'a> RootNode {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(allocator: &'a Allocator) -> JSXChild<'a> {
    JSXChild::Fragment(oxc_allocator::Box::new_in(
      JSXFragment::from_in(
        JSXFragment {
          span: Span::new(0, 1),
          opening_fragment: JSXOpeningFragment::from_in(
            JSXOpeningFragment { span: SPAN },
            allocator,
          ),
          children: oxc_allocator::Vec::new_in(allocator),
          closing_fragment: JSXClosingFragment::from_in(
            JSXClosingFragment { span: SPAN },
            allocator,
          ),
        },
        allocator,
      ),
      allocator,
    ))
  }
  pub fn from(allocator: &'a Allocator, expression: Expression<'a>) -> JSXChild<'a> {
    let mut is_fragment = false;
    let children = match expression {
      Expression::JSXFragment(node) => {
        is_fragment = true;
        oxc_allocator::Vec::from_array_in([JSXChild::Fragment(node)], allocator)
      }
      Expression::JSXElement(mut node) => oxc_allocator::Vec::from_array_in(
        [JSXChild::Element(oxc_allocator::Box::new_in(
          node.take_in(allocator),
          allocator,
        ))],
        allocator,
      ),
      _ => oxc_allocator::Vec::new_in(allocator),
    };

    let mut is_single_root = false;
    if !is_fragment {
      for child in children.iter() {
        if !is_empty_text(child) {
          if is_single_root {
            is_single_root = false;
            break;
          }
          is_single_root = true;
        }
      }
    }
    JSXChild::Fragment(oxc_allocator::Box::new_in(
      JSXFragment::from_in(
        JSXFragment {
          span: Span::new(
            if is_fragment {
              2
            } else if is_single_root {
              3
            } else {
              1
            },
            0,
          ),
          opening_fragment: JSXOpeningFragment::from_in(
            JSXOpeningFragment { span: SPAN },
            allocator,
          ),
          children,
          closing_fragment: JSXClosingFragment::from_in(
            JSXClosingFragment { span: SPAN },
            allocator,
          ),
        },
        allocator,
      ),
      allocator,
    ))
  }
  pub fn is_root(node: &JSXChild<'a>) -> bool {
    let span = node.span();
    if span.end == 0 {
      let offset = span.start - span.end;
      offset > 0 && offset < 4
    } else {
      false
    }
  }
  pub fn is_fragment(node: &JSXChild<'a>) -> bool {
    let span = node.span();
    if span.end == 0 {
      span.start - span.end == 2
    } else {
      false
    }
  }
  pub fn is_single_root(node: &JSXChild<'a>) -> bool {
    let span = node.span();
    if span.end == 0 {
      span.start - span.end == 3
    } else {
      false
    }
  }
}

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
  pub v_for: bool,
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
