use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use oxc_allocator::CloneIn;
use oxc_ast::NONE;

use oxc_ast::ast::Argument;
use oxc_ast::ast::AssignmentOperator;
use oxc_ast::ast::Expression;
use oxc_ast::ast::NumberBase;
use oxc_ast::ast::Statement;
use oxc_ast::ast::Statement::VariableDeclaration;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::FlushBeforeDynamic;
use crate::generate::operation::gen_operation_with_insertion_state;
use crate::ir::index::BlockIRNode;
use crate::ir::index::DynamicAccess;
use crate::ir::index::IRDynamicAccess;
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
  }

  if let Some(operation) = operation {
    let _context_block = context_block as *mut BlockIRNode;
    gen_operation_with_insertion_state(statements, *operation, context, unsafe {
      &mut *_context_block
    });
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
  // the variable of the previous access: adjacent lookups chain off it and
  // placeholders may reassign it
  let mut prev: Option<String> = None;

  let _context_block = context_block as *mut BlockIRNode;
  let mut block_statement_count = 0;
  let from = &from;

  for mut child in children.drain(..) {
    let Some(access) = child.access else {
      gen_self(
        statements,
        child,
        context,
        unsafe { &mut *_context_block },
        Rc::clone(&flush_before_dynamic),
      );
      continue;
    };

    let access_path = gen_access_path(
      context,
      from.clone_in(ast.allocator),
      &access,
      prev.as_deref(),
      child.is_text,
    );

    match access.kind {
      // no variable: the lookup is written into the only descendant access
      DynamicAccess::Inline => {
        let inserted = gen_children(
          statements,
          mem::take(&mut child.children),
          context,
          unsafe { &mut *_context_block },
          statement_index,
          access_path,
          Rc::clone(&flush_before_dynamic),
        );
        statement_index += inserted;
        block_statement_count += inserted;
        continue;
      }
      DynamicAccess::InlineReuse => {
        let inserted = gen_children(
          statements,
          mem::take(&mut child.children),
          context,
          unsafe { &mut *_context_block },
          statement_index,
          ast.expression_parenthesized(
            SPAN,
            ast.expression_assignment(
              SPAN,
              AssignmentOperator::Assign,
              ast
                .simple_assignment_target_assignment_target_identifier(
                  SPAN,
                  ast.str(prev.as_deref().unwrap()),
                )
                .into(),
              access_path,
            ),
          ),
          Rc::clone(&flush_before_dynamic),
        );
        statement_index += inserted;
        block_statement_count += inserted;
        continue;
      }
      DynamicAccess::Reuse => {
        let variable = prev.clone().unwrap();
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
      }
      DynamicAccess::Placeholder => {
        // p for "placeholder" variables that are meant for possible reuse by
        // other access paths
        let temp_id = context_block.temp_id;
        context_block.temp_id = temp_id + 1;
        let variable = format!("_p{temp_id}");
        prev = Some(variable.clone());
        statements.insert(
          statement_index,
          VariableDeclaration(ast.alloc_variable_declaration(
            SPAN,
            VariableDeclarationKind::Let,
            ast.vec1(ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Let,
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
      DynamicAccess::Reference => {
        let variable = format!("_n{}", access.id.unwrap());
        prev = Some(variable.clone());
        statements.insert(
          statement_index,
          VariableDeclaration(ast.alloc_variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            ast.vec1(ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Const,
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
    }

    // A child created by its own operation (component, block, createElement-
    // backed element) owns its subtree through gen_self; only children that
    // sit in the parent template are descended into from here.
    if child.operation.is_some() {
      gen_self(
        statements,
        child,
        context,
        unsafe { &mut *_context_block },
        Rc::clone(&flush_before_dynamic),
      );
    } else if child.has_dynamic_child {
      let inserted = gen_children(
        statements,
        mem::take(&mut child.children),
        context,
        unsafe { &mut *_context_block },
        statement_index,
        ast.expression_identifier(SPAN, ast.str(prev.as_deref().unwrap())),
        Rc::clone(&flush_before_dynamic),
      );
      statement_index += inserted;
      block_statement_count += inserted;
    }
  }

  block_statement_count
}

// Build one DOM lookup path: an adjacent sibling walks from the previous
// access with `_next`, otherwise the lookup starts over from the parent with
// `_child` or `_nthChild`. `is_text` appends a hint so a missing SSR text node
// is seeded in place.
fn gen_access_path<'a>(
  context: &CodegenContext<'a>,
  from: Expression<'a>,
  access: &IRDynamicAccess,
  prev: Option<&str>,
  is_text: bool,
) -> Expression<'a> {
  let ast = context.ast;
  if access.adjacent {
    return gen_access_call(
      context,
      "_next",
      vec![ast.expression_identifier(SPAN, ast.str(prev.unwrap()))],
      is_text,
    );
  }

  if access.index == 0 {
    return gen_access_call(context, "_child", vec![from], is_text);
  }

  // adjacent to the first child: chain off it instead of an indexed lookup
  if access.index == 1 {
    let first_child = gen_access_call(context, "_child", vec![from], false);
    return gen_access_call(context, "_next", vec![first_child], is_text);
  }

  gen_access_call(
    context,
    "_nthChild",
    vec![
      from,
      ast.expression_numeric_literal(SPAN, access.index as f64, None, NumberBase::Decimal),
    ],
    is_text,
  )
}

fn gen_access_call<'a>(
  context: &CodegenContext<'a>,
  helper: &'static str,
  args: Vec<Expression<'a>>,
  is_text: bool,
) -> Expression<'a> {
  let ast = context.ast;
  let mut arguments: oxc_allocator::Vec<'a, Argument<'a>> = ast.vec();
  for arg in args {
    arguments.push(arg.into());
  }
  if is_text {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }
  ast.expression_call(
    SPAN,
    ast.expression_identifier(SPAN, ast.str(context.options.helper(helper))),
    NONE,
    arguments,
    false,
  )
}
