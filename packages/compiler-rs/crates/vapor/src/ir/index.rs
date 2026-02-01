use common::directive::{DirectiveNode, Modifiers};
use indexmap::IndexSet;
use napi::Either;

use common::expression::SimpleExpressionNode;

use crate::ir::component::{IRProp, IRProps, IRSlots};

#[derive(Debug)]
pub struct BlockIRNode<'a> {
  pub dynamic: IRDynamicInfo<'a>,
  pub temp_id: i32,
  pub effect: Vec<IREffect<'a>>,
  pub operation: Vec<OperationNode<'a>>,
  pub returns: Vec<i32>,
  pub slots: Vec<IRSlots<'a>>,
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
      slots: Vec::new(),
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
pub struct RootIRNode {
  pub root_template_index: Option<usize>,
  pub component: IndexSet<String>,
  pub directive: IndexSet<String>,
  pub has_template_ref: bool,
  pub has_deferred_v_show: bool,
}
impl RootIRNode {
  pub fn new() -> Self {
    RootIRNode {
      component: IndexSet::new(),
      directive: IndexSet::new(),
      has_template_ref: false,
      root_template_index: None,
      has_deferred_v_show: false,
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
  pub append: bool,
  pub last: bool,
}

#[derive(Debug)]
pub struct KeyIRNode<'a> {
  pub id: i32,
  pub value: SimpleExpressionNode<'a>,
  pub block: BlockIRNode<'a>,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append: bool,
  pub last: bool,
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
  pub append: bool,
  pub last: bool,
}

#[derive(Debug)]
pub struct SetPropIRNode<'a> {
  pub set_prop: bool,
  pub element: i32,
  pub prop: IRProp<'a>,
  pub tag: String,
}

#[derive(Debug)]
pub struct SetDynamicPropsIRNode<'a> {
  pub set_dynamic_props: bool,
  pub element: i32,
  pub props: Vec<IRProps<'a>>,
  pub tag: String,
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
  pub generated: bool,
  pub is_component: bool,
}

#[derive(Debug)]
pub struct SetNodesIRNode<'a> {
  pub set_nodes: bool,
  pub element: i32,
  pub once: bool,
  pub values: Vec<SimpleExpressionNode<'a>>,
  pub generated: bool, // whether this is a generated empty text node by `processTextLikeContainer`
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
  pub is_component: bool,
}

#[derive(Debug)]
pub struct SetTemplateRefIRNode<'a> {
  pub set_template_ref: bool,
  pub element: i32,
  pub value: SimpleExpressionNode<'a>,
  pub ref_for: bool,
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
  pub builtin: bool,
  pub asset: bool,
  pub model_type: Option<String>,
  pub deferred: bool,
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
  pub is_custom_element: bool,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append: bool,
  pub last: bool,
}

#[derive(Debug)]
pub struct SlotOutletIRNode<'a> {
  pub id: i32,
  pub name: SimpleExpressionNode<'a>,
  pub props: Vec<IRProps<'a>>,
  pub fallback: Option<BlockIRNode<'a>>,
  pub no_slotted: bool,
  pub once: bool,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append: bool,
  pub last: bool,
}

#[derive(Debug)]
pub struct GetTextChildIRNode {
  pub get_text_child: bool,
  pub parent: i32,
}

#[derive(Debug)]
pub enum OperationNode<'a> {
  If(IfIRNode<'a>),
  For(ForIRNode<'a>),
  SetText(SetTextIRNode<'a>),
  SetProp(SetPropIRNode<'a>),
  SetDynamicProps(SetDynamicPropsIRNode<'a>),
  SetDynamicEvents(SetDynamicEventsIRNode<'a>),
  SetNodes(SetNodesIRNode<'a>),
  SetHtml(SetHtmlIRNode<'a>),
  SetEvent(SetEventIRNode<'a>),
  SetTemplateRef(SetTemplateRefIRNode<'a>),
  CreateNodes(CreateNodesIRNode<'a>),
  InsertNode(InsertNodeIRNode),
  Directive(DirectiveIRNode<'a>),
  CreateComponent(CreateComponentIRNode<'a>),
  SlotOutlet(SlotOutletIRNode<'a>),
  GetTextChild(GetTextChildIRNode),
  Key(KeyIRNode<'a>),
}

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
  pub has_dynamic_child: bool,
  pub operation: Option<Box<OperationNode<'a>>>,
}
impl<'a> IRDynamicInfo<'a> {
  pub fn new() -> Self {
    IRDynamicInfo {
      flags: DynamicFlag::Referenced as i32,
      children: Vec::new(),
      template: None,
      has_dynamic_child: false,
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
