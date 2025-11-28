use std::mem;

use napi::bindgen_prelude::Either3;
use oxc_allocator::CloneIn;
use oxc_ast::NONE;
use oxc_ast::ast::BinaryOperator;
use oxc_ast::ast::BindingPatternKind;
use oxc_ast::ast::Expression;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::ObjectPropertyKind;
use oxc_ast::ast::PropertyKind;
use oxc_ast::ast::Statement;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::directive::gen_directive_modifiers;
use crate::generate::directive::gen_directives_for_element;
use crate::generate::event::gen_event_handler;
use crate::generate::expression::gen_expression;
use crate::generate::prop::gen_prop_key;
use crate::generate::prop::gen_prop_value;
use crate::generate::slot::gen_raw_slots;
use crate::generate::v_model::gen_model_handler;
use crate::ir::component::IRProp;
use crate::ir::component::IRProps;
use crate::ir::component::IRPropsStatic;
use crate::ir::index::BlockIRNode;
use crate::ir::index::CreateComponentIRNode;
use crate::ir::index::Modifiers;
use crate::ir::index::SimpleExpressionNode;
use crate::utils::text::camelize;
use crate::utils::text::to_valid_asset_id;

pub fn gen_create_component<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  operation: CreateComponentIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
  let ast = &context.ast;
  let CreateComponentIRNode {
    tag,
    root,
    props,
    slots,
    once,
    id,
    dynamic,
    asset,
    ..
  } = operation;

  let is_dynamic = if let Some(dynamic) = &dynamic
    && !dynamic.is_static
  {
    true
  } else {
    false
  };
  let tag = if let Some(dynamic) = dynamic {
    if dynamic.is_static {
      ast
        .expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("resolveDynamicComponent"))),
          NONE,
          ast.vec1(gen_expression(dynamic, context, None, None).into()),
          false,
        )
        .into()
    } else {
      ast
        .expression_arrow_function(
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
            ast.vec1(ast.statement_expression(SPAN, gen_expression(dynamic, context, None, None))),
          ),
        )
        .into()
    }
  } else if asset {
    ast
      .expression_identifier(SPAN, ast.atom(&to_valid_asset_id(&tag, "component")))
      .into()
  } else {
    gen_expression(
      SimpleExpressionNode {
        content: tag,
        is_static: false,
        loc: SPAN,
        ast: None,
      },
      context,
      None,
      None,
    )
    .into()
  };

  let raw_props = gen_raw_props(props, context);
  let _context_block = context_block as *mut BlockIRNode;
  let raw_slots = gen_raw_slots(slots, context, unsafe { &mut *_context_block });

  let mut arguments = ast.vec1(tag);
  if let Some(raw_props) = raw_props {
    arguments.push(raw_props.into());
  } else if root || once || raw_slots.is_some() {
    arguments.push(ast.expression_null_literal(SPAN).into());
  }
  if let Some(raw_slots) = raw_slots {
    arguments.push(raw_slots.into());
  } else if root || once {
    arguments.push(ast.expression_null_literal(SPAN).into());
  }
  if root {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  } else if once {
    arguments.push(ast.expression_null_literal(SPAN).into())
  }
  if once {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }
  statements.push(Statement::VariableDeclaration(
    ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        ast.binding_pattern(
          BindingPatternKind::BindingIdentifier(
            ast.alloc_binding_identifier(SPAN, ast.atom(&format!("n{id}"))),
          ),
          NONE,
          false,
        ),
        Some(ast.expression_call(
          SPAN,
          ast.expression_identifier(
            SPAN,
            ast.atom(&context.helper(if is_dynamic {
              "createDynamicComponent"
            } else if asset {
              "createComponentWithFallback"
            } else {
              "createComponent"
            })),
          ),
          NONE,
          arguments,
          false,
        )),
        false,
      )),
      false,
    ),
  ));
  if let Some(directive_statement) = gen_directives_for_element(id, context, context_block) {
    statements.push(directive_statement);
  }
}

fn gen_raw_props<'a>(
  mut props: Vec<IRProps<'a>>,
  context: &'a CodegenContext<'a>,
) -> Option<Expression<'a>> {
  let props_len = props.len();
  if let Either3::A(static_props) = &props[0] {
    if static_props.is_empty() && props_len == 1 {
      return None;
    }
    let static_props = props.remove(0);
    if let Either3::A(static_props) = static_props {
      Some(gen_static_props(
        static_props,
        context,
        gen_dynamic_props(props, context),
      ))
    } else {
      None
    }
  } else if props_len > 0 {
    // all dynamic
    Some(gen_static_props(
      vec![],
      context,
      gen_dynamic_props(props, context),
    ))
  } else {
    None
  }
}

fn gen_static_props<'a>(
  props: IRPropsStatic<'a>,
  context: &'a CodegenContext<'a>,
  dynamic_props: Option<Expression<'a>>,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut properties = ast.vec();
  let _properties = &mut properties as *mut oxc_allocator::Vec<ObjectPropertyKind>;
  for prop in props {
    gen_prop(unsafe { &mut *_properties }, prop, context, true)
  }
  if let Some(dynamic_props) = dynamic_props {
    properties.push(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, ast.atom("$")),
      dynamic_props,
      false,
      false,
      false,
    ));
  }
  ast.expression_object(SPAN, properties)
}

