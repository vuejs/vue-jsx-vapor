use std::borrow::Cow;

use oxc_allocator::{Allocator, CloneIn, FromIn, TakeIn};
use oxc_ast::{
  AstBuilder,
  ast::{Expression, JSXAttributeValue},
};
use oxc_parser::Parser;
use oxc_span::{Atom, GetSpan, SPAN, SourceType, Span};
use phf::phf_set;

use crate::{options::TransformOptions, text::get_text_like_value};

pub fn get_constant_expression_text<'a>(
  exp: &Expression<'a>,
  options: &TransformOptions<'a>,
) -> Option<Cow<'a, str>> {
  if let Some(value) = get_text_like_value(exp, false) {
    Some(value)
  } else {
    let content = exp.span().source_text(&options.source_text.borrow());
    if is_literal_whitelisted(content) || is_globally_allowed(content) {
      Some(Cow::Borrowed(content))
    } else {
      None
    }
  }
}

static LITERAL_WHITELIST: [&str; 4] = ["true", "false", "null", "this"];
pub fn is_literal_whitelisted(key: &str) -> bool {
  LITERAL_WHITELIST.contains(&key)
}

static GLOBALLY_ALLOWED: phf::Set<&'static str> = phf_set! {
    "Infinity",
    "undefined",
    "NaN",
    "isFinite",
    "isNaN",
    "parseFloat",
    "parseInt",
    "decodeURI",
    "decodeURIComponent",
    "encodeURI",
    "encodeURIComponent",
    "Math",
    "Number",
    "Date",
    "Array",
    "Object",
    "Boolean",
    "String",
    "RegExp",
    "Map",
    "Set",
    "JSON",
    "Intl",
    "BigInt",
    "console",
    "Error",
    "Symbol",
};
pub fn is_globally_allowed(key: &str) -> bool {
  GLOBALLY_ALLOWED.contains(key)
}

pub fn parse_expression<'a>(
  source: &str,
  span: Span,
  allocator: &'a Allocator,
  source_type: SourceType,
) -> Option<Expression<'a>> {
  Parser::new(
    allocator,
    Atom::from_in(
      if span == SPAN {
        Cow::Borrowed(source)
      } else {
        Cow::Owned(format!(
          "/*{}*/({})",
          ".".repeat(span.start as usize - 5),
          source
        ))
      }
      .as_ref(),
      allocator,
    )
    .as_str(),
    source_type,
  )
  .parse_expression()
  .ok()
}

pub fn jsx_attribute_value_to_expression<'a>(
  value: &mut JSXAttributeValue<'a>,
  ast: &AstBuilder<'a>,
) -> Expression<'a> {
  match value {
    JSXAttributeValue::Element(value) => Expression::JSXElement(value.clone_in(ast.allocator)),
    JSXAttributeValue::Fragment(value) => Expression::JSXFragment(value.clone_in(ast.allocator)),
    JSXAttributeValue::StringLiteral(value) => {
      ast.expression_string_literal(value.span, value.value, value.raw)
    }
    JSXAttributeValue::ExpressionContainer(value) => {
      value.expression.to_expression_mut().take_in(ast.allocator)
    }
  }
}
