use napi::bindgen_prelude::Either16;
use oxc_ast::ast::{JSXAttribute, JSXElement};

use crate::{
  ir::index::{BlockIRNode, DirectiveIRNode, SimpleExpressionNode},
  transform::{DirectiveTransformResult, TransformContext},
  utils::{directive::resolve_directive, error::ErrorCodes},
};

pub fn transform_v_show<'a>(
  _dir: &'a mut JSXAttribute<'a>,
  _: &JSXElement,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let mut dir = resolve_directive(_dir, context);
  if dir.exp.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VShowNoExpression, dir.loc);
    dir.exp = Some(SimpleExpressionNode::default())
  }

  let element = context.reference(&mut context_block.dynamic);
  context.register_operation(
    context_block,
    Either16::M(DirectiveIRNode {
      directive: true,
      element,
      dir,
      name: String::from("show"),
      builtin: Some(true),
      asset: None,
      model_type: None,
    }),
    None,
  );
  None
}
