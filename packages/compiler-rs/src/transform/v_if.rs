use napi::{Either, bindgen_prelude::Either16};
use oxc_allocator::TakeIn;
use oxc_ast::ast::{Expression, JSXChild, JSXElement};
use oxc_span::SPAN;

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, IfIRNode, SimpleExpressionNode},
  transform::{ContextNode, TransformContext},
  utils::{
    check::{is_constant_node, is_template},
    directive::{find_prop, find_prop_mut, resolve_directive},
    error::ErrorCodes,
  },
};

/// # SAFETY
pub unsafe fn transform_v_if<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  _: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(JSXChild::Element(node)) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_template(node) && find_prop(node, Either::A("v-slot".to_string())).is_some() {
    return None;
  }
  let node = node as *mut oxc_allocator::Box<JSXElement>;

  let dir = find_prop_mut(
    unsafe { &mut *node },
    Either::B(vec![
      "v-if".to_string(),
      "v-else".to_string(),
      "v-else-if".to_string(),
    ]),
  )?;
  let seen = &mut context.seen.borrow_mut();
  let start = dir.span.start;
  if seen.contains(&start) {
    return None;
  }
  seen.insert(start);

  let mut dir = resolve_directive(dir, context);
  if dir.name != "else"
    && (dir.exp.is_none() || dir.exp.as_ref().unwrap().content.trim().is_empty())
  {
    context.options.on_error.as_ref()(ErrorCodes::VIfNoExpression, dir.loc);
    dir.exp = Some(SimpleExpressionNode {
      content: "true".to_string(),
      is_static: false,
      loc: SPAN,
      ast: None,
    });
  }

  let dynamic = &mut context_block.dynamic;
  dynamic.flags |= DynamicFlag::NonTemplate as i32;

  if dir.name == "if" {
    let id = context.reference(dynamic);
    dynamic.flags |= DynamicFlag::Insert as i32;
    let block = context_block as *mut BlockIRNode;
    let exit_block = context.create_block(
      unsafe { &mut *context_node },
      unsafe { &mut *block },
      Expression::JSXElement(oxc_allocator::Box::new_in(
        unsafe { &mut *node }.take_in(context.allocator),
        context.allocator,
      )),
      None,
    );
    return Some(Box::new(move || {
      let block = exit_block();

      context_block.dynamic.operation = Some(Box::new(Either16::A(IfIRNode {
        id,
        positive: block,
        once: *context.in_v_once.borrow()
          || is_constant_node(&dir.exp.as_ref().unwrap().ast.as_deref()),
        condition: dir.exp.unwrap(),
        negative: None,
        anchor: None,
        parent: None,
      })));
    }));
  }

  let siblings = &mut context.parent_dynamic.borrow_mut().children;
  let mut last_if_node = None;
  if !siblings.is_empty() {
    let mut i = siblings.len();
    while i > 0 {
      i -= 1;
      let sibling = siblings.get_mut(i).unwrap() as *mut IRDynamicInfo;
      if let Some(operation) = (unsafe { &mut *sibling }).operation.as_mut()
        && let Either16::A(operation) = operation.as_mut()
      {
        last_if_node = Some(operation);
        break;
      }
    }
  }

  // check if IfNode is the last operation and get the root IfNode
  let Some(mut last_if_node) = last_if_node else {
    context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, unsafe { &*node }.span);
    return None;
  };

  let mut last_if_node_ptr = last_if_node as *mut IfIRNode;
  while let Some(negative) = (unsafe { &mut *last_if_node_ptr }).negative.as_mut()
    && let Either::B(negative) = negative.as_mut()
  {
    last_if_node_ptr = negative as *mut IfIRNode;
  }
  last_if_node = unsafe { &mut *last_if_node_ptr };

  // Check if v-else was followed by v-else-if
  if dir.name == "else-if" && last_if_node.negative.is_some() {
    context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, dir.loc);
  };

  let exit_block = context.create_block(
    unsafe { &mut *context_node },
    context_block,
    Expression::JSXElement(oxc_allocator::Box::new_in(
      unsafe { &mut *node }.take_in(context.allocator),
      context.allocator,
    )),
    None,
  );

  Some(Box::new(move || {
    let block = exit_block();
    if dir.name == "else" {
      last_if_node.negative = Some(Box::new(Either::A(block)));
    } else {
      last_if_node.negative = Some(Box::new(Either::B(IfIRNode {
        id: -1,
        positive: block,
        once: *context.in_v_once.borrow()
          || is_constant_node(&dir.exp.as_ref().unwrap().ast.as_deref()),
        condition: dir.exp.unwrap(),
        parent: None,
        anchor: None,
        negative: None,
      })))
    }
  }))
}
