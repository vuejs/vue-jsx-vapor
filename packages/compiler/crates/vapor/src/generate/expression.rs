use oxc_allocator::CloneIn;
use oxc_ast::{
  NONE,
  ast::{AssignmentOperator, AssignmentTarget, Expression, FormalParameterKind},
};
use oxc_span::{GetSpan, GetSpanMut, SPAN, Span};

use crate::generate::CodegenContext;

use common::{expression::get_constant_expression_text, walk_mut::WalkIdentifiersMut};

pub fn gen_expression<'a>(
  mut expression: Expression<'a>,
  context: &'a CodegenContext<'a>,
  assignment: Option<Expression<'a>>,
  need_wrap: bool,
) -> Expression<'a> {
  let ast = context.ast;

  if let Expression::StringLiteral(_) = expression {
    return expression;
  }

  let span = expression.span();
  if let Some(content) = get_constant_expression_text(&expression, context.options) {
    return if let Some(assignment) = assignment {
      ast.expression_assignment(
        span,
        AssignmentOperator::Assign,
        AssignmentTarget::AssignmentTargetIdentifier(
          ast.alloc_identifier_reference(span, ast.str(&content)),
        ),
        assignment,
      )
    } else {
      ast.expression_identifier(span, ast.str(&content))
    };
  }

  WalkIdentifiersMut::new(
    Box::new(|id, _| Some(gen_identifier(id.name.as_str(), context, id.span, None))),
    context.options,
  )
  .visit(&mut expression);
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
  name: &'a str,
  context: &'a CodegenContext<'a>,
  loc: Span,
  assignment: Option<Expression<'a>>,
) -> Expression<'a> {
  let ast = &context.ast;
  let replacement = {
    let identifiers = context.identifiers.borrow();
    match identifiers.get(name).and_then(|ids| ids.last()) {
      // shadowed by a parameter of the same name - keep the name as is
      Some(None) => return plain_identifier(name, context, loc, assignment),
      Some(Some(expression)) => Some(expression.clone_in(ast.allocator)),
      None => None,
    }
  };
  if let Some(mut replacement) = replacement {
    *replacement.span_mut() = loc;
    // the replacement may itself reference a name needing resolution (an outer
    // loop alias used as a default). It is popped first so a self reference
    // falls back to the name itself instead of recursing.
    if let Some(popped) = context.pop_identifier(name) {
      let resolved = gen_expression(replacement, context, assignment, false);
      context.push_identifier(name, popped);
      return resolved;
    }
  }

  plain_identifier(name, context, loc, assignment)
}

fn plain_identifier<'a>(
  name: &str,
  context: &'a CodegenContext<'a>,
  loc: Span,
  assignment: Option<Expression<'a>>,
) -> Expression<'a> {
  let ast = &context.ast;
  if let Some(assignment) = assignment {
    ast.expression_assignment(
      loc,
      AssignmentOperator::Assign,
      AssignmentTarget::AssignmentTargetIdentifier(
        ast.alloc_identifier_reference(loc, ast.str(name)),
      ),
      assignment,
    )
  } else {
    ast.expression_identifier(loc, ast.str(name))
  }
}
