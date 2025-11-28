use napi::bindgen_prelude::{Either3, Either16};
use oxc_ast::ast::{JSXAttribute, JSXElement};

use crate::{
  ir::index::{BlockIRNode, GetTextChildIRNode, SetTextIRNode, SimpleExpressionNode},
  transform::{DirectiveTransformResult, TransformContext},
  utils::{check::is_void_tag, error::ErrorCodes},
};

pub fn transform_v_text<'a>(
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let exp = if let Some(value) = &mut dir.value {
    SimpleExpressionNode::new(Either3::C(value), context)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VTextNoExpression, dir.span);
    SimpleExpressionNode::default()
  };

  if !node.children.is_empty() {
    context.options.on_error.as_ref()(ErrorCodes::VTextWithChildren, node.span);
    return None;
  };

  // v-text on void tags do nothing
  if let Some(name) = &node.opening_element.name.get_identifier_name()
    && is_void_tag(name)
  {
    return None;
  }

  let literal = exp.get_literal_expression_value();
  if let Some(literal) = literal {
    *context.children_template.borrow_mut() = vec![literal];
  } else {
    *context.children_template.borrow_mut() = vec![" ".to_string()];
    let parent = context.reference(&mut context_block.dynamic);
    context.register_operation(
      context_block,
      Either16::P(GetTextChildIRNode {
        get_text_child: true,
        parent,
      }),
      None,
    );
    let element = context.reference(&mut context_block.dynamic);
    context.register_effect(
      context_block,
      context.is_operation(vec![&exp]),
      Either16::C(SetTextIRNode {
        set_text: true,
        values: vec![exp],
        element,
        generated: Some(true),
      }),
      None,
      None,
    );
  }
  None
}