fn gen_dynamic_props<'a>(
  props: Vec<IRProps<'a>>,
  context: &'a CodegenContext<'a>,
) -> Option<Expression<'a>> {
  let ast = &context.ast;
  let mut frags = ast.vec();
  for p in props {
    let mut expr = None;
    if let Either3::A(p) = p {
      if !p.is_empty() {
        frags.push(gen_static_props(p, context, None))
      }
      continue;
    } else if let Either3::B(p) = p {
      let mut properties = ast.vec();
      gen_prop(&mut properties, p, context, false);
      expr = Some(ast.expression_object(SPAN, properties));
    } else if let Either3::C(p) = p {
      let expression = gen_expression(p.value, context, None, None);
      expr = if p.handler.unwrap_or_default() {
        Some(ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("toHandlers"))),
          NONE,
          ast.vec1(expression.into()),
          false,
        ))
      } else {
        Some(expression)
      }
    }
    frags.push(ast.expression_arrow_function(
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
        if let Some(expr) = expr {
          ast.vec1(ast.statement_expression(SPAN, expr))
        } else {
          ast.vec()
        },
      ),
    ));
  }
  if !frags.is_empty() {
    return Some(
      ast.expression_array(SPAN, ast.vec_from_iter(frags.into_iter().map(|i| i.into()))),
    );
  }
  None
}

fn gen_prop<'a>(
  properties: &mut oxc_allocator::Vec<'a, ObjectPropertyKind<'a>>,
  mut prop: IRProp<'a>,
  context: &'a CodegenContext<'a>,
  is_static: bool,
) {
  let ast = &context.ast;
  let model = prop.model.unwrap_or_default();
  let handler = prop.handler.unwrap_or_default();
  let Modifiers {
    keys,
    non_keys,
    options,
  } = prop.handler_modifiers.unwrap_or(Modifiers {
    keys: vec![],
    non_keys: vec![],
    options: vec![],
  });
  let mut values = mem::take(&mut prop.values);

  let model_modifiers = prop.model_modifiers.take();
  let model = if model {
    Some(gen_model(
      prop.key.clone(),
      values[0].clone(),
      model_modifiers,
      context,
    ))
  } else {
    None
  };

  let value = if handler {
    gen_event_handler(
      context,
      Some(values.remove(0)),
      keys,
      non_keys,
      true, /* wrap handlers passed to components */
    )
  } else {
    let values = gen_prop_value(values, context);
    if is_static {
      ast.expression_arrow_function(
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
          ast.vec1(ast.statement_expression(SPAN, values)),
        ),
      )
    } else {
      values
    }
  };

  let key = gen_prop_key(
    prop.key,
    prop.runtime_camelize,
    prop.modifier,
    handler,
    options,
    context,
  );
  let computed = key.is_expression();
  properties.push(ast.object_property_kind_object_property(
    SPAN,
    PropertyKind::Init,
    key,
    value,
    false,
    false,
    computed,
  ));

  if let Some(model) = model {
    properties.extend(model);
  }
}

fn gen_model<'a>(
  key: SimpleExpressionNode<'a>,
  value: SimpleExpressionNode<'a>,
  model_modifiers: Option<Vec<String>>,
  context: &'a CodegenContext<'a>,
) -> oxc_allocator::Vec<'a, ObjectPropertyKind<'a>> {
  let ast = &context.ast;
  let mut properties = ast.vec();
  let is_static = key.is_static;
  let content = key.content.clone();
  let expression = gen_expression(key, context, None, None);

  let modifiers = if let Some(model_modifiers) = model_modifiers
    && !model_modifiers.is_empty()
  {
    let modifers_key = if is_static {
      ast
        .property_key_static_identifier(SPAN, ast.atom(&format!("{}Modifiers", camelize(&content))))
    } else {
      ast
        .expression_binary(
          SPAN,
          expression.clone_in(context.ast.allocator),
          BinaryOperator::Addition,
          ast.expression_string_literal(SPAN, ast.atom("Modifiers"), None),
        )
        .into()
    };
    let modifiers_val = Expression::ObjectExpression(gen_directive_modifiers(model_modifiers, ast));

    Some(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      modifers_key,
      ast.expression_arrow_function(
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
          ast.vec1(ast.statement_expression(SPAN, modifiers_val)),
        ),
      ),
      false,
      false,
      !is_static,
    ))
  } else {
    None
  };

  let name = if is_static {
    ast.property_key_static_identifier(
      SPAN,
      ast.atom(&format!("\"onUpdate:{}\"", camelize(&content))),
    )
  } else {
    ast
      .expression_binary(
        SPAN,
        ast.expression_string_literal(SPAN, ast.atom("onUpdate:"), None),
        BinaryOperator::Addition,
        expression,
      )
      .into()
  };

  let handler = gen_model_handler(value, context);
  properties.push(ast.object_property_kind_object_property(
    SPAN,
    PropertyKind::Init,
    name,
    ast.expression_arrow_function(
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
        ast.vec1(ast.statement_expression(SPAN, handler)),
      ),
    ),
    false,
    false,
    !is_static,
  ));

  if let Some(modifiers) = modifiers {
    properties.push(modifiers)
  }
  properties
}
