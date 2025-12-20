use napi::bindgen_prelude::Either3;

use oxc_allocator::{Allocator, FromIn, TakeIn};
use oxc_ast::ast::{Expression, FormalParameter, JSXAttributeValue, JSXChild};
use oxc_parser::Parser;
use oxc_span::{Atom, GetSpan, SPAN, SourceType, Span};
use phf::phf_set;

use crate::text::{get_text_like_value, resolve_jsx_text};

#[derive(Debug)]
pub struct SimpleExpressionNode<'a> {
  pub content: String,
  pub is_static: bool,
  pub loc: Span,
  pub ast: Option<&'a mut Expression<'a>>,
}

impl<'a> Clone for SimpleExpressionNode<'a> {
  fn clone(&self) -> Self {
    Self {
      content: self.content.clone(),
      is_static: self.is_static,
      loc: self.loc,
      ast: None,
    }
  }
}

impl<'a> Default for SimpleExpressionNode<'a> {
  fn default() -> Self {
    Self {
      content: String::new(),
      is_static: true,
      loc: SPAN,
      ast: None,
    }
  }
}

impl<'a> SimpleExpressionNode<'a> {
  pub fn new(
    node: Either3<&'a mut Expression<'a>, &'a mut JSXChild<'a>, &'a mut JSXAttributeValue<'a>>,
    source: &'a str,
  ) -> SimpleExpressionNode<'a> {
    let mut is_static = false;
    let mut ast = None;
    let mut loc = SPAN;
    let content = match node {
      Either3::A(node) => {
        loc = node.span();
        ast = Some(node);
        loc.source_text(source).to_string()
      }
      Either3::B(node) => match node {
        JSXChild::ExpressionContainer(node) => {
          let expression = node.expression.to_expression_mut();
          loc = expression.span();
          ast = Some(expression);
          loc.source_text(source).to_string()
        }
        JSXChild::Text(node) => {
          is_static = true;
          resolve_jsx_text(node)
        }
        JSXChild::Element(node) => {
          source[node.span.start as usize..node.span.end as usize].to_string()
        }
        JSXChild::Fragment(node) => {
          source[node.span.start as usize..node.span.end as usize].to_string()
        }
        JSXChild::Spread(node) => {
          source[node.span.start as usize..node.span.end as usize].to_string()
        }
      },
      Either3::C(node) => match node {
        JSXAttributeValue::ExpressionContainer(node) => {
          let expression = node.expression.to_expression_mut();
          is_static = matches!(expression, Expression::StringLiteral(_));
          loc = expression.span();
          ast = Some(expression);
          loc.source_text(source).to_string()
        }
        JSXAttributeValue::StringLiteral(node) => {
          is_static = true;
          loc = node.span;
          node.value.to_string()
        }
        JSXAttributeValue::Element(node) => {
          source[node.span.start as usize..node.span.end as usize].to_string()
        }
        JSXAttributeValue::Fragment(node) => {
          source[node.span.start as usize..node.span.end as usize].to_string()
        }
      },
    };
    SimpleExpressionNode {
      content,
      is_static,
      ast,
      loc,
    }
  }

  pub fn is_constant_expression(&self) -> bool {
    is_literal_whitelisted(&self.content)
      || is_globally_allowed(&self.content)
      || self.get_literal_expression_value().is_some()
  }

  pub fn get_literal_expression_value(&self) -> Option<String> {
    if let Some(ast) = &self.ast
      && let Some(res) = get_text_like_value(ast, false)
    {
      return Some(res);
    }
    if self.is_static {
      Some(self.content.to_string())
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

pub fn expression_to_params<'a>(
  exp: &Expression<'a>,
  source: &str,
  allocator: &'a Allocator,
  source_type: SourceType,
) -> Option<FormalParameter<'a>> {
  let span = exp.without_parentheses().span();
  if let Ok(Expression::ArrowFunctionExpression(mut exp)) = Parser::new(
    allocator,
    Atom::from_in(
      &format!(
        "/*{}*/({})=>{{}}",
        ".".repeat(span.start as usize - 5),
        span.source_text(source)
      ),
      allocator,
    )
    .as_str(),
    source_type,
  )
  .parse_expression()
  {
    Some(exp.params.items[0].take_in(allocator))
  } else {
    None
  }
}

pub fn parse_expression<'a>(
  source: &str,
  span: Span,
  allocator: &'a Allocator,
  source_type: SourceType,
) -> Option<Expression<'a>> {
  if let Ok(exp) = Parser::new(
    allocator,
    Atom::from_in(
      &format!("/*{}*/({})", ".".repeat(span.start as usize - 5), source),
      allocator,
    )
    .as_str(),
    source_type,
  )
  .parse_expression()
  {
    Some(exp)
  } else {
    None
  }
}
