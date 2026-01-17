use napi::bindgen_prelude::Either17;
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, NumberBase, Statement};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::component::gen_create_component;
use crate::generate::directive::gen_builtin_directive;
use crate::generate::dom::gen_insert_node;
use crate::generate::event::gen_set_dynamic_events;
use crate::generate::event::gen_set_event;
use crate::generate::html::gen_set_html;
use crate::generate::key::gen_key;
use crate::generate::prop::gen_dynamic_props;
use crate::generate::prop::gen_set_prop;
use crate::generate::template_ref::gen_declare_old_ref;
use crate::generate::template_ref::gen_set_template_ref;
use crate::generate::text::gen_create_nodes;
use crate::generate::text::gen_get_text_child;
use crate::generate::text::gen_set_nodes;
use crate::generate::text::gen_set_text;
use crate::generate::v_for::gen_for;
use crate::generate::v_if::gen_if;
use crate::ir::index::BlockIRNode;
use crate::ir::index::OperationNode;
use crate::ir::index::SetEventIRNode;

pub fn gen_operations<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  opers: Vec<OperationNode<'a>>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
  let event_opers = opers
    .iter()
    .filter_map(|op| {
      if let Either17::H(op) = op {
        Some(op.clone())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  let _context_block = context_block as *mut BlockIRNode;
  for operation in opers {
    gen_operation_with_insertion_state(
      statements,
      operation,
      context,
      unsafe { &mut *_context_block },
      &event_opers,
    );
  }
}

pub fn gen_operation_with_insertion_state<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  oper: OperationNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  event_opers: &Vec<SetEventIRNode>,
) {
  match &oper {
    Either17::A(if_ir_node) => {
      if let Some(parent) = if_ir_node.parent {
        statements.push(gen_insertion_state(
          parent,
          if_ir_node.anchor,
          if_ir_node.append,
          if_ir_node.last,
          context,
        ))
      }
    }
    Either17::B(for_ir_node) => {
      if let Some(parent) = for_ir_node.parent {
        statements.push(gen_insertion_state(
          parent,
          for_ir_node.anchor,
          for_ir_node.append,
          for_ir_node.last,
          context,
        ))
      }
    }
    Either17::N(create_component_ir_node) => {
      if let Some(parent) = create_component_ir_node.parent {
        statements.push(gen_insertion_state(
          parent,
          create_component_ir_node.anchor,
          create_component_ir_node.append,
          create_component_ir_node.last,
          context,
        ))
      }
    }
    Either17::Q(key_ir_node) => {
      if let Some(parent) = key_ir_node.parent {
        statements.push(gen_insertion_state(
          parent,
          key_ir_node.anchor,
          key_ir_node.append,
          key_ir_node.last,
          context,
        ))
      }
    }
    _ => (),
  };

  gen_operation(statements, oper, context, context_block, event_opers);
}

pub fn gen_operation<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  oper: OperationNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  event_opers: &Vec<SetEventIRNode>,
) {
  match oper {
    Either17::A(oper) => statements.push(gen_if(oper, context, context_block, false)),
    Either17::B(oper) => gen_for(statements, oper, context, context_block),
    Either17::C(oper) => statements.push(gen_set_text(oper, context)),
    Either17::D(oper) => statements.push(gen_set_prop(oper, context)),
    Either17::E(oper) => statements.push(gen_dynamic_props(oper, context)),
    Either17::F(oper) => statements.push(gen_set_dynamic_events(oper, context)),
    Either17::G(oper) => statements.push(gen_set_nodes(oper, context)),
    Either17::H(oper) => statements.push(gen_set_event(oper, context, event_opers)),
    Either17::I(oper) => statements.push(gen_set_html(oper, context)),
    Either17::J(oper) => statements.push(gen_set_template_ref(oper, context)),
    Either17::K(oper) => statements.push(gen_create_nodes(oper, context)),
    Either17::L(oper) => statements.push(gen_insert_node(oper, context)),
    Either17::M(oper) => {
      if let Some(statement) = gen_builtin_directive(oper, context) {
        statements.push(statement)
      }
    }
    Either17::N(oper) => gen_create_component(statements, oper, context, context_block),
    Either17::O(oper) => statements.push(gen_declare_old_ref(oper, context)),
    Either17::P(oper) => statements.push(gen_get_text_child(oper, context)),
    Either17::Q(oper) => statements.push(gen_key(oper, context, context_block)),
  }
}

pub fn gen_insertion_state<'a>(
  parent: i32,
  anchor: Option<i32>,
  append: bool,
  last: bool,
  context: &CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("setInsertionState"))),
      NONE,
      ast.vec_from_iter(
        [
          Some(Argument::Identifier(ast.alloc_identifier_reference(
            SPAN,
            ast.atom(&format!("n{}", parent)),
          ))),
          if let Some(anchor) = anchor {
            if anchor == -1 {
              // -1 indicates prepend
              Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                SPAN,
                0 as f64,
                None,
                NumberBase::Hex,
              ))) // runtime anchor value for prepend
            } else if append {
              // null or anchor > 0 for append
              // anchor > 0 is the logical index of append node - used for locate node during hydration
              if anchor == 0 {
                Some(Argument::NullLiteral(ast.alloc_null_literal(SPAN)))
              } else {
                Some(Argument::NumericLiteral(ast.alloc_numeric_literal(
                  SPAN,
                  anchor as f64,
                  None,
                  NumberBase::Hex,
                )))
              }
            } else {
              Some(Argument::Identifier(ast.alloc_identifier_reference(
                SPAN,
                ast.atom(&format!("n{anchor}")),
              )))
            }
          } else {
            None
          },
          if last {
            Some(Argument::BooleanLiteral(
              ast.alloc_boolean_literal(SPAN, true),
            ))
          } else {
            None
          },
        ]
        .into_iter()
        .flatten(),
      ),
      false,
    ),
  )
}
