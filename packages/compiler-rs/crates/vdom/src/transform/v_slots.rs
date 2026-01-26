use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::ast::{Expression, JSXAttributeItem, JSXAttributeValue, JSXChild, JSXElement};
use oxc_span::SPAN;

use crate::{ast::NodeTypes, transform::TransformContext};
use common::{
  check::is_jsx_component, directive::Directives, error::ErrorCodes, patch_flag::PatchFlags,
  text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_slots<'a>(
  directives: &mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };

  let node_span = node.span;
  let node_ptr = node as *mut oxc_allocator::Box<JSXElement>;
  let is_component = is_jsx_component(node, context.options);
  if is_component {
    let mut first_child_index = None;
    for (i, child) in unsafe { &mut *node_ptr }.children.iter().enumerate() {
      if !is_empty_text(child) {
        if first_child_index.is_some() {
          first_child_index = None;
          break;
        }
        first_child_index = Some(i);
      }
    }
    if let Some(first_child_index) = first_child_index
      && let Some(child) = unsafe { &mut *node_ptr }
        .children
        .get_mut(first_child_index)
      && let JSXChild::ExpressionContainer(exp) = child
      && let Some(exp) = exp.expression.as_expression_mut()
      && (matches!(exp, Expression::ObjectExpression(_)) || exp.is_function())
    {
      let ast = &context.ast;
      unsafe { &mut *node_ptr }
        .opening_element
        .attributes
        .push(ast.jsx_attribute_item_attribute(
          SPAN,
          ast.jsx_attribute_name_identifier(SPAN, "v-slots"),
          Some(
            ast.jsx_attribute_value_expression_container(SPAN, exp.take_in(ast.allocator).into()),
          ),
        ));
      if let JSXAttributeItem::Attribute(attribute) =
        node.opening_element.attributes.last_mut().unwrap()
      {
        directives.v_slots = Some(attribute);
      }
      unsafe { &mut *node_ptr }.children.remove(first_child_index);
    }
  }

  if let Some(dir) = directives.v_slots.as_mut() {
    if !is_component {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, node_span);
      return None;
    }

    if unsafe { &*node_ptr }
      .children
      .iter()
      .any(|c| !is_empty_text(c))
    {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, node_span);
      return None;
    }

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value {
      let mut expression = value.expression.take_in(context.allocator);
      let exp = context.process_expression(expression.to_expression_mut()).0;
      Some(Box::new(move || {
        if let Some(NodeTypes::VNodeCall(vnode_call)) =
          context.codegen_map.borrow_mut().get_mut(&node_span)
        {
          *context.options.in_v_slot.borrow_mut() += 1;
          vnode_call.children = Some(Either3::C(exp));
          let mut patch_flag = vnode_call.patch_flag.unwrap_or_default();
          patch_flag |= PatchFlags::DynamicSlots as i32;
          vnode_call.patch_flag = Some(patch_flag);
          *context.options.in_v_slot.borrow_mut() -= 1;
        }
      }))
    } else {
      context.options.on_error.as_ref()(ErrorCodes::VSlotsNoExpression, dir.span);
      None
    }
  } else {
    None
  }
}
