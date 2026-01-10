use common::directive::find_prop;
use napi::Either;
use oxc_ast::ast::JSXChild;

use crate::{ir::index::BlockIRNode, transform::TransformContext};

/// # SAFETY
pub unsafe fn transform_v_once<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  _: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &*context_node }) else {
    return None;
  };

  if find_prop(node, Either::A(String::from("v-once"))).is_some() {
    *context.in_v_once.borrow_mut() = true;
  }
  None
}
