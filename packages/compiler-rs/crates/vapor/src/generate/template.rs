use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use oxc_allocator::CloneIn;
use oxc_ast::NONE;

use oxc_ast::ast::AssignmentOperator;
use oxc_ast::ast::Expression;
use oxc_ast::ast::NumberBase;
use oxc_ast::ast::Statement;
use oxc_ast::ast::Statement::VariableDeclaration;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::FlushBeforeDynamic;
use crate::generate::directive::gen_directives_for_element;
use crate::generate::operation::gen_operation_with_insertion_state;
use crate::ir::index::BlockIRNode;
use crate::ir::index::DynamicFlag;
use crate::ir::index::IRDynamicInfo;

pub fn gen_self<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  mut dynamic: IRDynamicInfo<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  flush_before_dynamic: Rc<RefCell<FlushBeforeDynamic<'a>>>,
) {
  flush_before_dynamic.borrow_mut().as_mut()(&mut dynamic, statements);
  let ast = &context.ast;
  let IRDynamicInfo {
    id,
    children,
    template,
    operation,
    has_dynamic_child,
    ..
  } = dynamic;

  if let Some(id) = id
    && let Some(template) = template
  {
    statements.push(Statement::VariableDeclaration(
      ast.alloc_variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ast.vec1(ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{id}"))),
          NONE,
          Some(ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.str(&format!("_t{template}"))),
            NONE,
            ast.vec(),
            false,
          )),
          false,
        )),
        false,
      ),
    ));
    if let Some(directives) = gen_directives_for_element(id, context, context_block) {
      statements.push(directives)
    }
  }

  if let Some(operation) = operation {
    let _context_block = context_block as *mut BlockIRNode;
    gen_operation_with_insertion_state(
      statements,
      *operation,
      context,
      unsafe { &mut *_context_block },
      &vec![],
    );
  }

  if has_dynamic_child {
    gen_children(
      statements,
      children,
      context,
      context_block,
      statements.len(),
      ast.expression_identifier(SPAN, ast.str(format!("_n{}", id.unwrap_or(0)).as_str())),
      Rc::clone(&flush_before_dynamic),
    );
  }
}

