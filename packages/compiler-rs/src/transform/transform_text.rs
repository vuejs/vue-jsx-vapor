use std::collections::HashSet;

use napi::{
  Either,
  bindgen_prelude::{Either3, Either16},
};
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::ast::{ConditionalExpression, Expression, JSXChild, LogicalExpression};
use oxc_span::GetSpan;

use crate::{
  ir::index::{
    BlockIRNode, CreateNodesIRNode, DynamicFlag, GetTextChildIRNode, IfIRNode, SetNodesIRNode,
    SimpleExpressionNode,
  },
  transform::{ContextNode, TransformContext},
  utils::{
    check::{is_constant_node, is_fragment_node, is_jsx_component, is_template},
    directive::find_prop,
    text::{is_empty_text, resolve_jsx_text},
  },
};

/// # SAFETY
pub unsafe fn transform_text<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let dynamic = &mut context_block.dynamic;
  let span = node.span();
  let seen = &mut context.seen.borrow_mut();
  if seen.contains(&span.start) {
    dynamic.flags |= DynamicFlag::NonTemplate as i32;
    return None;
  }

  match node {
    JSXChild::Element(node) if !is_jsx_component(node) => {
      let is_template = is_template(node);
      let children = &mut node.children.iter_mut().collect() as *mut _;
      process_children(
        is_template,
        unsafe { &mut *children },
        context,
        context_block,
        seen,
      )
    }
    JSXChild::Fragment(node) => {
      let children = &mut node.children.iter_mut().collect() as *mut _;
      process_children(
        true,
        unsafe { &mut *children },
        context,
        context_block,
        seen,
      )
    }
    JSXChild::ExpressionContainer(node) => {
      if let Some(expression) = node.expression.as_expression_mut() {
        match expression
          .without_parentheses_mut()
          .get_inner_expression_mut()
        {
          Expression::ConditionalExpression(expression) => {
            return Some(process_conditional_expression(
              expression,
              unsafe { &mut *context_node },
              context,
              context_block,
              parent_node,
            ));
          }
          Expression::LogicalExpression(expression) => {
            return Some(process_logical_expression(
              expression,
              unsafe { &mut *context_node },
              context,
              context_block,
              parent_node,
            ));
          }
          _ => process_interpolation(
            unsafe { &mut *context_node },
            context,
            context_block,
            parent_node,
            seen,
          ),
        }
      } else {
        dynamic.flags |= DynamicFlag::NonTemplate as i32;
      }
    }
    JSXChild::Text(node) => {
      let value = resolve_jsx_text(node);
      if !value.is_empty() {
        let mut template = context.template.borrow_mut();
        *template = template.to_string() + &value;
      } else {
        dynamic.flags |= DynamicFlag::NonTemplate as i32;
      }
    }
    _ => (),
  };
  None
}

fn process_children<'a>(
  is_fragment: bool,
  children: &'a mut Vec<&mut JSXChild<'a>>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  seen: &mut HashSet<u32>,
) {
  if !children.is_empty() {
    let mut has_interp = false;
    let mut is_all_text_like = true;
    for child in children.iter() {
      if let JSXChild::ExpressionContainer(child) = child {
        let exp = child.expression.as_expression();
        if if let Some(exp) = exp {
          !matches!(
            exp.without_parentheses().get_inner_expression(),
            Expression::ConditionalExpression(_) | Expression::LogicalExpression(_),
          )
        } else {
          false
        } {
          has_interp = true
        }
      } else if !matches!(child, JSXChild::Text(_)) {
        is_all_text_like = false
      }
    }

    // all text like with interpolation
    if !is_fragment && is_all_text_like && has_interp {
      process_text_container(children, context, context_block, seen)
    } else if has_interp {
      // check if there's any text before interpolation, it needs to be merged
      for (i, child) in children.iter().enumerate() {
        let prev = if i > 0 { children.get(i - 1) } else { None };
        if let JSXChild::ExpressionContainer(_) = child
          && let Some(JSXChild::Text(_)) = prev
        {
          // mark leading text node for skipping
          mark_non_template(prev.unwrap(), seen);
        }
      }
    }
  }
}

