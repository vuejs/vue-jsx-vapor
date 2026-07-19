use napi::Either;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{Expression, JSXChild, JSXElement};
use oxc_span::GetSpan;

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, IfIRNode, OperationNode},
  transform::TransformContext,
};

use common::{
  ast::RootNode,
  check::{is_constant_node, is_template},
  directive::{Directives, find_prop},
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  patch_flag::{VaporBlockShape, VaporIfFlags},
};

/// # SAFETY
pub unsafe fn transform_v_if<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_template(node) && directives.v_slot.is_some() {
    return None;
  }
  let node = node as *mut oxc_allocator::Box<JSXElement>;

  let dir = directives
    .v_if
    .as_mut()
    .or(directives.v_else_if.as_mut().or(directives.v_else.as_mut()))
    .unwrap();
  let seen = &mut context.seen.borrow_mut();
  let start = dir.span.start;
  if seen.contains(&start) {
    return None;
  }
  seen.insert(start);

  let dir_name = dir.name.get_identifier().name;
  let dir_exp = dir
    .value
    .as_mut()
    .map(|value| jsx_attribute_value_to_expression(value, context.ast))
    .flatten();
  if dir_name != "v-else" && dir_exp.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VIfNoExpression, dir.span);
    return None;
  }

  let dynamic = &mut context_block.dynamic;
  dynamic.flags |= DynamicFlag::NonTemplate as i32;
  let force_multi_root = should_force_multi_root(parent_node);
  // Nested dynamic units are owned by an enclosing branch scope, so only mark
  // root-block branches with the compiler-proven no-scope flag.
  let allow_no_scope = context_block.root;
  if dir_name == "v-if" {
    let id = context.reference(dynamic);
    dynamic.flags |= DynamicFlag::Insert as i32;
    let block = context_block as *mut BlockIRNode;
    let exit_block = context.create_block(
      unsafe { &mut *context_node },
      unsafe { &mut *block },
      Expression::JSXElement(oxc_allocator::Box::new_in(
        unsafe { &mut *node }.take_in(context.allocator),
        context.allocator,
      )),
      false,
    );
    if RootNode::is_root(parent_node)
      && let JSXChild::Fragment(node) = unsafe { &mut *context_node }
    {
      node.span = parent_node.span();
    }
    return Some(Box::new(move || {
      let block = exit_block();

      context_block.dynamic.operation = Some(Box::new(OperationNode::If(IfIRNode {
        id,
        block_shape: encode_if_block_shape(&block, force_multi_root, None, allow_no_scope),
        positive: block,
        index: context.next_if_index(),
        once: *context.in_v_once.borrow() || is_constant_node(dir_exp.as_ref().unwrap()),
        condition: dir_exp.unwrap(),
        negative: None,
        anchor: None,
        logical_index: None,
        parent: None,
        append: false,
        operation_index: Some(*context.operation_index.borrow()),
        effect_index: Some(*context.effect_index.borrow()),
        slot_root: false,
      })));
    }));
  }

  let siblings = &mut context.parent_dynamic.borrow_mut().children;
  let mut last_if_node = None;
  if !siblings.is_empty() {
    let mut i = siblings.len();
    while i > 0 {
      i -= 1;
      let sibling = siblings.get_mut(i).unwrap() as *mut IRDynamicInfo;
      if let Some(operation) = (unsafe { &mut *sibling }).operation.as_mut()
        && let OperationNode::If(operation) = operation.as_mut()
      {
        last_if_node = Some(operation);
        break;
      }
    }
  }

  // check if IfNode is the last operation and get the root IfNode
  let Some(mut last_if_node) = last_if_node else {
    context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, unsafe { &*node }.span);
    return None;
  };

  let mut last_if_node_ptr = last_if_node as *mut IfIRNode;
  while let Some(negative) = (unsafe { &mut *last_if_node_ptr }).negative.as_mut()
    && let Either::B(negative) = negative.as_mut()
  {
    last_if_node_ptr = negative as *mut IfIRNode;
  }
  last_if_node = unsafe { &mut *last_if_node_ptr };

  // Check if v-else was followed by v-else-if
  if dir_name == "v-else-if" && last_if_node.negative.is_some() {
    context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, dir.span);
  };

  let exit_block = context.create_block(
    unsafe { &mut *context_node },
    context_block,
    Expression::JSXElement(oxc_allocator::Box::new_in(
      unsafe { &mut *node }.take_in(context.allocator),
      context.allocator,
    )),
    false,
  );

  Some(Box::new(move || {
    let block = exit_block();
    if dir_name == "v-else" {
      last_if_node.negative = Some(Box::new(Either::A(block)));
    } else {
      last_if_node.negative = Some(Box::new(Either::B(IfIRNode {
        id: -1,
        block_shape: VaporBlockShape::Empty as i32,
        positive: block,
        index: context.next_if_index(),
        once: *context.in_v_once.borrow() || is_constant_node(dir_exp.as_ref().unwrap()),
        condition: dir_exp.unwrap(),
        parent: None,
        anchor: None,
        logical_index: None,
        negative: None,
        append: false,
        operation_index: None,
        effect_index: None,
        slot_root: false,
      })))
    }

    if let Some(negative) = last_if_node.negative.as_mut()
      && let Either::B(negative) = negative.as_mut()
    {
      negative.block_shape =
        encode_if_block_shape(&negative.positive, force_multi_root, None, allow_no_scope)
    }
    last_if_node.block_shape = encode_if_block_shape(
      &last_if_node.positive,
      force_multi_root,
      last_if_node.negative.as_ref(),
      allow_no_scope,
    )
  }))
}

