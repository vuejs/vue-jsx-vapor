use std::collections::HashSet;

use napi::{Either, bindgen_prelude::Either16};
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::ast::{Expression, JSXChild};
use oxc_span::Span;

pub use crate::utils::expression::SimpleExpressionNode;

use crate::{
  ir::component::{IRProp, IRProps, IRSlots},
  utils::text::is_empty_text,
};

#[derive(Debug)]
pub struct RootNode<'a> {
  pub is_fragment: bool,
  pub is_single_root: bool,
  pub children: oxc_allocator::Vec<'a, JSXChild<'a>>,
}
impl<'a> RootNode<'a> {
  pub fn new(allocator: &'a Allocator) -> Self {
    RootNode {
      is_fragment: false,
      is_single_root: false,
      children: oxc_allocator::Vec::new_in(allocator),
    }
  }
  pub fn from(allocator: &'a Allocator, expression: Expression<'a>) -> Self {
    let mut is_fragment = false;
    let children = match expression {
      Expression::JSXFragment(mut node) => {
        is_fragment = true;
        node.children.take_in(allocator)
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
    RootNode {
      is_fragment,
      is_single_root,
      children,
    }
  }
}

#[derive(Debug)]
pub struct BlockIRNode<'a> {
  pub dynamic: IRDynamicInfo<'a>,
  pub temp_id: i32,
  pub effect: Vec<IREffect<'a>>,
  pub operation: Vec<OperationNode<'a>>,
  pub returns: Vec<i32>,
  pub props: Option<SimpleExpressionNode<'a>>,
}
impl<'a> BlockIRNode<'a> {
  pub fn new() -> Self {
    BlockIRNode {
      dynamic: IRDynamicInfo::new(),
      temp_id: 0,
      effect: Vec::new(),
      operation: Vec::new(),
      returns: Vec::new(),
      props: None,
    }
  }
}
impl<'a> Default for BlockIRNode<'a> {
  fn default() -> Self {
    BlockIRNode::new()
  }
}

#[derive(Debug, Default)]
pub struct RootIRNode<'a> {
  pub source: &'a str,
  pub root_template_index: Option<usize>,
  pub component: HashSet<String>,
  pub directive: HashSet<String>,
  pub block: BlockIRNode<'a>,
  pub has_template_ref: bool,
}
impl<'a> RootIRNode<'a> {
  pub fn new(source: &'a str) -> Self {
    RootIRNode {
      source,
      component: HashSet::new(),
      directive: HashSet::new(),
      block: BlockIRNode::new(),
      has_template_ref: false,
      root_template_index: None,
    }
  }
}

#[derive(Debug)]
pub struct IfIRNode<'a> {
  pub id: i32,
  pub condition: SimpleExpressionNode<'a>,
  pub positive: BlockIRNode<'a>,
  pub negative: Option<Box<Either<BlockIRNode<'a>, IfIRNode<'a>>>>,
  pub once: bool,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
}

#[derive(Debug)]
pub struct IRFor<'a> {
  pub source: Option<SimpleExpressionNode<'a>>,
  pub value: Option<SimpleExpressionNode<'a>>,
  pub key: Option<SimpleExpressionNode<'a>>,
  pub index: Option<SimpleExpressionNode<'a>>,
}

#[derive(Debug)]
pub struct ForIRNode<'a> {
  pub source: SimpleExpressionNode<'a>,
  pub value: Option<SimpleExpressionNode<'a>>,
  pub key: Option<SimpleExpressionNode<'a>>,
  pub index: Option<SimpleExpressionNode<'a>>,

  pub id: i32,
  pub key_prop: Option<SimpleExpressionNode<'a>>,
  pub render: BlockIRNode<'a>,
  pub once: bool,
  pub component: bool,
  pub only_child: bool,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
}

#[derive(Debug)]
pub struct SetPropIRNode<'a> {
  pub set_prop: bool,
  pub element: i32,
  pub prop: IRProp<'a>,
  pub root: bool,
  pub tag: String,
}

#[derive(Debug)]
pub struct SetDynamicPropsIRNode<'a> {
  pub set_dynamic_props: bool,
  pub element: i32,
  pub props: Vec<IRProps<'a>>,
  pub root: bool,
}

#[derive(Debug)]
pub struct SetDynamicEventsIRNode<'a> {
  pub set_dynamic_events: bool,
  pub element: i32,
  pub value: SimpleExpressionNode<'a>,
}

#[derive(Debug)]
pub struct SetTextIRNode<'a> {
  pub set_text: bool,
  pub element: i32,
  pub values: Vec<SimpleExpressionNode<'a>>,
  pub generated: Option<bool>,
}

#[derive(Debug)]
pub struct SetNodesIRNode<'a> {
  pub set_nodes: bool,
  pub element: i32,
  pub once: bool,
  pub values: Vec<SimpleExpressionNode<'a>>,
  pub generated: Option<bool>, // whether this is a generated empty text node by `processTextLikeContainer`
}

