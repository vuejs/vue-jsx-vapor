use html_escape::decode_html_entities;
use oxc_ast::ast::{Expression, JSXChild, JSXElementName, JSXExpression, JSXText};

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
  if is_all_empty_text(&node.value) {
    return String::new();
  }
  let mut value = decode_html_entities(node.value.as_str()).to_string();
  if start_with_newline_and_spaces(&value) {
    value = value.trim_start().to_owned();
  }
  if ends_with_newline_and_spaces(&value) {
    value = value.trim_end().to_owned();
  }
  if value.trim().is_empty() {
    return String::from(" ");
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

pub fn get_tag_name(name: &JSXElementName, source: &str) -> String {
  match name {
    JSXElementName::Identifier(name) => name.name.to_string(),
    JSXElementName::IdentifierReference(name) => name.name.to_string(),
    JSXElementName::NamespacedName(name) => {
      source[name.span.start as usize..name.span.end as usize].to_string()
    }
    JSXElementName::MemberExpression(name) => {
      source[name.span.start as usize..name.span.end as usize].to_string()
    }
    JSXElementName::ThisExpression(name) => {
      source[name.span.start as usize..name.span.end as usize].to_string()
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

pub fn capitalize(name: String) -> String {
  if let Some(first) = name.chars().next() {
    format!("{}{}", first.to_ascii_uppercase(), &name[1..])
  } else {
    String::new()
  }
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

pub fn get_text_like_value(node: &Expression, exclude_number: bool) -> Option<String> {
  let node = node.without_parentheses().get_inner_expression();
  if let Expression::StringLiteral(node) = node {
    return Some(node.value.to_string());
  } else if !exclude_number && node.is_number_literal() {
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
        let expression_value = get_text_like_value(expression, false)?;
        result += &expression_value;
      }
    }
    return Some(result);
  }
  None
}

pub fn is_text_like(node: &JSXChild) -> bool {
  if let JSXChild::ExpressionContainer(node) = node {
    !matches!(
      node
        .expression
        .to_expression()
        .without_parentheses()
        .get_inner_expression(),
      Expression::ConditionalExpression(_) | Expression::LogicalExpression(_)
    )
  } else {
    matches!(node, JSXChild::Text(_))
  }
}

pub fn escape_html<'a>(s: String) -> String {
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
  html
}
