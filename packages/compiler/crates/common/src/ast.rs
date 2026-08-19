use std::cell::Cell;

use oxc_allocator::{Allocator, FromIn, TakeIn};
use oxc_ast::{
  AstBuilder, NONE,
  ast::{Expression, JSXChild, JSXClosingFragment, JSXFragment, JSXOpeningFragment},
};
use oxc_semantic::NodeId;
use oxc_span::{GetSpan, SPAN, Span};

use crate::{options::TransformOptions, text::is_empty_text};

#[derive(Debug)]
pub struct RootNode;
impl<'a> RootNode {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(allocator: &'a Allocator) -> JSXChild<'a> {
    JSXChild::Fragment(oxc_allocator::Box::new_in(
      JSXFragment::from_in(
        JSXFragment {
          node_id: Cell::new(NodeId::DUMMY),
          span: Span::new(0, 1),
          opening_fragment: JSXOpeningFragment::from_in(
            JSXOpeningFragment {
              node_id: Cell::new(NodeId::DUMMY),
              span: SPAN,
            },
            allocator,
          ),
          children: oxc_allocator::Vec::new_in(allocator),
          closing_fragment: JSXClosingFragment::from_in(
            JSXClosingFragment {
              node_id: Cell::new(NodeId::DUMMY),
              span: SPAN,
            },
            allocator,
          ),
        },
        allocator,
      ),
      allocator,
    ))
  }
  pub fn from(
    ast: &'a AstBuilder,
    options: &TransformOptions<'a>,
    expression: Expression<'a>,
    ignore_fragment: bool,
    key: Option<i32>,
  ) -> JSXChild<'a> {
    let allocator = ast.allocator;
    let mut is_fragment = false;
    let key_attribute = key.map(|key| {
      ast.jsx_attribute_item_attribute(
        SPAN,
        ast.jsx_attribute_name_identifier(SPAN, "key"),
        Some(
          ast.jsx_attribute_value_expression_container(
            SPAN,
            ast
              .expression_numeric_literal(SPAN, key as f64, None, oxc_ast::ast::NumberBase::Decimal)
              .into(),
          ),
        ),
      )
    });
    let children =
      match expression {
        Expression::JSXFragment(mut node) => {
          is_fragment = true;
          if ignore_fragment {
            node.children.take_in(allocator)
          } else {
            if let Some(key_attribute) = key_attribute {
              let fargment = options.helper("_Fragment");
              ast.vec1(ast.jsx_child_element(
                SPAN,
                ast.jsx_opening_element(
                  SPAN,
                  ast.jsx_element_name_identifier_reference(SPAN, fargment),
                  NONE,
                  ast.vec1(key_attribute),
                ),
                node.children.take_in(allocator),
                Some(ast.alloc_jsx_closing_element(
                  SPAN,
                  ast.jsx_element_name_identifier(SPAN, fargment),
                )),
              ))
            } else {
              ast.vec1(JSXChild::Fragment(node))
            }
          }
        }
        Expression::JSXElement(mut node) => {
          if let Some(key_attribute) = key_attribute
            && node
              .opening_element
              .attributes
              .iter()
              .find(|attr| attr.as_attribute().is_some_and(|attr| attr.is_key()))
              .is_none()
          {
            node.opening_element.attributes.insert(0, key_attribute);
          }
          ast.vec1(JSXChild::Element(ast.alloc(node.take_in(allocator))))
        }
        _ => ast.vec(),
      };

    let mut is_single_root = false;
    if !is_fragment {
      for child in children.iter() {
        if !is_empty_text(child) {
          if is_single_root {
            is_single_root = false;
            break;
          }
          is_single_root = true;
        }
      }
    }
    JSXChild::Fragment(oxc_allocator::Box::new_in(
      JSXFragment::from_in(
        JSXFragment {
          node_id: Cell::new(NodeId::DUMMY),
          span: Span::new(
            if is_fragment {
              2
            } else if is_single_root {
              3
            } else {
              1
            },
            0,
          ),
          opening_fragment: JSXOpeningFragment::from_in(
            JSXOpeningFragment {
              node_id: Cell::new(NodeId::DUMMY),
              span: SPAN,
            },
            allocator,
          ),
          children,
          closing_fragment: JSXClosingFragment::from_in(
            JSXClosingFragment {
              node_id: Cell::new(NodeId::DUMMY),
              span: SPAN,
            },
            allocator,
          ),
        },
        allocator,
      ),
      allocator,
    ))
  }
  pub fn is_root(node: &JSXChild<'a>) -> bool {
    let span = node.span();
    if span.end == 0 {
      let offset = span.start - span.end;
      offset > 0 && offset < 4
    } else {
      false
    }
  }
  pub fn is_fragment(node: &JSXChild<'a>) -> bool {
    let span = node.span();
    if span.end == 0 {
      span.start - span.end == 2
    } else {
      false
    }
  }
  pub fn is_single_root(node: &JSXChild<'a>) -> bool {
    let span = node.span();
    if span.end == 0 {
      span.start - span.end == 3
    } else {
      false
    }
  }
}

pub fn get_first_child<'a, 'b>(
  children: &'a oxc_allocator::Vec<JSXChild<'b>>,
) -> Option<&'a JSXChild<'b>> {
  let mut first_child = None;
  for child in children.iter() {
    if !is_empty_text(child) {
      if first_child.is_some() {
        first_child = None;
        break;
      }
      first_child = Some(child);
    }
  }
  first_child
}
