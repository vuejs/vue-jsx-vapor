use indexmap::IndexMap;
use napi::{Either, bindgen_prelude::Either4};
use oxc_ast::ast::{JSXChild, JSXElement};

use crate::{
  ir::{
    component::{IRSlotDynamicBasic, IRSlotDynamicConditional, IRSlotType, IRSlots, IRSlotsStatic},
    index::{BlockIRNode, DirectiveNode, DynamicFlag, SimpleExpressionNode},
  },
  transform::{ContextNode, TransformContext, v_for::get_for_parse_result},
  utils::{
    check::{is_jsx_component, is_template},
    directive::resolve_directive,
    directive::{find_prop, find_prop_mut},
    error::ErrorCodes,
    text::is_empty_text,
  },
};

/// # SAFETY
pub unsafe fn transform_v_slot<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(JSXChild::Element(node)) = (unsafe { &mut *context_node }) else {
    return None;
  };

  let node = node as *mut oxc_allocator::Box<JSXElement>;
  let dir = find_prop_mut(unsafe { &mut *node }, Either::A(String::from("v-slot")))
    .map(|dir| resolve_directive(dir, context));
  let is_component = is_jsx_component(unsafe { &*node });
  let is_slot_template = is_template(unsafe { &*node })
    && if let Either::B(JSXChild::Element(parent_node)) = parent_node
      && is_jsx_component(parent_node)
    {
      true
    } else {
      false
    };

  if is_component && !unsafe { &mut *node }.children.is_empty() {
    return Some(transform_component_slot(
      dir,
      unsafe { &mut *node },
      context,
      context_block,
    ));
  } else if is_slot_template && let Some(dir) = dir {
    return Some(transform_template_slot(dir, node, context, context_block));
  } else if !is_component && dir.is_some() {
    context.options.on_error.as_ref()(ErrorCodes::VSlotMisplaced, unsafe { &*node }.span);
  }
  None
}

// <Foo v-slot:default>
fn transform_component_slot<'a>(
  dir: Option<DirectiveNode<'a>>,
  node: &'a mut JSXElement<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Box<dyn FnOnce() + 'a> {
  let has_dir = dir.is_some();

  let (arg, exp) = if let Some(DirectiveNode { arg, exp, .. }) = dir {
    (arg, exp)
  } else {
    (None, None)
  };

  let non_slot_template_children_len = node
    .children
    .iter()
    .filter(|n| {
      !is_empty_text(n)
        && if let JSXChild::Element(n) = n {
          find_prop(n, Either::A(String::from("v-slot"))).is_none()
        } else {
          true
        }
    })
    .count();

  let exit_block = create_slot_block(exp, context, context_block, false);

  Box::new(move || {
    let mut slots = context.slots.take();

    let block = exit_block();
    let has_other_slots = !slots.is_empty();
    if has_dir && has_other_slots {
      // already has on-component slot - this is incorrect usage.
      context.options.on_error.as_ref()(ErrorCodes::VSlotMixedSlotUsage, node.span);
      return;
    }

    if non_slot_template_children_len > 0 {
      if has_static_slot(&slots, "default") {
        context.options.on_error.as_ref()(
          ErrorCodes::VSlotExtraneousDefaultSlotChildren,
          node.span,
        );
      } else {
        register_slot(&mut slots, arg, block);
        *context.slots.borrow_mut() = slots;
      }
    } else if has_other_slots {
      *context.slots.borrow_mut() = slots
    }
  })
}

