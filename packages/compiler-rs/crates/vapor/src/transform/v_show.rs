use napi::bindgen_prelude::Either17;
use oxc_ast::ast::{JSXAttribute, JSXChild};

use crate::{
  ir::index::{BlockIRNode, DirectiveIRNode},
  transform::{DirectiveTransformResult, TransformContext},
};
use common::{
  directive::{find_prop, resolve_directive},
  error::ErrorCodes,
  expression::SimpleExpressionNode,
  text::get_tag_name,
};

pub fn transform_v_show<'a>(
  _dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &mut JSXChild,
) -> Option<DirectiveTransformResult<'a>> {
  let mut dir = resolve_directive(_dir, context.source_text);
  if dir.exp.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VShowNoExpression, dir.loc);
    dir.exp = Some(SimpleExpressionNode::default())
  }

  // lazy apply vshow if the node is inside a transition with appear
  let mut should_deferred = false;
  if let JSXChild::Element(parent_node) = parent_node {
    should_deferred = matches!(
      get_tag_name(&parent_node.opening_element.name, context.source_text).as_str(),
      "VaporTransition" | "VaporTransitionGroup"
    ) && find_prop(parent_node, vec!["appear"]).is_some();

    if should_deferred {
      let has_deferred_v_show = &mut context.ir.borrow_mut().has_deferred_v_show;
      *has_deferred_v_show = true;
    }
  }

  let element = context.reference(&mut context_block.dynamic);
  context.register_operation(
    context_block,
    Either17::M(DirectiveIRNode {
      directive: true,
      element,
      dir,
      name: String::from("show"),
      builtin: true,
      asset: false,
      model_type: None,
      deferred: should_deferred,
    }),
    None,
  );
  None
}
