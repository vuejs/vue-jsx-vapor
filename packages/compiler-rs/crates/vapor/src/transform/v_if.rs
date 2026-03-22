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
  directive::Directives,
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  patch_flag::VaporBlockShape,
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
    .map(|value| jsx_attribute_value_to_expression(value, context.ast));
  if dir_name != "v-else" && dir_exp.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VIfNoExpression, dir.span);
    return None;
  }

  let dynamic = &mut context_block.dynamic;
  dynamic.flags |= DynamicFlag::NonTemplate as i32;

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
        block_shape: encode_if_block_shape(&block, None),
        positive: block,
        index: context.next_if_index(),
        once: *context.in_v_once.borrow() || is_constant_node(dir_exp.as_ref().unwrap()),
        condition: dir_exp.unwrap(),
        negative: None,
        anchor: None,
        logical_index: None,
        parent: None,
        append: false,
        last: false,
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
        last: false,
      })))
    }

    if let Some(negative) = last_if_node.negative.as_mut()
      && let Either::B(negative) = negative.as_mut()
    {
      negative.block_shape = encode_if_block_shape(&negative.positive, None)
    }
    last_if_node.block_shape =
      encode_if_block_shape(&last_if_node.positive, last_if_node.negative.as_ref())
  }))
}

pub fn encode_if_block_shape(
  positive: &BlockIRNode,
  negative: Option<&Box<Either<BlockIRNode, IfIRNode>>>,
) -> i32 {
  // Pack the true/false branch shapes into one integer so runtime `createIf()`
  // can decode the selected branch with a single bit-mask operation.
  get_block_shape(positive) | (get_negative_block_shape(negative) << 2)
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
