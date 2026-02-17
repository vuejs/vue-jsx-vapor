use oxc_ast::{
  AstBuilder,
  ast::{Expression, JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXElement},
};
use oxc_span::{SPAN, SourceType, Span};

use crate::{
  check::{is_event_option_modifier, is_keyboard_event, is_non_key_modifier, maybe_key_modifier},
  expression::{jsx_attribute_value_to_expression, parse_expression},
};

#[derive(Debug)]
pub struct DirectiveNode<'a> {
  pub exp: Option<Expression<'a>>,
  pub arg: Option<Expression<'a>>,
  pub modifiers: Vec<String>,
  pub span: Span,
}

pub fn resolve_directive<'a>(
  node: &'a mut JSXAttribute<'a>,
  ast: &AstBuilder<'a>,
) -> DirectiveNode<'a> {
  let mut arg_string = String::new();
  let (arg_span, name_string) = match &node.name {
    JSXAttributeName::Identifier(name) => (name.span, name.name.to_string()),
    JSXAttributeName::NamespacedName(name) => {
      arg_string = name.name.name.to_string();
      (
        Span::new(name.name.span.start + 1, name.name.span.end - 1),
        name.namespace.name.to_string(),
      )
    }
  };
  let mut modifiers: Vec<String> = vec![];
  let mut is_static = true;

  if !matches!(node.name, JSXAttributeName::NamespacedName(_)) {
    let name_string_splited: Vec<&str> = name_string.split("_").collect();
    if name_string_splited.len() > 1 {
      modifiers = name_string_splited[1..]
        .iter()
        .map(|s| s.to_string())
        .collect();
    }
  } else {
    let cloned = arg_string.clone();
    let splited = &mut cloned.split("$").collect::<Vec<_>>();
    if splited.len() > 2 {
      is_static = false;
      arg_string = splited[1].replace("_", ".");
      if !splited[2].is_empty() {
        modifiers = splited[2][1..]
          .split("_")
          .map(|s| s.to_string())
          .collect::<Vec<_>>();
      }
    } else {
      let mut splited = cloned.split("_").map(|i| i.to_string()).collect::<Vec<_>>();
      arg_string = splited.remove(0);
      modifiers = splited;
    }
  }

  let arg = if !arg_string.is_empty()
    && let JSXAttributeName::NamespacedName(_) = &node.name
  {
    if is_static {
      Some(ast.expression_string_literal(SPAN, ast.atom(&arg_string), None))
    } else {
      parse_expression(&arg_string, arg_span, ast.allocator, SourceType::jsx())
    }
  } else {
    None
  };

  DirectiveNode {
    arg,
    exp: node
      .value
      .as_mut()
      .map(|value| jsx_attribute_value_to_expression(value, ast)),
    modifiers,
    span: node.span,
  }
}

macro_rules! define_find_prop {
  ($fn_name:ident, $node_type: ty, $ret_type: ty, $iter: tt) => {
    pub fn $fn_name<'a>(node: $node_type, key: Vec<&str>) -> Option<$ret_type> {
      for attr in node.opening_element.attributes.$iter() {
        if let JSXAttributeItem::Attribute(attr) = attr {
          let name = match &attr.name {
            JSXAttributeName::Identifier(name) => name.name.to_string(),
            JSXAttributeName::NamespacedName(name) => name.namespace.name.to_string(),
          };
          let name = name.split('_').collect::<Vec<&str>>()[0];
          if !name.eq("") && key.contains(&name) {
            return Some(attr);
          }
        }
      }
      None
    }
  };
}
define_find_prop!(find_prop, &'a JSXElement<'a>, &'a JSXAttribute<'a>, iter);
define_find_prop!(
  find_prop_mut,
  &'a mut JSXElement<'a>,
  &'a mut JSXAttribute<'a>,
  iter_mut
);

