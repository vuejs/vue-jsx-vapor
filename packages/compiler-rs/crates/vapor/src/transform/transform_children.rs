use std::{borrow::Cow, collections::VecDeque, mem};

use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::ast::{JSXChild, JSXExpression};

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, InsertNodeIRNode, OperationNode},
  transform::{
    TransformContext,
    transform_element::{get_child_template_close_tags, is_in_same_template_as_parent},
  },
};

use common::{
  ast::RootNode,
  check::is_fragment_node,
  directive::Directives,
  text::{get_tag_name, is_empty_text},
};

/// # SAFETY
pub unsafe fn transform_children<'a>(
  directives: &Directives<'a>,
  node: &mut JSXChild<'a>,
  context: &TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: Option<&JSXChild<'a>>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let is_fragment_or_component =
    RootNode::is_root(node) || is_fragment_node(node) || directives.is_component;

  if !matches!(&node, JSXChild::Element(_)) && !is_fragment_or_component {
    return None;
  }

  let _node = node as *mut _;
  let parent_tag_name = directives.tag_name;
  let child_template_close_tags = if !is_fragment_or_component {
    get_child_template_close_tags(parent_tag_name, parent_node, context)
  } else {
    Default::default()
  };
  let children = match node {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => unreachable!(),
  };
  let children_ptr = children as *mut oxc_allocator::Vec<JSXChild>;
  let mut parent_children_template = context.children_template.take();
  let grand_parent_dynamic = context
    .parent_dynamic
    .replace(mem::take(&mut context_block.dynamic));
  let _context_block = context_block as *mut BlockIRNode;
  let mut i = 0;
  if let Some(last) = children.last()
    && is_empty_text(last)
  {
    children.pop();
  }
  let mut children_len = children.len();
  while let Some(child) = children.get_mut(i) {
    if is_empty_text(child) {
      children.remove(i);
      children_len -= 1;
      continue;
    } else if let JSXChild::Fragment(child) = child {
      children_len += child.children.len();
      unsafe { &mut *children_ptr }.splice(i..i + 1, child.children.take_in(context.allocator));
      continue;
    }
    let mut tag = "";
    let mut next_is_interpolation = false;
    let is_text_child = matches!(&child, JSXChild::Text(_));
    let exit_context = context.create(
      if is_text_child
        && let Some(next) = unsafe { &mut *children_ptr }.get_mut(i + 1)
        && let JSXChild::ExpressionContainer(exp) = next
        && !matches!(
          exp.expression,
          JSXExpression::ConditionalExpression(_) | JSXExpression::EmptyExpression(_)
        )
      {
        next_is_interpolation = true;
        child.clone_in(context.allocator)
      } else {
        if let JSXChild::Element(child) = child {
          tag = get_tag_name(child, context.options);
        }
        child.take_in(context.allocator)
      },
      i as i32,
      if !parent_tag_name.is_empty() {
        children_len == i + 1
      } else {
        true
      },
      parent_tag_name,
      unsafe { &mut *_context_block },
    );
    let is_same_template = is_in_same_template_as_parent(tag, parent_tag_name);
    if is_same_template {
      *context.template_close_tags.borrow_mut() = child_template_close_tags.clone();
    } else {
      context.template_close_tags.borrow_mut().clear();
    }
    context.transform_node(
      Some(unsafe { &mut *_context_block }),
      Some(unsafe { &mut *_node }),
    );

    let mut parent_dynamic = context.parent_dynamic.borrow_mut();
    let flags = context_block.dynamic.flags;
    if is_fragment_or_component {
      if next_is_interpolation {
        context.template.borrow_mut().clear();
      } else {
        context.register_template(
          context_block,
          Some(tag),
          is_text_child || context.can_use_static_template(&context_block, tag),
        );
        context.reference(&mut context_block.dynamic);
        if flags & DynamicFlag::NonTemplate as i32 == 0 || flags & DynamicFlag::Insert as i32 != 0 {
          context_block
            .returns
            .push(context_block.dynamic.id.unwrap());
        }
      }
    } else {
      parent_children_template.push(Cow::Owned(context.template.take()));
    }

    if context_block.dynamic.has_dynamic_child
      || context_block.dynamic.id.is_some()
      || flags & DynamicFlag::NonTemplate as i32 != 0
      || flags & DynamicFlag::Insert as i32 != 0
    {
      parent_dynamic.has_dynamic_child = true;
    }

    parent_dynamic
      .children
      .insert(i, mem::take(&mut context_block.dynamic));

    exit_context();
    i += 1;
  }
  *context.children_template.borrow_mut() = parent_children_template;
  context_block.dynamic = context.parent_dynamic.replace(grand_parent_dynamic);

  if !is_fragment_or_component {
    process_dynamic_children(context, context_block);
  }

  None
}

