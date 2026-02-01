use std::mem;

use common::check::is_simple_identifier;
use common::directive::Modifiers;
use common::expression::SimpleExpressionNode;
use common::text::capitalize;
use indexmap::IndexMap;
use napi::bindgen_prelude::Either3;
use oxc_allocator::CloneIn;
use oxc_ast::NONE;
use oxc_ast::ast::BinaryOperator;
use oxc_ast::ast::Expression;
use oxc_ast::ast::FormalParameterKind;
use oxc_ast::ast::ObjectPropertyKind;
use oxc_ast::ast::PropertyKey;
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
use common::text::to_valid_asset_id;

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
    is_custom_element,
    ..
  } = operation;

  let is_dynamic = if let Some(dynamic) = &dynamic
    && !dynamic.is_static
  {
    true
  } else {
    false
  };
  let tag = if is_custom_element {
    ast
      .expression_string_literal(SPAN, ast.atom(&tag), None)
      .into()
  } else if let Some(dynamic) = dynamic {
    if dynamic.is_static {
      ast
        .expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("resolveDynamicComponent"))),
          NONE,
          ast.vec1(gen_expression(dynamic, context, None, false).into()),
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
            ast.vec1(ast.statement_expression(SPAN, gen_expression(dynamic, context, None, false))),
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
      false,
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
        ast.binding_pattern_binding_identifier(SPAN, ast.atom(&format!("_n{id}"))),
        NONE,
        Some(ast.expression_call(
          SPAN,
          ast.expression_identifier(
            SPAN,
            ast.atom(&context.helper(if is_dynamic {
              "createDynamicComponent"
            } else if is_custom_element {
              "createPlainElement"
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

pub fn gen_raw_props<'a>(
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

struct HandlerGroup<'a> {
  pub key_frag: PropertyKey<'a>,
  pub handlers: Vec<Expression<'a>>,
  index: usize,
}
fn add_handler<'a>(
  handler_groups: &mut IndexMap<String, HandlerGroup<'a>>,
  properties: &mut oxc_allocator::Vec<ObjectPropertyKind>,
  key_name: String,
  key_frag: PropertyKey<'a>,
  handler_exp: Expression<'a>,
) {
  if handler_groups.get_mut(&key_name).is_none() {
    let index = properties.len();
    handler_groups.insert(
      key_name.clone(),
      HandlerGroup {
        key_frag,
        index,
        handlers: vec![],
      },
    );
  }
  handler_groups
    .get_mut(&key_name)
    .unwrap()
    .handlers
    .push(handler_exp);
}

fn gen_static_props<'a>(
  props: IRPropsStatic<'a>,
  context: &'a CodegenContext<'a>,
  dynamic_props: Option<Expression<'a>>,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut properties = ast.vec();
  let mut handler_groups = IndexMap::new();

  for mut prop in props {
    if prop.handler {
      let key_name = format!("\"on{}\"", capitalize(prop.key.content.clone()));
      if key_name.is_empty() {
        // dynamic key handlers are emitted as-is
        gen_prop(&mut properties, prop, context, true);
        continue;
      }

      let Modifiers {
        keys,
        non_keys,
        options,
      } = prop.handler_modifiers.unwrap_or(Modifiers {
        keys: vec![],
        non_keys: vec![],
        options: vec![],
      });

      let key_frag = gen_prop_key(
        prop.key,
        prop.runtime_camelize,
        prop.modifier,
        prop.handler,
        options,
        context,
      );
      let has_modifiers = !keys.is_empty() || !non_keys.is_empty();
      if has_modifiers || prop.values.len() <= 1 {
        let handler_exp = gen_event_handler(
          context,
          prop.values.into_iter().map(Some).collect(),
          &keys,
          &non_keys,
          false,
        );
        add_handler(
          &mut handler_groups,
          &mut properties,
          key_name,
          key_frag,
          handler_exp,
        );
      } else {
        // no modifiers: flatten multiple handler values
        for value in prop.values.drain(..) {
          let handler_exp = gen_event_handler(context, vec![Some(value)], &keys, &non_keys, false);
          add_handler(
            &mut handler_groups,
            &mut properties,
            key_name.clone(),
            key_frag.clone_in(ast.allocator),
            handler_exp,
          );
        }
      }
      continue;
    }

    // v-model on component: synthesize onUpdate:* and modifiers props, and
    // dedupe/merge with user provided @update:* handlers.
    if prop.model {
      let mut prop_clone = prop.clone();
      prop_clone.model = false;
      // normal (non-handler) props
      gen_prop(&mut properties, prop_clone, context, true);
      gen_model(
        Some(&mut handler_groups),
        &mut properties,
        prop.key,
        prop.values.remove(0),
        prop.model_modifiers,
        context,
      );
    } else {
      gen_prop(&mut properties, prop, context, true);
    }
  }

  // fill handler placeholders
  for mut group in handler_groups.into_values() {
    let handler_value = if group.handlers.len() > 1 {
      ast.expression_array(
        SPAN,
        ast.vec_from_iter(group.handlers.into_iter().map(|e| e.into())),
      )
    } else {
      group.handlers.remove(0)
    };
    properties.insert(
      group.index,
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        group.key_frag,
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
            ast.vec1(ast.statement_expression(SPAN, handler_value)),
          ),
        ),
        false,
        false,
        false,
      ),
    );
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
      let expression = gen_expression(p.value, context, None, false);
      expr = if p.handler {
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
  let model = prop.model;
  let handler = prop.handler;
  let Modifiers {
    keys,
    non_keys,
    options,
  } = prop.handler_modifiers.unwrap_or(Modifiers {
    keys: vec![],
    non_keys: vec![],
    options: vec![],
  });
  let values = mem::take(&mut prop.values);

  let model_modifiers = prop.model_modifiers.take();
  let model = if model {
    let mut properties = ast.vec();
    gen_model(
      None,
      &mut properties,
      prop.key.clone(),
      values[0].clone(),
      model_modifiers,
      context,
    );
    Some(properties)
  } else {
    None
  };

  let value = if handler {
    gen_event_handler(
      context,
      values.into_iter().map(Some).collect(),
      &keys,
      &non_keys,
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
  handler_groups: Option<&mut IndexMap<String, HandlerGroup<'a>>>,
  properties: &mut oxc_allocator::Vec<ObjectPropertyKind<'a>>,
  key: SimpleExpressionNode<'a>,
  value: SimpleExpressionNode<'a>,
  model_modifiers: Option<Vec<String>>,
  context: &'a CodegenContext<'a>,
) {
  let ast = context.ast;

  // modelModifiers prop
  let is_static = key.is_static;
  let content = key.content.clone();
  let modifiers = if let Some(model_modifiers) = model_modifiers
    && !model_modifiers.is_empty()
  {
    let modifers_key = if key.is_static {
      ast.property_key_static_identifier(
        SPAN,
        ast.atom(&if is_simple_identifier(&content) {
          format!("{}Modifiers", &content)
        } else {
          format!("\"{}Modifiers\"", &content)
        }),
      )
    } else {
      ast
        .expression_binary(
          SPAN,
          gen_expression(key.clone(), context, None, false).into(),
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

  // onUpdate:* handler
  let handler_value = gen_model_handler(value, context);
  if is_static {
    let key_name = format!("\"onUpdate:{}\"", key.content);
    let key_frag = ast.property_key_static_identifier(key.loc, ast.atom(&key_name));
    if let Some(handler_groups) = handler_groups {
      add_handler(
        handler_groups,
        properties,
        key_name,
        key_frag,
        handler_value,
      );
    } else {
      properties.push(ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        key_frag,
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
            ast.vec1(ast.statement_expression(SPAN, handler_value)),
          ),
        ),
        false,
        false,
        false,
      ));
    }
  } else {
    properties.push(
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast
          .expression_binary(
            SPAN,
            ast.expression_string_literal(SPAN, ast.atom("onUpdate:"), None),
            BinaryOperator::Addition,
            gen_expression(key.clone(), context, None, false),
          )
          .into(),
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
            ast.vec1(ast.statement_expression(SPAN, handler_value)),
          ),
        ),
        false,
        false,
        !is_static,
      ),
    );
  };

  if let Some(modifiers) = modifiers {
    properties.push(modifiers)
  }
}
