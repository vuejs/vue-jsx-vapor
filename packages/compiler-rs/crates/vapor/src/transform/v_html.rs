use common::{
  check::is_jsx_component, error::ErrorCodes, expression::jsx_attribute_value_to_expression,
  text::is_empty_text,
};
use oxc_ast::ast::{JSXAttribute, JSXElement};

use crate::{
  ir::index::{BlockIRNode, OperationNode, SetHtmlIRNode},
  transform::{DirectiveTransformResult, TransformContext},
};

pub fn transform_v_html<'a>(
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let exp = if let Some(value) = &mut dir.value {
    jsx_attribute_value_to_expression(value, context.ast)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VHtmlNoExpression, dir.span);
    return None;
  };

  if node.children.iter().any(|c| !is_empty_text(c)) {
    context.options.on_error.as_ref()(ErrorCodes::VHtmlWithChildren, node.span);
    return None;
  }

  let element = context.reference(&mut context_block.dynamic);
  context.register_effect(
    context_block,
    context.is_operation(vec![&exp]),
    OperationNode::SetHtml(SetHtmlIRNode {
      set_html: true,
      element,
      value: exp,
      is_component: is_jsx_component(node, false, context.options),
    }),
    None,
    None,
  );
  None
}
