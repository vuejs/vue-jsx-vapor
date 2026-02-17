use common::directive::Modifiers;
use oxc_ast::NONE;
use oxc_ast::ast::{
  AssignmentTarget, Expression, FormalParameterKind, ObjectPropertyKind, PropertyKind, Statement,
  StringLiteral,
};
use oxc_span::{GetSpan, SPAN};

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::{SetDynamicEventsIRNode, SetEventIRNode};

pub fn gen_set_event<'a>(
  oper: SetEventIRNode<'a>,
  context: &'a CodegenContext<'a>,
  event_opers: &Vec<(i32, StringLiteral)>,
) -> Statement<'a> {
  let ast = &context.ast;
  let SetEventIRNode {
    element,
    key,
    value,
    modifiers: Modifiers {
      options,
      keys,
      non_keys,
    },
    delegate,
    effect,
    ..
  } = oper;

  let key_content = if let Expression::StringLiteral(key) = &key {
    key.value.as_str()
  } else {
    ""
  };
  let key_strat = key.span().start;
  let name = gen_expression(key, context, None, false);
  let event_options = if options.is_empty() && !effect {
    None
  } else {
    let mut properties = ast.vec();
    if effect {
      properties.push(ObjectPropertyKind::ObjectProperty(
        ast.alloc_object_property(
          SPAN,
          PropertyKind::Init,
          ast.property_key_static_identifier(SPAN, ast.atom("effect")),
          ast.expression_boolean_literal(SPAN, true),
          false,
          false,
          false,
        ),
      ))
    }
    properties.extend(options.into_iter().map(|option| {
      ObjectPropertyKind::ObjectProperty(ast.alloc_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, ast.atom(&option)),
        ast.expression_boolean_literal(SPAN, true),
        false,
        false,
        false,
      ))
    }));
    Some(ast.expression_object(SPAN, properties))
  };
  let handler = gen_event_handler(context, vec![value], &keys, &non_keys, false);

  if delegate {
    // key is static
    context
      .options
      .delegates
      .borrow_mut()
      .insert(key_content.to_string());
    // if this is the only delegated event of this name on this element,
    // we can generate optimized handler attachment code
    // e.g. n1.$evtclick = () => {}
    if !event_opers
      .iter()
      .any(|op| op.1.span.start != key_strat && op.0 == oper.element && op.1.value == key_content)
    {
      return ast.statement_expression(
        SPAN,
        ast.expression_assignment(
          SPAN,
          oxc_ast::ast::AssignmentOperator::Assign,
          AssignmentTarget::StaticMemberExpression(ast.alloc_static_member_expression(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&format!("_n{element}"))),
            ast.identifier_name(SPAN, ast.atom(&format!("$evt{key_content}"))),
            false,
          )),
          handler,
        ),
      );
    }
  }

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.atom(&format!("_n{element}")))
      .into(),
  );
  arguments.push(name.into());
  arguments.push(handler.into());
  if let Some(event_options) = event_options {
    arguments.push(event_options.into());
  }

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(
        SPAN,
        ast.atom(&context.helper(if delegate { "delegate" } else { "on" })),
      ),
      NONE,
      arguments,
      false,
    ),
  )
}

pub fn gen_event_handler<'a>(
  context: &'a CodegenContext<'a>,
  values: Vec<Expression<'a>>,
  keys: &[String],
  non_keys: &[String],
  // passed as component prop - need additional wrap
  extra_wrap: bool,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut values = values
    .into_iter()
    .map(|value| gen_expression(value, context, None, false));

  let mut handler_exp = if values.len() > 1 {
    ast.expression_array(SPAN, ast.vec_from_iter(values.map(|value| value.into())))
  } else {
    values.next().unwrap()
  };

  if !non_keys.is_empty() {
    handler_exp = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("withModifiers"))),
      NONE,
      ast.vec_from_array([
        handler_exp.into(),
        ast
          .expression_array(
            SPAN,
            ast.vec_from_iter(non_keys.iter().map(|key| {
              ast
                .expression_string_literal(SPAN, ast.atom(key), None)
                .into()
            })),
          )
          .into(),
      ]),
      false,
    )
  }

  if !keys.is_empty() {
    handler_exp = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("withKeys"))),
      NONE,
      ast.vec_from_array([
        handler_exp.into(),
        ast
          .expression_array(
            SPAN,
            ast.vec_from_iter(keys.iter().map(|key| {
              ast
                .expression_string_literal(SPAN, ast.atom(key), None)
                .into()
            })),
          )
          .into(),
      ]),
      false,
    )
  }

  if extra_wrap {
    handler_exp = ast.expression_arrow_function(
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
        ast.vec1(ast.statement_expression(SPAN, handler_exp)),
      ),
    )
  }
  handler_exp
}

pub fn gen_set_dynamic_events<'a>(
  oper: SetDynamicEventsIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("setDynamicEvents"))),
      NONE,
      ast.vec_from_array([
        ast
          .expression_identifier(SPAN, ast.atom(&format!("_n{}", oper.element)))
          .into(),
        gen_expression(oper.value, context, None, false).into(),
      ]),
      false,
    ),
  )
}
