use std::cell::RefCell;
use std::mem::{self};
use std::rc::Rc;

use napi::Either;
use oxc_ast::NONE;
use oxc_ast::ast::{
  ArrayExpressionElement, Expression, FormalParameter, FormalParameterKind, Statement,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::operation::gen_operations;
use crate::generate::template::gen_self;
use crate::ir::index::{BlockIRNode, ForIRNode, IRDynamicInfo, IREffect, IfIRNode, OperationNode};
use common::patch_flag::VaporSlotFlags;

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
  // Built-in v-model needs to run after initial DOM props are applied. This is
  // especially important for inputs with a dynamic type, since the runtime
  // selects the text, checkbox, or radio implementation from the DOM property.
  let model_operations = Rc::new(RefCell::new(Vec::new()));
  let deferred_model_operations = Rc::clone(&model_operations);
  let flush_before_dynamic = Rc::new(RefCell::new(Box::new(
    move |dynamic: &mut IRDynamicInfo<'a>,
          statements: &mut oxc_allocator::Vec<'a, Statement<'a>>| {
      if let Some(operation) = &mut dynamic.operation
        && let Some((Some(operation_end), Some(effect_end))) = match operation.as_ref() {
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
      {
        if operation_index < operation_end {
          gen_operations(
            statements,
            unsafe { &mut *context_block }
              .operation
              .drain(0..operation_end - operation_index)
              .collect::<Vec<_>>(),
            Some(&mut deferred_model_operations.borrow_mut()),
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
              .collect::<_>(),
            context,
            context_block,
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
    Some(&mut model_operations.borrow_mut()),
    context,
    unsafe { &mut *context_block },
  );
  if let Some(statement) = gen_effects(
    mem::take(&mut unsafe { &mut *context_block }.effect),
    context,
    context_block,
  ) {
    statements.push(statement);
  }
  if let Some(gen_extra_frag) = gen_effects_extra_frag {
    gen_extra_frag(&mut statements, unsafe { &mut *context_block })
  }
  gen_operations(
    &mut statements,
    mem::take(&mut model_operations.borrow_mut()),
    None,
    context,
    unsafe { &mut *context_block },
  );

  let mut return_nodes = unsafe { &mut *context_block }.returns.iter().map(|n| {
    ast
      .expression_identifier(SPAN, ast.str(&format!("_n{n}")))
      .into()
  });
  statements.push(ast.statement_return(
    SPAN,
    Some(
      if return_nodes.len() == 1
        && let Some(node) = return_nodes.next()
        && let ArrayExpressionElement::Identifier(node) = node
      {
        ast.expression_identifier(SPAN, node.name)
      } else {
        ast.expression_array(SPAN, ast.vec_from_iter(return_nodes))
      },
    ),
  ));

  if let Some(reset_block) = reset_block {
    reset_block();
  }
  statements
}

fn gen_effects<'a>(
  effects: Vec<IREffect<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: *mut BlockIRNode<'a>,
) -> Option<Statement<'a>> {
  let ast = &context.ast;
  let mut statements = ast.vec();
  let mut operations_count = 0;

  for effect in effects {
    operations_count += effect.operations.len();
    gen_operations(&mut statements, effect.operations, None, context, unsafe {
      &mut *context_block
    });
  }

  if operations_count > 0 {
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
  } else {
    None
  }
}

pub fn mark_slot_root_operations<'a>(
  block: &mut BlockIRNode<'a>,
  context: &CodegenContext<'a>,
  mut shared_fallback: bool,
) {
  if has_stable_slot_root(block, context) {
    return;
  }

  shared_fallback = shared_fallback || has_multiple_dynamic_slot_roots(block);

  let block_ptr = block as *mut _;
  for returned in block.returns.iter() {
    let Some(child) = find_returned_dynamic(unsafe { &mut *block_ptr }, *returned) else {
      continue;
    };
    let Some(operation) = &mut child.operation else {
      continue;
    };

    match operation.as_mut() {
      OperationNode::If(operation) => mark_slot_root_if(operation, context, shared_fallback),
      OperationNode::For(operation) => mark_slot_root_for(operation, context),
      OperationNode::Key(operation) => {
        operation.slot_root = true;
        mark_slot_root_operations(&mut operation.block, context, shared_fallback);
      }
      OperationNode::SlotOutlet(operation) => {
        operation.flags |= VaporSlotFlags::Forwarded as i32;
        if shared_fallback {
          operation.flags |= VaporSlotFlags::SharedFallback as i32;
        }
      }
      _ => {}
    }
  }
}

fn mark_slot_root_if<'a>(
  operation: &mut IfIRNode<'a>,
  context: &CodegenContext<'a>,
  shared_fallback: bool,
) {
  if !operation.once {
    operation.slot_root = true;
  }
  mark_slot_root_operations(&mut operation.positive, context, shared_fallback);

  let Some(negative) = operation.negative.as_mut() else {
    return;
  };
  match negative.as_mut() {
    Either::A(negative) => mark_slot_root_operations(negative, context, shared_fallback),
    Either::B(negative) => mark_slot_root_if(negative, context, shared_fallback),
  }
}

fn mark_slot_root_for<'a>(operation: &mut ForIRNode<'a>, context: &CodegenContext<'a>) {
  if !operation.once {
    operation.slot_root = true;
  }
  mark_slot_root_operations(&mut operation.render, context, true);
}

fn has_multiple_dynamic_slot_roots<'a>(block: &mut BlockIRNode<'a>) -> bool {
  let mut count = 0;
  let block_ptr = block as *mut BlockIRNode;
  for id in block.returns.iter() {
    let Some(child) = find_returned_dynamic(unsafe { &mut *block_ptr }, *id) else {
      continue;
    };
    if child.operation.is_some() {
      count += 1;
      if count > 1 {
        return true;
      }
    }
  }
  false
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

// A slot can skip fallback/boundary tracking when at least one root is stable.
// Components count as valid even if their own render result is a comment.
pub fn has_stable_slot_root<'a>(block: &mut BlockIRNode<'a>, context: &CodegenContext<'a>) -> bool {
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
        continue;
      }
      OperationNode::Key(operation) => {
        if has_stable_slot_root(&mut operation.block, context) {
          has_valid_root = true;
          continue;
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
    .is_some_and(|entry| !entry.content.is_empty())
}
