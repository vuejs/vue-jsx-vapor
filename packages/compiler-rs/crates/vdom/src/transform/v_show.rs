use oxc_ast::ast::JSXAttribute;

use crate::transform::{
  DirectiveTransformResult, TransformContext, transform_element::build_directive_args,
};
use common::{directive::resolve_directive, error::ErrorCodes};

pub fn transform_v_show<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  if dir.value.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VShowNoExpression, dir.span);
  }

  Some(DirectiveTransformResult {
    props: vec![],
    runtime: Some(build_directive_args(
      &resolve_directive(dir, *context.source.borrow()),
      context,
      &context.helper("vShow"),
    )),
  })
}