fn process_interpolation<'a>(
  context_node: &'a mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
  seen: &mut HashSet<u32>,
) {
  let children = match parent_node {
    ContextNode::A(e) => &mut e.children,
    ContextNode::B(e) => match e {
      JSXChild::Element(e) => &mut e.children,
      JSXChild::Fragment(e) => &mut e.children,
      _ => return,
    },
  };
  if children.is_empty() {
    return;
  }
  let children = children as *mut oxc_allocator::Vec<JSXChild>;
  let index = *context.index.borrow() as usize;
  let nodes: &mut Vec<_> = &mut (unsafe { &mut *children })[index..].iter_mut().collect();
  match context_node {
    Either::A(_) => (),
    Either::B(node) => nodes[0] = node,
  };
  let idx = nodes.iter().position(|n| !is_text_like(n));
  if let Some(idx) = idx {
    nodes.truncate(idx)
  };

  // merge leading text
  if index > 0
    && let Some(prev) = (unsafe { &mut *children }).get_mut(index - 1)
    && let JSXChild::Text(_) = prev
  {
    nodes.insert(0, prev);
  };

  let nodes = nodes as *mut Vec<&mut JSXChild>;
  let values = process_text_like_expressions(unsafe { &mut *nodes }, context, seen);
  let dynamic = &mut context_block.dynamic;
  if values.is_empty() {
    dynamic.flags |= DynamicFlag::NonTemplate as i32;
    return;
  }

  let id = context.reference(dynamic);
  let once = *context.in_v_once.borrow();
  if match parent_node {
    Either::A(_) => true,
    Either::B(parent) => {
      is_fragment_node(parent)
        || matches!(parent, JSXChild::Element(parent) if find_prop(parent, Either::A(String::from("v-slot"))).is_some())
    }
  } {
    context.register_operation(
      context_block,
      Either16::K(CreateNodesIRNode {
        create_nodes: true,
        id,
        once,
        values,
      }),
      None,
    );
  } else {
    let mut template = context.template.borrow_mut();
    *template = template.to_string() + " ";
    context.register_operation(
      context_block,
      Either16::G(SetNodesIRNode {
        set_nodes: true,
        element: id,
        once,
        values,
        generated: None,
      }),
      None,
    );
  };
}

fn mark_non_template(node: &JSXChild, seen: &mut HashSet<u32>) {
  seen.insert(node.span().start);
}

fn process_text_container<'a>(
  children: &'a mut Vec<&mut JSXChild<'a>>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  seen: &mut HashSet<u32>,
) {
  let values = process_text_like_expressions(children, context, seen);
  let literals = values
    .iter()
    .map(|e| e.get_literal_expression_value())
    .collect::<Vec<Option<String>>>();
  if literals.iter().all(|l| l.is_some()) {
    *context.children_template.borrow_mut() = literals.into_iter().flatten().collect();
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
    context.register_operation(
      context_block,
      Either16::G(SetNodesIRNode {
        set_nodes: true,
        element,
        once: *context.in_v_once.borrow(),
        values,
        // indicates this node is generated, so prefix should be "x" instead of "n"
        generated: Some(true),
      }),
      None,
    );
  }
}

fn process_text_like_expressions<'a>(
  nodes: &'a mut Vec<&mut JSXChild<'a>>,
  context: &'a TransformContext<'a>,
  seen: &mut HashSet<u32>,
) -> Vec<SimpleExpressionNode<'a>> {
  let mut values = vec![];
  for node in nodes {
    mark_non_template(node, seen);
    if is_empty_text(node) {
      continue;
    }
    values.push(SimpleExpressionNode::new(Either3::B(node), context))
  }
  values
}

fn is_text_like(node: &JSXChild) -> bool {
  if let JSXChild::ExpressionContainer(node) = node {
    !matches!(
      node
        .expression
        .to_expression()
        .without_parentheses()
        .get_inner_expression(),
      Expression::ConditionalExpression(_) | Expression::LogicalExpression(_)
    )
  } else {
    matches!(node, JSXChild::Text(_))
  }
}

pub fn process_conditional_expression<'a>(
  node: &'a mut ConditionalExpression<'a>,
  context_node: &'a mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) -> Box<dyn FnOnce() + 'a> {
  let test = &mut node.test;
  let consequent = node
    .consequent
    .without_parentheses_mut()
    .get_inner_expression_mut()
    .take_in(context.allocator);
  let alternate = node
    .alternate
    .without_parentheses_mut()
    .get_inner_expression_mut()
    .take_in(context.allocator);

  let dynamic = &mut context_block.dynamic;
  dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  let id = context.reference(dynamic);
  let block = context_block as *mut BlockIRNode;
  let exit_block = context.create_block(context_node, unsafe { &mut *block }, consequent, None);

  let is_const_test = is_constant_node(&Some(test));
  let test = SimpleExpressionNode::new(Either3::A(test), context);

  Box::new(move || {
    let block = exit_block();

    let mut operation = IfIRNode {
      id,
      positive: block,
      once: *context.in_v_once.borrow() || is_const_test,
      condition: test,
      negative: None,
      parent: None,
      anchor: None,
    };
    let _context_block = context_block as *mut BlockIRNode;
    set_negative(
      alternate,
      &mut operation,
      context_node,
      context,
      unsafe { &mut *_context_block },
      parent_node,
    );
    context_block.dynamic.operation = Some(Box::new(Either16::A(operation)));
  })
}

