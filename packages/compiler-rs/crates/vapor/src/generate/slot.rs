use std::borrow::Cow;

use indexmap::IndexMap;
use napi::bindgen_prelude::{Either, Either4};
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{Expression, FormalParameterKind, ObjectPropertyKind, PropertyKind, Str},
};
use oxc_span::{GetSpan, SPAN};

use crate::{
  generate::{
    CodegenContext,
    block::{find_returned_dynamic, gen_block, mark_slot_root_operations},
    expression::gen_expression,
  },
  ir::{
    component::{IRSlotDynamicBasic, IRSlotDynamicConditional, IRSlots},
    index::{BlockIRNode, IRFor, OperationNode},
  },
};

use common::{check::is_simple_identifier, patch_flag::VaporSlotFlags};

pub fn gen_raw_slots<'a>(
  mut slots: Vec<IRSlots<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Expression<'a>> {
  if slots.is_empty() {
    return None;
  }
  if let Either4::A(_) = &slots[0] {
    let mut static_slots = slots.remove(0);
    if let Either4::A(static_slots) = &mut static_slots
      && static_slots.slots.len() == 1
      && let Some(default_slot) = static_slots.slots.shift_remove("default")
    {
      return Some(gen_slot_block_with_props(
        default_slot,
        context,
        context_block,
        true,
      ));
    }
    // single static slot
    if let Either4::A(static_slots) = static_slots {
      Some(gen_static_slots(
        static_slots.slots,
        context,
        context_block,
        if slots.len() > 1 { Some(slots) } else { None },
      ))
    } else {
      None
    }
  } else {
    Some(gen_static_slots(
      IndexMap::new(),
      context,
      context_block,
      Some(slots),
    ))
  }
}

fn gen_static_slots<'a>(
  mut slots: IndexMap<Str<'a>, BlockIRNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  dynamic_slots: Option<Vec<IRSlots<'a>>>,
) -> Expression<'a> {
  let ast = context.ast;
  let mut properties = ast.vec();
  let context_block = context_block as *mut BlockIRNode;
  for name in slots.keys().cloned().collect::<Vec<_>>() {
    let oper = slots.shift_remove(&name).unwrap();
    let name = if is_simple_identifier(&name) {
      name.as_str()
    } else {
      &format!("\"{}\"", name)
    };
    properties.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, ast.str(name)),
      gen_slot_block_with_props(oper, context, unsafe { &mut *context_block }, true),
      false,
      false,
      false,
    ))
  }
  if let Some(mut dynamic_slots) = dynamic_slots {
    if dynamic_slots.len() == 1
      && let Some(Either4::D(slot)) = dynamic_slots.get_mut(0)
      && let Expression::ObjectExpression(slots) = &mut slot.slots
      && slots.properties.len() == 1
      && let Some(ObjectPropertyKind::ObjectProperty(prop)) = slots.properties.get_mut(0)
      && prop.key.is_specific_id("default")
      && prop.value.is_function()
    {
      return gen_expression(prop.value.take_in(ast.allocator), context, None, false);
    }
    properties.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, ast.str("$")),
      gen_dynamic_slots(dynamic_slots, context, unsafe { &mut *context_block }),
      false,
      false,
      false,
    ));
  }
  ast.expression_object(SPAN, properties)
}

fn gen_dynamic_slots<'a>(
  slots: Vec<IRSlots<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Expression<'a> {
  let ast = context.ast;
  let mut elements = ast.vec();
  let context_block = context_block as *mut BlockIRNode;
  for slot in slots {
    elements.push(match slot {
      Either4::A(slot) => {
        gen_static_slots(slot.slots, context, unsafe { &mut *context_block }, None).into()
      }
      Either4::B(slot) => gen_dynamic_slot(slot, context, unsafe { &mut *context_block }).into(),
      Either4::C(slot) => {
        gen_conditional_slot(slot, context, unsafe { &mut *context_block }, true).into()
      }
      Either4::D(slot) => {
        let expression = gen_expression(slot.slots, context, None, false);
        if slot.dynamic {
          ast
            .expression_arrow_function(
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
                ast.vec1(ast.statement_expression(
                  SPAN,
                  ast.expression_call(
                    SPAN,
                    ast.expression_identifier(
                      SPAN,
                      ast.str(context.options.helper("_normalizeVaporSlots")),
                    ),
                    NONE,
                    ast.vec1(expression.into()),
                    false,
                  ),
                )),
              ),
            )
            .into()
        } else {
          expression.into()
        }
      }
    })
  }
  ast.expression_array(SPAN, elements)
}

