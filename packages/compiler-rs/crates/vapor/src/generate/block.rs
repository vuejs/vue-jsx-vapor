use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use common::patch_flag::VaporSlotFlags;
use napi::Either;
use oxc_ast::NONE;
use oxc_ast::ast::{
  ArrayExpressionElement, Expression, FormalParameter, FormalParameterKind, Statement,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::operation::gen_operations;
use crate::generate::template::gen_self;
use crate::ir::index::{
  BlockIRNode, ForIRNode, IRDynamicInfo, IREffect, IfIRNode, OperationNode, SlotOutletIRNode,
};

pub fn gen_block<'a>(
  oper: BlockIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  args: oxc_allocator::Vec<'a, FormalParameter<'a>>,
) -> Expression<'a> {
  let ast = context.ast;
  ast.expression_arrow_function(
    SPAN,
    false,
    false,
    NONE,
    ast.alloc_formal_parameters(SPAN, FormalParameterKind::ArrowFormalParameters, args, NONE),
    NONE,
    ast.alloc_function_body(
      SPAN,
      ast.vec(),
      gen_block_content(Some(oper), context, context_block, None),
    ),
  )
}

type GenEffectsExtraFrag<'a> =
  Option<Box<dyn FnOnce(&mut oxc_allocator::Vec<'a, Statement<'a>>, &'a mut BlockIRNode<'a>) + 'a>>;

pub type FlushBeforeDynamic<'a> =
  Box<dyn FnMut(&mut IRDynamicInfo<'a>, &mut oxc_allocator::Vec<'a, Statement<'a>>) + 'a>;

pub fn gen_block_content<'a>(
  block: Option<BlockIRNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  gen_effects_extra_frag: GenEffectsExtraFrag<'a>,
) -> oxc_allocator::Vec<'a, Statement<'a>> {
  let ast = &context.ast;
  let mut statements = ast.vec();
  let mut reset_block = None;
  let context_block = context_block as *mut BlockIRNode;
  if let Some(block) = block {
    reset_block = Some(context.enter_block(block, unsafe { &mut *context_block }));
  }

  let mut operation_index = 0;
  let mut effect_index = 0;
  let flush_before_dynamic = Rc::new(RefCell::new(Box::new(
    move |dynamic: &mut IRDynamicInfo<'a>,
          statements: &mut oxc_allocator::Vec<'a, Statement<'a>>| {
      if let Some(operation) = &mut dynamic.operation
        && let Some((operation_end, effect_end)) = match operation.as_ref() {
          OperationNode::If(operation) => Some((operation.operation_index, operation.effect_index)),
          OperationNode::For(operation) => {
            Some((operation.operation_index, operation.effect_index))
          }
          OperationNode::Key(operation) => {
            Some((operation.operation_index, operation.effect_index))
          }
          OperationNode::CreateComponent(operation) => {
            Some((operation.operation_index, operation.effect_index))
          }
          OperationNode::SlotOutlet(operation) => {
            Some((operation.operation_index, operation.effect_index))
          }
          _ => None,
        }
        && let Some(operation_end) = operation_end
        && let Some(effect_end) = effect_end
      {
        if operation_index < operation_end {
          gen_operations(
            statements,
            unsafe { &mut *context_block }
              .operation
              .drain(0..operation_end - operation_index)
              .collect::<Vec<_>>(),
            context,
            unsafe { &mut *context_block },
          );
          operation_index = operation_end
        }

        if effect_index < effect_end {
          if let Some(statement) = gen_effects(
            unsafe { &mut *context_block }
              .effect
              .drain(0..effect_end - effect_index)
              .collect::<Vec<_>>(),
            context,
            unsafe { &mut *context_block },
          ) {
            statements.push(statement);
          };
          effect_index = effect_end
        }
      };
    },
  ) as FlushBeforeDynamic<'a>));

  for child in mem::take(&mut unsafe { &mut *context_block }.dynamic.children) {
    gen_self(
      &mut statements,
      child,
      context,
      unsafe { &mut *context_block },
      Rc::clone(&flush_before_dynamic),
    );
  }

  gen_operations(
    &mut statements,
    mem::take(&mut unsafe { &mut *context_block }.operation),
    context,
    unsafe { &mut *context_block },
  );
  if let Some(statement) = gen_effects(
    mem::take(&mut unsafe { &mut *context_block }.effect),
    context,
    unsafe { &mut *context_block },
  ) {
    statements.push(statement);
  }
  if let Some(gen_extra_frag) = gen_effects_extra_frag {
    gen_extra_frag(&mut statements, unsafe { &mut *context_block })
  }

  let mut return_nodes = unsafe { &mut *context_block }.returns.iter().map(|n| {
    ast
      .expression_identifier(SPAN, ast.str(&format!("_n{n}")))
      .into()
  });
  statements.push(ast.statement_return(
    SPAN,
    Some(if return_nodes.len() > 1 {
      ast.expression_array(SPAN, ast.vec_from_iter(return_nodes))
    } else if let Some(node) = return_nodes.next()
      && let ArrayExpressionElement::Identifier(node) = node
    {
      ast.expression_identifier(SPAN, node.name)
    } else {
      ast.expression_null_literal(SPAN)
    }),
  ));

  if let Some(reset_block) = reset_block {
    reset_block();
  }
  statements
}

pub fn gen_effects<'a>(
  effects: Vec<IREffect<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Option<Statement<'a>> {
  let ast = &context.ast;
  let mut statements = ast.vec();
  let mut operations_count = 0;

  let effects_is_empty = effects.is_empty();
  for effect in effects {
    operations_count += effect.operations.len();
    let _context_block = context_block as *mut BlockIRNode;
    gen_operations(&mut statements, effect.operations, context, unsafe {
      &mut *_context_block
    });
  }

  if effects_is_empty {
    None
  } else {
    Some(
      ast.statement_expression(
        SPAN,
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.str(context.options.helper("_renderEffect"))),
          NONE,
          ast.vec1(
            ast
              .expression_arrow_function(
                SPAN,
                operations_count == 1,
                false,
                NONE,
                ast.formal_parameters(
                  SPAN,
                  FormalParameterKind::ArrowFormalParameters,
                  ast.vec(),
                  NONE,
                ),
                NONE,
                ast.function_body(SPAN, ast.vec(), statements),
              )
              .into(),
          ),
          false,
        ),
      ),
    )
  }
}

