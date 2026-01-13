use std::collections::HashSet;

use common::{
  check::{is_jsx_component, is_template},
  directive::{Directives, find_prop},
  error::ErrorCodes,
  expression::expression_to_params,
  patch_flag::SlotFlags,
  text::get_tag_name,
};
use napi::Either;
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{
    Argument, ArrayExpressionElement, ConditionalExpression, Expression, FormalParameterKind,
    JSXAttributeName, JSXAttributeValue, JSXChild, JSXElement, NumberBase, ObjectPropertyKind,
    PropertyKey, PropertyKind, Statement,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{ForNode, VNodeCallChildren},
  transform::{
    TransformContext,
    cache_static::cache_static_children,
    v_for::{create_for_loop_params, get_for_parse_result},
    v_if::get_parent_condition,
  },
};

/// # SAFETY
// A NodeTransform that:
// 1. Tracks scope identifiers for scoped slots so that they don't get prefixed
//    by transformExpression. This is only applied in non-browser builds with
//    { prefixIdentifiers: true }.
// 2. Track v-slot depths so that we know a slot is inside another slot.
//    Note the exit callback is executed before buildSlots() on the same node,
//    so only nested slots see positive numbers.
pub unsafe fn track_slot_scopes<'a>(
  directives: &mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  unsafe {
    let node = &mut *context_node;
    let JSXChild::Element(node) = node else {
      return None;
    };
    let is_component = is_jsx_component(node);
    if is_component || is_template(node) {
      // We are only checking non-empty v-slot here
      // since we only care about slots that introduce scope variables.
      if let Some(v_slot) = directives.v_slot.as_ref() {
        let mut identifiers = vec![];
        if let Some(JSXAttributeValue::ExpressionContainer(slot_props)) = v_slot.value.as_ref() {
          identifiers = context.add_identifiers(&Some(slot_props.expression.to_expression()));
        }
        if is_component {
          context
            .options
            .slot_identifiers
            .borrow_mut()
            .insert(node.span, (0, identifiers.clone()));
        }
        *context.options.in_v_slot.borrow_mut() += 1;
        return Some(Box::new(move || {
          *context.options.in_v_slot.borrow_mut() -= 1;
          context.remove_identifiers(identifiers);
        }));
      } else if is_component {
        context
          .options
          .slot_identifiers
          .borrow_mut()
          .insert(node.span, (0, vec![]));
      }
    }
    None
  }
}

