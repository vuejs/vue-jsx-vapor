use napi::bindgen_prelude::Either4;
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{
    Expression, FormalParameterKind, JSXAttributeItem, JSXAttributeValue, JSXChild, JSXElement,
    ObjectPropertyKind, PropertyKind,
  },
};
use oxc_semantic::ScopeFlags;
use oxc_span::{SPAN, Span};

use crate::{
  ir::{
    component::{IRSlotType, IRSlotsExpression},
    index::BlockIRNode,
  },
  transform::TransformContext,
};
use common::{directive::Directives, error::ErrorCodes, text::is_empty_text};

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
      context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, unsafe { &*node_ptr }.span);
      return None;
    }

    let mut dynamic = false;
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
        for prop in exp.properties.iter_mut() {
          match prop {
            ObjectPropertyKind::ObjectProperty(prop)
              if prop.computed || !prop.value.is_function() =>
            {
              dynamic = true;
              continue;
            }
            ObjectPropertyKind::SpreadProperty(_) => {
              dynamic = true;
              continue;
            }
            _ => {}
          }
        }
      } else {
        dynamic = true;
      }

      proccess_default_children(node_ptr, expression, context);

      let slots = expression.take_in(context.allocator);
      Some(Box::new(move || {
        context_block.slots = vec![Either4::D(IRSlotsExpression {
          slot_type: IRSlotType::EXPRESSION,
          dynamic,
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

fn proccess_default_children<'a>(
  node_ptr: *mut oxc_allocator::Box<JSXElement<'a>>,
  expression: &mut Expression<'a>,
  context: &'a TransformContext,
) {
  let ast = context.ast;
  let node = unsafe { &*node_ptr };
  if node.children.iter().any(|c| !is_empty_text(c)) {
    let (scope_id, node_id) = {
      let semantic = &context.options.semantic.borrow();
      let node = semantic.nodes().get_node(node.node_id());
      (node.scope_id(), node.id())
    };
    let semantic = &mut context.options.semantic.borrow_mut();
    let scope_id = semantic.scoping_mut().add_scope(
      Some(scope_id),
      node_id,
      ScopeFlags::Arrow | ScopeFlags::Function,
    );
    let default_slot = ast.object_property_kind_object_property(
      SPAN,
      oxc_ast::ast::PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, "default"),
      ast.expression_arrow_function_with_scope_id_and_pure_and_pife(
        SPAN,
        true,
        false,
        NONE,
        ast.alloc_formal_parameters(
          SPAN,
          FormalParameterKind::ArrowFormalParameters,
          ast.vec(),
          NONE,
        ),
        NONE,
        ast.alloc_function_body(
          SPAN,
          ast.vec(),
          ast.vec1(
            ast.statement_expression(
              SPAN,
              ast.expression_jsx_fragment(
                Span::new(0, 0),
                ast.jsx_opening_fragment(SPAN),
                unsafe { &mut *node_ptr }
                  .children
                  .take_in(context.allocator),
                ast.jsx_closing_fragment(SPAN),
              ),
            ),
          ),
        ),
        scope_id,
        false,
        false,
      ),
      false,
      false,
      false,
    );
    match expression {
      Expression::ObjectExpression(obj) => {
        obj.properties.insert(0, default_slot);
      }
      Expression::Identifier(_) => {
        *expression = ast.expression_object(
          SPAN,
          ast.vec_from_array([
            ast.object_property_kind_spread_property(SPAN, expression.take_in(ast.allocator)),
            default_slot,
          ]),
        );
      }
      _ => {}
    }
  }
}
