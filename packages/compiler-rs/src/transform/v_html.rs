use napi::bindgen_prelude::{Either3, Either16};
use oxc_ast::ast::{JSXAttribute, JSXElement};

use crate::{
  ir::index::{BlockIRNode, SetHtmlIRNode, SimpleExpressionNode},
  transform::{DirectiveTransformResult, TransformContext},
  utils::error::ErrorCodes,
};

pub fn transform_v_html<'a>(
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let exp = if let Some(value) = &mut dir.value {
    SimpleExpressionNode::new(Either3::C(value), context)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VHtmlNoExpression, dir.span);
    SimpleExpressionNode::default()
  };

  if !node.children.is_empty() {
    context.options.on_error.as_ref()(ErrorCodes::VHtmlWithChildren, node.span);
    return None;
  }

  let element = context.reference(&mut context_block.dynamic);
  context.register_effect(
    context_block,
    context.is_operation(vec![&exp]),
    Either16::I(SetHtmlIRNode {
      set_html: true,
      element,
      value: exp,
    }),
    None,
    None,
  );
  None
}
