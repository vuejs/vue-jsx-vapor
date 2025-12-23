use napi::Either;
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  NONE,
  ast::{
    AssignmentOperator, AssignmentTarget, BinaryOperator, Expression, FormalParameterKind,
    JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXElement,
    ObjectPropertyKind, PropertyKey, PropertyKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::transform::{
  DirectiveTransformResult, TransformContext, transform_element::build_directive_args,
};
use common::{
  check::{is_jsx_component, is_simple_identifier},
  directive::{find_prop, get_modifier_prop_name, resolve_directive},
  error::ErrorCodes,
  text::get_tag_name,
};

pub fn transform_v_model<'a>(
  _dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
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
  let exp = if let JSXAttributeValue::ExpressionContainer(exp) = exp
    && let Some(exp) = exp.expression.as_expression_mut()
    && (exp.is_identifier_reference() || exp.is_member_expression())
  {
    exp
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VModelMalformedExpression, exp.span());
    return None;
  };

  if context
    .options
    .identifiers
    .borrow()
    .contains_key(exp.span().source_text(*context.source.borrow()))
  {
    context.options.on_error.as_ref()(ErrorCodes::VModelOnScopeVariable, exp.span());
    return None;
  }

  let dir = resolve_directive(_dir, *context.source.borrow());
  let tag = get_tag_name(&node.opening_element.name, *context.source.borrow());
  let is_custom_element = context.options.is_custom_element.as_ref()(tag.to_string());
  let is_component = is_jsx_component(node) && !is_custom_element;
  let mut cloned_dir = if !is_component {
    Some(dir.clone())
  } else {
    None
  };

  let arg_is_some = dir.arg.is_some();
  let mut computed = false;

  let prop_name = if let Some(arg) = &dir.arg {
    if arg.is_static {
      ast.property_key_static_identifier(arg.loc, ast.atom(&arg.content))
    } else {
      context.parse_dynamic_arg(&arg.content, arg.loc).into()
    }
  } else {
    ast.property_key_static_identifier(Span::new(dir.loc.start, dir.loc.start + 7), "modelValue")
  };

  // modelModifiers: { foo: true, "bar-baz": true }
  let modfiiers = if !dir.modifiers.is_empty() && is_component {
    let modifiers = dir.modifiers.into_iter().map(|m| {
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(
          SPAN,
          ast.atom(&if is_simple_identifier(&m.content) {
            m.content
          } else {
            format!("\"{}\"", m.content)
          }),
        ),
        ast.expression_boolean_literal(SPAN, true),
        false,
        false,
        false,
      )
    });
    let modifiers_key = if let Some(arg) = &dir.arg {
      if arg.is_static {
        ast.property_key_static_identifier(SPAN, ast.atom(&get_modifier_prop_name(&arg.content)))
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

  let event_name = if let Some(arg) = dir.arg {
    if arg.is_static {
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
      ast.vec1(ast.formal_parameter(
        SPAN,
        ast.vec(),
        ast.binding_pattern(
          ast.binding_pattern_kind_binding_identifier(SPAN, "$event"),
          NONE,
          false,
        ),
        None,
        false,
        false,
      )),
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
          match exp {
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
  if !*context.options.in_v_once.borrow() && !context.has_scope_ref(exp) {
    assignment_exp = context.cache(assignment_exp, false, false, false)
  }

  if !is_component {
    cloned_dir.as_mut().unwrap().exp.as_mut().unwrap().ast =
      Some(unsafe { &mut *(&mut exp.clone_in(context.allocator) as *mut Expression) });
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
      exp.take_in(context.allocator),
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
    });
  }

  if arg_is_some {
    context.options.on_error.as_ref()(ErrorCodes::VModelArgOnElement, dir.loc);
  }

  let mut runtime_name = None;
  if matches!(tag.as_str(), "input" | "textarea" | "select") || is_custom_element {
    let mut directive_to_use = "vModelText";
    let mut is_invalid_type = false;
    if tag == "input" || is_custom_element {
      if let Some(_type) = find_prop(node, Either::A("type".to_string())) {
        let value = &_type.value;
        if let Some(JSXAttributeValue::ExpressionContainer(_)) = value {
          // type={foo}
          directive_to_use = "vModelDynamic"
        } else if let Some(JSXAttributeValue::StringLiteral(value)) = value {
          match value.value.as_str() {
            "radio" => directive_to_use = "vModelRadio",
            "checkbox" => directive_to_use = "vModelCheckbox",
            "file" => {
              is_invalid_type = true;
              context.options.on_error.as_ref()(ErrorCodes::VModelOnFileInputElement, node.span);
            }
            // text type
            _ => check_duplicated_value(node, context),
          }
        }
      } else if has_dynamic_key_v_bind(node) {
        // element has bindings with dynamic keys, which can possibly contain "type".
        directive_to_use = "vModelDynamic";
      } else {
        // text type
        check_duplicated_value(node, context)
      }
    } else if tag == "select" {
      directive_to_use = "vModelSelect"
    } else {
      // textarea
      check_duplicated_value(node, context)
    }
    // inject runtime directive
    // by returning the helper symbol via needRuntime
    // the import will replaced a resolveDirective call.
    if !is_invalid_type {
      runtime_name = Some(context.helper(directive_to_use));
    }
  } else if !is_custom_element {
    context.options.on_error.as_ref()(ErrorCodes::VModelOnInvalidElement, node.span)
  }

  // native vmodel doesn't need the `modelValue` props since they are also
  // passed to the runtime as `binding.value`. removing it reduces code size.
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

  Some(DirectiveTransformResult {
    props,
    runtime: runtime_name
      .map(|runtime_name| build_directive_args(cloned_dir.unwrap(), context, &runtime_name)),
  })
}

fn check_duplicated_value(node: &JSXElement, context: &TransformContext) {
  let value = find_prop(node, Either::A("value".to_string()));
  if let Some(value) = value
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
