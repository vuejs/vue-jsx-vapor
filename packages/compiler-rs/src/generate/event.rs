use oxc_ast::NONE;
use oxc_ast::ast::{
  AssignmentTarget, Expression, FormalParameterKind, ObjectPropertyKind, PropertyKind, Statement,
};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::index::{Modifiers, SetDynamicEventsIRNode, SetEventIRNode, SimpleExpressionNode};

pub fn gen_set_event<'a>(
  oper: SetEventIRNode<'a>,
  context: &'a CodegenContext<'a>,
  event_opers: &Vec<SetEventIRNode>,
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

  let key_content = key.content.clone();
  let oper_key_strat = key.loc.start;
  let name = gen_expression(key, context, None, None);
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
  let handler = gen_event_handler(context, value, keys, non_keys, false);

  if delegate {
    // key is static
    context
      .options
      .delegates
      .borrow_mut()
      .insert(key_content.clone());
    // if this is the only delegated event of this name on this element,
    // we can generate optimized handler attachment code
    // e.g. n1.$evtclick = () => {}
    if !event_opers.iter().any(|op| {
      op.key.loc.start != oper_key_strat
        && op.delegate
        && op.element == oper.element
        && op.key.content == key_content
    }) {
      return ast.statement_expression(
        SPAN,
        ast.expression_assignment(
          SPAN,
          oxc_ast::ast::AssignmentOperator::Assign,
          AssignmentTarget::StaticMemberExpression(ast.alloc_static_member_expression(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&format!("n{element}"))),
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
      .expression_identifier(SPAN, ast.atom(&format!("n{element}")))
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
  value: Option<SimpleExpressionNode<'a>>,
  keys: Vec<String>,
  non_keys: Vec<String>,
  // passed as component prop - need additional wrap
  extra_wrap: bool,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut handler_exp = if let Some(value) = value
    && !value.content.trim().is_empty()
  {
    gen_expression(value, context, None, None)
  } else {
    ast.expression_arrow_function(
      SPAN,
      false,
      false,
      NONE,
      ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ast.vec(),
        NONE,
      ),
      NONE,
      ast.function_body(SPAN, ast.vec(), ast.vec()),
    )
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
            ast.vec_from_iter(non_keys.into_iter().map(|key| {
              ast
                .expression_string_literal(SPAN, ast.atom(&key), None)
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
            ast.vec_from_iter(keys.into_iter().map(|key| {
              ast
                .expression_string_literal(SPAN, ast.atom(&key), None)
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
          .expression_identifier(SPAN, ast.atom(&format!("n{}", oper.element)))
          .into(),
        gen_expression(oper.value, context, None, None).into(),
      ]),
      false,
    ),
  )
}
