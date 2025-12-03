use napi::bindgen_prelude::Either16;
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
      if let Either16::H(op) = op {
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
    Either16::A(if_ir_node) => {
      if let Some(parent) = if_ir_node.parent {
        statements.push(gen_insertion_state(parent, if_ir_node.anchor, context))
      }
    }
    Either16::B(for_ir_node) => {
      if let Some(parent) = for_ir_node.parent {
        statements.push(gen_insertion_state(parent, for_ir_node.anchor, context))
      }
    }
    Either16::N(create_component_ir_node) => {
      if let Some(parent) = create_component_ir_node.parent {
        statements.push(gen_insertion_state(
          parent,
          create_component_ir_node.anchor,
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
    Either16::A(oper) => statements.push(gen_if(oper, context, context_block, false)),
    Either16::B(oper) => gen_for(statements, oper, context, context_block),
    Either16::C(oper) => statements.push(gen_set_text(oper, context)),
    Either16::D(oper) => statements.push(gen_set_prop(oper, context)),
    Either16::E(oper) => statements.push(gen_dynamic_props(oper, context)),
    Either16::F(oper) => statements.push(gen_set_dynamic_events(oper, context)),
    Either16::G(oper) => statements.push(gen_set_nodes(oper, context)),
    Either16::H(oper) => statements.push(gen_set_event(oper, context, event_opers)),
    Either16::I(oper) => statements.push(gen_set_html(oper, context)),
    Either16::J(oper) => statements.push(gen_set_template_ref(oper, context)),
    Either16::K(oper) => statements.push(gen_create_nodes(oper, context)),
    Either16::L(oper) => statements.push(gen_insert_node(oper, context)),
    Either16::M(oper) => {
      if let Some(statement) = gen_builtin_directive(oper, context) {
        statements.push(statement)
      }
    }
    Either16::N(oper) => gen_create_component(statements, oper, context, context_block),
    Either16::O(oper) => statements.push(gen_declare_old_ref(oper, context)),
    Either16::P(oper) => statements.push(gen_get_text_child(oper, context)),
  }
}

pub fn gen_insertion_state<'a>(
  parent: i32,
  anchor: Option<i32>,
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
            } else {
              Some(Argument::Identifier(ast.alloc_identifier_reference(
                SPAN,
                ast.atom(&format!("n{anchor}")),
              )))
            }
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
