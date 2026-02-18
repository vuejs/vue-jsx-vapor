use std::borrow::Cow;

use html_escape::decode_html_entities;
use oxc_ast::ast::{Expression, JSXChild, JSXElement, JSXElementName, JSXExpression, JSXText};

use crate::options::TransformOptions;

fn is_all_empty_text(s: &str) -> bool {
  let mut has_newline = false;
  for c in s.chars() {
    if !c.is_ascii_whitespace() {
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
    if matches!(c, '\t' | '\x0C' | ' ') {
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
    if matches!(c, '\t' | '\x0C' | ' ') {
      continue;
    } else {
      return c == '\n' || c == '\r';
    }
  }
  false
}

pub fn resolve_jsx_text<'a>(node: &'a JSXText) -> Cow<'a, str> {
  if is_all_empty_text(&node.value) {
    return Cow::Owned(String::new());
  }

  let mut value = decode_html_entities(&node.value);

  if start_with_newline_and_spaces(value.as_ref()) {
    value = match value {
      Cow::Borrowed(s) => Cow::Borrowed(s.trim_start_matches(|c: char| c.is_ascii_whitespace())),
      Cow::Owned(s) => Cow::Owned(
        s.trim_start_matches(|c: char| c.is_ascii_whitespace())
          .to_string(),
      ),
    };
  }

  if ends_with_newline_and_spaces(value.as_ref()) {
    value = match value {
      Cow::Borrowed(s) => Cow::Borrowed(s.trim_end_matches(|c: char| c.is_ascii_whitespace())),
      Cow::Owned(s) => Cow::Owned(
        s.trim_end_matches(|c: char| c.is_ascii_whitespace())
          .to_string(),
      ),
    };
  }

  if value.trim().is_empty() {
    return Cow::Owned(String::from(" "));
  }

  value
}

pub fn is_empty_text(node: &JSXChild) -> bool {
  match node {
    JSXChild::Text(node) => is_all_empty_text(&node.value),
    JSXChild::ExpressionContainer(node) => {
      matches!(node.expression, JSXExpression::EmptyExpression(_))
    }
    _ => false,
  }
}

pub fn get_tag_name<'a>(node: &JSXElement<'a>, options: &TransformOptions<'a>) -> &'a str {
  match &node.opening_element.name {
    JSXElementName::Identifier(name) => name.name.as_str(),
    JSXElementName::IdentifierReference(name) => name.name.as_str(),
    JSXElementName::NamespacedName(name) => name.span.source_text(&options.source_text.borrow()),
    JSXElementName::MemberExpression(name) => name.span.source_text(&options.source_text.borrow()),
    JSXElementName::ThisExpression(name) => name.span.source_text(&options.source_text.borrow()),
  }
}

pub fn camelize<'a>(str: Cow<'a, str>) -> Cow<'a, str> {
  let splited = str.split('-').collect::<Vec<_>>();
  if splited.len() == 1 {
    str
  } else {
    let mut out = String::with_capacity(str.len());
    for (idx, word) in splited.iter().enumerate() {
      if idx == 0 {
        out.push_str(word);
      } else {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
          out.push_str(&(first.to_ascii_uppercase().to_string() + chars.as_str()));
        }
      }
    }
    Cow::Owned(out)
  }
}

pub fn capitalize<'a>(name: Cow<'a, str>) -> Cow<'a, str> {
  if let Some(first) = name.chars().next()
    && !first.is_ascii_uppercase()
  {
    Cow::Owned(format!("{}{}", first.to_ascii_uppercase(), &name[1..]))
  } else {
    name
  }
}

pub fn to_valid_asset_id(name: &str, _type: &str) -> String {
  let mut out = String::with_capacity(name.len() * 2);
  for c in name.chars() {
    if c == '-' {
      out.push('_')
    } else if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
      out.push(c)
    } else {
      out.push_str(&(c as u32).to_string());
    }
  }
  format!("_{_type}_{out}")
}

pub fn get_text_like_value<'a>(
  node: &Expression<'a>,
  exclude_number: bool,
) -> Option<Cow<'a, str>> {
  let node = node.without_parentheses().get_inner_expression();
  if let Expression::StringLiteral(node) = node {
    return Some(Cow::Borrowed(node.value.as_str()));
  } else if !exclude_number && node.is_number_literal() {
    if let Expression::NumericLiteral(node) = node {
      return Some(Cow::Owned(node.value.to_string()));
    } else if let Expression::BigIntLiteral(node) = node {
      return Some(Cow::Borrowed(node.value.as_str()));
    }
  } else if let Expression::TemplateLiteral(node) = node {
    let mut result = String::new();
    for i in 0..node.quasis.len() {
      result += node.quasis[i].value.cooked.unwrap().as_ref();
      if let Some(expression) = node.expressions.get(i) {
        let expression_value = get_text_like_value(expression, false)?;
        result += &expression_value;
      }
    }
    return Some(Cow::Owned(result));
  }
  None
}

pub fn is_text_like(node: &JSXChild) -> bool {
  if let JSXChild::ExpressionContainer(node) = node
    && let Some(expression) = node.expression.as_expression()
  {
    !matches!(
      expression.without_parentheses().get_inner_expression(),
      Expression::ConditionalExpression(_) | Expression::LogicalExpression(_)
    )
  } else {
    matches!(node, JSXChild::Text(_))
  }
}

pub fn escape_html<'a>(s: Cow<'a, str>) -> Cow<'a, str> {
  let bytes = s.as_bytes();
  let mut last_index = 0;
  let mut html = String::new();

  for (index, &byte) in bytes.iter().enumerate() {
    let escaped = match byte {
      b'"' => "&quot;",
      b'&' => "&amp;",
      b'\'' => "&#39;",
      b'<' => "&lt;",
      b'>' => "&gt;",
      _ => continue,
    };

    if html.is_empty() {
      html.reserve(s.len() + 16);
    }

    html.push_str(&s[last_index..index]);
    html.push_str(escaped);
    last_index = index + 1;
  }

  if last_index == 0 {
    return s;
  }

  html.push_str(&s[last_index..]);
  Cow::Owned(html)
}