fn gen_children<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  mut children: Vec<IRDynamicInfo<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  mut statement_index: usize,
  from: Expression<'a>,
  flush_before_dynamic: Rc<RefCell<FlushBeforeDynamic<'a>>>,
) -> usize {
  let ast = &context.ast;

  let mut offset: i32 = 0;
  // `reusable` means the previous access target is a p* cursor that can be
  // reassigned by the next lookup. Referenced n* variables must stay stable.
  let mut prev: Option<(String, i32, bool)> = None;

  let _context_block = context_block as *mut BlockIRNode;
  let mut block_statement_count = 0;
  let mut index = 0;
  let mut iter = children.drain(..);
  let from = &from;
  while let Some(mut child) = iter.next() {
    let rest = iter.as_slice();
    if child.flags & DynamicFlag::NonTemplate as i32 != 0 {
      offset -= 1;
    }

    if child.flags & DynamicFlag::Insert as i32 != 0 && child.template.is_some() {
      gen_self(
        statements,
        child,
        context,
        unsafe { &mut *_context_block },
        Rc::clone(&flush_before_dynamic),
      );
      index += 1;
      continue;
    }

    let id = if child.flags & DynamicFlag::Referenced as i32 != 0 {
      if child.flags & DynamicFlag::Insert as i32 != 0 {
        child.anchor
      } else {
        child.id
      }
    } else {
      None
    };

    if id.is_none() && !child.has_dynamic_child {
      gen_self(
        statements,
        child,
        context,
        unsafe { &mut *_context_block },
        Rc::clone(&flush_before_dynamic),
      );
      index += 1;
      continue;
    }

    let element_index = index as i32 + offset;
    let logical_index = child.logical_index;

    let inline_placeholder = id.is_none()
      && child.template.is_none()
      && child.operation.is_none()
      && child.flags & (DynamicFlag::Insert as i32 | DynamicFlag::NonTemplate as i32) == 0
      && can_inline_placehoder(&child);
    let access_path = gen_access_path(
      context,
      from.clone_in(ast.allocator),
      &child,
      element_index,
      logical_index,
      &prev,
    );

    if inline_placeholder {
      if let Some(prev) = &mut prev
        && prev.2
      {
        let child_children = mem::take(&mut child.children);
        let inserted = gen_children(
          statements,
          child_children,
          context,
          unsafe { &mut *_context_block },
          statement_index,
          ast.expression_parenthesized(
            SPAN,
            ast.expression_assignment(
              SPAN,
              AssignmentOperator::Assign,
              ast
                .simple_assignment_target_assignment_target_identifier(SPAN, ast.str(&prev.0))
                .into(),
              access_path,
            ),
          ),
          Rc::clone(&flush_before_dynamic),
        );
        statement_index += inserted;
        block_statement_count += inserted;
        prev.1 = element_index;
        prev.2 = true;
        index += 1;
        continue;
      }

      if !has_adjacent_following_access_child(rest, offset) {
        let child_children = mem::take(&mut child.children);
        let inserted = gen_children(
          statements,
          child_children,
          context,
          unsafe { &mut *_context_block },
          statement_index,
          access_path,
          Rc::clone(&flush_before_dynamic),
        );
        statement_index += inserted;
        block_statement_count += inserted;
        index += 1;
        continue;
      }
    }

    let variable: String;
    if id.is_none()
      && let Some(prev) = &prev
      && prev.2
    {
      variable = prev.0.clone();
      statements.insert(
        statement_index,
        ast.statement_expression(
          SPAN,
          ast.expression_assignment(
            SPAN,
            AssignmentOperator::Assign,
            ast
              .simple_assignment_target_assignment_target_identifier(SPAN, ast.str(&variable))
              .into(),
            access_path,
          ),
        ),
      );
      statement_index += 1;
      block_statement_count += 1;
    } else {
      // p for "placeholder" variables that are meant for possible reuse by
      // other access paths
      variable = if let Some(id) = id {
        format!("_n{id}")
      } else {
        let temp_id = context_block.temp_id;
        context_block.temp_id = temp_id + 1;
        format!("_p{}", temp_id)
      };
      let kind = if id.is_none() {
        VariableDeclarationKind::Let
      } else {
        VariableDeclarationKind::Const
      };
      statements.insert(
        statement_index,
        VariableDeclaration(ast.alloc_variable_declaration(
          SPAN,
          kind,
          ast.vec1(ast.variable_declarator(
            SPAN,
            kind,
            ast.binding_pattern_binding_identifier(SPAN, ast.str(&variable)),
            NONE,
            Some(access_path),
            false,
          )),
          false,
        )),
      );
      statement_index += 1;
      block_statement_count += 1;
    }

    let child_children = mem::take(&mut child.children);
    if id.eq(&child.anchor) && !child.has_dynamic_child {
      gen_self(
        statements,
        child,
        context,
        unsafe { &mut *_context_block },
        Rc::clone(&flush_before_dynamic),
      );
    }

    if let Some(id) = id
      && let Some(directives) =
        gen_directives_for_element(id, context, unsafe { &mut *_context_block })
    {
      statements.push(directives)
    };

    let inserted = gen_children(
      statements,
      child_children,
      context,
      unsafe { &mut *_context_block },
      statement_index,
      ast.expression_identifier(SPAN, ast.str(&variable)),
      Rc::clone(&flush_before_dynamic),
    );
    statement_index += inserted;
    block_statement_count += inserted;
    prev = Some((variable, element_index, id.is_none()));
    index += 1;
  }

  block_statement_count
}

// Build one DOM lookup path while preserving the fast sibling walk:
// adjacent nodes use _next(prev), otherwise fall back to _nthChild(parent).
fn gen_access_path<'a>(
  context: &CodegenContext<'a>,
  from: Expression<'a>,
  child: &IRDynamicInfo<'a>,
  element_index: i32,
  logical_index: Option<i32>,
  prev: &Option<(String, i32, bool)>,
) -> Expression<'a> {
  let ast = context.ast;
  if let Some(prev) = prev {
    return if element_index - prev.1 == 1 {
      ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(context.options.helper("_next"))),
        NONE,
        ast.vec_from_iter(
          [
            Some(ast.expression_identifier(SPAN, ast.str(&prev.0)).into()),
            logical_index.map(|logical_index| {
              ast
                .expression_numeric_literal(SPAN, logical_index as f64, None, NumberBase::Decimal)
                .into()
            }),
          ]
          .into_iter()
          .flatten(),
        ),
        false,
      )
    } else {
      gen_nth_child(
        context,
        from.clone_in(ast.allocator),
        element_index,
        logical_index,
      )
    };
  }

  if element_index == 0 {
    return ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_child"))),
      NONE,
      ast.vec_from_iter(
        [
          Some(from.clone_in(ast.allocator).into()),
          child.logical_index.and_then(|logical_index| {
            if logical_index != 0 {
              Some(
                ast
                  .expression_numeric_literal(SPAN, logical_index as f64, None, NumberBase::Decimal)
                  .into(),
              )
            } else {
              None
            }
          }),
        ]
        .into_iter()
        .flatten(),
      ),
      false,
    );
  }

  if element_index == 1 {
    let first_child = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_child"))),
      NONE,
      ast.vec1(from.clone_in(ast.allocator).into()),
      false,
    );

    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_next"))),
      NONE,
      ast.vec_from_iter(
        [
          Some(first_child.into()),
          logical_index.map(|logical_index| {
            ast
              .expression_numeric_literal(SPAN, logical_index as f64, None, NumberBase::Decimal)
              .into()
          }),
        ]
        .into_iter()
        .flatten(),
      ),
      false,
    )
  } else {
    gen_nth_child(context, from, element_index, logical_index)
  }
}

