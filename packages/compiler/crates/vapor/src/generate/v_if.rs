use common::expression::gen_getter;
use common::patch_flag::VaporBlockShape;
use common::patch_flag::VaporIfFlags;
use napi::Either;
use oxc_ast::NONE;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::NumberBase;
use oxc_ast::ast::Statement;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::gen_block;
use crate::generate::expression::gen_expression;
use crate::ir::index::BlockIRNode;
use crate::ir::index::IfIRNode;
use crate::transform::v_if::is_long_if_chain;

pub fn gen_if<'a>(
  oper: IfIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  is_nested: bool,
) -> Statement<'a> {
  if !is_nested && is_long_if_chain(&oper) {
    return gen_if_chain(oper, context, context_block);
  }
  let ast = &context.ast;
  let IfIRNode {
    id,
    condition,
    positive,
    negative,
    once,
    index,
    block_shape,
    slot_root,
    ..
  } = oper;
  let flags = gen_if_flags(
    block_shape,
    once,
    slot_root,
    if negative.is_some() {
      Some(index)
    } else {
      None
    },
  );

  let condition_expr = gen_getter(gen_expression(condition, context, None, false), ast);

  let _context_block = context_block as *mut BlockIRNode;
  let positive_arg = gen_block(
    positive,
    context,
    unsafe { &mut *_context_block },
    ast.vec(),
  );

  let mut negative_arg = None;
  if let Some(negative) = negative {
    let negative = *negative;
    negative_arg = Some(match negative {
      Either::A(negative) => gen_block(negative, context, context_block, ast.vec()),
      Either::B(negative) => ast.expression_arrow_function(
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
          ast.vec1(gen_if(negative, context, context_block, true)),
        ),
      ),
    });
  }

  let expression = ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.str(context.options.helper("_createIf"))),
    NONE,
    ast.vec_from_iter(
      [
        Some(condition_expr.into()),
        Some(positive_arg.into()),
        if let Some(negative_arg) = negative_arg {
          Some(negative_arg.into())
        } else if flags.is_some() {
          Some(ast.expression_null_literal(SPAN).into())
        } else {
          None
        },
        flags.map(|flags| {
          ast
            .expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Decimal)
            .into()
        }),
      ]
      .into_iter()
      .flatten(),
    ),
    false,
  );

  if !is_nested {
    Statement::VariableDeclaration(ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{}", id))),
        NONE,
        Some(expression),
        false,
      )),
      false,
    ))
  } else {
    ast.statement_expression(SPAN, expression)
  }
}

fn gen_if_flags(block_shape: i32, once: bool, slot_root: bool, index: Option<i32>) -> Option<i32> {
  let mut flags = block_shape;
  if slot_root {
    flags |= VaporIfFlags::SlotRoot as i32;
  }
  if once {
    flags |= VaporIfFlags::Once as i32;
  } else if let Some(index) = index {
    // The encoded index is shifted by +1 so runtime can use 0 as the unkeyed
    // sentinel while preserving source index 0.
    flags |= (index + 1) << VaporIfFlags::IndexShift as i32;
  }

  // This is the only omitted-flags case: true branch is single-root, false
  // branch is empty, and there is no once/index metadata.
  if flags == VaporBlockShape::SingleRoot as i32 {
    return None;
  }

  Some(flags)
}

// `() => (a) ? 0 : (b) ? 1 : 2` selects the branch, `blocks.length` past the
// last one; branch flags describe every branch plus that empty state.
fn gen_if_chain<'a>(
  oper: IfIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let IfIRNode {
    id,
    condition,
    positive,
    negative,
    once,
    index,
    block_shape,
    slot_root,
    ..
  } = oper;

  let context_block = context_block as *mut BlockIRNode;
  let mut conditions = Vec::new();
  let mut blocks = Vec::new();
  let mut branch_flags = Vec::new();
  let mut has_slot_root = false;

  let mut node_condition = condition;
  let mut node_positive = positive;
  let mut node_negative = negative;
  let mut node_shape = block_shape;
  let mut node_slot_root = slot_root;

  loop {
    conditions.push(gen_expression(node_condition, context, None, false));
    blocks.push(gen_block(
      node_positive,
      context,
      unsafe { &mut *context_block },
      ast.vec(),
    ));
    branch_flags.push(node_shape & (0b11 | VaporIfFlags::TrueNoScope as i32));
    has_slot_root |= node_slot_root;

    match node_negative.map(|negative| *negative) {
      Some(Either::B(next)) => {
        node_condition = next.condition;
        node_positive = next.positive;
        node_negative = next.negative;
        node_shape = next.block_shape;
        node_slot_root = next.slot_root;
      }
      Some(Either::A(block)) => {
        blocks.push(gen_block(
          block,
          context,
          unsafe { &mut *context_block },
          ast.vec(),
        ));
        branch_flags.push(if_chain_false_branch_flags(node_shape));
        break;
      }
      None => {
        branch_flags.push(if_chain_false_branch_flags(node_shape));
        break;
      }
    }
  }

  let condition_count = conditions.len();
  let mut selector =
    ast.expression_numeric_literal(SPAN, condition_count as f64, None, NumberBase::Decimal);
  for (i, condition) in conditions.into_iter().enumerate().rev() {
    selector = ast.expression_conditional(
      SPAN,
      condition,
      ast.expression_numeric_literal(SPAN, i as f64, None, NumberBase::Decimal),
      selector,
    );
  }

  let mut flags = 0;
  if once {
    flags |= VaporIfFlags::Once as i32;
  } else {
    flags |= (index + 1) << VaporIfFlags::IndexShift as i32;
  }
  if has_slot_root {
    flags |= VaporIfFlags::SlotRoot as i32;
  }

  // the runtime default: scoped single roots, and an empty state when the
  // chain ends without a `v-else` (whose branch would be a block of its own)
  let block_count = blocks.len();
  let is_default_branch_flags = branch_flags.iter().enumerate().all(|(i, flags)| {
    *flags
      == if i < block_count {
        VaporBlockShape::SingleRoot as i32
      } else {
        VaporBlockShape::Empty as i32
      }
  });
  let branch_flags = if is_default_branch_flags {
    ast.expression_null_literal(SPAN)
  } else {
    ast.expression_array(
      SPAN,
      ast.vec_from_iter(branch_flags.into_iter().map(|flags| {
        ast
          .expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Decimal)
          .into()
      })),
    )
  };

  let expression = ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.str(context.options.helper("_createIfChain"))),
    NONE,
    ast.vec_from_iter(
      [
        gen_getter(selector, ast),
        ast.expression_array(SPAN, ast.vec_from_iter(blocks.into_iter().map(Into::into))),
        branch_flags,
        ast.expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Decimal),
      ]
      .into_iter()
      .map(Into::into),
    ),
    false,
  );

  Statement::VariableDeclaration(ast.alloc_variable_declaration(
    SPAN,
    VariableDeclarationKind::Const,
    ast.vec1(ast.variable_declarator(
      SPAN,
      VariableDeclarationKind::Const,
      ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{}", id))),
      NONE,
      Some(expression),
      false,
    )),
    false,
  ))
}

// the `v-else` branch, or the empty state the selector lands on without one
fn if_chain_false_branch_flags(block_shape: i32) -> i32 {
  ((block_shape >> 2) & 0b11)
    | if block_shape & VaporIfFlags::FalseNoScope as i32 != 0 {
      VaporIfFlags::TrueNoScope as i32
    } else {
      0
    }
}
