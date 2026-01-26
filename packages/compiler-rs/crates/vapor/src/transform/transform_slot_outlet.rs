use std::{cell::RefCell, rc::Rc};

use napi::{Either, bindgen_prelude::Either3};
use oxc_allocator::TakeIn;
use oxc_ast::ast::{JSXChild, JSXElement};
use oxc_span::SPAN;

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, OperationNode, SlotOutletNodeIRNode},
  transform::{TransformContext, transform_element::build_props},
};
use common::{
  directive::Directives, error::ErrorCodes, expression::SimpleExpressionNode, text::is_empty_text,
};

pub fn transform_slot_outlet<'a>(
  directives: &Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
  get_effect_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    unreachable!()
  };
  let id = context.reference(&mut context_block.dynamic);
  let context_block_ptr = context_block as *mut _;
  context_block.dynamic.flags =
    context_block.dynamic.flags | DynamicFlag::Insert as i32 | DynamicFlag::NonTemplate as i32;

  let mut slot_name = SimpleExpressionNode {
    content: "default".to_string(),
    is_static: true,
    loc: SPAN,
    ast: None,
  };
  let props = &mut node.opening_element.attributes;
  let mut ir_props = vec![];
  if !props.is_empty() {
    let props_result = build_props(
      directives,
      unsafe { &mut *(node.as_mut() as *mut _) },
      parent_node,
      context,
      unsafe { &mut *context_block_ptr },
      false,
      true,
      get_effect_index,
      get_operation_index,
    );
    match props_result.props {
      Either::A(props) => {
        ir_props = props;
      }
      Either::B(props) => {
        ir_props = vec![Either3::A(props)];
      }
    }

    if let Some(name_prop) = props_result.name_prop
      && let Some(value) = &mut name_prop.value
    {
      slot_name = SimpleExpressionNode::new(Either3::C(value), context.source_text)
    }

    if let Some(runtime_directive) =
      unsafe { &*context_block_ptr }
        .operation
        .iter()
        .find_map(|oper| {
          if let OperationNode::Directive(oper) = oper
            && oper.element == id
          {
            Some(oper)
          } else {
            None
          }
        })
    {
      context.options.on_error.as_ref()(
        ErrorCodes::VSlotUnexpectedDirectiveOnSlotOutlet,
        runtime_directive.dir.loc,
      );
    }
  }

  let exit_block = create_fallback(node, context_node, context, unsafe {
    &mut *context_block_ptr
  });

  Some(Box::new(move || {
    let fallback = if let Some(exit_block) = exit_block {
      Some(exit_block())
    } else {
      None
    };
    context_block.dynamic.operation = Some(Box::new(OperationNode::SlotOutletNode(
      SlotOutletNodeIRNode {
        id,
        name: slot_name,
        props: ir_props,
        fallback,
        no_slotted: false,
        once: *context.in_v_once.borrow(),
        parent: None,
        anchor: None,
        append: false,
        last: false,
      },
    )));
  }))
}

fn create_fallback<'a>(
  node: &mut JSXElement<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Box<dyn FnOnce() -> BlockIRNode<'a> + 'a>> {
  if node
    .children
    .iter()
    .filter(|child| !is_empty_text(child))
    .count()
    == 0
  {
    return None;
  }

  let ast = context.ast;
  *unsafe { &mut *context_node } = ast.jsx_child_fragment(
    SPAN,
    ast.jsx_opening_fragment(SPAN),
    node.children.take_in(ast.allocator),
    ast.jsx_closing_fragment(SPAN),
  );

  let fallback = BlockIRNode::new();
  Some(context.enter_block(context_block, fallback, false))
}
