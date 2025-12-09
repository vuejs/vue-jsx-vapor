use std::mem;

use oxc_ast::{
  NONE,
  ast::{Expression, FormalParameter, FormalParameterKind, JSXChild, Statement},
};
use oxc_span::{GetSpan, SPAN};

use crate::{ast::NodeTypes, generate::CodegenContext, ir::index::BlockIRNode};

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
      ast.vec(),
      // gen_block_content(Some(oper), context, context_block),
    ),
  )
}

pub fn gen_block_content<'a>(
  block: Option<BlockIRNode<'a>>,
  context: &'a mut CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> oxc_allocator::Vec<'a, Statement<'a>> {
  unsafe {
    let context = context as *mut CodegenContext;
    let ast = (&*context).ast;
    let codegen_map = &mut (*context).codegen_map;
    let mut statements = ast.vec();
    // let mut reset_block = None;
    // let context_block = context_block as *mut BlockIRNode;
    // if let Some(block) = block {
    //   reset_block = Some(context.enter_block(block, unsafe { &mut *context_block }));
    // }

    // if let JSXChild::Fragment(node) = &mut (*context).root_node {
    //   for child in &node.children {
    //     if let Some(NodeTypes::VNodeCall(vnode_call)) = codegen_map.remove(&child.span()) {
    //       statements
    //         .push(ast.statement_return(SPAN, Some(gen_vnode_call(vnode_call, &mut *context))));
    //     }
    //   }
    // }

    // statements.push(ast.statement_return(SPAN, argument))

    // if let Some(reset_block) = reset_block {
    //   reset_block();
    // }
    statements
  }
}
