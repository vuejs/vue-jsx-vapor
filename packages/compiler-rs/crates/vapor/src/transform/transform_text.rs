use std::{borrow::Cow, collections::HashSet};

use napi::Either;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{ConditionalExpression, Expression, JSXChild};
use oxc_span::GetSpan;

use crate::{
  ir::index::{
    BlockIRNode, CreateNodesIRNode, DynamicFlag, GetTextChildIRNode, IfIRNode, OperationNode,
    SetNodesIRNode,
  },
  transform::{TransformContext, v_if::encode_if_block_shape},
};

use common::{
  ast::RootNode,
  check::{is_constant_node, is_custom_element, is_fragment_node, is_jsx_component, is_template},
  directive::Directives,
  patch_flag::VaporBlockShape,
  text::{
    escape_html, get_tag_name, get_text_like_value, is_empty_text, is_text_like, resolve_jsx_text,
  },
};

/// # SAFETY
pub unsafe fn transform_text<'a>(
  directives: &Directives,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let node = unsafe { &mut *context_node };
  let dynamic = &mut context_block.dynamic;
  let span = node.span();
  let seen = &mut context.seen.borrow_mut();
  if seen.contains(&span.start) {
    dynamic.flags |= DynamicFlag::NonTemplate as i32;
    return None;
  }

  match node {
    JSXChild::Element(node) if !directives.is_component => {
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
      // Check if this is a root-level text node (parent is ROOT or fragment)
      // Root-level text nodes go through createNode() which doesn't need escaping
      // Element children go through innerHTML which needs escaping
      let is_root_text = RootNode::is_root(parent_node)
        || if let JSXChild::Element(parent_node) = parent_node {
          is_jsx_component(parent_node)
            || is_custom_element(parent_node)
            || get_tag_name(parent_node, context.options) == "template"
        } else {
          false
        };
      let value = if is_root_text {
        resolve_jsx_text(node)
      } else {
        escape_html(resolve_jsx_text(node))
      };
      if !value.is_empty() {
        let mut template = context.template.borrow_mut();
        *template = format!("{}{}", template, value);
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
            Expression::ConditionalExpression(_),
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
  context_node: &'a mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
  seen: &mut HashSet<u32>,
) {
  let children = match parent_node {
    JSXChild::Element(e) => &mut e.children,
    JSXChild::Fragment(e) => &mut e.children,
    _ => return,
  };
  if children.is_empty() {
    return;
  }
  let children = children as *mut oxc_allocator::Vec<JSXChild>;
  let index = *context.index.borrow() as usize;
  let nodes: &mut Vec<_> = &mut (unsafe { &mut *children })[index..].iter_mut().collect();
  if !RootNode::is_root(context_node) {
    nodes[0] = context_node
  }
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
  if values.is_empty() {
    return;
  }

  let id = context.reference(&mut context_block.dynamic);
  let once = *context.in_v_once.borrow();
  if if RootNode::is_root(parent_node) {
    true
  } else {
    is_fragment_node(parent_node)
      || if let JSXChild::Element(parent_node) = parent_node {
        is_jsx_component(parent_node) || is_custom_element(parent_node)
      } else {
        false
      }
  } {
    context.register_operation(
      context_block,
      OperationNode::CreateNodes(CreateNodesIRNode {
        create_nodes: true,
        id,
        once,
        values,
      }),
      None,
    );
  } else {
    let mut template = context.template.borrow_mut();
    *template = format!("{} ", template);
    context.register_operation(
      context_block,
      OperationNode::SetNodes(SetNodesIRNode {
        set_nodes: true,
        element: id,
        once,
        values,
        generated: false,
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
    .map(|e| get_text_like_value(e, false))
    .collect::<Vec<_>>();
  if literals.iter().all(|l| l.is_some()) {
    *context.children_template.borrow_mut() = literals
      .into_iter()
      .flatten()
      .map(escape_html)
      .collect::<Vec<_>>();
  } else {
    *context.children_template.borrow_mut() = vec![Cow::Borrowed(" ")];
    let parent = context.reference(&mut context_block.dynamic);
    context.register_operation(
      context_block,
      OperationNode::GetTextChild(GetTextChildIRNode {
        get_text_child: true,
        parent,
      }),
      None,
    );
    let element = context.reference(&mut context_block.dynamic);
    context.register_operation(
      context_block,
      OperationNode::SetNodes(SetNodesIRNode {
        set_nodes: true,
        element,
        once: *context.in_v_once.borrow(),
        values,
        // indicates this node is generated, so prefix should be "x" instead of "n"
        generated: true,
      }),
      None,
    );
  }
}

fn process_text_like_expressions<'a>(
  nodes: &'a mut Vec<&mut JSXChild<'a>>,
  context: &'a TransformContext<'a>,
  seen: &mut HashSet<u32>,
) -> Vec<Expression<'a>> {
  let ast = context.ast;
  let mut values = vec![];
  for node in nodes {
    mark_non_template(node, seen);
    if is_empty_text(node) {
      continue;
    }
    values.push(match node {
      JSXChild::ExpressionContainer(node) => node
        .expression
        .to_expression_mut()
        .take_in(context.allocator),
      JSXChild::Text(node) => {
        ast.expression_string_literal(node.span, ast.atom(&resolve_jsx_text(node)), None)
      }
      JSXChild::Element(node) => Expression::JSXElement(node.take_in_box(context.allocator)),
      JSXChild::Fragment(node) => Expression::JSXFragment(node.take_in_box(context.allocator)),
      JSXChild::Spread(node) => node.expression.take_in(ast.allocator),
    });
  }
  values
}

pub fn process_conditional_expression<'a>(
  node: &'a mut ConditionalExpression<'a>,
  context_node: &'a mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
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
  let exit_block = context.create_block(context_node, unsafe { &mut *block }, consequent, false);

  let is_const_test = is_constant_node(test);
  let test = test.take_in(context.allocator);

  Box::new(move || {
    let block = exit_block();

    let mut operation = IfIRNode {
      id,
      block_shape: encode_if_block_shape(&block, None),
      positive: block,
      index: context.next_if_index(),
      once: *context.in_v_once.borrow() || is_const_test,
      condition: test,
      negative: None,
      parent: None,
      anchor: None,
      logical_index: None,
      append: false,
      last: false,
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
    context_block.dynamic.operation = Some(Box::new(OperationNode::If(operation)));
  })
}

fn set_negative<'a>(
  mut node: Expression<'a>,
  operation: &mut IfIRNode<'a>,
  context_node: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
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
      false,
    );
    context.transform_node(Some(unsafe { &mut *_context_block }), Some(parent_node));
    let block = exit_block();
    let test = &mut unsafe { &mut *node }.test;
    let mut negative = IfIRNode {
      id: -1,
      block_shape: VaporBlockShape::Empty as i32,
      once: *context.in_v_once.borrow() || is_constant_node(test),
      condition: test.take_in(context.allocator),
      positive: block,
      index: context.next_if_index(),
      negative: None,
      anchor: None,
      logical_index: None,
      parent: None,
      append: false,
      last: false,
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
  } else {
    let block = context_block as *mut BlockIRNode;
    let exit_block = context.create_block(
      context_node,
      unsafe { &mut *block },
      node.take_in(context.allocator),
      false,
    );
    context.transform_node(Some(context_block), Some(parent_node));
    let block = exit_block();
    operation.negative = Some(Box::new(Either::A(block)));
  }

  if let Some(negative) = operation.negative.as_mut()
    && let Either::B(negative) = negative.as_mut()
  {
    negative.block_shape = encode_if_block_shape(&negative.positive, None)
  }
  operation.block_shape = encode_if_block_shape(&operation.positive, operation.negative.as_ref())
}