// <template v-slot:foo>
fn transform_template_slot<'a>(
  dir: DirectiveNode<'a>,
  node: *mut oxc_allocator::Box<'a, JSXElement<'a>>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Box<dyn FnOnce() + 'a> {
  let dynamic = &mut context_block.dynamic;
  dynamic.flags |= DynamicFlag::NonTemplate as i32;

  let DirectiveNode { arg, exp, .. } = dir;
  let exit_block = create_slot_block(exp, context, context_block, true);

  let v_for = find_prop_mut(unsafe { &mut *node }, Either::A(String::from("v-for")));
  let for_parse_result = if let Some(v_for) = v_for {
    get_for_parse_result(v_for, context)
  } else {
    None
  };
  let v_if = find_prop_mut(unsafe { &mut *node }, Either::A(String::from("v-if")));
  let v_if_dir = v_if.map(|v_if| resolve_directive(v_if, context));
  let v_else = find_prop_mut(
    unsafe { &mut *node },
    Either::B(vec![String::from("v-else"), String::from("v-else-if")]),
  );
  let v_else_dir = v_else.map(|v_else| resolve_directive(v_else, context));

  Box::new(move || {
    let slots = &mut context.slots.borrow_mut();
    let block = exit_block();
    if v_if_dir.is_none() && v_else_dir.is_none() && for_parse_result.is_none() {
      let slot_name = if let Some(arg) = &arg {
        arg.content.clone()
      } else {
        String::from("default")
      };
      if !slot_name.is_empty() && has_static_slot(slots, &slot_name) {
        context.options.on_error.as_ref()(ErrorCodes::VSlotDuplicateSlotNames, dir.loc)
      } else {
        register_slot(slots, arg, block);
      }
    } else if let Some(v_if_dir) = v_if_dir {
      slots.push(Either4::C(IRSlotDynamicConditional {
        slot_type: IRSlotType::CONDITIONAL,
        condition: v_if_dir.exp.unwrap(),
        negative: None,
        positive: IRSlotDynamicBasic {
          slot_type: IRSlotType::DYNAMIC,
          name: arg.unwrap(),
          _fn: block,
          _loop: None,
        },
      }));
    } else if let Some(v_else_dir) = v_else_dir {
      if let Some(last_slot) = slots.last_mut() {
        if let Either4::C(v_if_slot) = last_slot {
          let positive = IRSlotDynamicBasic {
            slot_type: IRSlotType::DYNAMIC,
            name: arg.unwrap(),
            _fn: block,
            _loop: None,
          };
          let negative = if let Some(exp) = v_else_dir.exp {
            Either::B(IRSlotDynamicConditional {
              slot_type: IRSlotType::CONDITIONAL,
              condition: exp,
              positive,
              negative: None,
            })
          } else {
            Either::A(positive)
          };
          set_slot(v_if_slot, negative);
        } else {
          context.options.on_error.as_ref()(ErrorCodes::VElseNoAdjacentIf, v_else_dir.loc)
        }
      }
    } else if let Some(for_parse_result) = for_parse_result
      && for_parse_result.source.is_some()
    {
      slots.push(Either4::B(IRSlotDynamicBasic {
        slot_type: IRSlotType::DYNAMIC,
        name: arg.unwrap(),
        _fn: block,
        _loop: Some(for_parse_result),
      }))
    }
  })
}

fn set_slot<'a>(
  v_if_slot: &mut IRSlotDynamicConditional<'a>,
  slot: Either<IRSlotDynamicBasic<'a>, IRSlotDynamicConditional<'a>>,
) {
  if let Some(Either::B(negative)) = v_if_slot.negative.as_mut().map(|a| a.as_mut()) {
    set_slot(negative, slot)
  } else {
    v_if_slot.negative = Some(Box::new(slot));
  }
}

fn register_slot<'a>(
  slots: &mut Vec<IRSlots<'a>>,
  name: Option<SimpleExpressionNode<'a>>,
  block: BlockIRNode<'a>,
) {
  let is_static = if let Some(name) = &name {
    name.is_static
  } else {
    true
  };
  if is_static {
    if slots.is_empty()
      || slots
        .last()
        .is_some_and(|last_slot| !matches!(last_slot, Either4::A(_)))
    {
      slots.push(Either4::A(IRSlotsStatic {
        slot_type: IRSlotType::STATIC,
        slots: IndexMap::new(),
      }));
    }
    if let Some(Either4::A(slot)) = slots.last_mut() {
      slot.slots.insert(
        if let Some(name) = &name {
          name.content.clone()
        } else {
          String::from("default")
        },
        block,
      );
    }
  } else {
    slots.push(Either4::B(IRSlotDynamicBasic {
      slot_type: IRSlotType::DYNAMIC,
      name: name.unwrap(),
      _fn: block,
      _loop: None,
    }));
  }
}

fn has_static_slot(slots: &Vec<IRSlots>, name: &str) -> bool {
  slots.iter().any(|slot| match slot {
    Either4::A(static_slot) => {
      matches!(static_slot.slot_type, IRSlotType::STATIC) && static_slot.slots.get(name).is_some()
    }
    _ => false,
  })
}

fn create_slot_block<'a>(
  props: Option<SimpleExpressionNode<'a>>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  exclude_slots: bool,
) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
  let mut block = BlockIRNode::new();
  block.props = props;

  (context.enter_block(context_block, block, false, exclude_slots)) as _
}
