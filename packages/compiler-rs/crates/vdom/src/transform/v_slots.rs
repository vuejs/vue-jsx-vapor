use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{JSXAttributeValue, JSXChild};

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
  let JSXChild::Element(node) = (unsafe { &*context_node }) else {
    return None;
  };

  let node_span = node.span;

  if let Some(dir) = directives.v_slots.as_mut() {
    if !is_jsx_component(&*node) {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, node_span);
      return None;
    }

    if node.children.iter().any(|c| !is_empty_text(c)) {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, node_span);
      return None;
    }

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value {
      let mut expression = value.expression.take_in(context.allocator);
      Some(Box::new(move || {
        if let Some(NodeTypes::VNodeCall(vnode_call)) =
          context.codegen_map.borrow_mut().get_mut(&node_span)
        {
          *context.options.in_v_slot.borrow_mut() += 1;
          vnode_call.children = Some(Either3::C(
            context.process_jsx_expression(&mut expression).0,
          ));
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