pub fn build_slots<'a>(
  directives: &mut Directives<'a>,
  node: &'a mut JSXElement<'a>,
  context: &'a TransformContext<'a>,
) -> (Expression<'a>, bool, Vec<String>) {
  let ast = &context.ast;
  let _node = node as *mut JSXElement;
  let tag_name = get_tag_name(&node.opening_element.name, *context.source.borrow());
  let is_fragment = tag_name == "Fragment" || tag_name == "_Fragment";
  let mut slots_properties = ast.vec();
  let mut dynamic_slots = ast.vec();
  let mut should_removed_identifiers = vec![];

  // If the slot is inside a v-for or another v-slot, force it to be dynamic
  // since it likely uses a scope variable.
  let mut has_dynamic_slots = if *context.options.ssr.borrow() {
    *context.options.in_v_slot.borrow() > 0 || *context.options.in_v_for.borrow() > 0
  } else {
    !context.options.identifiers.borrow().is_empty()
  };
  // This can be further optimized to make
  // it dynamic when the slot children use the scope variables.
  if !*context.options.ssr.borrow() && (has_dynamic_slots || !context.options.optimize_slots) {
    has_dynamic_slots = context
      .options
      .slot_identifiers
      .borrow()
      .get(&node.span)
      .map(|n| n.0 > 0)
      .unwrap_or_default();
  }

  // 1. Check for slot with slotProps on component itself.
  //    <Comp v-slot="{ prop }"/>
  let on_component_slot = directives.v_slot.as_ref();
  if let Some(on_component_slot) = on_component_slot {
    let mut arg_name = None;
    let mut arg_span = SPAN;
    let mut computed = false;
    if let JSXAttributeName::NamespacedName(arg) = &on_component_slot.name {
      let splited = arg.name.name.split("$").collect::<Vec<_>>();
      arg_name = Some(if splited.len() > 2 {
        has_dynamic_slots = true;
        computed = true;
        arg_span = Span::new(arg.name.span.start + 1, arg.name.span.end - 1);
        ast.atom(&splited[1].replace("_", "."))
      } else {
        arg_span = arg.name.span;
        arg.name.name
      })
    };
    slots_properties.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      if let Some(arg_name) = arg_name {
        ast.property_key_static_identifier(arg_span, arg_name)
      } else {
        ast.property_key_static_identifier(SPAN, "default")
      },
      ast.expression_null_literal(SPAN),
      false,
      false,
      computed,
    ))
  }

  // 2. Iterate through children and check for template slots
  //    <template v-slot:foo="{ prop }">
  let mut has_template_slots = false;
  let mut has_named_default_slot = false;
  let mut implicit_default_children = ast.vec();
  let mut seen_slot_names = HashSet::new();
  let mut conditional_branch_index = 0;

  for (i, slot_element) in node.children.iter_mut().enumerate() {
    let mut slot_dir = None;

    let _slot_element = slot_element as *mut JSXChild;
    let JSXChild::Element(slot_element) = slot_element else {
      implicit_default_children.push(unsafe { &mut *_slot_element }.take_in(context.allocator));
      continue;
    };
    let slot_element_ptr = slot_element as *mut oxc_allocator::Box<JSXElement>;
    let mut slot_directives = Directives::new(unsafe { &mut *slot_element_ptr });
    if if is_template(slot_element) {
      slot_dir = slot_directives.v_slot.as_ref();
      slot_dir.is_none()
    } else {
      true
    } {
      // not a <template v-slot>, skip.
      implicit_default_children.push(unsafe { &mut *_slot_element }.take_in(context.allocator));
      continue;
    }

    if let Some(on_component_slot) = on_component_slot {
      // already has on-component slot - this is incorrect usage.
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, on_component_slot.span);
      break;
    }

    has_template_slots = true;
    // check if name is dynamic.
    let mut static_slot_name = None;
    let slot_dir = slot_dir.unwrap();
    let slot_name = if let JSXAttributeName::NamespacedName(name) = &slot_dir.name {
      if name.name.name.split("$").count() > 2 {
        has_dynamic_slots = true;
        ast.expression_identifier(
          name.span,
          ast.atom(&name.name.name[1..name.name.name.len() - 1].replace("_", ".")),
        )
      } else {
        static_slot_name = Some(name.name.name.to_string());
        ast.expression_identifier(name.span, name.name.name)
      }
    } else {
      static_slot_name = Some(String::from("default"));
      ast.expression_identifier(SPAN, "default")
    };
    let slot_props = if let Some(JSXAttributeValue::ExpressionContainer(value)) = &slot_dir.value {
      expression_to_params(
        value.expression.to_expression(),
        *context.source.borrow(),
        context.allocator,
        context.options.source_type,
      )
    } else {
      None
    };
    let dir_loc = slot_dir.span;

    let slot_function = ast.expression_arrow_function(
      SPAN,
      true,
      false,
      NONE,
      ast.alloc_formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        if let Some(slot_props) = slot_props {
          ast.vec1(slot_props)
        } else {
          ast.vec()
        },
        NONE,
      ),
      NONE,
      ast.function_body(
        SPAN,
        ast.vec(),
        ast.vec1(ast.statement_expression(
          SPAN,
          gen_cache_node_list(&mut slot_element.children, context),
        )),
      ),
    );

    // check if this slot is conditional (v-if/v-for)
    if let Some(v_if) = slot_directives.v_if.as_mut() {
      has_dynamic_slots = true;
      dynamic_slots.push(
        ast
          .expression_conditional(
            SPAN,
            if let Some(value) = v_if.value.as_mut() {
              context.jsx_attribute_value_to_expression(value)
            } else {
              ast.expression_null_literal(SPAN)
            },
            build_dynamic_slot(
              slot_name,
              slot_function,
              Some(conditional_branch_index),
              context,
            ),
            ast.expression_identifier(SPAN, "undefined"),
          )
          .into(),
      );
    } else if let Some(v_else) = slot_directives
      .v_else
      .as_mut()
      .or(slot_directives.v_else_if.as_mut())
    {
      // find adjacent v-if
      let prev = if i > 0 {
        unsafe { &mut *_node }.children.get_mut(i - 1)
      } else {
        None
      };
      if let Some(JSXChild::Element(prev)) = prev
        && is_template(prev)
        && find_prop(
          prev,
          vec!["v-if", "v-else-if"],
        )
        .is_some()
        // attach this slot to previous conditional
        && let Some(ArrayExpressionElement::ConditionalExpression(conditional)) =
          dynamic_slots.last_mut()
      {
        let ret = get_parent_condition(&mut conditional.alternate)
          .unwrap_or(conditional as *mut oxc_allocator::Box<ConditionalExpression>);
        conditional_branch_index += 1;
        unsafe { &mut *ret }.alternate = if let Some(value) = &mut v_else.value {
          ast.expression_conditional(
            SPAN,
            context.jsx_attribute_value_to_expression(value),
            build_dynamic_slot(
              slot_name,
              slot_function,
              Some(conditional_branch_index),
              context,
            ),
            ast.expression_identifier(SPAN, "undefined"),
          )
        } else {
          build_dynamic_slot(
            slot_name,
            slot_function,
            Some(conditional_branch_index),
            context,
          )
        };
      } else {
        context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, v_else.span);
      }
    } else if let Some(v_for) = slot_directives.v_for.as_mut() {
      has_dynamic_slots = true;
      if let Some(ForNode {
        source,
        value,
        key,
        index,
        identifiers,
      }) = get_for_parse_result(v_for, context)
      {
        should_removed_identifiers = identifiers;
        // Render the dynamic slots as an array and add it to the createSlot()
        // args. The runtime knows how to handle it appropriately.
        dynamic_slots.push(
          ast
            .expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("renderList"))),
              NONE,
              ast.vec_from_array([
                source.unwrap().into(),
                ast
                  .expression_arrow_function(
                    SPAN,
                    true,
                    false,
                    NONE,
                    create_for_loop_params(value, key, index, None, context),
                    NONE,
                    ast.function_body(
                      SPAN,
                      ast.vec(),
                      ast.vec1(ast.statement_expression(
                        SPAN,
                        build_dynamic_slot(slot_name, slot_function, None, context),
                      )),
                    ),
                  )
                  .into(),
              ]),
              false,
            )
            .into(),
        );
      };
    } else {
      // check duplicate static names
      if let Some(static_slot_name) = static_slot_name.as_ref() {
        if seen_slot_names.contains(static_slot_name) {
          context.options.on_error.as_ref()(ErrorCodes::VSlotDuplicateSlotNames, dir_loc);
          continue;
        }
        seen_slot_names.insert(static_slot_name.clone());
        if static_slot_name == "default" {
          has_named_default_slot = true;
        }
      }

      slots_properties.push(ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        slot_name.into(),
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("withCtx"))),
          NONE,
          ast.vec1(slot_function.into()),
          false,
        ),
        false,
        false,
        static_slot_name.is_none(),
      ))
    }
  }

  if let Some(on_component_slot) = on_component_slot {
    if let Some(ObjectPropertyKind::ObjectProperty(prop)) = slots_properties.first_mut() {
      prop.value = ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.atom(&context.helper("withCtx"))),
        NONE,
        ast.vec1(
          ast
            .expression_arrow_function(
              SPAN,
              true,
              false,
              NONE,
              ast.alloc_formal_parameters(
                SPAN,
                FormalParameterKind::ArrowFormalParameters,
                if let Some(JSXAttributeValue::ExpressionContainer(value)) =
                  &on_component_slot.value
                {
                  ast.vec1(
                    expression_to_params(
                      value.expression.to_expression(),
                      *context.source.borrow(),
                      context.allocator,
                      context.options.source_type,
                    )
                    .unwrap(),
                  )
                } else {
                  ast.vec()
                },
                NONE,
              ),
              NONE,
              ast.function_body(
                SPAN,
                ast.vec(),
                ast.vec1(ast.statement_expression(
                  SPAN,
                  gen_cache_node_list(&mut implicit_default_children, context),
                )),
              ),
            )
            .into(),
        ),
        false,
      )
    }
  } else if !has_template_slots {
    // implicit default slot (on component)
    slots_properties.push(
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, "default"),
        ast.expression_call(
          SPAN,
          ast.expression_identifier(
            SPAN,
            ast.atom(&if is_fragment {
              String::from("withCtx")
            } else {
              context.helper("withCtx")
            }),
          ),
          NONE,
          ast.vec1(
            ast
              .expression_arrow_function(
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
                ast.function_body(
                  SPAN,
                  ast.vec(),
                  ast.vec1(ast.statement_expression(
                    SPAN,
                    gen_cache_node_list(&mut implicit_default_children, context),
                  )),
                ),
              )
              .into(),
          ),
          false,
        ),
        false,
        false,
        false,
      ),
    );
  } else if !implicit_default_children.is_empty() {
    // implicit default slot (mixed with named slots)
    if has_named_default_slot {
      context.options.on_error.as_ref()(
        ErrorCodes::VSlotExtraneousDefaultSlotChildren,
        implicit_default_children[0].span(),
      )
    } else {
      slots_properties.push(
        ast.object_property_kind_object_property(
          SPAN,
          PropertyKind::Init,
          ast.property_key_static_identifier(SPAN, "default"),
          ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&context.helper("withCtx"))),
            NONE,
            ast.vec1(
              ast
                .expression_arrow_function(
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
                  ast.function_body(
                    SPAN,
                    ast.vec(),
                    ast.vec1(ast.statement_expression(
                      SPAN,
                      gen_cache_node_list(&mut implicit_default_children, context),
                    )),
                  ),
                )
                .into(),
            ),
            false,
          ),
          false,
          false,
          false,
        ),
      );
    }
  }

  let slot_flag = if has_dynamic_slots {
    SlotFlags::DYNAMIC
  } else {
    SlotFlags::STABLE
  };
  slots_properties.push(ast.object_property_kind_object_property(
    SPAN,
    PropertyKind::Init,
    ast.property_key_static_identifier(SPAN, "_"),
    // 2 = compiled but dynamic = can skip normalization, but must run diff
    // 1 = compiled and static = can skip normalization AND diff as optimized
    ast.expression_numeric_literal(SPAN, slot_flag as i32 as f64, None, NumberBase::Hex),
    false,
    false,
    false,
  ));
  // covert Fragment's object slots to array
  let mut slots = if is_fragment {
    has_dynamic_slots = false;
    slots_properties
      .into_iter()
      .find_map(|prop| {
        if let ObjectPropertyKind::ObjectProperty(mut prop) = prop
          && let PropertyKey::StaticIdentifier(key) = &prop.key
          && key.name == "default"
          && let Expression::CallExpression(exp) = &mut prop.value
          && let Some(Argument::ArrowFunctionExpression(value)) = exp.arguments.first_mut()
          && let Some(Statement::ExpressionStatement(stmt)) = value.body.statements.get_mut(0)
        {
          Some(stmt.expression.take_in(context.allocator))
        } else {
          None
        }
      })
      .unwrap()
  } else {
    ast.expression_object(SPAN, slots_properties)
  };
  if !dynamic_slots.is_empty() {
    slots = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("createSlots"))),
      NONE,
      ast.vec_from_array([
        slots.into(),
        ast.expression_array(SPAN, dynamic_slots).into(),
      ]),
      false,
    )
  }

  (slots, has_dynamic_slots, should_removed_identifiers)
}

fn build_dynamic_slot<'a>(
  name: Expression<'a>,
  _fn: Expression<'a>,
  index: Option<i32>,
  context: &TransformContext<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut props = ast.vec_from_array([
    ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, "name"),
      name,
      false,
      false,
      false,
    ),
    ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, "fn"),
      _fn,
      false,
      false,
      false,
    ),
  ]);
  if let Some(index) = index {
    props.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, "key"),
      ast.expression_numeric_literal(SPAN, index as f64, None, NumberBase::Hex),
      false,
      false,
      false,
    ))
  }
  ast.expression_object(SPAN, props)
}

fn gen_cache_node_list<'a>(
  node_children: &mut oxc_allocator::Vec<'a, JSXChild<'a>>,
  context: &'a TransformContext<'a>,
) -> Expression<'a> {
  context.gen_node_list(
    {
      let mut children = VNodeCallChildren::B(node_children);
      cache_static_children(
        Some(Either::B(&mut children)),
        node_children,
        context,
        &mut context.codegen_map.borrow_mut(),
        false,
      );
      children
    },
    &mut context.codegen_map.borrow_mut(),
  )
}
