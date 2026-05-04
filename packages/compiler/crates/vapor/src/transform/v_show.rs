use std::borrow::Cow;

use oxc_ast::ast::JSXAttribute;

use crate::{
  ir::index::{BlockIRNode, DirectiveIRNode, OperationNode},
  transform::{DirectiveTransformResult, TransformContext},
};
use common::{directive::resolve_directive, error::ErrorCodes};

pub fn transform_v_show<'a>(
  _dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let dir = resolve_directive(_dir, context.ast);
  if dir.exp.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VShowNoExpression, dir.span);
    return None;
  }

  let element = context.reference(&mut context_block.dynamic);
  context.register_operation(
    context_block,
    OperationNode::Directive(DirectiveIRNode {
      directive: true,
      element,
      dir,
      name: Cow::Borrowed("show"),
      builtin: true,
      asset: false,
      model_type: None,
    }),
    None,
  );
  None
}
