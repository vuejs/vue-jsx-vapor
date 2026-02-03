use std::mem;

use oxc_ast::NONE;
use oxc_ast::ast::Argument;
use oxc_ast::ast::NumberBase;
use oxc_ast::ast::Statement;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::directive::gen_directives_for_element;
use crate::generate::operation::gen_operation_with_insertion_state;
use crate::ir::index::BlockIRNode;
use crate::ir::index::DynamicFlag;
use crate::ir::index::IRDynamicInfo;

pub fn gen_self<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  dynamic: IRDynamicInfo<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
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
          ast.binding_pattern_binding_identifier(SPAN, ast.atom(&format!("_n{id}"))),
          NONE,
          Some(ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&format!("_t{template}"))),
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
      format!("_n{}", id.unwrap_or(0)),
    );
  }
}

fn gen_children<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  mut children: Vec<IRDynamicInfo<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  mut statement_index: usize,
  from: String,
) {
  let ast = &context.ast;

  let mut offset = 0;
  let mut prev: Option<(String, i32)> = None;

  let _context_block = context_block as *mut BlockIRNode;
  for (index, mut child) in children.drain(..).enumerate() {
    if child.flags & DynamicFlag::NonTemplate as i32 != 0 {
      offset -= 1;
    }

    if child.flags & DynamicFlag::Insert as i32 != 0 && child.template.is_some() {
      gen_self(statements, child, context, unsafe { &mut *_context_block });
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
      gen_self(statements, child, context, unsafe { &mut *_context_block });
      continue;
    }

    let element_index = index as i32 + offset;
    let logical_index = child.logical_index;
    // p for "placeholder" variables that are meant for possible reuse by
    // other access paths
    let variable = if let Some(id) = id {
      format!("_n{id}")
    } else {
      let temp_id = context_block.temp_id;
      context_block.temp_id = temp_id + 1;
      format!("_p{}", temp_id)
    };

    let expression_call = if let Some(prev) = prev {
      if element_index - prev.1 == 1 {
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("next"))),
          NONE,
          ast.vec_from_iter(
            [
              Some(Argument::Identifier(
                ast.alloc_identifier_reference(SPAN, ast.atom(&prev.0)),
              )),
              if let Some(logical_index) = logical_index {
                Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                  SPAN,
                  logical_index as f64,
                  None,
                  NumberBase::Hex,
                )))
              } else {
                None
              },
            ]
            .into_iter()
            .flatten(),
          ),
          false,
        )
      } else {
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("nthChild"))),
          NONE,
          ast.vec_from_iter(
            [
              Some(Argument::Identifier(
                ast.alloc_identifier_reference(SPAN, ast.atom(&from)),
              )),
              Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                SPAN,
                element_index as f64,
                None,
                NumberBase::Hex,
              ))),
              if let Some(logical_index) = logical_index {
                Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                  SPAN,
                  logical_index as f64,
                  None,
                  NumberBase::Hex,
                )))
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
    } else if element_index == 0 {
      ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.atom(&context.helper("child"))),
        NONE,
        ast.vec_from_iter(
          [
            Some(Argument::Identifier(
              ast.alloc_identifier_reference(SPAN, ast.atom(&from)),
            )),
            if let Some(logical_index) = logical_index
              && logical_index != 0
            {
              Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                SPAN,
                logical_index as f64,
                None,
                NumberBase::Hex,
              )))
            } else {
              None
            },
          ]
          .into_iter()
          .flatten(),
        ),
        false,
      )
    } else {
      // check if there's a node that we can reuse from
      if element_index == 1 {
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("next"))),
          NONE,
          ast.vec_from_iter(
            [
              Some(Argument::CallExpression(ast.alloc_call_expression(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&context.helper("child"))),
                NONE,
                ast.vec1(Argument::Identifier(
                  ast.alloc_identifier_reference(SPAN, ast.atom(&from)),
                )),
                false,
              ))),
              if let Some(logical_index) = logical_index {
                Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                  SPAN,
                  logical_index as f64,
                  None,
                  NumberBase::Hex,
                )))
              } else {
                None
              },
            ]
            .into_iter()
            .flatten(),
          ),
          false,
        )
      } else if element_index > 1 {
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("nthChild"))),
          NONE,
          ast.vec_from_iter(
            [
              Some(Argument::Identifier(
                ast.alloc_identifier_reference(SPAN, ast.atom(&from)),
              )),
              Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                SPAN,
                element_index as f64,
                None,
                NumberBase::Hex,
              ))),
              if let Some(logical_index) = logical_index {
                Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                  SPAN,
                  logical_index as f64,
                  None,
                  NumberBase::Hex,
                )))
              } else {
                None
              },
            ]
            .into_iter()
            .flatten(),
          ),
          false,
        )
      } else {
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("child"))),
          NONE,
          ast.vec1(Argument::Identifier(
            ast.alloc_identifier_reference(SPAN, ast.atom(&from)),
          )),
          false,
        )
      }
    };

    statements.insert(
      statement_index,
      Statement::VariableDeclaration(ast.alloc_variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ast.vec1(ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern_binding_identifier(SPAN, ast.atom(&variable)),
          NONE,
          Some(expression_call),
          false,
        )),
        false,
      )),
    );
    statement_index += 1;

    let child_children = mem::take(&mut child.children);
    if id.eq(&child.anchor) && !child.has_dynamic_child {
      gen_self(statements, child, context, unsafe { &mut *_context_block });
    }

    if let Some(id) = id
      && let Some(directives) =
        gen_directives_for_element(id, context, unsafe { &mut *_context_block })
    {
      statements.push(directives)
    };

    prev = Some((variable.clone(), element_index));
    gen_children(
      statements,
      child_children,
      context,
      unsafe { &mut *_context_block },
      statement_index,
      variable,
    );
  }
}
