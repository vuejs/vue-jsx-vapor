use napi::{
  Either,
  bindgen_prelude::{Either3, Either4},
};
use oxc_ast::ast::{JSXAttributeValue, JSXChild, JSXElement};

use crate::{
  ir::{
    component::{IRSlotType, IRSlotsExpression},
    index::{BlockIRNode, SimpleExpressionNode},
  },
  transform::{ContextNode, TransformContext},
  utils::{check::is_jsx_component, directive::find_prop_mut, error::ErrorCodes},
};

/// # SAFETY
pub unsafe fn transform_v_slots<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  _: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(JSXChild::Element(node)) = (unsafe { &mut *context_node }) else {
    return None;
  };

  let node = node as *mut oxc_allocator::Box<'a, JSXElement<'a>>;

  if let Some(dir) = find_prop_mut(unsafe { &mut *node }, Either::A("v-slots".to_string())) {
    if !is_jsx_component(unsafe { &*node }) {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, unsafe { &*node }.span);
      return None;
    }

    if !unsafe { &mut *node }.children.is_empty() {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, unsafe { &*node }.span);
      return None;
    }

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value {
      let slots =
        SimpleExpressionNode::new(Either3::A(value.expression.to_expression_mut()), context);
      Some(Box::new(move || {
        *context.slots.borrow_mut() = vec![Either4::D(IRSlotsExpression {
          slot_type: IRSlotType::EXPRESSION,
          slots,
        })];
      }))
    } else {
      context.options.on_error.as_ref()(ErrorCodes::VSlotsNoExpression, dir.span);
      None
    }
  } else {
    None
  }
}
