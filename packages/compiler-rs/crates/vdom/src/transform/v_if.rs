use std::collections::HashMap;

use napi::{Either, bindgen_prelude::Either3};
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{ConditionalExpression, Expression, JSXChild, JSXElement, NumberBase, PropertyKind},
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{IfBranchNode, NodeTypes, VNodeCall},
  ir::index::BlockIRNode,
  transform::{TransformContext, cache_static::cache_static, utils::inject_prop},
};

use common::{
  check::is_template,
  directive::{find_prop, find_prop_mut},
  error::ErrorCodes,
  text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_if<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
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

  let ast = &context.ast;

  let dir_name = dir.name.get_identifier().name;
  if dir_name != "v-else" && dir.value.is_none() {
    context.options.on_error.as_ref()(ErrorCodes::VIfNoExpression, dir.span);
    dir.value = Some(ast.jsx_attribute_value_expression_container(
      SPAN,
      ast.expression_boolean_literal(SPAN, true).into(),
    ));
  }

  let node_span = unsafe { &*node }.span;
  let mut fragment_span = SPAN;
  let mut last_if_node: Option<*mut VNodeCall> = None;

  if dir_name == "v-if" {
    fragment_span = Span::new(node_span.end, node_span.start);
    *unsafe { &mut *context_node } = context.wrap_fragment(
      Expression::JSXElement(unsafe { &mut *node }.take_in_box(context.allocator)),
      fragment_span,
    );
    let branch = if let JSXChild::Fragment(node) = unsafe { &mut *context_node }
      && let Some(child) = node.children.get_mut(0)
      && let JSXChild::Element(_) = child
    {
      Some(IfBranchNode::new(child, dir, context))
    } else {
      None
    }
    .unwrap();
    context.codegen_map.borrow_mut().insert(
      fragment_span,
      NodeTypes::VNodeCall(VNodeCall {
        tag: context.helper("Fragment"),
        props: None,
        children: None,
        patch_flag: None,
        dynamic_props: None,
        directives: None,
        is_block: true,
        disable_tracking: false,
        is_component: false,
        v_for: None,
        v_if: Some(vec![branch]),
        loc: node_span,
      }),
    );
  } else {
    let siblings = match parent_node {
      JSXChild::Element(node) => Some(&node.children),
      JSXChild::Fragment(node) => Some(&node.children),
      _ => None,
    };
    if let Some(siblings) = siblings
      && let Some(index) = siblings.iter().position(|s| s.span().eq(&node_span))
    {
      for sibling in siblings[..index].iter().rev() {
        if is_empty_text(sibling) {
          continue;
        }

        if let Some(NodeTypes::VNodeCall(sibling)) =
          context.codegen_map.borrow_mut().get_mut(&sibling.span())
          && let Some(_) = sibling.v_if
        {
          last_if_node = Some(sibling as *mut _);
          break;
        }
      }
    }

    // Check if v-else was followed by v-else-if or there are two adjacent v-else
    if let Some(last_if_node) = last_if_node
      && let Some(branchs) = &mut unsafe { &mut *last_if_node }.v_if
      && let Some(branch) = &branchs.last()
      && branch.condition.is_some()
    {
      let fragment_span = Span::new(node_span.end, node_span.start);
      *unsafe { &mut *context_node } = context.wrap_fragment(
        Expression::JSXElement(unsafe { &mut *node }.take_in_box(context.allocator)),
        fragment_span,
      );
      let branch = if let JSXChild::Fragment(node) = unsafe { &mut *context_node }
        && let Some(child) = node.children.get_mut(0)
        && let JSXChild::Element(_) = child
      {
        Some(IfBranchNode::new(child, dir, context))
      } else {
        None
      }
      .unwrap();
      branchs.push(branch);
    } else {
      context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, unsafe { &*node }.span);
      return None;
    }
  }

  // #1587: We need to dynamically increment the key based on the current
  // node's sibling nodes, since chained v-if/else branches are
  // rendered at the same depth
  let siblings = match parent_node {
    JSXChild::Element(node) => Some(&node.children),
    JSXChild::Fragment(node) => Some(&node.children),
    _ => None,
  };
  let mut key = 0;
  if let Some(siblings) = siblings {
    for sibling in siblings {
      let sibling_span = sibling.span();
      if sibling_span.eq(&fragment_span) {
        break;
      }
      if let Some(NodeTypes::VNodeCall(vnode_call)) =
        context.codegen_map.borrow().get(&sibling_span)
        && let Some(branchs) = &vnode_call.v_if
      {
        key += branchs.len();
      }
    }
  }

  // Exit callback. Complete the codegenNode when all children have been
  // transformed.
  return Some(Box::new(move || {
    let codegen_map = context.codegen_map.as_ptr();
    if dir_name == "v-if" {
      if let Some(NodeTypes::VNodeCall(fragment_codegen)) =
        unsafe { &mut *codegen_map }.get_mut(&fragment_span)
        && let Some(branchs) = &mut fragment_codegen.v_if
      {
        let branch = &mut branchs[0];
        cache_static(unsafe { &mut *branch.node }, context);
        fragment_codegen.children = Some(Either3::C(create_codegen_node_for_branch(
          branch,
          key,
          unsafe { &mut *codegen_map },
          context,
        )));
      }
    } else if let Some(if_node) = last_if_node
      && let if_node = unsafe { &mut *if_node }
      && let Some(Either3::C(children)) = &mut if_node.children
      && let Some(branchs) = if_node.v_if.as_mut()
    {
      let branch = branchs.last_mut().unwrap();
      cache_static(unsafe { &mut *branch.node }, context);
      // attach this branch's codegen node to the v-if root.
      let parent_condition = unsafe { &mut *get_parent_condition(children).unwrap() };
      parent_condition.alternate =
        create_codegen_node_for_branch(branch, key - 1, unsafe { &mut *codegen_map }, context);
    }
  }));
}

