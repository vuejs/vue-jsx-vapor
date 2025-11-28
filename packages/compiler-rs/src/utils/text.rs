use oxc_ast::ast::{Expression, JSXChild, JSXElementName, JSXExpression, JSXText};

use crate::transform::TransformContext;

fn is_all_empty_text(s: &str) -> bool {
  let mut has_newline = false;
  for c in s.chars() {
    if !c.is_whitespace() {
      return false;
    }
    if c == '\n' || c == '\r' {
      has_newline = true;
    }
  }
  has_newline
}

fn start_with_newline_and_spaces(s: &str) -> bool {
  let chars = s.chars();

  for c in chars {
    if c.is_whitespace() && c != '\n' && c != '\r' {
      continue;
    } else {
      return c == '\n' || c == '\r';
    }
  }
  false
}

fn ends_with_newline_and_spaces(s: &str) -> bool {
  let chars = s.chars().rev();

  for c in chars {
    if c.is_whitespace() && c != '\n' && c != '\r' {
      continue;
    } else {
      return c == '\n' || c == '\r';
    }
  }
  false
}

pub fn resolve_jsx_text(node: &JSXText) -> String {
  if is_all_empty_text(&node.raw.unwrap()) {
    return String::new();
  }
  let mut value = node.value.to_string();
  if start_with_newline_and_spaces(&value) {
    value = value.trim_start().to_owned();
  }
  if ends_with_newline_and_spaces(&value) {
    value = value.trim_end().to_owned();
  }
  value
}

pub fn is_empty_text(node: &JSXChild) -> bool {
  match node {
    JSXChild::Text(node) => is_all_empty_text(&node.raw.unwrap()),
    JSXChild::ExpressionContainer(node) => {
      matches!(node.expression, JSXExpression::EmptyExpression(_))
    }
    _ => false,
  }
}

pub fn get_tag_name(name: &JSXElementName, context: &TransformContext) -> String {
  match name {
    JSXElementName::Identifier(name) => name.name.to_string(),
    JSXElementName::IdentifierReference(name) => name.name.to_string(),
    JSXElementName::NamespacedName(name) => {
      context.ir.borrow().source[name.span.start as usize..name.span.end as usize].to_string()
    }
    JSXElementName::MemberExpression(name) => {
      context.ir.borrow().source[name.span.start as usize..name.span.end as usize].to_string()
    }
    JSXElementName::ThisExpression(name) => {
      context.ir.borrow().source[name.span.start as usize..name.span.end as usize].to_string()
    }
  }
}

pub fn camelize(str: &str) -> String {
  str
    .split('-')
    .enumerate()
    .map(|(idx, word)| {
      if idx == 0 {
        word.to_string()
      } else {
        let mut chars = word.chars();
        match chars.next() {
          Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
          None => String::new(),
        }
      }
    })
    .collect()
}

pub fn to_valid_asset_id(name: &str, _type: &str) -> String {
  let name = name
    .chars()
    .map(|c| {
      if c == '-' {
        "_".to_string()
      } else if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
        c.to_string()
      } else {
        (c as u32).to_string()
      }
    })
    .collect::<String>();
  format!("_{_type}_{name}")
}

pub fn get_text_like_value(node: &Expression, exclude_number: Option<bool>) -> Option<String> {
  let node = node.without_parentheses().get_inner_expression();
  if let Expression::StringLiteral(node) = node {
    return Some(node.value.to_string());
  } else if !exclude_number.unwrap_or(false) && node.is_number_literal() {
    if let Expression::NumericLiteral(node) = node {
      return Some(node.value.to_string());
    } else if let Expression::BigIntLiteral(node) = node {
      return Some(node.value.to_string());
    }
  } else if let Expression::TemplateLiteral(node) = node {
    let mut result = String::new();
    for i in 0..node.quasis.len() {
      result += node.quasis[i].value.cooked.unwrap().as_ref();
      if let Some(expression) = node.expressions.get(i) {
        let expression_value = get_text_like_value(expression, None)?;
        result += &expression_value;
      }
    }
    return Some(result);
  }
  None
}
