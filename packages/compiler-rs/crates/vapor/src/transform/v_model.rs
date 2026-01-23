use napi::bindgen_prelude::Either17;
use oxc_ast::ast::{
  JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXElement,
};
use oxc_span::SPAN;

use crate::{
  ir::index::{BlockIRNode, DirectiveIRNode},
  transform::{DirectiveTransformResult, TransformContext},
};
use common::{
  check::is_jsx_component,
  directive::{Directives, resolve_directive},
  error::ErrorCodes,
  expression::SimpleExpressionNode,
  text::get_tag_name,
};

pub fn transform_v_model<'a>(
  directives: &Directives<'a>,
  _dir: &'a mut JSXAttribute<'a>,
  node: &JSXElement,
  context: &'a TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
) -> Option<DirectiveTransformResult<'a>> {
  let dir = resolve_directive(_dir, context.source_text);

  let Some(exp) = &dir.exp else {
    context.options.on_error.as_ref()(ErrorCodes::VModelNoExpression, dir.loc);
    return None;
  };

  if exp.content.trim().is_empty()
    || !exp
      .ast
      .as_ref()
      .map(|ast| ast.is_identifier_reference() || ast.is_member_expression())
      .unwrap_or_default()
  {
    context.options.on_error.as_ref()(ErrorCodes::VModelMalformedExpression, exp.loc);
    return None;
  }

  let is_component = is_jsx_component(node);
  if is_component {
    return Some(DirectiveTransformResult {
      key: if let Some(arg) = dir.arg {
        arg
      } else {
        SimpleExpressionNode {
          content: "modelValue".to_string(),
          is_static: true,
          loc: SPAN,
          ast: None,
        }
      },
      value: dir.exp.unwrap(),
      model: true,
      model_modifiers: Some(
        dir
          .modifiers
          .iter()
          .map(|m| m.content.to_string())
          .collect(),
      ),
      handler: false,
      handler_modifiers: None,
      modifier: None,
      runtime_camelize: false,
    });
  }

  if dir.arg.is_some() {
    context.options.on_error.as_ref()(ErrorCodes::VModelArgOnElement, dir.loc);
  }

  let tag = get_tag_name(&node.opening_element.name, context.source_text);
  let is_custom_element = context.options.is_custom_element.as_ref()(tag.to_string());
  let mut model_type = "text";
  // TODO let runtimeDirective: VaporHelper | undefined = 'vModelText'
  if matches!(tag.as_str(), "input" | "textarea" | "select") || is_custom_element {
    if tag == "input" || is_custom_element {
      if let Some(_type) = directives._type.as_ref() {
        let value = &_type.value;
        if let Some(JSXAttributeValue::ExpressionContainer(_)) = value {
          // type={foo}
          model_type = "dynamic"
        } else if let Some(JSXAttributeValue::StringLiteral(value)) = value {
          match value.value.as_str() {
            "radio" => model_type = "radio",
            "checkbox" => model_type = "checkbox",
            "file" => {
              model_type = "";
              context.options.on_error.as_ref()(ErrorCodes::VModelOnFileInputElement, node.span);
            }
            // text type
            _ => check_duplicated_value(directives, context),
          }
        }
      } else if has_dynamic_key_v_bind(node) {
        // element has bindings with dynamic keys, which can possibly contain "type".
        model_type = "dynamic";
      } else {
        // text type
        check_duplicated_value(directives, context)
      }
    } else if tag == "select" {
      model_type = "select"
    } else {
      // textarea
      check_duplicated_value(directives, context)
    }
  } else if !is_custom_element {
    context.options.on_error.as_ref()(ErrorCodes::VModelOnInvalidElement, node.span)
  }

  if !model_type.is_empty() {
    let element = context.reference(&mut context_block.dynamic);
    context.register_operation(
      context_block,
      Either17::M(DirectiveIRNode {
        directive: true,
        element,
        dir,
        name: "model".to_string(),
        model_type: Some(model_type.to_string()),
        builtin: true,
        asset: false,
        deferred: false,
      }),
      None,
    )
  }

  None
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