fn create_codegen_node_for_branch<'a>(
  branch: &mut IfBranchNode<'a>,
  key_index: usize,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
  context: &TransformContext<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  if let Some(condition) = &mut branch.condition {
    ast.expression_conditional(
      SPAN,
      condition.take_in(context.allocator),
      create_children_codegen_node(branch, key_index, codegen_map, context),
      // make sure to pass in asBlock: true so that the comment node call
      // closes the current block.
      ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.atom(&context.helper("createCommentVNode"))),
        NONE,
        ast.vec_from_array([
          ast.expression_string_literal(SPAN, "", None).into(),
          ast.expression_boolean_literal(SPAN, true).into(),
        ]),
        false,
      ),
    )
  } else {
    create_children_codegen_node(branch, key_index, codegen_map, context)
  }
}

pub fn create_children_codegen_node<'a>(
  branch: &mut IfBranchNode<'a>,
  key_index: usize,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
  context: &TransformContext<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  let key_property = ast.object_property(
    SPAN,
    PropertyKind::Init,
    ast.property_key_static_identifier(SPAN, "key"),
    ast.expression_numeric_literal(SPAN, key_index as f64, None, NumberBase::Hex),
    false,
    false,
    false,
  );
  let ret = if let Some(NodeTypes::VNodeCall(codegent_node)) =
    codegen_map.remove(&unsafe { &*branch.node }.span())
  {
    Some(codegent_node)
  } else {
    None
  }
  .unwrap();
  let mut vnode_call = ret;
  // Change createVNode to createBlock.
  vnode_call.is_block = true;
  inject_prop(&mut vnode_call, key_property, context);
  return context.gen_vnode_call(vnode_call, codegen_map);
}

fn get_parent_condition<'a>(
  mut node: &mut Expression<'a>,
) -> Option<*mut oxc_allocator::Box<'a, ConditionalExpression<'a>>> {
  let mut ret = None;
  loop {
    if let Expression::ConditionalExpression(exp) = node {
      ret = Some(exp as *mut _);
      if let Expression::ConditionalExpression(alternate) = &mut exp.alternate {
        ret = Some(alternate as *mut _);
        node = &mut exp.alternate
      } else {
        return ret;
      }
    } else {
      return ret;
    }
  }
}