fn process_logical_expression<'a>(
  node: &'a mut LogicalExpression<'a>,
  context_node: &'a mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) -> Box<dyn FnOnce() + 'a> {
  let left = node
    .left
    .without_parentheses_mut()
    .get_inner_expression_mut();
  let right = node
    .right
    .without_parentheses_mut()
    .get_inner_expression_mut()
    .take_in(context.allocator);

  let dynamic = &mut context_block.dynamic;
  dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  let id = context.reference(dynamic);
  let block = context_block as *mut BlockIRNode;
  let (_left, _right) = if node.operator.is_and() {
    (right, left.clone_in(context.allocator))
  } else {
    (left.clone_in(context.allocator), right)
  };
  let exit_block = context.create_block(context_node, unsafe { &mut *block }, _left, None);

  Box::new(move || {
    let block = exit_block();

    let mut operation = IfIRNode {
      id,
      positive: block,
      once: *context.in_v_once.borrow() || is_constant_node(&Some(left)),
      condition: SimpleExpressionNode::new(Either3::A(left), context),
      negative: None,
      anchor: None,
      parent: None,
    };
    let _context_block = context_block as *mut BlockIRNode;

    set_negative(
      _right,
      &mut operation,
      context_node,
      context,
      unsafe { &mut *_context_block },
      parent_node,
    );

    context_block.dynamic.operation = Some(Box::new(Either16::A(operation)));
  })
}

fn set_negative<'a>(
  mut node: Expression<'a>,
  operation: &mut IfIRNode<'a>,
  context_node: &mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) {
  let node = node.without_parentheses_mut().get_inner_expression_mut();
  if let Expression::ConditionalExpression(node) = node {
    let node = node as *mut oxc_allocator::Box<ConditionalExpression>;
    let _context_block = context_block as *mut BlockIRNode;
    let exit_block = context.create_block(
      context_node,
      unsafe { &mut *_context_block },
      unsafe { &mut *node }
        .consequent
        .without_parentheses_mut()
        .get_inner_expression_mut()
        .take_in(context.allocator),
      None,
    );
    context.transform_node(Some(unsafe { &mut *_context_block }), Some(parent_node));
    let block = exit_block();
    let mut negative = IfIRNode {
      id: -1,
      condition: SimpleExpressionNode::new(Either3::A(&mut unsafe { &mut *node }.test), context),
      positive: block,
      once: *context.in_v_once.borrow() || is_constant_node(&Some(&unsafe { &*node }.test)),
      negative: None,
      anchor: None,
      parent: None,
    };
    set_negative(
      unsafe { &mut *node }.alternate.take_in(context.allocator),
      &mut negative,
      context_node,
      context,
      context_block,
      parent_node,
    );
    operation.negative = Some(Box::new(Either::B(negative)));
  } else if let Expression::LogicalExpression(node) = node {
    let node = node as *mut oxc_allocator::Box<LogicalExpression>;
    let left = unsafe { &mut *node }
      .left
      .without_parentheses_mut()
      .get_inner_expression_mut();
    let right = unsafe { &mut *node }
      .right
      .without_parentheses_mut()
      .get_inner_expression_mut()
      .take_in(context.allocator);
    let (_left, mut _right) = if unsafe { &mut *node }.operator.is_and() {
      (right, left.clone_in(context.allocator))
    } else {
      (left.clone_in(context.allocator), right)
    };
    let block = context_block as *mut BlockIRNode;
    let exit_block = context.create_block(context_node, unsafe { &mut *block }, _left, None);
    context.transform_node(Some(unsafe { &mut *block }), Some(parent_node));
    let block = exit_block();
    let mut negative = IfIRNode {
      id: -1,
      once: *context.in_v_once.borrow() || is_constant_node(&Some(left)),
      condition: SimpleExpressionNode::new(Either3::A(left), context),
      positive: block,
      negative: None,
      anchor: None,
      parent: None,
    };
    set_negative(
      _right,
      &mut negative,
      context_node,
      context,
      context_block,
      parent_node,
    );
    operation.negative = Some(Box::new(Either::B(negative)));
  } else {
    let block = context_block as *mut BlockIRNode;
    let exit_block = context.create_block(
      context_node,
      unsafe { &mut *block },
      node.take_in(context.allocator),
      None,
    );
    context.transform_node(Some(context_block), Some(parent_node));
    let block = exit_block();
    operation.negative = Some(Box::new(Either::A(block)));
  }
}
