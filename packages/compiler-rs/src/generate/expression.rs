use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  NONE,
  ast::{AssignmentOperator, AssignmentTarget, Expression, FormalParameterKind},
};
use oxc_span::{GetSpan, SPAN, Span};
use oxc_traverse::Ancestor;

use crate::{
  generate::CodegenContext, ir::index::SimpleExpressionNode, utils::walk::WalkIdentifiers,
};

pub fn gen_expression<'a>(
  node: SimpleExpressionNode<'a>,
  context: &'a CodegenContext<'a>,
  assignment: Option<Expression<'a>>,
  need_wrap: Option<bool>,
) -> Expression<'a> {
  let ast = &context.ast;

  let content = &node.content;
  let loc = node.loc;
  let need_wrap = need_wrap.unwrap_or(false);

  if node.is_static {
    return ast.expression_string_literal(loc, ast.atom(content), None);
  }

  if node.is_constant_expression() {
    return if let Some(assignment) = assignment {
      ast.expression_assignment(
        loc,
        AssignmentOperator::Assign,
        AssignmentTarget::AssignmentTargetIdentifier(
          ast.alloc_identifier_reference(loc, ast.atom(content)),
        ),
        assignment,
      )
    } else {
      ast.expression_identifier(loc, ast.atom(content))
    };
  }

  let Some(ast) = node.ast else {
    return gen_identifier(content, context, loc, assignment);
  };

  let span = ast.span();
  let mut expression = if let Expression::Identifier(ast) = ast {
    gen_identifier(&ast.name, context, span, None)
  } else {
    WalkIdentifiers::new(
      context,
      Box::new(|id, parent, _, _, _| {
        let span = id.span();
        let content = span.source_text(context.ir.source);
        if let Ancestor::ObjectPropertyKey(parent) = parent
          && !parent.computed()
        {
          return None;
        };
        Some(gen_identifier(content, context, span, None))
      }),
      false,
    )
    .traverse(ast.take_in(context.ast.allocator))
  };
  if let Some(assignment) = assignment {
    let span = expression.span();
    expression = context.ast.expression_assignment(
      span,
      AssignmentOperator::Assign,
      match expression {
        Expression::Identifier(id) => AssignmentTarget::AssignmentTargetIdentifier(id),
        Expression::StaticMemberExpression(id) => AssignmentTarget::StaticMemberExpression(id),
        Expression::ComputedMemberExpression(id) => AssignmentTarget::ComputedMemberExpression(id),
        Expression::PrivateFieldExpression(id) => AssignmentTarget::PrivateFieldExpression(id),
        _ => unimplemented!(),
      },
      assignment,
    );
  }

  if need_wrap {
    expression = context.ast.expression_arrow_function(
      SPAN,
      true,
      false,
      NONE,
      context.ast.alloc_formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        context.ast.vec(),
        NONE,
      ),
      NONE,
      context.ast.alloc_function_body(
        SPAN,
        context.ast.vec(),
        context.ast.vec1(
          context
            .ast
            .statement_expression(expression.span(), expression),
        ),
      ),
    );
  }
  expression
}

pub fn gen_identifier<'a>(
  name: &str,
  context: &CodegenContext<'a>,
  loc: Span,
  assignment: Option<Expression<'a>>,
) -> Expression<'a> {
  let ast = &context.ast;
  if let Some(id_map) = context.identifiers.borrow().get(name)
    && !id_map.is_empty()
    && let Some(replacement) = id_map.first()
  {
    return replacement.clone_in(ast.allocator);
  }

  if let Some(assignment) = assignment {
    ast.expression_assignment(
      loc,
      AssignmentOperator::Assign,
      AssignmentTarget::AssignmentTargetIdentifier(
        ast.alloc_identifier_reference(loc, ast.atom(name)),
      ),
      assignment,
    )
  } else {
    ast.expression_identifier(loc, ast.atom(name))
  }
}
