use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::mem::{self};
use std::rc::Rc;

use napi::Either;
use oxc_ast::NONE;
use oxc_ast::ast::{
  ArrayExpressionElement, Expression, FormalParameter, FormalParameterKind, Statement,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::directive::gen_custom_directives;
use crate::generate::operation::gen_operations;
use crate::generate::template::gen_self;
use crate::ir::index::{BlockIRNode, ForIRNode, IRDynamicInfo, IREffect, IfIRNode, OperationNode};
use common::patch_flag::VaporSlotFlags;
use indexmap::IndexMap;

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

  // the flush closure below advances these, so they are shared with it
  let operation_index = Rc::new(Cell::new(0));
  let effect_index = Rc::new(Cell::new(0));
  let custom_directives = Rc::new(RefCell::new(IndexMap::new()));
  let deferred_custom_directives = Rc::clone(&custom_directives);
  let flush_operation_index = Rc::clone(&operation_index);
  let flush_effect_index = Rc::clone(&effect_index);
  let flush_before_dynamic = Rc::new(RefCell::new(Box::new(
    move |dynamic: &mut IRDynamicInfo<'a>,
          statements: &mut oxc_allocator::Vec<'a, Statement<'a>>| {
      if let Some(operation) = &mut dynamic.operation
        && let Some((operation_end, effect_end)) = operation.as_mut().block_boundary()
        && let (Some(operation_end), Some(effect_end)) = (*operation_end, *effect_end)
      {
        let effect_end = effect_end as usize;
        let drained = flush_operation_index.get();
        if drained < operation_end {
          gen_operations(
            statements,
            unsafe { &mut *context_block }
              .operation
              .drain(0..operation_end - drained)
              .collect::<Vec<_>>(),
            Some(&mut deferred_custom_directives.borrow_mut()),
            context,
            unsafe { &mut *context_block },
          );
          flush_operation_index.set(operation_end)
        }

        let drained = flush_effect_index.get();
        if drained < effect_end {
          if let Some(statement) = gen_effects(
            unsafe { &mut *context_block }
              .effect
              .drain(0..effect_end - drained)
              .collect::<_>(),
            context,
            context_block,
          ) {
            statements.push(statement);
          };
          flush_effect_index.set(effect_end)
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

  let (deferred_operation_index, deferred_effect_index) = {
    let block = unsafe { &*context_block };
    (block.deferred_operation_index, block.deferred_effect_index)
  };
  gen_operations(
    &mut statements,
    unsafe { &mut *context_block }
      .operation
      .drain(0..deferred_operation_index - operation_index.get())
      .collect(),
    Some(&mut custom_directives.borrow_mut()),
    context,
    unsafe { &mut *context_block },
  );
  if let Some(statement) = gen_effects(
    unsafe { &mut *context_block }
      .effect
      .drain(0..deferred_effect_index - effect_index.get())
      .collect(),
    context,
    context_block,
  ) {
    statements.push(statement);
  }
  if let Some(gen_extra_frag) = gen_effects_extra_frag {
    gen_extra_frag(&mut statements, unsafe { &mut *context_block })
  }
  // v-model directives and the listeners of their elements, see
  // `TransformContext::finalize_block`
  gen_operations(
    &mut statements,
    mem::take(&mut unsafe { &mut *context_block }.operation),
    None,
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
  statements.extend(gen_custom_directives(
    mem::take(&mut custom_directives.borrow_mut()),
    context,
  ));

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

// Marks the root operations of a slot block that has no stable root, and
// returns whether it has one (nothing is marked then).
pub fn mark_slot_root_operations<'a>(
  block: &mut BlockIRNode<'a>,
  context: &CodegenContext<'a>,
) -> bool {
  let roots = get_returned_dynamic_indices(block);
  if has_stable_slot_root(block, &roots, context) {
    return true;
  }

  for root in roots {
    let Some(child) = root.and_then(|index| block.dynamic.children.get_mut(index)) else {
      continue;
    };
    let Some(operation) = &mut child.operation else {
      continue;
    };

    match operation.as_mut() {
      OperationNode::If(operation) => mark_slot_root_if(operation, context),
      OperationNode::For(operation) => mark_slot_root_for(operation, context),
      OperationNode::Key(operation) => {
        operation.slot_root = true;
        mark_slot_root_operations(&mut operation.block, context);
      }
      OperationNode::SlotOutlet(operation) => {
        operation.flags |= VaporSlotFlags::Forwarded as i32;
      }
      _ => {}
    }
  }
  false
}

fn mark_slot_root_if<'a>(operation: &mut IfIRNode<'a>, context: &CodegenContext<'a>) {
  if !operation.once {
    operation.slot_root = true;
  }
  mark_slot_root_operations(&mut operation.positive, context);

  let Some(negative) = operation.negative.as_mut() else {
    return;
  };
  match negative.as_mut() {
    Either::A(negative) => {
      mark_slot_root_operations(negative, context);
    }
    Either::B(negative) => mark_slot_root_if(negative, context),
  }
}

fn mark_slot_root_for<'a>(operation: &mut ForIRNode<'a>, context: &CodegenContext<'a>) {
  if !operation.once {
    operation.slot_root = true;
  }
  mark_slot_root_operations(&mut operation.render, context);
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

// the dynamic info of each returned root, in `returns` order
fn get_returned_dynamic_indices(block: &BlockIRNode) -> Vec<Option<usize>> {
  let children = &block.dynamic.children;
  let mut by_id: HashMap<i32, usize> = HashMap::with_capacity(children.len());
  for (index, child) in children.iter().enumerate() {
    if let Some(id) = child.id {
      by_id.insert(id, index);
    }
  }
  block
    .returns
    .iter()
    .map(|id| by_id.get(id).copied())
    .collect()
}

// A slot can skip fallback/boundary tracking when at least one root is stable.
// Components count as valid even if their own render result is a comment.
fn has_stable_slot_root<'a>(
  block: &BlockIRNode<'a>,
  roots: &[Option<usize>],
  context: &CodegenContext<'a>,
) -> bool {
  for &root in roots {
    let Some(child) = root.and_then(|index| block.dynamic.children.get(index)) else {
      continue;
    };
    let Some(operation) = &child.operation else {
      if is_stable_template_slot_root(child.template, context) {
        return true;
      }
      continue;
    };

    match operation.as_ref() {
      OperationNode::CreateComponent(_) => return true,
      OperationNode::Key(operation) => {
        let nested = get_returned_dynamic_indices(&operation.block);
        if has_stable_slot_root(&operation.block, &nested, context) {
          return true;
        }
      }
      _ => {}
    }
  }
  false
}

fn is_stable_template_slot_root(template: Option<i32>, context: &CodegenContext) -> bool {
  let Some(template) = template else {
    return false;
  };
  context
    .options
    .templates
    .borrow()
    .entries
    .get(template as usize)
    .is_some_and(|entry| !entry.content.is_empty())
}
