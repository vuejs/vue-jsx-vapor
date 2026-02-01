use napi::bindgen_prelude::Either3;
use oxc_ast::ast::JSXChild;

use crate::{
  ir::index::{BlockIRNode, OperationNode, SetTemplateRefIRNode},
  transform::TransformContext,
};
use common::{check::is_fragment_node, directive::Directives, expression::SimpleExpressionNode};

/// # SAFETY
pub unsafe fn transform_template_ref<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let node = unsafe { &mut *context_node };
  if is_fragment_node(node) {
    return None;
  }
  let dir = directives._ref.as_mut()?;
  let Some(value) = &mut dir.value else {
    return None;
  };
  context.ir.borrow_mut().has_template_ref = true;

  let value = SimpleExpressionNode::new(Either3::C(value), context.source_text);
  Some(Box::new(move || {
    let id = context.reference(&mut context_block.dynamic);

    context.register_effect(
      context_block,
      context.is_operation(vec![&value]),
      OperationNode::SetTemplateRef(SetTemplateRefIRNode {
        set_template_ref: true,
        element: id,
        value,
        ref_for: *context.in_v_for.borrow() != 0,
      }),
      None,
      None,
    );
  }))
}
