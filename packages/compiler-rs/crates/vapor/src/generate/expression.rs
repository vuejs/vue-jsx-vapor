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
          ast.alloc_identifier_reference(span, ast.atom(&content)),
        ),
        assignment,
      )
    } else {
      ast.expression_identifier(span, ast.atom(&content))
    };
  }

  WalkIdentifiersMut::new(
    Box::new(|id, _| Some(gen_identifier(&id.name, context, id.span, None))),
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
    let mut replacement = replacement.clone_in(ast.allocator);
    *replacement.span_mut() = loc;
    return replacement;
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