pub fn mark_slot_root_operations<'a>(block: &mut BlockIRNode<'a>) {
  let block_ptr = block as *mut _;
  for returned in block.returns.iter() {
    let Some(child) = find_returned_dynamic(unsafe { &mut *block_ptr }, *returned) else {
      continue;
    };
    let Some(operation) = &mut child.operation else {
      continue;
    };

    match operation.as_mut() {
      OperationNode::If(operation) => mark_slot_root_if(operation),
      OperationNode::For(operation) => mark_slot_root_for(operation),
      OperationNode::SlotOutlet(operation) => mark_slot_root_slot_outlet(operation),
      _ => {}
    }
  }
}

fn mark_slot_root_if(operation: &mut IfIRNode) {
  if !operation.once {
    operation.slot_root = true;
  }
  mark_slot_root_operations(&mut operation.positive);

  let Some(negative) = operation.negative.as_mut() else {
    return;
  };
  match negative.as_mut() {
    Either::A(negative) => mark_slot_root_operations(negative),
    Either::B(negative) => mark_slot_root_if(negative),
  }
}

fn mark_slot_root_for(operation: &mut ForIRNode) {
  if !operation.once {
    operation.slot_root = true;
  }
  mark_slot_root_operations(&mut operation.render);
}

fn mark_slot_root_slot_outlet(operation: &mut SlotOutletIRNode) {
  operation.flags |= VaporSlotFlags::SlotRoot as i32;
  if let Some(fallback) = operation.fallback.as_mut() {
    mark_slot_root_operations(fallback);
  }
}

pub fn find_returned_dynamic<'a>(
  block: &'a mut BlockIRNode<'a>,
  id: i32,
) -> Option<&'a mut IRDynamicInfo<'a>> {
  block
    .dynamic
    .children
    .iter_mut()
    .find(|child| child.id.is_some_and(|i| i == id))
}
