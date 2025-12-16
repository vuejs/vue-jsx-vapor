use napi::{Either, bindgen_prelude::Either3};
use oxc_ast::ast::{JSXAttributeValue, JSXChild, JSXElement};

use crate::{ast::NodeTypes, ir::index::BlockIRNode, transform::TransformContext};
use common::{
  check::is_jsx_component, directive::find_prop_mut, error::ErrorCodes, patch_flag::PatchFlags,
};

/// # SAFETY
pub unsafe fn transform_v_slots<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  _: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };

  let node_span = node.span;
  let node = node as *mut oxc_allocator::Box<'a, JSXElement<'a>>;

  if let Some(dir) = find_prop_mut(unsafe { &mut *node }, Either::A("v-slots".to_string())) {
    if !is_jsx_component(unsafe { &*node }) {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, node_span);
      return None;
    }

    if !unsafe { &*node }.children.is_empty() {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, node_span);
      return None;
    }

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value {
      Some(Box::new(move || {
        if let Some(NodeTypes::VNodeCall(vnode_call)) =
          context.codegen_map.borrow_mut().get_mut(&node_span)
        {
          vnode_call.children = Some(Either3::C(
            context.jsx_expression_to_expression(&mut value.expression),
          ));
          vnode_call.patch_flag = Some(
            if let Some(patch_flag) = vnode_call.patch_flag {
              patch_flag
            } else {
              0
            } | PatchFlags::DynamicSlots as i32,
          )
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
