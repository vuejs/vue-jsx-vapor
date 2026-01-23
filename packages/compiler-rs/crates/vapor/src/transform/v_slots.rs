use napi::bindgen_prelude::{Either3, Either4};
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{
    Expression, FormalParameterKind, JSXAttributeItem, JSXAttributeValue, JSXChild, JSXElement,
    ObjectPropertyKind, PropertyKind,
  },
};
use oxc_span::SPAN;

use crate::{
  ir::{
    component::{IRSlotType, IRSlotsExpression},
    index::BlockIRNode,
  },
  transform::TransformContext,
};
use common::{
  check::is_jsx_component, directive::Directives, error::ErrorCodes,
  expression::SimpleExpressionNode, text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_slots<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };

  let ast = &context.ast;
  let node_ptr = node as *mut oxc_allocator::Box<JSXElement>;
  let is_component = is_jsx_component(node);
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
    if !is_jsx_component(unsafe { &*node_ptr }) {
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, unsafe { &*node_ptr }.span);
      return None;
    }

    if unsafe { &*node_ptr }
      .children
      .iter()
      .any(|c| !is_empty_text(c))
    {
      context.options.on_error.as_ref()(
        ErrorCodes::VSlotMixedSlotUsage,
        unsafe { &*node_ptr }.span,
      );
      return None;
    }

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value {
      let expression = value.expression.to_expression_mut();
      if expression.is_function() {
        *expression = ast.expression_object(
          SPAN,
          ast.vec1(ast.object_property_kind_object_property(
            SPAN,
            PropertyKind::Init,
            ast.property_key_static_identifier(SPAN, "default"),
            expression.take_in(context.allocator),
            false,
            false,
            false,
          )),
        );
      } else if let Expression::ObjectExpression(exp) = expression {
        for prop in exp.properties.iter() {
          if let ObjectPropertyKind::ObjectProperty(prop) = prop
            && prop.computed
          {
            *expression = ast.expression_arrow_function(
              SPAN,
              true,
              false,
              NONE,
              ast.formal_parameters(
                SPAN,
                FormalParameterKind::ArrowFormalParameters,
                ast.vec(),
                NONE,
              ),
              NONE,
              ast.function_body(
                SPAN,
                ast.vec(),
                ast.vec1(ast.statement_expression(SPAN, expression.take_in(context.allocator))),
              ),
            );
            break;
          }
        }
      }
      let slots = SimpleExpressionNode::new(Either3::A(expression), context.source_text);
      Some(Box::new(move || {
        context_block.slots = vec![Either4::D(IRSlotsExpression {
          slot_type: IRSlotType::EXPRESSION,
          slots,
        })];
      }))
    } else {
      context.options.on_error.as_ref()(ErrorCodes::VSlotsNoExpression, dir.span);
      None
    }
  } else {
    None
  }
}