fn process_dynamic_children<'a>(
  context: &TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
  let mut prev_dynamics = VecDeque::new();
  let mut static_count = 0;
  let children = &mut context_block.dynamic.children as *mut Vec<IRDynamicInfo>;

  // Track logical index for each child.
  // logicalIndex represents the position in SSR DOM, used during hydration
  // to locate the correct DOM node. Each child (static element, component,
  // v-if/v-else-if/v-else chain, v-for, slot) counts as one logical unit.
  let mut logical_index = 0;

  for (index, child) in unsafe { &mut *children }.iter_mut().enumerate() {
    let flags = child.flags;
    let child_ptr = child as *mut IRDynamicInfo;
    if flags & DynamicFlag::Insert as i32 != 0 {
      child.logical_index = Some(logical_index);
      prev_dynamics.push_back(child);
      logical_index += 1;
    }

    if flags & DynamicFlag::NonTemplate as i32 == 0 {
      unsafe { &mut *child_ptr }.logical_index = Some(logical_index);
      if !prev_dynamics.is_empty() {
        if static_count > 0 {
          context.children_template.borrow_mut()[index - prev_dynamics.len()] =
            Cow::Borrowed("<!>");
          prev_dynamics[0].flags -= DynamicFlag::NonTemplate as i32;
          let anchor = context.increase_id();
          prev_dynamics[0].anchor = Some(anchor);
          register_insertion(&mut prev_dynamics, context, context_block, anchor, false);
        } else {
          register_insertion(
            &mut prev_dynamics,
            context,
            context_block,
            -1, /* prepend */
            false,
          );
        }
        prev_dynamics.clear();
      }
      static_count += 1;
      logical_index += 1;
    }
  }

  if !prev_dynamics.is_empty() {
    let logical_index = prev_dynamics.get(0).unwrap().logical_index.unwrap();
    register_insertion(
      &mut prev_dynamics,
      context,
      context_block,
      // the logical index of append child
      logical_index,
      true,
    );
  }
}

fn register_insertion<'a>(
  dynamics: &mut VecDeque<&mut IRDynamicInfo>,
  context: &TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
  anchor: i32,
  append: bool,
) {
  let ids = dynamics
    .iter()
    .filter_map(|child| child.id)
    .collect::<Vec<i32>>();
  for child in dynamics {
    let logical_index = child.logical_index;
    if child.template.is_some() {
      let parent = context.reference(&mut context_block.dynamic);
      // template node due to invalid nesting - generate actual insertion
      context.register_operation(
        context_block,
        OperationNode::InsertNode(InsertNodeIRNode {
          insert_node: true,
          elements: ids.clone(),
          parent,
          anchor: if append { None } else { Some(anchor) },
        }),
        None,
      );
    } else if let Some(operation) = &mut child.operation {
      // block types
      match operation.as_mut() {
        OperationNode::If(if_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          if_ir_node.parent = Some(parent);
          if_ir_node.anchor = Some(anchor);
          if_ir_node.logical_index = logical_index;
          if_ir_node.append = append;
        }
        OperationNode::For(for_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          for_ir_node.parent = Some(parent);
          for_ir_node.anchor = Some(anchor);
          for_ir_node.logical_index = logical_index;
          for_ir_node.append = append;
        }
        OperationNode::CreateComponent(create_component_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          create_component_ir_node.parent = Some(parent);
          create_component_ir_node.anchor = Some(anchor);
          create_component_ir_node.logical_index = logical_index;
          create_component_ir_node.append = append;
        }
        OperationNode::SlotOutlet(slot_outlet_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          slot_outlet_ir_node.parent = Some(parent);
          slot_outlet_ir_node.anchor = Some(anchor);
          slot_outlet_ir_node.logical_index = logical_index;
          slot_outlet_ir_node.append = append;
        }
        OperationNode::Key(key_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          key_ir_node.parent = Some(parent);
          key_ir_node.anchor = Some(anchor);
          key_ir_node.logical_index = logical_index;
          key_ir_node.append = append;
        }
        _ => (),
      };
    }
  }
}
