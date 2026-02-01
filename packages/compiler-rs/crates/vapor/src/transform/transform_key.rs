use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{Expression, JSXChild};

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, KeyIRNode},
  transform::TransformContext,
};
use common::{directive::Directives, expression::SimpleExpressionNode};

/// # SAFETY
pub unsafe fn transform_v_key<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let key = directives.key.as_mut()?;
  let value = key.value.as_mut()?;

  let seen = &mut context.seen.borrow_mut();
  let start = key.span.start;
  if seen.contains(&start) {
    return None;
  }
  seen.insert(start);

  let value = SimpleExpressionNode::new(Either3::C(value), context.source_text);

  let dynamic = &mut context_block.dynamic;
  dynamic.flags = DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;

  let id = context.reference(dynamic);
  let block = context_block as *mut BlockIRNode;
  let exit_block = context.create_block(
    unsafe { &mut *context_node },
    unsafe { &mut *block },
    Expression::JSXElement(oxc_allocator::Box::new_in(
      node.take_in(context.allocator),
      context.allocator,
    )),
    false,
  );

  Some(Box::new(move || {
    let block = exit_block();
    context_block.dynamic.operation =
      Some(Box::new(crate::ir::index::OperationNode::Key(KeyIRNode {
        id,
        value,
        block,
        anchor: None,
        logical_index: None,
        parent: None,
        append: false,
        last: false,
      })))
  }))
}