pub fn get_modifier_prop_name(name: &str) -> String {
  format!(
    "{}Modifiers{}",
    if name == "modelValue" || name == "model-value" {
      "model"
    } else {
      name
    },
    if name == "model" { "$" } else { "" }
  )
}

#[derive(Clone, Debug)]
pub struct Modifiers {
  // modifiers for addEventListener() options, e.g. .passive & .capture
  pub options: Vec<String>,
  // modifiers that needs runtime guards, withKeys
  pub keys: Vec<String>,
  // modifiers that needs runtime guards, withModifiers
  pub non_keys: Vec<String>,
}

pub fn resolve_modifiers(key_string: &str, modifiers: Vec<&str>) -> Modifiers {
  let mut key_modifiers: Vec<String> = vec![];
  let mut non_key_modifiers: Vec<String> = vec![];
  let mut event_option_modifiers: Vec<String> = vec![];

  for modifier in modifiers {
    let modifier = modifier.to_string();
    if is_event_option_modifier(&modifier) {
      // eventOptionModifiers: modifiers for addEventListener() options,
      // e.g. .passive & .capture
      event_option_modifiers.push(modifier);
    } else {
      // runtimeModifiers: modifiers that needs runtime guards
      if maybe_key_modifier(&modifier) {
        if !key_string.is_empty() {
          if is_keyboard_event(key_string) {
            key_modifiers.push(modifier);
          } else {
            non_key_modifiers.push(modifier)
          }
        } else {
          key_modifiers.push(modifier.clone());
          non_key_modifiers.push(modifier)
        }
      } else if is_non_key_modifier(&modifier) {
        non_key_modifiers.push(modifier)
      } else {
        key_modifiers.push(modifier)
      }
    }
  }

  Modifiers {
    keys: key_modifiers,
    non_keys: non_key_modifiers,
    options: event_option_modifiers,
  }
}

#[derive(Default, Debug)]
pub struct Directives<'a> {
  pub v_if: Option<&'a mut JSXAttribute<'a>>,
  pub v_else_if: Option<&'a mut JSXAttribute<'a>>,
  pub v_else: Option<&'a mut JSXAttribute<'a>>,
  pub v_for: Option<&'a mut JSXAttribute<'a>>,
  pub v_once: Option<&'a mut JSXAttribute<'a>>,
  pub v_memo: Option<&'a mut JSXAttribute<'a>>,
  pub v_slots: Option<&'a mut JSXAttribute<'a>>,
  pub v_slot: Option<&'a mut JSXAttribute<'a>>,
  pub key: Option<&'a mut JSXAttribute<'a>>,
  pub _ref: Option<&'a mut JSXAttribute<'a>>,
  pub _type: Option<&'a mut JSXAttribute<'a>>,
  pub value: Option<&'a mut JSXAttribute<'a>>,
}

impl<'a> Directives<'a> {
  pub fn new(element: &'a mut JSXElement<'a>) -> Directives<'a> {
    let mut directives = Directives::default();
    for dir in element.opening_element.attributes.iter_mut() {
      if let JSXAttributeItem::Attribute(dir) = dir {
        let dir_name = match &dir.name {
          JSXAttributeName::Identifier(name) => name.name,
          JSXAttributeName::NamespacedName(name) => name.namespace.name,
        };
        match dir_name.as_str() {
          "v-if" => directives.v_if = Some(dir),
          "v-else-if" => directives.v_else_if = Some(dir),
          "v-else" => directives.v_else = Some(dir),
          "v-for" => directives.v_for = Some(dir),
          "v-once" => directives.v_once = Some(dir),
          "v-memo" => directives.v_memo = Some(dir),
          "v-slot" => directives.v_slot = Some(dir),
          "v-slots" => directives.v_slots = Some(dir),
          "key" => directives.key = Some(dir),
          "ref" => directives._ref = Some(dir),
          "type" => directives._type = Some(dir),
          "value" => directives.value = Some(dir),
          _ => (),
        }
      }
    }
    directives
  }
}