fn gen_dynamic_slot<'a>(
  slot: IRSlotDynamicBasic<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Expression<'a> {
  let frag = if slot._loop.is_none() {
    gen_basic_dynamic_slot(slot, context, context_block)
  } else {
    gen_loop_slot(slot, context, context_block)
  };

  frag
}

fn gen_basic_dynamic_slot<'a>(
  slot: IRSlotDynamicBasic<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  ast.expression_object(
    SPAN,
    ast.vec_from_array([
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.str("name")),
        gen_expression(slot.name, context, None, false),
        false,
        false,
        false,
      ),
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.str("fn")),
        gen_slot_block_with_props(slot._fn, context, context_block, false),
        false,
        false,
        false,
      ),
    ]),
  )
}

fn gen_loop_slot<'a>(
  slot: IRSlotDynamicBasic<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  let IRSlotDynamicBasic {
    name, _fn, _loop, ..
  } = slot;
  let IRFor {
    value,
    key,
    index,
    source,
  } = _loop.unwrap();
  let raw_value = value.and_then(|value| {
    if let Expression::Identifier(value) = value {
      Some(value.name)
    } else {
      None
    }
  });
  let raw_key = key.and_then(|key| {
    if let Expression::Identifier(key) = key {
      Some(key.name)
    } else {
      None
    }
  });
  let raw_index = index.and_then(|index| {
    if let Expression::Identifier(index) = index {
      Some(index.name)
    } else {
      None
    }
  });

  let slot_expr = ast.expression_object(
    SPAN,
    ast.vec_from_array([
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.str("name")),
        gen_expression(name, context, None, false),
        false,
        false,
        false,
      ),
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.str("fn")),
        gen_slot_block_with_props(_fn, context, context_block, false),
        false,
        false,
        false,
      ),
    ]),
  );

  ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.str(context.options.helper("_createForSlots"))),
    NONE,
    ast.vec_from_array([
      gen_expression(source.unwrap(), context, None, false).into(),
      ast
        .expression_arrow_function(
          SPAN,
          true,
          false,
          NONE,
          ast.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            ast.vec_from_iter(
              [
                if let Some(raw_value) = raw_value {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, ast.str(&raw_value)),
                  ))
                } else if raw_key.is_some() && raw_index.is_some() {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, ast.str("_")),
                  ))
                } else {
                  None
                },
                if let Some(raw_key) = raw_key {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, ast.str(&raw_key)),
                  ))
                } else if raw_index.is_some() {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, ast.str("__")),
                  ))
                } else {
                  None
                },
                raw_index.map(|raw_index| {
                  ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, ast.str(&raw_index)),
                  )
                }),
              ]
              .into_iter()
              .flatten(),
            ),
            NONE,
          ),
          NONE,
          ast.function_body(
            SPAN,
            ast.vec(),
            ast.vec1(ast.statement_expression(SPAN, slot_expr)),
          ),
        )
        .into(),
    ]),
    false,
  )
}

fn gen_conditional_slot<'a>(
  slot: IRSlotDynamicConditional<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  with_function: bool,
) -> Expression<'a> {
  let ast = &context.ast;
  let IRSlotDynamicConditional {
    condition,
    positive,
    negative,
    ..
  } = slot;
  let context_block = context_block as *mut BlockIRNode;

  let expression = ast.expression_conditional(
    SPAN,
    gen_expression(condition, context, None, false),
    gen_dynamic_slot(positive, context, unsafe { &mut *context_block }),
    if let Some(negative) = negative {
      match *negative {
        Either::A(negative) => gen_dynamic_slot(negative, context, unsafe { &mut *context_block }),
        Either::B(negative) => {
          gen_conditional_slot(negative, context, unsafe { &mut *context_block }, false)
        }
      }
    } else {
      ast.expression_identifier(SPAN, "undefined")
    },
  );

  if with_function {
    ast.expression_arrow_function(
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
        ast.vec1(ast.statement_expression(SPAN, expression)),
      ),
    )
  } else {
    expression
  }
}

