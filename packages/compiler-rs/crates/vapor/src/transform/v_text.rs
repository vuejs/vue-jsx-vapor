use std::borrow::Cow;

use common::{
  check::is_void_tag,
  directive::Directives,
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  text::{escape_html, get_text_like_value, is_empty_text},
};
use oxc_ast::ast::{JSXAttribute, JSXElement};

use crate::{
  ir::index::{BlockIRNode, GetTextChildIRNode, OperationNode, SetTextIRNode},
  transform::{DirectiveTransformResult, TransformContext},
};

pub fn transform_v_text<'a>(
  directives: &Directives,
  dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let exp = if let Some(value) = &mut dir.value {
    jsx_attribute_value_to_expression(value, context.ast)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VTextNoExpression, dir.span);
    return None;
  };

  if node.children.iter().any(|c| !is_empty_text(c)) {
    context.options.on_error.as_ref()(ErrorCodes::VTextWithChildren, node.span);
    return None;
  };

  let tag_name = node
    .opening_element
    .name
    .get_identifier_name()
    .map(|name| name.as_str())
    .unwrap_or_default();
  // v-text on void tags do nothing
  if is_void_tag(tag_name) {
    return None;
  }

  let literal = get_text_like_value(&exp, false);
  if let Some(literal) = literal {
    *context.children_template.borrow_mut() = vec![escape_html(literal)];
  } else {
    *context.children_template.borrow_mut() = vec![Cow::Borrowed(" ")];
    let parent = context.reference(&mut context_block.dynamic);
    let is_component = if directives.is_custom_element {
      false
    } else {
      directives.is_component
    };
    if !is_component {
      context.register_operation(
        context_block,
        OperationNode::GetTextChild(GetTextChildIRNode {
          get_text_child: true,
          parent,
        }),
        None,
      );
    }
    let element = context.reference(&mut context_block.dynamic);
    context.register_effect(
      context_block,
      context.is_operation(vec![&exp]),
      OperationNode::SetText(SetTextIRNode {
        set_text: true,
        values: vec![exp],
        element,
        generated: true,
        is_component,
      }),
      None,
      None,
    );
  }
  None
}
