use napi::{
  Either,
  bindgen_prelude::{Either3, Either16},
};
use oxc_allocator::TakeIn;
use oxc_ast::ast::{
  BinaryExpression, Expression, JSXAttribute, JSXAttributeValue, JSXChild, JSXElement,
};

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, ForIRNode, IRFor, SimpleExpressionNode},
  transform::{ContextNode, TransformContext},
  utils::{
    check::{is_constant_node, is_jsx_component, is_template},
    directive::{find_prop, find_prop_mut},
    error::ErrorCodes,
    text::is_empty_text,
  },
};

/// # SAFETY
pub unsafe fn transform_v_for<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(JSXChild::Element(node)) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let node = node as *mut oxc_allocator::Box<JSXElement>;
  if is_template(unsafe { &*node })
    && find_prop(unsafe { &*node }, Either::A("v-slot".to_string())).is_some()
  {
    return None;
  }

  let dir = find_prop_mut(unsafe { &mut *node }, Either::A("v-for".to_string()))?;
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

  let key_prop = if let Some(key_prop) =
    find_prop_mut(unsafe { &mut *node }, Either::A("key".to_string()))
    && let Some(value) = &mut key_prop.value
  {
    Some(SimpleExpressionNode::new(Either3::C(value), context))
  } else {
    None
  };

  let component =
    is_jsx_component(unsafe { &*node }) || is_template_with_single_component(unsafe { &*node });
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
    Some(true),
  );

  // if v-for is the only child of a parent element, it can go the fast path
  // when the entire list is emptied
  let mut only_child = false;
  if let Either::B(JSXChild::Element(parent_node)) = parent_node
    && !is_jsx_component(parent_node)
  {
    let index = *context.index.borrow() as usize;
    for (i, child) in parent_node.children.iter().enumerate() {
      let child = if index == i {
        match unsafe { &mut *context_node } {
          Either::A(_) => child,
          Either::B(node) => node,
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

    context_block.dynamic.operation = Some(Box::new(Either16::B(ForIRNode {
      id,
      value,
      key,
      index,
      key_prop,
      render: block,
      once: *context.in_v_once.borrow() || is_constant_node(&source.ast.as_deref()),
      source,
      component,
      only_child,
      parent: None,
      anchor: None,
    })));
  }))
}

pub fn get_for_parse_result<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<IRFor<'a>> {
  let mut value: Option<SimpleExpressionNode> = None;
  let mut index: Option<SimpleExpressionNode> = None;
  let mut key: Option<SimpleExpressionNode> = None;
  let mut source: Option<SimpleExpressionNode> = None;
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
          .map(|e| SimpleExpressionNode::new(Either3::A(e), context));
        key = unsafe { &mut *expressions }
          .get_mut(1)
          .map(|e| SimpleExpressionNode::new(Either3::A(e), context));
        index = unsafe { &mut *expressions }
          .get_mut(2)
          .map(|e| SimpleExpressionNode::new(Either3::A(e), context));
      } else {
        value = Some(SimpleExpressionNode::new(Either3::A(left), context));
      };
      source = Some(SimpleExpressionNode::new(
        Either3::A(&mut unsafe { &mut *expression }.right),
        context,
      ));
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

fn is_template_with_single_component<'a>(node: &'a JSXElement<'a>) -> bool {
  let non_comment_children = node
    .children
    .iter()
    .filter(|c| !is_empty_text(c))
    .collect::<Vec<_>>();

  non_comment_children.len() == 1
    && matches!(non_comment_children[0],JSXChild::Element(child)if is_jsx_component(child))
}
