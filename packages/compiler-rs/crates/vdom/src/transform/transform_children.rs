use oxc_ast::ast::JSXChild;

use crate::transform::TransformContext;

use common::{check::is_fragment_node, directive::Directives, text::is_empty_text};

/// # SAFETY
pub unsafe fn transform_children<'a>(
  directives: &Directives,
  node: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) {
  unsafe {
    let is_fragment_or_component = is_fragment_node(node) || directives.is_component;

    if !matches!(&node, JSXChild::Element(_)) && !is_fragment_or_component {
      return;
    }

    let children = match node {
      JSXChild::Element(node) => &mut node.children,
      JSXChild::Fragment(node) => &mut node.children,
      _ => unreachable!(),
    } as *mut oxc_allocator::Vec<JSXChild>;
    (&mut *children).retain_mut(|child| {
      if is_empty_text(child) {
        false
      } else {
        context.transform_node(child, Some(node));
        true
      }
    });
  }
}
