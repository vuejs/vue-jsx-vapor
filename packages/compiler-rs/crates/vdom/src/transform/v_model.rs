use oxc_allocator::CloneIn;
use oxc_ast::{
  NONE,
  ast::{
    AssignmentOperator, AssignmentTarget, BinaryOperator, Expression, FormalParameterKind,
    JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXElement, JSXExpression,
    ObjectPropertyKind, PropertyKey, PropertyKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::transform::{
  DirectiveTransformResult, TransformContext, transform_element::build_directive_args,
};
use common::{
  check::is_simple_identifier,
  directive::{Directives, get_modifier_prop_name, resolve_directive},
  error::ErrorCodes,
};

pub fn transform_v_model<'a>(
  directives: &Directives<'a>,
  _dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement<'a>,
  context: &'a TransformContext<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let ast = &context.ast;
  let dir_ref = _dir as *mut JSXAttribute;
  let Some(exp) = &mut unsafe { &mut *dir_ref }.value else {
    context.options.on_error.as_ref()(ErrorCodes::VModelNoExpression, _dir.span);
    return None;
  };

  // we assume v-model directives are always parsed
  // (not artificially created by a transform)
  let (exp, has_scope_ref) = if let JSXAttributeValue::ExpressionContainer(exp) = exp
    && !matches!(exp.expression, JSXExpression::EmptyExpression(_))
    && let (exp, has_scope_ref, ..) = context.process_expression(
      exp
        .expression
        .clone_in(context.allocator)
        .to_expression_mut(),
    )
    && (exp.is_identifier_reference() || exp.is_member_expression())
  {
    (exp, has_scope_ref)
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VModelMalformedExpression, exp.span());
    return None;
  };

  if context
    .options
    .identifiers
    .borrow()
    .contains_key(exp.span().source_text(context.source_text))
  {
    context.options.on_error.as_ref()(ErrorCodes::VModelOnScopeVariable, exp.span());
    return None;
  }

  let dir = resolve_directive(_dir, context.ast);
  let tag = node
    .opening_element
    .name
    .get_identifier_name()
    .map(|name| name.as_str())
    .unwrap_or_default();
  let is_custom_element = directives.is_custom_element;
  let is_component = is_custom_element || directives.is_component;

  let arg_is_some = dir.arg.is_some();
  let mut computed = false;

  let prop_name = if let Some(arg) = dir.arg.as_ref() {
    arg.clone_in(context.allocator).into()
  } else {
    ast.property_key_static_identifier(Span::new(dir.span.start, dir.span.start + 7), "modelValue")
  };

  // modelModifiers: { foo: true, "bar-baz": true }
  let modfiiers = if !dir.modifiers.is_empty() && is_component {
    let modifiers = dir.modifiers.iter().map(|modifier| {
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(
          SPAN,
          ast.atom(&if is_simple_identifier(modifier) {
            modifier.clone()
          } else {
            format!("\"{}\"", modifier)
          }),
        ),
        ast.expression_boolean_literal(SPAN, true),
        false,
        false,
        false,
      )
    });
    let modifiers_key = if let Some(arg) = dir.arg.as_ref() {
      if let Expression::StringLiteral(arg) = arg {
        ast.property_key_static_identifier(SPAN, ast.atom(&get_modifier_prop_name(&arg.value)))
      } else {
        computed = true;
        ast
          .expression_binary(
            SPAN,
            prop_name
              .as_expression()
              .unwrap()
              .clone_in(context.allocator),
            BinaryOperator::Addition,
            ast.expression_string_literal(SPAN, "modifiers", None),
          )
          .into()
      }
    } else {
      ast.property_key_static_identifier(SPAN, "modelModifiers")
    };
    Some(ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      modifiers_key,
      ast.expression_object(SPAN, ast.vec_from_iter(modifiers)),
      false,
      false,
      computed,
    ))
  } else {
    None
  };

  let event_name = if let Some(arg) = dir.arg.as_ref() {
    if let Expression::StringLiteral(_) = arg {
      ast.property_key_static_identifier(
        SPAN,
        ast.atom(&format!("\"onUpdate:{}\"", prop_name.name().unwrap())),
      )
    } else {
      computed = true;
      ast
        .expression_binary(
          SPAN,
          ast.expression_string_literal(SPAN, "onUpdate:", None),
          BinaryOperator::Addition,
          prop_name
            .as_expression()
            .unwrap()
            .clone_in(context.allocator),
        )
        .into()
    }
  } else {
    ast.property_key_static_identifier(SPAN, "\"onUpdate:modelValue\"")
  };

  let mut assignment_exp = ast.expression_arrow_function(
    SPAN,
    true,
    false,
    NONE,
    ast.formal_parameters(
      SPAN,
      FormalParameterKind::ArrowFormalParameters,
      ast.vec1(
        ast.plain_formal_parameter(SPAN, ast.binding_pattern_binding_identifier(SPAN, "$event")),
      ),
      NONE,
    ),
    NONE,
    ast.function_body(
      SPAN,
      ast.vec(),
      ast.vec1(ast.statement_expression(
        SPAN,
        ast.expression_assignment(
          SPAN,
          AssignmentOperator::Assign,
          match &exp {
            Expression::Identifier(exp) => AssignmentTarget::AssignmentTargetIdentifier(
              ast.alloc_identifier_reference(exp.span, exp.name),
            ),
            Expression::StaticMemberExpression(exp) => {
              AssignmentTarget::StaticMemberExpression(exp.clone_in(context.allocator))
            }
            Expression::ComputedMemberExpression(exp) => {
              AssignmentTarget::ComputedMemberExpression(exp.clone_in(context.allocator))
            }
            _ => unimplemented!(),
          },
          ast.expression_identifier(SPAN, "$event"),
        ),
      )),
    ),
  );

  // cache v-model handler if applicable (when it's not a computed member expression)
  if !*context.options.in_v_once.borrow() && !has_scope_ref {
    assignment_exp = context.cache(assignment_exp, false, false, false)
  }

  let mut props = vec![
    ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      if let PropertyKey::StaticIdentifier(name) = &prop_name
        && !is_simple_identifier(&name.name)
      {
        ast.property_key_static_identifier(name.span, ast.atom(&format!("\"{}\"", name.name)))
      } else {
        prop_name
      },
      exp,
      false,
      false,
      computed,
    ),
    ast.object_property_kind_object_property(
      SPAN,
      PropertyKind::Init,
      event_name,
      assignment_exp,
      false,
      false,
      computed,
    ),
  ];

  if let Some(modfiiers) = modfiiers {
    props.push(modfiiers)
  }

  if is_component {
    return Some(DirectiveTransformResult {
      props,
      runtime: None,
      has_jsx: false,
    });
  }

  if arg_is_some {
    context.options.on_error.as_ref()(ErrorCodes::VModelArgOnElement, dir.span);
  }

  let mut runtime_name = None;
  if matches!(tag, "input" | "textarea" | "select") || is_custom_element {
    let mut directive_to_use = "_vModelText";
    let mut is_invalid_type = false;
    if tag == "input" || is_custom_element {
      if let Some(_type) = directives._type.as_ref() {
        let value = &_type.value;
        if let Some(JSXAttributeValue::ExpressionContainer(_)) = value {
          // type={foo}
          directive_to_use = "_vModelDynamic"
        } else if let Some(JSXAttributeValue::StringLiteral(value)) = value {
          match value.value.as_str() {
            "radio" => directive_to_use = "_vModelRadio",
            "checkbox" => directive_to_use = "_vModelCheckbox",
            "file" => {
              is_invalid_type = true;
              context.options.on_error.as_ref()(ErrorCodes::VModelOnFileInputElement, node.span);
            }
            // text type
            _ => check_duplicated_value(directives, context),
          }
        }
      } else if has_dynamic_key_v_bind(node) {
        // element has bindings with dynamic keys, which can possibly contain "type".
        directive_to_use = "_vModelDynamic";
      } else {
        // text type
        check_duplicated_value(directives, context)
      }
    } else if tag == "select" {
      directive_to_use = "_vModelSelect"
    } else {
      // textarea
      check_duplicated_value(directives, context)
    }
    // inject runtime directive
    // by returning the helper symbol via needRuntime
    // the import will replaced a resolveDirective call.
    if !is_invalid_type {
      runtime_name = Some(context.options.helper(directive_to_use));
    }
  } else if !is_custom_element {
    context.options.on_error.as_ref()(ErrorCodes::VModelOnInvalidElement, node.span)
  }

  // native vmodel doesn't need the `modelValue` props since they are also
  // passed to the runtime as `binding.value`. removing it reduces code size.
  if !is_custom_element {
    props = props
      .into_iter()
      .filter(|p| {
        if let ObjectPropertyKind::ObjectProperty(p) = p
          && let PropertyKey::StaticIdentifier(key) = &p.key
          && key.name == "modelValue"
        {
          false
        } else {
          true
        }
      })
      .collect::<Vec<_>>();
  }

  Some(DirectiveTransformResult {
    props,
    runtime: runtime_name.map(|runtime_name| build_directive_args(dir, context, runtime_name)),
    has_jsx: false,
  })
}

fn check_duplicated_value(directives: &Directives, context: &TransformContext) {
  if let Some(value) = directives.value.as_ref()
    && !matches!(value.value, Some(JSXAttributeValue::StringLiteral(_)))
  {
    context.options.on_error.as_ref()(ErrorCodes::VModelUnnecessaryValue, value.span);
  }
}

fn has_dynamic_key_v_bind(node: &JSXElement) -> bool {
  node.opening_element.attributes.iter().any(|p| match p {
    JSXAttributeItem::Attribute(p) => match &p.name {
      JSXAttributeName::NamespacedName(name) => !name.namespace.name.starts_with("v-"),
      _ => false,
    },
    JSXAttributeItem::SpreadAttribute(_) => true,
  })
}
