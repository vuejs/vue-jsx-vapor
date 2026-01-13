use common::{ast::get_first_child, directive::find_prop};
use oxc_ast::ast::{JSXChild, JSXElement};
use oxc_span::SPAN;

use crate::transform::TransformContext;

pub fn transform_transition<'a>(node: &mut JSXElement<'a>, context: &'a TransformContext<'a>) {
  let ast = &context.ast;
  // check if it's s single child w/ v-show
  // if yes, inject "persisted: true" to the transition props
  if let Some(child) = get_first_child(&node.children)
    && let JSXChild::Element(child) = child
    && find_prop(child, vec!["v-show"]).is_some()
  {
    node
      .opening_element
      .attributes
      .push(ast.jsx_attribute_item_attribute(
        SPAN,
        ast.jsx_attribute_name_identifier(SPAN, "persisted"),
        None,
      ));
  }
}