// Only inline a placeholder when materializing it would not save a parent
// lookup. If its child tree needs the parent more than once, keep p* so the
// generated code does not duplicate _child/_nthChild work.
fn can_inline_placehoder(dynamic: &IRDynamicInfo) -> bool {
  dynamic.has_dynamic_child && count_parent_access_usages(dynamic) == 1
}

// A following access can reuse the current placeholder cursor only when it is
// the next DOM sibling. Gapped siblings need _nthChild(parent, index) instead.
fn has_adjacent_following_access_child(children: &[IRDynamicInfo], offset: i32) -> bool {
  let mut future_offset = offset;
  let mut i = 0;
  while i < children.len() {
    let child = &children[i];
    if child.flags & DynamicFlag::NonTemplate as i32 > 0 {
      future_offset -= 1;
    }
    if !(child.flags & DynamicFlag::Insert as i32 > 0 && child.template.is_some())
      && (child.flags & DynamicFlag::Referenced as i32 > 0 || child.has_dynamic_child)
    {
      return i as i32 + future_offset - offset == 0;
    }
    i += 1;
  }

  false
}

// Mirrors genChildren's traversal closely enough to count how many emitted
// access paths would start from this placeholder's parent. This is the guard
// that keeps inline placeholders from duplicating parent lookups.
fn count_parent_access_usages(dynamic: &IRDynamicInfo) -> i32 {
  let mut usages = 0;
  let mut offset = 0;
  let mut prev: Option<(i32, bool)> = None;

  for (index, child) in dynamic.children.iter().enumerate() {
    if child.flags & DynamicFlag::NonTemplate as i32 > 0 {
      offset -= 1;
    }

    if child.flags & DynamicFlag::Insert as i32 > 0 && child.template.is_some() {
      continue;
    }

    let id = if child.flags & DynamicFlag::Referenced as i32 > 0 {
      if child.flags & DynamicFlag::Insert as i32 > 0 {
        child.anchor
      } else {
        child.id
      }
    } else {
      None
    };

    if id.is_none() && !child.has_dynamic_child {
      continue;
    }

    let element_index = index as i32 + offset;
    let uses_parent = if let Some(prev) = prev {
      (element_index - prev.0) != 1
    } else {
      true
    };
    let inline_placeholder = id.is_none()
      && child.template.is_none()
      && child.operation.is_none()
      && child.flags & (DynamicFlag::Insert as i32 | DynamicFlag::NonTemplate as i32) == 0
      && can_inline_placehoder(child);

    if inline_placeholder {
      if let Some(prev_state) = prev
        && prev_state.1
      {
        if uses_parent {
          usages += 1;
        }
        prev = Some((element_index, true));
        continue;
      }

      if !has_adjacent_following_access_child(&dynamic.children[index + 1..], offset) {
        if uses_parent {
          usages += 1;
        }
        continue;
      }
    }

    if uses_parent {
      usages += 1;
    }
    prev = Some((element_index, id.is_none()));
  }
  usages
}

fn gen_nth_child<'a>(
  context: &CodegenContext<'a>,
  from: Expression<'a>,
  element_index: i32,
  logical_index: Option<i32>,
) -> Expression<'a> {
  let ast = context.ast;
  ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.str(context.options.helper("_nthChild"))),
    NONE,
    ast.vec_from_iter(
      [
        Some(from.into()),
        Some(
          ast
            .expression_numeric_literal(SPAN, element_index as f64, None, NumberBase::Decimal)
            .into(),
        ),
        // nthChild defaults the logical index to the element index at runtime, so
        // the third argument is only needed when hydration uses a different index.
        if let Some(logical_index) = logical_index {
          if logical_index == element_index {
            None
          } else {
            Some(
              ast
                .expression_numeric_literal(SPAN, logical_index as f64, None, NumberBase::Decimal)
                .into(),
            )
          }
        } else {
          None
        },
      ]
      .into_iter()
      .flatten(),
    ),
    false,
  )
}
