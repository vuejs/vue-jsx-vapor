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
  transform::{TransformContext, cache_static::cache_static_children, utils::inject_prop},
};

use common::{
  check::is_template,
  directive::{find_prop, find_prop_mut},
  error::ErrorCodes,
};

/// # SAFETY
pub unsafe fn transform_v_if<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let is_template_node = is_template(node);
  if is_template_node && find_prop(node, Either::A("v-slot".to_string())).is_some() {
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
  let mut last_if_span = None;

  let context_v_if_map = &mut context.v_if_map.borrow_mut();
  let v_if_map = context_v_if_map.entry(parent_node.span()).or_default();

  if dir_name == "v-if" {
    fragment_span = Span::new(node_span.end, node_span.start + 1);
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
        tag: if is_template_node {
          context.helper("Fragment")
        } else {
          String::new()
        },
        props: None,
        children: None,
        patch_flag: None,
        dynamic_props: None,
        directives: None,
        is_block: true,
        disable_tracking: false,
        is_component: true,
        v_for: None,
        v_if: Some(vec![branch]),
        loc: node_span,
      }),
    );
    v_if_map.1.push(fragment_span);
  } else {
    let mut last_if_node: Option<*mut VNodeCall> = None;
    if let Some(v_if_span) = v_if_map.1.last()
      && let Some(NodeTypes::VNodeCall(v_if)) = context.codegen_map.borrow_mut().get_mut(v_if_span)
    {
      last_if_span = Some(*v_if_span);
      last_if_node = Some(v_if as *mut _);
    }

    // Check if v-else was followed by v-else-if or there are two adjacent v-else
    if let Some(last_if_node) = last_if_node
      && let Some(branchs) = unsafe { &mut *last_if_node }.v_if.as_mut()
      && let Some(branch) = &branchs.last()
      && branch.condition.is_some()
    {
      *unsafe { &mut *context_node } = context.wrap_fragment(
        Expression::JSXElement(unsafe { &mut *node }.take_in_box(context.allocator)),
        Span::new(node_span.end, node_span.start),
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

  let key = v_if_map.0;
  v_if_map.0 += 1;

  // Exit callback. Complete the codegenNode when all children have been
  // transformed.
  Some(Box::new(move || {
    let codegen_map_ptr = context.codegen_map.as_ptr();
    let codegen_map = unsafe { &mut *codegen_map_ptr };
    if dir_name == "v-if" {
      if let Some(NodeTypes::VNodeCall(fragment_codegen)) =
        unsafe { &mut *codegen_map_ptr }.get_mut(&fragment_span)
        && let Some(branchs) = &mut fragment_codegen.v_if
      {
        let branch = &mut branchs[0];
        fragment_codegen.children = Some(Either3::C(create_codegen_node_for_branch(
          branch,
          key,
          context,
          codegen_map,
        )));
      }
    } else if let Some(NodeTypes::VNodeCall(if_node)) =
      unsafe { &mut *codegen_map_ptr }.get_mut(&last_if_span.unwrap())
      && let Some(Either3::C(children)) = &mut if_node.children
      && let Some(branchs) = if_node.v_if.as_mut()
      && let Some(branch) = branchs.last_mut()
    {
      // attach this branch's codegen node to the v-if root.
      let parent_condition = unsafe { &mut *get_parent_condition(children).unwrap() };
      parent_condition.alternate =
        create_codegen_node_for_branch(branch, key, context, codegen_map);
    }
  }))
}

fn create_codegen_node_for_branch<'a>(
  branch: &mut IfBranchNode<'a>,
  key_index: usize,
  context: &'a TransformContext<'a>,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
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
  context: &'a TransformContext<'a>,
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
  let span = unsafe { &*branch.node }.span();
  if let Some(NodeTypes::VNodeCall(vnode_call)) = codegen_map.get_mut(&span) {
    // Change createVNode to createBlock.
    vnode_call.is_block = true;
    inject_prop(vnode_call, key_property, context);
  }
  cache_static_children(
    None,
    vec![unsafe { &mut *branch.node }],
    context,
    codegen_map,
    false,
  );
  match codegen_map.remove(&span).unwrap() {
    NodeTypes::VNodeCall(vnode_call) => context.gen_vnode_call(vnode_call, codegen_map),
    NodeTypes::TextCallNode(exp) => exp,
    NodeTypes::CacheExpression(exp) => exp,
  }
}

pub fn get_parent_condition<'a>(
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
