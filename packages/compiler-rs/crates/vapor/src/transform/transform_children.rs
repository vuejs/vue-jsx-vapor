use std::{collections::VecDeque, mem};

use napi::bindgen_prelude::Either17;
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::ast::{JSXChild, JSXExpression};

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, InsertNodeIRNode},
  transform::TransformContext,
};

use common::{
  ast::RootNode,
  check::{is_fragment_node, is_jsx_component},
  text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_children<'a>(
  node: &mut JSXChild<'a>,
  context: &TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let is_fragment_or_component = RootNode::is_root(node)
    || is_fragment_node(node)
    || match node {
      JSXChild::Element(node) => is_jsx_component(node),
      _ => false,
    };

  if !matches!(&node, JSXChild::Element(_)) && !is_fragment_or_component {
    return None;
  }

  let _node = node as *mut _;
  let children = match node {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => unreachable!(),
  };
  children.retain_mut(|child| !is_empty_text(child));
  let children_ptr = children as *mut oxc_allocator::Vec<JSXChild>;
  let mut parent_children_template = context.children_template.take();
  let grand_parent_dynamic = context
    .parent_dynamic
    .replace(mem::take(&mut context_block.dynamic));
  let _context_block = context_block as *mut BlockIRNode;
  let mut i = 0;
  while let Some(child) = children.get_mut(i) {
    let exit_context = context.create(
      if let JSXChild::Text(_) = child
        && let Some(next) = unsafe { &mut *children_ptr }.get_mut(1)
        && let JSXChild::ExpressionContainer(exp) = next
        && !matches!(
          exp.expression,
          JSXExpression::ConditionalExpression(_) | JSXExpression::LogicalExpression(_)
        )
      {
        child.clone_in(context.allocator)
      } else {
        child.take_in(context.allocator)
      },
      i as i32,
      unsafe { &mut *_context_block },
    );
    context.transform_node(
      Some(unsafe { &mut *_context_block }),
      Some(unsafe { &mut *_node }),
    );

    let mut parent_dynamic = context.parent_dynamic.borrow_mut();
    let child_dynamic = &mut context_block.dynamic;
    let flags = child_dynamic.flags;
    if is_fragment_or_component {
      context.register_template(child_dynamic);
      context.reference(child_dynamic);

      if flags & DynamicFlag::NonTemplate as i32 == 0 || flags & DynamicFlag::Insert as i32 != 0 {
        context_block.returns.push(child_dynamic.id.unwrap());
      }
    } else {
      parent_children_template.push(context.template.borrow().to_string());
    }

    if child_dynamic.has_dynamic_child
      || child_dynamic.id.is_some()
      || flags & DynamicFlag::NonTemplate as i32 != 0
      || flags & DynamicFlag::Insert as i32 != 0
    {
      parent_dynamic.has_dynamic_child = true;
    }

    parent_dynamic.children.insert(i, mem::take(child_dynamic));

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
  let mut dynamic_count = 0;
  let mut last_insertion_child = None;
  let children = &mut context_block.dynamic.children as *mut Vec<IRDynamicInfo>;

  for (index, child) in unsafe { &mut *children }.iter_mut().enumerate() {
    let flags = child.flags;
    if flags & DynamicFlag::Insert as i32 != 0 {
      last_insertion_child = Some(unsafe { &mut *(child as *mut IRDynamicInfo) });
      prev_dynamics.push_back(child);
    }

    if flags & DynamicFlag::NonTemplate as i32 == 0 {
      if !prev_dynamics.is_empty() {
        if static_count > 0 {
          context
            .children_template
            .borrow_mut()
            .insert(index - prev_dynamics.len(), "<!>".to_string());
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
        dynamic_count += prev_dynamics.len();
        prev_dynamics.clear();
      }
      static_count += 1;
    }
  }

  if !prev_dynamics.is_empty() {
    register_insertion(
      &mut prev_dynamics,
      context,
      context_block,
      // the logical index of append child
      (dynamic_count + static_count) as i32,
      true,
    );
  }

  if let Some(last_insertion_child) = last_insertion_child
    && let Some(operation) = last_insertion_child.operation.as_mut()
  {
    match operation.as_mut() {
      Either17::A(operation) => operation.last = true,
      Either17::B(operation) => operation.last = true,
      Either17::N(operation) => operation.last = true,
      Either17::Q(operation) => operation.last = true,
      _ => (),
    };
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
    if child.template.is_some() {
      let parent = context.reference(&mut context_block.dynamic);
      // template node due to invalid nesting - generate actual insertion
      context.register_operation(
        context_block,
        Either17::L(InsertNodeIRNode {
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
        Either17::A(if_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          if_ir_node.parent = Some(parent);
          if_ir_node.anchor = Some(anchor);
          if_ir_node.append = append;
        }
        Either17::B(for_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          for_ir_node.parent = Some(parent);
          for_ir_node.anchor = Some(anchor);
          for_ir_node.append = append;
        }
        Either17::N(create_component_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          create_component_ir_node.parent = Some(parent);
          create_component_ir_node.anchor = Some(anchor);
          create_component_ir_node.append = append;
        }
        Either17::Q(key_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          key_ir_node.parent = Some(parent);
          key_ir_node.anchor = Some(anchor);
          key_ir_node.append = append;
        }
        _ => (),
      };
    }
  }
}
