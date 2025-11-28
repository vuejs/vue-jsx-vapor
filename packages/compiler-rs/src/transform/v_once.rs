use napi::Either;
use oxc_ast::ast::JSXChild;

use crate::{
  ir::index::BlockIRNode,
  transform::{ContextNode, TransformContext},
  utils::directive::find_prop,
};

pub unsafe fn transform_v_once<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  _: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(node) = (unsafe { &*context_node }) else {
    return None;
  };

  if let JSXChild::Element(node) = &node
    && find_prop(node, Either::A(String::from("v-once"))).is_some()
  {
    *context.in_v_once.borrow_mut() = true;
  }
  None
}
