use std::borrow::Cow;

use common::directive::{DirectiveNode, Modifiers};
use indexmap::IndexSet;
use napi::Either;

use oxc_ast::ast::Expression;
use oxc_span::Span;

use crate::ir::component::{IRProp, IRProps, IRSlots};

#[derive(Debug)]
pub struct BlockIRNode<'a> {
  pub dynamic: IRDynamicInfo<'a>,
  pub temp_id: i32,
  pub effect: Vec<IREffect<'a>>,
  pub operation: Vec<OperationNode<'a>>,
  pub returns: Vec<i32>,
  pub slots: Vec<IRSlots<'a>>,
  pub props: Option<Expression<'a>>,
  pub root: bool,
  /// Whether the block's owner has only text/interpolation children (such a
  /// block cannot be a single root when hydrating).
  pub node_text_only: bool,
  /// Elements carrying a `<select>` value binding: their value ops/effects
  /// are deferred after the children, since a select's value only takes
  /// effect once its options exist (#1318). Filled while transforming, read
  /// by `TransformContext::finalize_block`.
  pub select_value_elements: Vec<i32>,
  /// The operations / effects from these indexes on run after every render
  /// effect: the block's v-model directives and the listeners and select
  /// values that depend on them, see `TransformContext::finalize_block`.
  pub deferred_operation_index: usize,
  pub deferred_effect_index: usize,
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
      root: false,
      node_text_only: false,
      select_value_elements: Vec::new(),
      deferred_operation_index: 0,
      deferred_effect_index: 0,
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
  pub components: IndexSet<&'a str>,
  pub directives: IndexSet<&'a str>,
  pub has_template_ref: bool,
}

#[derive(Debug)]
pub struct IfIRNode<'a> {
  pub id: i32,
  pub block_shape: i32,
  pub condition: Expression<'a>,
  pub positive: BlockIRNode<'a>,
  pub negative: Option<Box<Either<BlockIRNode<'a>, IfIRNode<'a>>>>,
  pub once: bool,
  pub slot_root: bool,
  pub index: i32,

  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append_index: Option<i32>,

  pub operation_index: Option<usize>,
  pub effect_index: Option<u64>,
}

#[derive(Debug)]
pub struct KeyIRNode<'a> {
  pub id: i32,
  pub value: Expression<'a>,
  pub block: BlockIRNode<'a>,
  pub slot_root: bool,

  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append_index: Option<i32>,

  pub operation_index: Option<usize>,
  pub effect_index: Option<u64>,
}

#[derive(Debug)]
pub struct IRFor<'a> {
  pub source: Option<Expression<'a>>,
  pub value: Option<Expression<'a>>,
  pub key: Option<Expression<'a>>,
  pub index: Option<Expression<'a>>,
}

#[derive(Debug)]
pub struct ForIRNode<'a> {
  pub source: Expression<'a>,
  pub value: Option<Expression<'a>>,
  pub key: Option<Expression<'a>>,
  pub index: Option<Expression<'a>>,

  pub id: i32,
  pub key_prop: Option<Expression<'a>>,
  pub render: BlockIRNode<'a>,
  pub once: bool,
  pub slot_root: bool,
  pub component: bool,
  pub only_child: bool,
  pub wrapped_rows: bool,
  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append_index: Option<i32>,

  pub operation_index: Option<usize>,
  pub effect_index: Option<u64>,
}

#[derive(Debug)]
pub struct SetPropIRNode<'a> {
  pub set_prop: bool,
  pub element: i32,
  pub prop: IRProp<'a>,
  pub tag: &'a str,
  pub is_svg: bool,
}

#[derive(Debug)]
pub struct SetDynamicPropsIRNode<'a> {
  pub set_dynamic_props: bool,
  pub element: i32,
  pub props: Vec<IRProps<'a>>,
  pub is_svg: bool,
}

#[derive(Debug)]
pub struct SetDynamicEventsIRNode<'a> {
  pub set_dynamic_events: bool,
  pub element: i32,
  pub value: Expression<'a>,
}

#[derive(Debug)]
pub struct SetTextIRNode<'a> {
  pub element: i32,
  pub values: Vec<Expression<'a>>,
  pub generated: bool,
}

#[derive(Debug)]
pub struct SetNodesIRNode<'a> {
  pub set_nodes: bool,
  pub element: i32,
  pub once: bool,
  pub values: Vec<Expression<'a>>,
  pub generated: bool, // whether this is a generated empty text node by `processTextLikeContainer`
}

#[derive(Debug)]
pub struct SetEventIRNode<'a> {
  pub set_event: bool,
  pub element: i32,
  pub key: Expression<'a>,
  pub value: Expression<'a>,
  pub modifiers: Modifiers<'a>,
  pub delegate: bool,
  // Whether it's in effect
  pub effect: bool,
}

#[derive(Debug)]
pub struct SetHtmlIRNode<'a> {
  pub set_html: bool,
  pub element: i32,
  pub value: Expression<'a>,
}

#[derive(Debug)]
pub struct SetTemplateRefIRNode<'a> {
  pub set_template_ref: bool,
  pub element: i32,
  pub value: Expression<'a>,
  pub ref_for: bool,
}

