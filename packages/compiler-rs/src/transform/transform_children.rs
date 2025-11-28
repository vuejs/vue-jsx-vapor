use std::{collections::VecDeque, mem};

use napi::{Either, bindgen_prelude::Either16};
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::ast::JSXChild;

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, InsertNodeIRNode},
  transform::{ContextNode, TransformContext},
  utils::check::{is_fragment_node, is_jsx_component},
};

/// # SAFETY
pub unsafe fn transform_children<'a>(
  node: &mut ContextNode<'a>,
  context: &TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let is_fragment_or_component = match &node {
    Either::A(_) => true,
    Either::B(node) => {
      is_fragment_node(node)
        || match node {
          JSXChild::Element(node) => is_jsx_component(node),
          _ => false,
        }
    }
  };

  if !matches!(&node, Either::B(JSXChild::Element(_))) && !is_fragment_or_component {
    return None;
  }

  let _node = node as *mut ContextNode;
  let children = match node {
    Either::A(node) => &mut node.children,
    Either::B(node) => match node {
      JSXChild::Element(node) => &mut node.children,
      JSXChild::Fragment(node) => &mut node.children,
      _ => unreachable!(),
    },
  } as *mut oxc_allocator::Vec<JSXChild>;
  let mut parent_children_template = context.children_template.take();
  let grand_parent_dynamic = context
    .parent_dynamic
    .replace(mem::take(&mut context_block.dynamic));
  let _context_block = context_block as *mut BlockIRNode;
  let mut i = 0;
  while let Some(child) = unsafe { &mut *children }.get_mut(i) {
    let exit_context = context.create(
      if let Some(next) = unsafe { &mut *children }.get_mut(i + 1)
        && let JSXChild::ExpressionContainer(_) = next
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

    if child_dynamic.has_dynamic_child.unwrap_or(false)
      || child_dynamic.id.is_some()
      || flags & DynamicFlag::NonTemplate as i32 != 0
      || flags & DynamicFlag::Insert as i32 != 0
    {
      parent_dynamic.has_dynamic_child = Some(true);
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
  let mut has_static_template = false;

  let children = &mut context_block.dynamic.children as *mut Vec<IRDynamicInfo>;
  for (index, child) in unsafe { &mut *children }.iter_mut().enumerate() {
    let flags = child.flags;
    if flags & DynamicFlag::Insert as i32 != 0 {
      prev_dynamics.push_back(child);
    }

    if flags & DynamicFlag::NonTemplate as i32 == 0 {
      if !prev_dynamics.is_empty() {
        if has_static_template {
          context
            .children_template
            .borrow_mut()
            .insert(index - prev_dynamics.len(), "<!>".to_string());
          prev_dynamics[0].flags -= DynamicFlag::NonTemplate as i32;
          let anchor = context.increase_id();
          prev_dynamics[0].anchor = Some(anchor);
          register_insertion(&mut prev_dynamics, context, context_block, Some(anchor));
        } else {
          register_insertion(
            &mut prev_dynamics,
            context,
            context_block,
            Some(-1), /* prepend */
          );
        }
        prev_dynamics.clear();
      }
      has_static_template = true;
    }
  }

  if !prev_dynamics.is_empty() {
    register_insertion(&mut prev_dynamics, context, context_block, None);
  }
}

fn register_insertion<'a>(
  dynamics: &mut VecDeque<&mut IRDynamicInfo>,
  context: &TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
  anchor: Option<i32>,
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
        Either16::L(InsertNodeIRNode {
          insert_node: true,
          elements: ids.clone(),
          parent,
          anchor,
        }),
        None,
      );
    } else if let Some(operation) = &mut child.operation {
      // block types
      match operation.as_mut() {
        Either16::A(if_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          if_ir_node.parent = Some(parent);
          if_ir_node.anchor = anchor;
        }
        Either16::B(for_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          for_ir_node.parent = Some(parent);
          for_ir_node.anchor = anchor;
        }
        Either16::N(create_component_ir_node) => {
          let parent = context.reference(&mut context_block.dynamic);
          create_component_ir_node.parent = Some(parent);
          create_component_ir_node.anchor = anchor;
        }
        _ => (),
      };
    }
  }
}
