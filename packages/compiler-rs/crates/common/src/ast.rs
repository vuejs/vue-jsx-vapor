use oxc_allocator::{Allocator, FromIn, TakeIn};
use oxc_ast::ast::{Expression, JSXChild, JSXClosingFragment, JSXFragment, JSXOpeningFragment};
use oxc_span::{GetSpan, SPAN, Span};

use crate::text::is_empty_text;

#[derive(Debug)]
pub struct RootNode;
impl<'a> RootNode {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(allocator: &'a Allocator) -> JSXChild<'a> {
    JSXChild::Fragment(oxc_allocator::Box::new_in(
      JSXFragment::from_in(
        JSXFragment {
          span: Span::new(0, 1),
          opening_fragment: JSXOpeningFragment::from_in(
            JSXOpeningFragment { span: SPAN },
            allocator,
          ),
          children: oxc_allocator::Vec::new_in(allocator),
          closing_fragment: JSXClosingFragment::from_in(
            JSXClosingFragment { span: SPAN },
            allocator,
          ),
        },
        allocator,
      ),
      allocator,
    ))
  }
  pub fn from(
    allocator: &'a Allocator,
    expression: Expression<'a>,
    ignore_fragment: bool,
  ) -> JSXChild<'a> {
    let mut is_fragment = false;
    let children = match expression {
      Expression::JSXFragment(mut node) => {
        is_fragment = true;
        if ignore_fragment {
          node.children.take_in(allocator)
        } else {
          oxc_allocator::Vec::from_array_in([JSXChild::Fragment(node)], allocator)
        }
      }
      Expression::JSXElement(mut node) => oxc_allocator::Vec::from_array_in(
        [JSXChild::Element(oxc_allocator::Box::new_in(
          node.take_in(allocator),
          allocator,
        ))],
        allocator,
      ),
      _ => oxc_allocator::Vec::new_in(allocator),
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
            JSXOpeningFragment { span: SPAN },
            allocator,
          ),
          children,
          closing_fragment: JSXClosingFragment::from_in(
            JSXClosingFragment { span: SPAN },
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
