use common::expression::SimpleExpressionNode;
pub use common::options::TransformOptions;
use common::walk::WalkIdentifiers;
use oxc_allocator::{Allocator, CloneIn, TakeIn};
use oxc_ast::ast::{
  ArrayExpressionElement, AssignmentOperator, AssignmentTarget, Expression, JSXAttributeValue,
  JSXChild, JSXClosingFragment, JSXExpression, JSXExpressionContainer, JSXFragment,
  JSXOpeningFragment, LogicalOperator, NumberBase, ObjectPropertyKind,
};
use oxc_ast::{AstBuilder, NONE};
use oxc_span::{GetSpan, SPAN, Span};
use std::collections::HashMap;
use std::{cell::RefCell, collections::HashSet, rc::Rc};
pub mod cache_static;
pub mod transform_children;
pub mod transform_element;
pub mod transform_text;
pub mod utils;
pub mod v_bind;
pub mod v_for;
pub mod v_html;
pub mod v_if;
pub mod v_memo;
pub mod v_model;
pub mod v_on;
pub mod v_once;
pub mod v_show;
pub mod v_slot;
pub mod v_slots;
pub mod v_text;

use crate::ast::{ConstantTypes, NodeTypes, RootNode};
use crate::transform::cache_static::cache_static;
use crate::transform::v_memo::transform_v_memo;
use crate::transform::v_slot::track_slot_scopes;
use crate::transform::{
  transform_children::transform_children, transform_element::transform_element,
  transform_text::transform_text, v_for::transform_v_for, v_if::transform_v_if,
  v_once::transform_v_once, v_slots::transform_v_slots,
};

use common::check::{is_constant_node, is_template};

pub struct DirectiveTransformResult<'a> {
  pub props: Vec<ObjectPropertyKind<'a>>,
  pub runtime: Option<Expression<'a>>,
}

pub struct TransformContext<'a> {
  pub allocator: &'a Allocator,
  pub index: RefCell<i32>,

  pub options: &'a TransformOptions<'a>,

  pub template: RefCell<String>,
  pub children_template: RefCell<Vec<String>>,

  pub in_v_once: RefCell<bool>,
  pub in_v_for: RefCell<i32>,
  pub in_v_slot: RefCell<i32>,

  pub seen: Rc<RefCell<HashSet<u32>>>,

  pub source: RefCell<&'a str>,
  pub root_node: RefCell<JSXChild<'a>>,

  pub ast: AstBuilder<'a>,
  pub constant_cache: RefCell<HashMap<Span, ConstantTypes>>,
  pub codegen_map: RefCell<HashMap<Span, NodeTypes<'a>>>,
  pub cache_index: RefCell<usize>,
  pub components: RefCell<HashSet<String>>,
  pub directives: RefCell<HashSet<String>>,
}

impl<'a> TransformContext<'a> {
  pub fn new(allocator: &'a Allocator, options: &'a TransformOptions<'a>) -> Self {
    TransformContext {
      allocator,
      index: RefCell::new(0),
      template: RefCell::new(String::new()),
      children_template: RefCell::new(Vec::new()),
      in_v_once: RefCell::new(*options.in_v_once.borrow()),
      in_v_for: RefCell::new(*options.in_v_for.borrow()),
      in_v_slot: RefCell::new(0),
      seen: Rc::new(RefCell::new(HashSet::new())),
      root_node: RefCell::new(RootNode::new(allocator)),
      source: RefCell::new(""),
      ast: AstBuilder::new(allocator),
      constant_cache: RefCell::new(HashMap::new()),
      codegen_map: RefCell::new(HashMap::new()),
      cache_index: RefCell::new(0),
      components: RefCell::new(HashSet::new()),
      directives: RefCell::new(HashSet::new()),
      options,
    }
  }

