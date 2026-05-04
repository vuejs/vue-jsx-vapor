use oxc_allocator::TakeIn;
use oxc_ast::ast::{Expression, JSXAttributeValue, JSXChild};
use oxc_span::{GetSpan, SPAN};

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, KeyIRNode},
  transform::TransformContext,
};
use common::{
  check::is_constant_node, directive::Directives, expression::jsx_attribute_value_to_expression,
};

/// # SAFETY
pub unsafe fn transform_key<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let key = directives.key.as_mut()?;
  let value = key.value.as_mut()?;
  let JSXAttributeValue::ExpressionContainer(value) = value else {
    return None;
  };
  let value = value.expression.as_expression_mut()?;
  if is_constant_node(value) {
    return None;
  }

  let seen = &mut context.seen.borrow_mut();
  let start = key.span.start;
  if seen.contains(&start) {
    return None;
  }
  seen.insert(start);

  let dynamic = &mut context_block.dynamic;
  dynamic.flags = DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;

  let id = context.reference(dynamic);
  let block = context_block as *mut BlockIRNode;
  let exit_block = context.create_block(
    unsafe { &mut *context_node },
    unsafe { &mut *block },
    Expression::JSXElement(oxc_allocator::Box::new_in(
      node.take_in(context.allocator),
      context.allocator,
    )),
    false,
  );

  Some(Box::new(move || {
    let block = exit_block();
    context_block.dynamic.operation =
      Some(Box::new(crate::ir::index::OperationNode::Key(KeyIRNode {
        id,
        value: value.take_in(context.allocator),
        block,
        anchor: None,
        logical_index: None,
        parent: None,
        append: false,
        last: false,
      })))
  }))
}

pub fn resolve_static_key<'a>(
  directives: &mut Directives<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Expression<'a>> {
  let ast = context.ast;
  let key = directives.key.as_mut()?;
  if let Some(value) = &mut key.value {
    if match value {
      JSXAttributeValue::ExpressionContainer(value)
        if value.expression.as_expression().is_some() && value.expression.span() != SPAN =>
      {
        is_constant_node(value.expression.to_expression())
      }
      JSXAttributeValue::StringLiteral(_) => true,
      _ => false,
    } {
      Some(jsx_attribute_value_to_expression(value, ast))
    } else {
      None
    }
  } else {
    Some(ast.expression_boolean_literal(SPAN, true))
  }
}
