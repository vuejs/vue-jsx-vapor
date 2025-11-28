use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use napi::bindgen_prelude::{Either, Either4};
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{BindingPatternKind, Expression, FormalParameterKind, PropertyKind},
};
use oxc_span::SPAN;

use crate::{
  generate::{CodegenContext, block::gen_block, expression::gen_expression},
  ir::{
    component::{IRSlotDynamicBasic, IRSlotDynamicConditional, IRSlots},
    index::{BlockIRNode, IRFor},
  },
  utils::{check::is_simple_identifier, walk::WalkIdentifiers},
};

pub fn gen_raw_slots<'a>(
  mut slots: Vec<IRSlots<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Expression<'a>> {
  if slots.is_empty() {
    return None;
  }
  if let Either4::A(_) = &slots[0] {
    // single static slot
    let static_slots = slots.remove(0);
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
  mut slots: IndexMap<String, BlockIRNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  dynamic_slots: Option<Vec<IRSlots<'a>>>,
) -> Expression<'a> {
  let ast = context.ast;
  let mut properties = ast.vec();
  let context_block = context_block as *mut BlockIRNode;
  for name in slots.keys().cloned().collect::<Vec<String>>() {
    let oper = slots.shift_remove(&name).unwrap();
    let name = if is_simple_identifier(&name) {
      &name
    } else {
      &format!("\"{}\"", name)
    };
    properties.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, ast.atom(name)),
      gen_slot_block_with_props(oper, context, unsafe { &mut *context_block }),
      false,
      false,
      false,
    ))
  }
  if let Some(dynamic_slots) = dynamic_slots {
    properties.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, ast.atom("$")),
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
      Either4::B(slot) => {
        gen_dynamic_slot(slot, context, unsafe { &mut *context_block }, true).into()
      }
      Either4::C(slot) => {
        gen_conditional_slot(slot, context, unsafe { &mut *context_block }, true).into()
      }
      Either4::D(slot) => gen_expression(slot.slots, context, None, None).into(),
    })
  }
  ast.expression_array(SPAN, elements)
}

