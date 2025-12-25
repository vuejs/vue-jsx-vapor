use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{JSXAttributeValue, JSXChild, JSXElement};

use crate::{ast::NodeTypes, transform::TransformContext};
use common::{
  check::is_jsx_component, directive::Directives, error::ErrorCodes, text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_slots<'a>(
  directives: &mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };

  let node_span = node.span;
  let node = node as *mut oxc_allocator::Box<'a, JSXElement<'a>>;

  if let Some(dir) = directives.v_slots.as_mut() {
    if !is_jsx_component(unsafe { &*node }) {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, node_span);
      return None;
    }

    if unsafe { &*node }
      .children
      .iter()
      .filter(|c| !is_empty_text(c))
      .count()
      > 0
    {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, node_span);
      return None;
    }

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value {
      let expression = value.expression.take_in(context.allocator);
      Some(Box::new(move || {
        if let Some(NodeTypes::VNodeCall(vnode_call)) =
          context.codegen_map.borrow_mut().get_mut(&node_span)
        {
          *context.options.in_v_slot.borrow_mut() += 1;
          vnode_call.children = Some(Either3::C(context.jsx_expression_to_expression(expression)));
          vnode_call.patch_flag = Some(vnode_call.patch_flag.unwrap_or_default());
          *context.options.in_v_slot.borrow_mut() -= 1;
        }
      }))
    } else {
      context.options.on_error.as_ref()(ErrorCodes::VSlotsNoExpression, dir.span);
      None
    }
  } else {
    None
  }
}
