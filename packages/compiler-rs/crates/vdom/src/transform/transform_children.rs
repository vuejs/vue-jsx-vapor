use oxc_ast::ast::JSXChild;

use crate::{ir::index::BlockIRNode, transform::TransformContext};

use common::check::{is_fragment_node, is_jsx_component};

/// # SAFETY
pub unsafe fn transform_children<'a>(
  node: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let is_fragment_or_component = is_fragment_node(node)
    || match node {
      JSXChild::Element(node) => is_jsx_component(node),
      _ => false,
    };

  if !matches!(&node, JSXChild::Element(_)) && !is_fragment_or_component {
    return None;
  }

  let children = match node {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => unreachable!(),
  } as *mut oxc_allocator::Vec<JSXChild>;
  let _context_block = context_block as *mut BlockIRNode;
  for child in unsafe { &mut *children }.iter_mut() {
    // if matches!(child, JSXChild::Element(_) | JSXChild::Fragment(_)) {
    context.transform_node(child, Some(unsafe { &mut *_context_block }), Some(node));
    // }
  }
  None
}