fn gen_dynamic_slot<'a>(
  slot: IRSlotDynamicBasic<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  with_function: bool,
) -> Expression<'a> {
  let ast = &context.ast;
  let frag = if slot._loop.is_none() {
    gen_basic_dynamic_slot(slot, context, context_block)
  } else {
    gen_loop_slot(slot, context, context_block)
  };
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
        ast.vec1(ast.statement_expression(SPAN, frag)),
      ),
    )
  } else {
    frag
  }
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
        ast.property_key_static_identifier(SPAN, ast.atom("name")),
        gen_expression(slot.name, context, None, None),
        false,
        false,
        false,
      ),
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.atom("fn")),
        gen_slot_block_with_props(slot._fn, context, context_block),
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
    ..
  } = _loop.unwrap();
  let raw_value = value.map(|value| value.content);
  let raw_key = key.map(|key| key.content);
  let raw_index = index.map(|index| index.content);

  let slot_expr = ast.expression_object(
    SPAN,
    ast.vec_from_array([
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.atom("name")),
        gen_expression(name, context, None, None),
        false,
        false,
        false,
      ),
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.atom("fn")),
        gen_slot_block_with_props(_fn, context, context_block),
        false,
        false,
        false,
      ),
    ]),
  );

  ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.atom(&context.helper("createForSlots"))),
    NONE,
    ast.vec_from_array([
      gen_expression(source.unwrap(), context, None, None).into(),
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
                  Some(ast.formal_parameter(
                    SPAN,
                    ast.vec(),
                    ast.binding_pattern(
                      BindingPatternKind::BindingIdentifier(
                        ast.alloc_binding_identifier(SPAN, ast.atom(&raw_value)),
                      ),
                      NONE,
                      false,
                    ),
                    None,
                    false,
                    false,
                  ))
                } else if raw_key.is_some() && raw_index.is_some() {
                  Some(ast.formal_parameter(
                    SPAN,
                    ast.vec(),
                    ast.binding_pattern(
                      BindingPatternKind::BindingIdentifier(
                        ast.alloc_binding_identifier(SPAN, ast.atom("_")),
                      ),
                      NONE,
                      false,
                    ),
                    None,
                    false,
                    false,
                  ))
                } else {
                  None
                },
                if let Some(raw_key) = raw_key {
                  Some(ast.formal_parameter(
                    SPAN,
                    ast.vec(),
                    ast.binding_pattern(
                      BindingPatternKind::BindingIdentifier(
                        ast.alloc_binding_identifier(SPAN, ast.atom(&raw_key)),
                      ),
                      NONE,
                      false,
                    ),
                    None,
                    false,
                    false,
                  ))
                } else if raw_index.is_some() {
                  Some(ast.formal_parameter(
                    SPAN,
                    ast.vec(),
                    ast.binding_pattern(
                      BindingPatternKind::BindingIdentifier(
                        ast.alloc_binding_identifier(SPAN, ast.atom("__")),
                      ),
                      NONE,
                      false,
                    ),
                    None,
                    false,
                    false,
                  ))
                } else {
                  None
                },
                raw_index.map(|raw_index| {
                  ast.formal_parameter(
                    SPAN,
                    ast.vec(),
                    ast.binding_pattern(
                      BindingPatternKind::BindingIdentifier(
                        ast.alloc_binding_identifier(SPAN, ast.atom(&raw_index)),
                      ),
                      NONE,
                      false,
                    ),
                    None,
                    false,
                    false,
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
    gen_expression(condition, context, None, None),
    gen_dynamic_slot(positive, context, unsafe { &mut *context_block }, false),
    if let Some(negative) = negative {
      match *negative {
        Either::A(negative) => {
          gen_dynamic_slot(negative, context, unsafe { &mut *context_block }, false)
        }
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
) -> Expression<'a> {
  let mut is_destructure_assignment = false;
  let mut props_name = String::new();
  let mut props_loc = SPAN;
  let mut exit_scope = None;
  let mut ids_of_props = HashSet::new();

  if let Some(props) = oper.props.take() {
    let raw_props = props.content.clone();
    props_loc = props.loc;
    if let Some(ast) = &props.ast
      && let Expression::ObjectExpression(_) = ast.without_parentheses().get_inner_expression()
    {
      is_destructure_assignment = true;
      let scope = context.enter_scope();
      props_name = format!("_slotProps{}", scope.0);
      if let Some(ast) = props.ast {
        WalkIdentifiers::new(
          context,
          Box::new(|id, _, _, _, _| {
            ids_of_props.insert(id.name.to_string());
            None
          }),
          false,
        )
        .traverse(ast.take_in(context.ast.allocator));
      }
      exit_scope = Some(scope.1);
    } else {
      props_name = raw_props.clone();
      ids_of_props.insert(raw_props);
    }
  }

  let mut id_map = HashMap::new();

  let ast = &context.ast;
  for id in ids_of_props {
    id_map.insert(
      id.clone(),
      if is_destructure_assignment {
        Some(Expression::StaticMemberExpression(
          ast.alloc_static_member_expression(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&props_name)),
            ast.identifier_name(SPAN, ast.atom(&id)),
            false,
          ),
        ))
      } else {
        None
      },
    );
  }

  let block_fn = context.with_id(
    || {
      gen_block(
        oper,
        context,
        context_block,
        ast.vec1(ast.formal_parameter(
          SPAN,
          ast.vec(),
          ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(
              ast.alloc_binding_identifier(props_loc, ast.atom(&props_name)),
            ),
            NONE,
            false,
          ),
          None,
          false,
          false,
        )),
        false,
      )
    },
    id_map,
  );
  if let Some(exit_scope) = exit_scope {
    exit_scope();
  };
  block_fn
}