  pub fn transform(&'a self, expression: Expression<'a>, source: &'a str) -> Expression<'a> {
    let allocator = self.allocator;
    *self.root_node.borrow_mut() = RootNode::from(allocator, expression);
    *self.source.borrow_mut() = source;
    unsafe {
      self.transform_node(self.root_node.as_ptr(), None);
    }
    self.generate()
  }

  pub fn helper(&self, name: &str) -> String {
    self.options.helpers.borrow_mut().insert(name.to_string());
    format!("_{name}")
  }

  pub fn hoist(&self, exp: &mut Expression<'a>) -> Expression<'a> {
    let span = exp.span();
    self
      .options
      .hoists
      .borrow_mut()
      .push(exp.take_in(self.allocator));
    self.ast.expression_identifier(
      span,
      self
        .ast
        .atom(&format!("_hoisted_{}", self.options.hoists.borrow().len())),
    )
  }

  pub fn cache(
    &self,
    value: Expression<'a>,
    is_v_node: bool,
    in_v_once: bool,
    need_array_spread: bool,
  ) -> Expression<'a> {
    let ast = &self.ast;
    let index = *self.cache_index.borrow();
    let cache = ast.alloc_computed_member_expression(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom("_cache")),
      ast.expression_identifier(SPAN, ast.atom(&index.to_string())),
      false,
    );
    let mut assing_expression = ast.expression_parenthesized(
      SPAN,
      ast.expression_assignment(
        SPAN,
        AssignmentOperator::Assign,
        AssignmentTarget::ComputedMemberExpression(cache.clone_in(ast.allocator)),
        value,
      ),
    );
    if is_v_node {
      let mut arguments = ast.vec1(
        ast
          .expression_numeric_literal(SPAN, -1_f64, None, NumberBase::Hex)
          .into(),
      );
      if in_v_once {
        arguments.push(ast.expression_boolean_literal(SPAN, true).into());
      }
      assing_expression = ast.expression_sequence(
        SPAN,
        ast.vec_from_array([
          ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&self.helper("setBlockTracking"))),
            NONE,
            arguments,
            false,
          ),
          ast.expression_assignment(
            SPAN,
            AssignmentOperator::Assign,
            AssignmentTarget::StaticMemberExpression(ast.alloc_static_member_expression(
              SPAN,
              assing_expression,
              ast.identifier_name(SPAN, "cacheIndex"),
              false,
            )),
            ast.expression_numeric_literal(SPAN, index as f64, None, NumberBase::Hex),
          ),
          ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&self.helper("setBlockTracking"))),
            NONE,
            ast.vec1(
              ast
                .expression_numeric_literal(SPAN, 1_f64, None, NumberBase::Hex)
                .into(),
            ),
            false,
          ),
          Expression::ComputedMemberExpression(cache.clone_in(ast.allocator)),
        ]),
      );
    }
    let exp = ast.expression_logical(
      SPAN,
      Expression::ComputedMemberExpression(cache),
      LogicalOperator::Or,
      assing_expression,
    );
    *self.cache_index.borrow_mut() += 1;
    if need_array_spread {
      ast.expression_array(
        SPAN,
        ast.vec1(ArrayExpressionElement::SpreadElement(
          ast.alloc_spread_element(SPAN, exp),
        )),
      )
    } else {
      exp
    }
  }

  pub fn is_operation(&self, expressions: Vec<&SimpleExpressionNode>) -> bool {
    if self.in_v_once.borrow().eq(&true) {
      return true;
    }
    let expressions: Vec<&SimpleExpressionNode> = expressions
      .into_iter()
      .filter(|exp| !exp.is_constant_expression())
      .collect();
    if expressions.is_empty() {
      return true;
    }
    expressions
      .iter()
      .all(|exp| is_constant_node(&exp.ast.as_deref()))
  }

  pub fn wrap_fragment(&self, mut node: Expression<'a>, span: Span) -> JSXChild<'a> {
    let ast = self.ast;
    if let Expression::JSXFragment(node) = node {
      JSXChild::Fragment(node)
    } else if let Expression::JSXElement(node) = &mut node
      && is_template(node)
    {
      let name = ast.jsx_element_name_identifier(node.span, ast.atom(&self.helper("Fragment")));
      ast.jsx_child_fragment(
        span,
        ast.jsx_opening_fragment(SPAN),
        ast.vec1(
          ast.jsx_child_element(
            node.span,
            ast.jsx_opening_element(
              node.opening_element.span,
              name.clone_in(ast.allocator),
              NONE,
              node.opening_element.attributes.take_in(self.allocator),
            ),
            node.children.take_in(self.allocator),
            node
              .closing_element
              .take()
              .map(|e| ast.jsx_closing_element(e.span, name)),
          ),
        ),
        ast.jsx_closing_fragment(SPAN),
      )
    } else {
      JSXChild::Fragment(oxc_allocator::Box::new_in(
        JSXFragment {
          span,
          opening_fragment: JSXOpeningFragment { span: SPAN },
          closing_fragment: JSXClosingFragment { span: SPAN },
          children: oxc_allocator::Vec::from_array_in(
            [match node {
              Expression::JSXElement(node) => JSXChild::Element(node),
              Expression::JSXFragment(node) => JSXChild::Fragment(node),
              _ => JSXChild::ExpressionContainer(oxc_allocator::Box::new_in(
                JSXExpressionContainer {
                  span: SPAN,
                  expression: node.into(),
                },
                self.allocator,
              )),
            }],
            self.allocator,
          ),
        },
        self.allocator,
      ))
    }
  }

  pub fn jsx_attribute_value_to_expression(
    &'a self,
    value: &mut JSXAttributeValue<'a>,
  ) -> Expression<'a> {
    match value.take_in(self.allocator) {
      JSXAttributeValue::Element(value) => Expression::JSXElement(value),
      JSXAttributeValue::Fragment(value) => Expression::JSXFragment(value),
      JSXAttributeValue::StringLiteral(value) => Expression::StringLiteral(value),
      JSXAttributeValue::ExpressionContainer(mut value) => {
        self.jsx_expression_to_expression(&mut value.expression)
      }
    }
  }

  pub fn jsx_expression_to_expression(&'a self, value: &mut JSXExpression<'a>) -> Expression<'a> {
    let value = value.to_expression_mut().take_in(self.allocator);
    if matches!(
      value,
      Expression::Identifier(_) | Expression::StaticMemberExpression(_)
    ) {
      value
    } else {
      WalkIdentifiers::new(
        Box::new(|_, _, _, _, _| None),
        &self.ast,
        *self.source.borrow(),
        self.options,
        false,
      )
      .traverse(value)
    }
  }

  /// # SAFETY
  pub unsafe fn transform_node(
    self: &'a TransformContext<'a>,
    node: *mut JSXChild<'a>,
    parent_node: Option<&mut JSXChild<'a>>,
  ) {
    unsafe {
      let mut exit_fns = vec![];

      let is_root = RootNode::is_root(&*node);
      if !is_root {
        let context = self as *const TransformContext;
        let parent_node = parent_node.unwrap() as *mut JSXChild;
        for node_transform in [
          transform_v_once,
          transform_v_if,
          transform_v_memo,
          transform_v_for,
          transform_v_slots,
          transform_element,
          track_slot_scopes,
          transform_text,
        ] {
          let on_exit = node_transform(&mut *node, &*context, &mut *parent_node);
          if let Some(on_exit) = on_exit {
            exit_fns.push(on_exit);
          }
        }
      }

      transform_children(&mut *node, self);

      let mut i = exit_fns.len();
      while i > 0 {
        i -= 1;
        let on_exit = exit_fns.pop().unwrap();
        on_exit();
      }

      if is_root {
        cache_static(&mut *node, self, &mut self.codegen_map.borrow_mut());
      }
    }
  }
}
