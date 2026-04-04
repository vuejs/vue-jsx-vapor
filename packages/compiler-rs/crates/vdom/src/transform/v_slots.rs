use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{
    Expression, JSXAttributeItem, JSXAttributeValue, JSXChild, JSXElement, NumberBase,
    ObjectPropertyKind,
  },
};
use oxc_span::{GetSpan, SPAN};

use crate::{ast::NodeTypes, transform::TransformContext};
use common::{
  directive::Directives, error::ErrorCodes, options::SlotScope, patch_flag::PatchFlags,
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

  let ast = context.ast;
  let node_span = node.span;
  let node_ptr = node as *mut oxc_allocator::Box<JSXElement>;
  let is_component = directives.is_component;
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

    if let Some(JSXAttributeValue::ExpressionContainer(value)) = &mut dir.value
      && let Some(expression) = value.expression.as_expression_mut()
    {
      *context.options.in_v_slot.borrow_mut() += 1;
      if expression.is_function() {
        *expression = ast.expression_object(
          SPAN,
          ast.vec1(ast.object_property_kind_object_property(
            SPAN,
            oxc_ast::ast::PropertyKind::Init,
            ast.property_key_static_identifier(SPAN, "default"),
            expression.take_in(context.allocator),
            false,
            false,
            false,
          )),
        )
      }
      let mut has_dynamic_slots = true;
      if let Expression::ObjectExpression(obj) = expression
        && context.options.optimize
      {
        let semantic = context.options.semantic.as_ptr();
        if !obj.properties.iter().any(|p| match p {
          ObjectPropertyKind::ObjectProperty(p) => p.computed || !p.value.is_function(),
          _ => true,
        }) {
          has_dynamic_slots = false;
          for prop in obj.properties.iter_mut() {
            if let ObjectPropertyKind::ObjectProperty(prop) = prop
              && let Some((scope_id, span)) = match &prop.value {
                Expression::FunctionExpression(f) => Some((f.scope_id(), f.span)),
                Expression::ArrowFunctionExpression(f) => Some((f.scope_id(), f.span)),
                _ => None,
              }
            {
              let identifiers = unsafe { &*semantic }
                .scoping()
                .get_bindings(scope_id)
                .keys()
                .map(|id| id.as_str())
                .collect::<Vec<_>>();
              context.options.slot_scopes.borrow_mut().insert(
                span,
                SlotScope {
                  dynamic: false,
                  forwarded: false,
                  identifiers,
                },
              );
              prop.value = context.process_expression(&mut prop.value).0;
            }
          }
        }
      }
      let mut exp = if has_dynamic_slots {
        context.process_expression(expression).0
      } else {
        expression.take_in(context.allocator)
      };
      Some(Box::new(move || {
        if let Some(NodeTypes::VNodeCall(vnode_call)) =
          context.codegen_map.borrow_mut().get_mut(&node_span)
        {
          let mut patch_flag = vnode_call.patch_flag.unwrap_or_default();
          if let Expression::ObjectExpression(obj) = &mut exp
            && !has_dynamic_slots
          {
            has_dynamic_slots = obj.properties.iter().any(|prop| {
              if let ObjectPropertyKind::ObjectProperty(prop) = prop
                && prop.value.is_function()
              {
                context
                  .options
                  .slot_scopes
                  .borrow_mut()
                  .shift_remove(&prop.value.span())
                  .is_some_and(|n| n.dynamic)
              } else {
                false
              }
            });
            if !has_dynamic_slots {
              for prop in obj.properties.iter_mut() {
                if let ObjectPropertyKind::ObjectProperty(prop) = prop {
                  prop.value = ast.expression_call(
                    SPAN,
                    ast.expression_identifier(
                      SPAN,
                      ast.atom(context.options.helper("_normalizeSlot")),
                    ),
                    NONE,
                    ast.vec1(prop.value.take_in(context.allocator).into()),
                    false,
                  )
                }
              }
              obj.properties.insert(
                0,
                ast.object_property_kind_object_property(
                  SPAN,
                  oxc_ast::ast::PropertyKind::Init,
                  ast.property_key_static_identifier(SPAN, "_"),
                  ast.expression_numeric_literal(SPAN, 1 as f64, None, NumberBase::Hex),
                  false,
                  false,
                  false,
                ),
              );
            }
          }
          if has_dynamic_slots {
            patch_flag |= PatchFlags::DynamicSlots as i32;
            vnode_call.patch_flag = Some(patch_flag);
          }
          vnode_call.children = Some(Either3::C(exp));
        }
        *context.options.in_v_slot.borrow_mut() -= 1;
      }))
    } else {
      context.options.on_error.as_ref()(ErrorCodes::VSlotsNoExpression, dir.span);
      None
    }
  } else {
    None
  }
}