pub fn encode_if_block_shape(
  positive: &BlockIRNode,
  force_multi_root: bool,
  negative: Option<&Box<Either<BlockIRNode, IfIRNode>>>,
  allow_no_scope: bool,
) -> i32 {
  // Pack the true/false branch shapes into one integer so runtime `createIf()`
  // can decode the selected branch with a single bit-mask operation.
  let mut flags = if force_multi_root {
    VaporBlockShape::MultiRoot as i32 | (VaporBlockShape::MultiRoot as i32) << 2
  } else {
    get_block_shape(positive) | (get_negative_block_shape(negative) << 2)
  };

  if allow_no_scope && can_skip_if_branch_scope(positive) {
    flags |= VaporIfFlags::TrueNoScope as i32;
  }
  if allow_no_scope
    && negative.is_some_and(|negative| match negative.as_ref() {
      Either::A(negative) => can_skip_if_branch_scope(negative),
      Either::B(_) => false,
    })
  {
    flags |= VaporIfFlags::FalseNoScope as i32;
  }

  flags
}

// SSR renders `v-if` inside `<template v-for>` always output <!--[-->...<!--]-->.
// should mark the block as multi-root
pub fn should_force_multi_root(parent: &JSXChild) -> bool {
  if let JSXChild::Element(parent) = parent
    && is_template(parent)
    && find_prop(parent, vec!["v-for"]).is_some()
  {
    true
  } else {
    false
  }
}

fn get_negative_block_shape(negative: Option<&Box<Either<BlockIRNode, IfIRNode>>>) -> i32 {
  if let Some(negative) = negative {
    match negative.as_ref() {
      Either::A(block) => get_block_shape(block),
      Either::B(_) => VaporBlockShape::SingleRoot as i32,
    }
  } else {
    VaporBlockShape::Empty as i32
  }
}

fn can_skip_if_branch_scope(block: &BlockIRNode) -> bool {
  if !block.effect.is_empty() || !block.operation.is_empty() {
    return false;
  }

  if block.returns.is_empty() || block.dynamic.children.len() != block.returns.len() {
    return false;
  }

  return block.returns.iter().all(|id| {
    let Some(returned) = find_returned_dynamic(&block, *id) else {
      return false;
    };
    returned.template.is_some()
      && returned.operation.is_none()
      && !returned.has_dynamic_child
      && (returned.flags & (DynamicFlag::Insert as i32 | DynamicFlag::NonTemplate as i32) == 0)
  });
}

fn find_returned_dynamic<'a>(block: &'a BlockIRNode, id: i32) -> Option<&'a IRDynamicInfo<'a>> {
  return block
    .dynamic
    .children
    .iter()
    .find(|child| child.id.is_some_and(|i| i == id));
}

fn get_block_shape(block: &BlockIRNode) -> i32 {
  if block.returns.is_empty() {
    return VaporBlockShape::Empty as i32;
  }
  if block.returns.len() == 1 {
    VaporBlockShape::SingleRoot as i32
  } else {
    VaporBlockShape::MultiRoot as i32
  }
}