#[derive(Clone, Debug)]
pub struct Modifiers {
  // modifiers for addEventListener() options, e.g. .passive & .capture
  pub options: Vec<String>,
  // modifiers that needs runtime guards, withKeys
  pub keys: Vec<String>,
  // modifiers that needs runtime guards, withModifiers
  pub non_keys: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SetEventIRNode<'a> {
  pub set_event: bool,
  pub element: i32,
  pub key: SimpleExpressionNode<'a>,
  pub value: Option<SimpleExpressionNode<'a>>,
  pub modifiers: Modifiers,
  pub delegate: bool,
  // Whether it's in effect
  pub effect: bool,
}

#[derive(Debug)]
pub struct SetHtmlIRNode<'a> {
  pub set_html: bool,
  pub element: i32,
  pub value: SimpleExpressionNode<'a>,
}

#[derive(Debug)]
pub struct SetTemplateRefIRNode<'a> {
  pub set_template_ref: bool,
  pub element: i32,
  pub value: SimpleExpressionNode<'a>,
  pub ref_for: bool,
  pub effect: bool,
}

#[derive(Debug)]
pub struct CreateNodesIRNode<'a> {
  pub create_nodes: bool,
  pub id: i32,
  pub once: bool,
  pub values: Vec<SimpleExpressionNode<'a>>,
}

#[derive(Debug)]
pub struct InsertNodeIRNode {
  pub insert_node: bool,
  pub elements: Vec<i32>,
  pub parent: i32,
  pub anchor: Option<i32>,
}

#[derive(Debug)]
pub struct DirectiveIRNode<'a> {
  pub directive: bool,
  pub element: i32,
  pub dir: DirectiveNode<'a>,
  pub name: String,
  pub builtin: Option<bool>,
  pub asset: Option<bool>,
  pub model_type: Option<String>,
}

#[derive(Debug)]
pub struct CreateComponentIRNode<'a> {
  pub create_component: bool,
  pub id: i32,
  pub tag: String,
  pub props: Vec<IRProps<'a>>,
  pub slots: Vec<IRSlots<'a>>,
  pub asset: bool,
  pub root: bool,
  pub once: bool,
  pub dynamic: Option<SimpleExpressionNode<'a>>,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
}

#[derive(Debug)]
pub struct DeclareOldRefIRNode {
  pub declare_older_ref: bool,
  pub id: i32,
}

#[derive(Debug)]
pub struct GetTextChildIRNode {
  pub get_text_child: bool,
  pub parent: i32,
}

pub type OperationNode<'a> = Either16<
  IfIRNode<'a>,
  ForIRNode<'a>,
  SetTextIRNode<'a>,
  SetPropIRNode<'a>,
  SetDynamicPropsIRNode<'a>,
  SetDynamicEventsIRNode<'a>,
  SetNodesIRNode<'a>,
  SetEventIRNode<'a>,
  SetHtmlIRNode<'a>,
  SetTemplateRefIRNode<'a>,
  CreateNodesIRNode<'a>,
  InsertNodeIRNode,
  DirectiveIRNode<'a>,
  CreateComponentIRNode<'a>,
  DeclareOldRefIRNode,
  GetTextChildIRNode,
>;

pub enum DynamicFlag {
  None = 0,
  // This node is referenced and needs to be saved as a variable.
  Referenced = 1 << 0,
  // This node is not generated from template, but is generated dynamically.
  NonTemplate = 1 << 1,
  // const REFERENCED_AND_NON_TEMPLATE = 3;
  // This node needs to be inserted back into the template.
  Insert = 1 << 2,
  // REFERENCED_AND_INSERT = 5,
  // NONE_TEMPLAET_AND_INSERT = 6,
  // REFERENCED_AND_NON_TEMPLATE_AND_INSERT = 7,
}

#[derive(Debug)]
pub struct IRDynamicInfo<'a> {
  pub id: Option<i32>,
  pub flags: i32,
  pub anchor: Option<i32>,
  pub children: Vec<IRDynamicInfo<'a>>,
  pub template: Option<i32>,
  pub has_dynamic_child: Option<bool>,
  pub operation: Option<Box<OperationNode<'a>>>,
}
impl<'a> IRDynamicInfo<'a> {
  pub fn new() -> Self {
    IRDynamicInfo {
      flags: DynamicFlag::Referenced as i32,
      children: Vec::new(),
      template: None,
      has_dynamic_child: None,
      operation: None,
      id: None,
      anchor: None,
    }
  }
}
impl<'a> Default for IRDynamicInfo<'a> {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug)]
pub struct IREffect<'a> {
  pub expressions: Vec<SimpleExpressionNode<'a>>,
  pub operations: Vec<OperationNode<'a>>,
}

#[derive(Debug)]
pub struct DirectiveNode<'a> {
  // the normalized name without prefix or shorthands, e.g. "bind", "on"
  pub name: String,
  pub exp: Option<SimpleExpressionNode<'a>>,
  pub arg: Option<SimpleExpressionNode<'a>>,
  pub modifiers: Vec<SimpleExpressionNode<'a>>,
  pub loc: Span,
}
