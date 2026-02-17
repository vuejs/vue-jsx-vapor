use oxc_allocator::TakeIn;
use oxc_ast::ast::{
  BinaryExpression, Expression, JSXAttribute, JSXAttributeValue, JSXChild, JSXElement,
};

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, ForIRNode, IRFor, OperationNode},
  transform::TransformContext,
};
use common::{
  ast::RootNode,
  check::{is_constant_node, is_jsx_component, is_template},
  directive::Directives,
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_for<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let node = node as *mut oxc_allocator::Box<JSXElement>;
  if is_template(unsafe { &*node }) && directives.v_slot.is_some() {
    return None;
  }

  let dir = directives.v_for.as_mut()?;
  let seen = &mut context.seen.borrow_mut();
  let span = dir.span;
  if seen.contains(&span.start) {
    return None;
  }
  seen.insert(span.start);

  let IRFor {
    value,
    index,
    key,
    source,
  } = get_for_parse_result(dir, context)?;

  let Some(source) = source else {
    context.options.on_error.as_ref()(ErrorCodes::VForMalformedExpression, span);
    return None;
  };

  let key_prop = if let Some(key_prop) = directives.key.as_mut()
    && let Some(value) = &mut key_prop.value
  {
    seen.insert(key_prop.span.start);
    Some(jsx_attribute_value_to_expression(value, context.ast))
  } else {
    None
  };

  let is_component = is_jsx_component(unsafe { &*node }, true, context.options)
    || is_template_with_single_component(unsafe { &*node }, context);
  let dynamic = &mut context_block.dynamic;
  let id = context.reference(dynamic);
  dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  let block = context_block as *mut BlockIRNode;
  let exit_block = context.create_block(
    unsafe { &mut *context_node },
    unsafe { &mut *block },
    Expression::JSXElement(oxc_allocator::Box::new_in(
      unsafe { &mut *node }.take_in(context.allocator),
      context.allocator,
    )),
    true,
  );

  // if v-for is the only child of a parent element, it can go the fast path
  // when the entire list is emptied
  let mut only_child = false;
  if let JSXChild::Element(parent_node) = parent_node
    && !is_jsx_component(parent_node, true, context.options)
  {
    let index = *context.index.borrow() as usize;
    for (i, child) in parent_node.children.iter().enumerate() {
      let child = if index == i {
        if RootNode::is_root(unsafe { &*context_node }) {
          child
        } else {
          unsafe { &*context_node }
        }
      } else {
        child
      };
      if !is_empty_text(child) {
        if only_child {
          only_child = false;
          break;
        }
        only_child = true;
      }
    }
  };

  Some(Box::new(move || {
    let block = exit_block();

    context_block.dynamic.operation = Some(Box::new(OperationNode::For(ForIRNode {
      id,
      value,
      key,
      index,
      key_prop,
      render: block,
      once: *context.in_v_once.borrow() || is_constant_node(&source),
      source,
      component: is_component,
      only_child,
      parent: None,
      anchor: None,
      logical_index: None,
      append: false,
      last: false,
    })));
  }))
}

pub fn get_for_parse_result<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<IRFor<'a>> {
  let mut value: Option<Expression> = None;
  let mut index: Option<Expression> = None;
  let mut key: Option<Expression> = None;
  let mut source: Option<Expression> = None;
  if let Some(dir_value) = &mut dir.value {
    let expression = if let JSXAttributeValue::ExpressionContainer(dir_value) = dir_value {
      Some(
        dir_value
          .expression
          .to_expression_mut()
          .without_parentheses_mut()
          .get_inner_expression_mut(),
      )
    } else {
      None
    };
    if let Some(expression) = expression
      && let Expression::BinaryExpression(expression) = expression
    {
      let expression = expression as *mut oxc_allocator::Box<BinaryExpression>;
      let left = unsafe { &mut *expression }
        .left
        .without_parentheses_mut()
        .get_inner_expression_mut();
      if let Expression::SequenceExpression(left) = left {
        let expressions = &mut left.expressions as *mut oxc_allocator::Vec<Expression>;
        value = unsafe { &mut *expressions }
          .get_mut(0)
          .map(|e| e.take_in(context.allocator));
        key = unsafe { &mut *expressions }
          .get_mut(1)
          .map(|e| e.take_in(context.allocator));
        index = unsafe { &mut *expressions }
          .get_mut(2)
          .map(|e| e.take_in(context.allocator));
      } else {
        value = Some(left.take_in(context.allocator));
      };
      source = Some((unsafe { &mut *expression }.right).take_in(context.allocator));
    }
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VForNoExpression, dir.span);
    return None;
  }
  Some(IRFor {
    value,
    index,
    key,
    source,
  })
}

fn is_template_with_single_component<'a>(
  node: &'a JSXElement<'a>,
  context: &TransformContext,
) -> bool {
  let non_comment_children = node
    .children
    .iter()
    .filter(|c| !is_empty_text(c))
    .collect::<Vec<_>>();

  non_comment_children.len() == 1
    && matches!(non_comment_children[0], JSXChild::Element(child)
      if is_jsx_component(child, true, context.options)
    )
}