#[derive(Debug)]
pub struct CreateNodesIRNode<'a> {
  pub create_nodes: bool,
  pub id: i32,
  pub once: bool,
  pub values: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct InsertNodeIRNode {
  pub elements: Vec<i32>,
  pub parent: i32,
  pub anchor: Option<i32>,
}

#[derive(Debug)]
pub struct DirectiveIRNode<'a> {
  pub directive: bool,
  pub element: i32,
  pub dir: DirectiveNode<'a>,
  pub name: Cow<'a, str>,
  pub builtin: bool,
  pub asset: bool,
  pub model_type: Option<&'a str>,
  // The helper creates its own effects, so a v-once site runs it in the once
  // ambient instead of eliding a compiler effect.
  pub once: bool,
}

#[derive(Debug)]
pub struct CreateComponentIRNode<'a> {
  pub create_component: bool,
  pub id: i32,
  pub tag: &'a str,
  pub tag_span: Span,
  pub props: Vec<IRProps<'a>>,
  pub slots: Vec<IRSlots<'a>>,
  pub asset: bool,
  // `isSingleRoot` of the creation call: 1 = the single root of a component
  // template, 2 = `VaporSingleRoot::SLOT`, the single root of a slot body
  pub root: i32,
  pub once: bool,
  // created by `createPlainElement` rather than `createComponent`
  pub use_create_element: bool,
  // namespace of the surrounding template (0 = HTML, 1 = SVG, 2 = MathML),
  // used when the component falls back to a plain element at runtime
  pub ns: i32,

  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append_index: Option<i32>,

  pub operation_index: Option<usize>,
  pub effect_index: Option<u64>,
}

#[derive(Debug)]
pub struct SlotOutletIRNode<'a> {
  pub id: i32,
  pub name: Expression<'a>,
  pub props: Vec<IRProps<'a>>,
  pub fallback: Option<BlockIRNode<'a>>,
  pub flags: i32,

  pub parent: Option<i32>,
  pub anchor: Option<i32>,
  pub append_index: Option<i32>,

  pub operation_index: Option<usize>,
  pub effect_index: Option<u64>,
}

#[derive(Debug)]
pub struct GetTextChildIRNode {
  pub get_text_child: bool,
  pub parent: i32,
}

#[derive(Debug)]
pub struct SetBlockKeyIRNode<'a> {
  pub element: i32,
  pub value: Expression<'a>,
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
  SetBlockKey(SetBlockKeyIRNode<'a>),
}

impl<'a> OperationNode<'a> {
  /// Where the operation's own block content starts among the operations and
  /// effects of the enclosing block. Until `finalize_block` settles them, the
  /// effect boundary is the registration order key of the operation's node
  /// rather than an index.
  pub fn block_boundary(&mut self) -> Option<(&mut Option<usize>, &mut Option<u64>)> {
    match self {
      OperationNode::If(operation) => {
        Some((&mut operation.operation_index, &mut operation.effect_index))
      }
      OperationNode::For(operation) => {
        Some((&mut operation.operation_index, &mut operation.effect_index))
      }
      OperationNode::Key(operation) => {
        Some((&mut operation.operation_index, &mut operation.effect_index))
      }
      OperationNode::CreateComponent(operation) => {
        Some((&mut operation.operation_index, &mut operation.effect_index))
      }
      OperationNode::SlotOutlet(operation) => {
        Some((&mut operation.operation_index, &mut operation.effect_index))
      }
      _ => None,
    }
  }
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

/// How the parent reaches a child codegen holds on to. Decided once the
/// parent's children are final (`plan_access`), printed by `gen_children`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicAccess {
  /// `const _n<id> = ...`
  Reference,
  /// `let _p<n> = ...`, a cursor later lookups may reassign
  Placeholder,
  /// `_p<n> = ...`, reassigning the previous cursor
  Reuse,
  /// no variable: written into the lookup of its only descendant access
  Inline,
  /// `(_p<n> = ...)` written inline, reassigning the previous cursor
  InlineReuse,
}

#[derive(Debug, Clone, Copy)]
pub struct IRDynamicAccess {
  pub kind: DynamicAccess,
  /// position among the parent's rendered child nodes
  pub index: i32,
  /// reached with `_next()` from the previous access instead of the parent
  pub adjacent: bool,
  /// the located node, for `Reference`
  pub id: Option<i32>,
}

#[derive(Debug)]
pub struct IRDynamicInfo<'a> {
  pub id: Option<i32>,
  pub flags: i32,
  pub anchor: Option<i32>,
  pub children: Vec<IRDynamicInfo<'a>>,
  pub template: Option<i32>,
  pub is_text: bool,
  pub has_dynamic_child: bool,
  pub operation: Option<Box<OperationNode<'a>>>,
  pub access: Option<IRDynamicAccess>,
}
impl<'a> IRDynamicInfo<'a> {
  pub fn new() -> Self {
    IRDynamicInfo {
      flags: DynamicFlag::Referenced as i32,
      children: Vec::new(),
      template: None,
      is_text: false,
      has_dynamic_child: false,
      operation: None,
      id: None,
      anchor: None,
      access: None,
    }
  }
}
impl<'a> Default for IRDynamicInfo<'a> {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Default)]
pub struct IREffect<'a> {
  pub operations: Vec<OperationNode<'a>>,
  /// Registration order key, settled into the emitted order by
  /// `TransformContext::finalize_block`.
  pub order: u64,
}
