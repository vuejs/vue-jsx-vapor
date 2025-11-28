use napi::{
  Either,
  bindgen_prelude::{Either3, Either16},
};
use oxc_ast::ast::JSXChild;

use crate::{
  ir::index::{BlockIRNode, DeclareOldRefIRNode, SetTemplateRefIRNode, SimpleExpressionNode},
  transform::{ContextNode, TransformContext},
  utils::{check::is_fragment_node, directive::find_prop_mut},
};

/// # SAFETY
pub unsafe fn transform_template_ref<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  _: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_fragment_node(node) {
    return None;
  }
  let JSXChild::Element(node) = node else {
    return None;
  };
  let dir = find_prop_mut(node, Either::A(String::from("ref")))?;
  let Some(value) = &mut dir.value else {
    return None;
  };
  context.ir.borrow_mut().has_template_ref = true;

  let value = SimpleExpressionNode::new(Either3::C(value), context);
  Some(Box::new(move || {
    let id = context.reference(&mut context_block.dynamic);
    let effect = !value.is_constant_expression();
    if effect {
      context.register_operation(
        context_block,
        Either16::O(DeclareOldRefIRNode {
          declare_older_ref: true,
          id,
        }),
        None,
      );
    }

    context.register_effect(
      context_block,
      context.is_operation(vec![&value]),
      Either16::J(SetTemplateRefIRNode {
        set_template_ref: true,
        element: id,
        value,
        ref_for: *context.in_v_for.borrow() != 0,
        effect,
      }),
      None,
      None,
    );
  }))
}