fn gen_slot_block_with_props<'a>(
  mut oper: BlockIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  emit_non_stable_flag: bool,
) -> Expression<'a> {
  let mut props_name = Cow::Borrowed("");
  let mut props_loc = SPAN;
  let mut props_ast = None;
  let mut exit_scope = None;

  if let Some(props) = oper.props.as_mut() {
    props_loc = props.span();
    match props.without_parentheses().get_inner_expression() {
      Expression::ObjectExpression(_) => {
        props_ast = Some(props);
        let scope = context.enter_scope();
        props_name = Cow::Owned(format!("_slotProps{}", scope.0));
        exit_scope = Some(scope.1);
      }
      Expression::Identifier(props) => {
        props_name = Cow::Borrowed(props.name.as_str());
      }
      _ => {}
    }
  }

  let ast = context.ast;
  let id_map = context.parse_value_destructure(
    props_ast,
    context
      .ast
      .expression_identifier(SPAN, ast.str(&props_name)),
  );

  let ast = &context.ast;
  let has_stable_root = has_stable_slot_root(&mut oper, context);
  if !has_stable_root {
    mark_slot_root_operations(&mut oper);
  }
  let exit_slot_block = context.enter_slot_block();
  let mut block_fn = context.with_id(
    || {
      gen_block(
        oper,
        context,
        context_block,
        if props_name.is_empty() {
          ast.vec()
        } else {
          ast.vec1(ast.plain_formal_parameter(
            SPAN,
            ast.binding_pattern_binding_identifier(props_loc, ast.str(&props_name)),
          ))
        },
      )
    },
    id_map,
  );
  // Dynamic slot sources keep rawSlots.$, so runtime stays conservative.
  if emit_non_stable_flag && !has_stable_root {
    block_fn = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_extend"))),
      NONE,
      ast.vec_from_array([
        block_fn.into(),
        ast
          .expression_object(
            SPAN,
            ast.vec1(ast.object_property_kind_object_property(
              SPAN,
              PropertyKind::Init,
              ast.property_key_static_identifier(SPAN, "_"),
              ast.expression_numeric_literal(
                SPAN,
                VaporSlotFlags::NonStable as i32 as f64,
                None,
                oxc_ast::ast::NumberBase::Decimal,
              ),
              false,
              false,
              false,
            )),
          )
          .into(),
      ]),
      false,
    )
  }
  exit_slot_block();
  if let Some(exit_scope) = exit_scope {
    exit_scope();
  };

  block_fn
}

// A slot can skip fallback/boundary tracking when at least one root is stable.
// Components count as valid even if their own render result is a comment.
fn has_stable_slot_root<'a>(block: &mut BlockIRNode<'a>, context: &CodegenContext<'a>) -> bool {
  let mut has_valid_root = false;
  let block_ptr = block as *mut BlockIRNode;
  for id in block.returns.iter() {
    let Some(child) = find_returned_dynamic(unsafe { &mut *block_ptr }, *id) else {
      continue;
    };
    let Some(operation) = child.operation.as_mut() else {
      if is_stable_template_slot_root(child.template, context) {
        has_valid_root = true
      }
      continue;
    };

    match operation.as_mut() {
      OperationNode::CreateComponent(_) => {
        has_valid_root = true;
      }
      OperationNode::Key(operation) => {
        if has_stable_slot_root(&mut operation.block, context) {
          has_valid_root = true;
        }
      }
      _ => {}
    }
  }
  has_valid_root
}

fn is_stable_template_slot_root(template: Option<i32>, context: &CodegenContext) -> bool {
  let Some(template) = template else {
    return false;
  };
  context
    .options
    .templates
    .borrow()
    .get(template as usize)
    .is_some_and(|i| !i.content.is_empty())
}
