use common::ast::{RootNode, get_first_child};
use common::directive::Directives;
pub use common::options::TransformOptions;
use common::walk::WalkIdentifiers;
use common::walk_mut::{GetNodeId, WalkIdentifiersMut};
use indexmap::IndexSet;
use oxc_allocator::{Allocator, CloneIn, TakeIn};
use oxc_ast::ast::{
  ArrayExpressionElement, AssignmentOperator, AssignmentTarget, Expression, IdentifierReference,
  JSXAttributeValue, JSXChild, LogicalOperator, NumberBase, ObjectPropertyKind,
};
use oxc_ast::{AstBuilder, AstKind, NONE};
use oxc_semantic::NodeId;
use oxc_span::{GetSpan, SPAN, Span};
use std::collections::HashMap;
use std::{cell::RefCell, collections::HashSet, rc::Rc};
pub mod cache_static;
pub mod transform_children;
pub mod transform_element;
pub mod transform_slot_outlet;
pub mod transform_text;
pub mod transform_transition;
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

use crate::ast::{ConstantTypes, NodeTypes};
use crate::transform::cache_static::cache_static;
use crate::transform::v_memo::transform_v_memo;
use crate::transform::v_slot::track_slot_scopes;
use crate::transform::{
  transform_children::transform_children, transform_element::transform_element,
  transform_text::transform_text, v_for::transform_v_for, v_if::transform_v_if,
  v_once::transform_v_once, v_slots::transform_v_slots,
};

use common::check::{is_jsx_component, is_template};

pub struct DirectiveTransformResult<'a> {
  pub props: Vec<ObjectPropertyKind<'a>>,
  pub runtime: Option<Expression<'a>>,
}

pub struct TransformContext<'a> {
  pub allocator: &'a Allocator,

  pub options: &'a TransformOptions<'a>,

  pub seen: Rc<RefCell<HashSet<u32>>>,

  pub source_text: &'a str,
  pub root_node: RefCell<JSXChild<'a>>,

  pub ast: &'a AstBuilder<'a>,
  pub constant_cache: RefCell<HashMap<Span, ConstantTypes>>,
  pub codegen_map: RefCell<HashMap<Span, NodeTypes<'a>>>,
  pub v_if_map: RefCell<HashMap<Span, (usize, Vec<Span>)>>,
  pub cache_index: RefCell<usize>,
  pub components: RefCell<IndexSet<&'a str>>,
  pub directives: RefCell<IndexSet<&'a str>>,
  pub has_temp: RefCell<bool>,
  pub has_slot: RefCell<bool>,
  pub reference_expressions: RefCell<HashMap<Span, bool>>,
  pub should_optimize: RefCell<bool>,
}

impl<'a> TransformContext<'a> {
  pub fn new(options: &'a TransformOptions<'a>, ast: &'a AstBuilder<'a>) -> Self {
    let allocator = &options.allocator;
    TransformContext {
      allocator,
      seen: Rc::new(RefCell::new(HashSet::new())),
      root_node: RefCell::new(RootNode::new(allocator)),
      source_text: *options.source_text.borrow(),
      ast,
      constant_cache: RefCell::new(HashMap::new()),
      codegen_map: RefCell::new(HashMap::new()),
      v_if_map: RefCell::new(HashMap::new()),
      cache_index: RefCell::new(0),
      components: RefCell::new(IndexSet::new()),
      directives: RefCell::new(IndexSet::new()),
      reference_expressions: RefCell::new(HashMap::new()),
      has_temp: RefCell::new(false),
      has_slot: RefCell::new(false),
      should_optimize: RefCell::new(false),
      options,
    }
  }

  pub fn transform(&'a self, expression: Expression<'a>) -> Expression<'a> {
    let allocator = self.allocator;
    *self.should_optimize.borrow_mut() = self.should_optimize(expression.node_id());
    if let Expression::JSXFragment(frag) = &expression
      && let Some(child) = get_first_child(&frag.children)
      && let JSXChild::Text(child) = child
    {
      return self
        .ast
        .expression_string_literal(child.span, child.value, child.raw);
    }
    *self.root_node.borrow_mut() = RootNode::from(allocator, expression, false);
    unsafe {
      self.transform_node(self.root_node.as_ptr(), None);
    }
    self.generate()
  }

  fn should_optimize(&self, node_id: NodeId) -> bool {
    let semantic = self.options.semantic.borrow();
    let scope_id = semantic.nodes().get_node(node_id).scope_id();
    match semantic
      .nodes()
      .get_node(semantic.scoping().get_node_id(scope_id))
      .kind()
    {
      AstKind::ArrowFunctionExpression(scope) => scope.params.is_empty(),
      AstKind::Function(scope) => scope.params.is_empty(),
      AstKind::BlockStatement(stmt) => match semantic.nodes().parent_kind(stmt.node_id()) {
        AstKind::ForStatement(_) | AstKind::ForInStatement(_) | AstKind::ForOfStatement(_) => false,
        _ => true,
      },
      _ => true,
    }
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
            ast.expression_identifier(SPAN, ast.atom(self.options.helper("_setBlockTracking"))),
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
            ast.expression_identifier(SPAN, ast.atom(self.options.helper("_setBlockTracking"))),
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

  pub fn wrap_fragment(&self, mut node: Expression<'a>, span: Span) -> JSXChild<'a> {
    let ast = self.ast;
    if let Expression::JSXFragment(node) = node {
      JSXChild::Fragment(node)
    } else if let Expression::JSXElement(node) = &mut node
      && is_template(node)
    {
      let name =
        ast.jsx_element_name_identifier(node.span, ast.atom(self.options.helper("_Fragment")));
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
      ast.jsx_child_fragment(
        span,
        ast.jsx_opening_fragment(SPAN),
        ast.vec1(match node {
          Expression::JSXElement(node) => JSXChild::Element(node),
          Expression::JSXFragment(node) => JSXChild::Fragment(node),
          _ => ast.jsx_child_expression_container(SPAN, node.into()),
        }),
        ast.jsx_closing_fragment(SPAN),
      )
    }
  }

  pub fn jsx_attribute_value_to_expression(
    &'a self,
    value: &mut JSXAttributeValue<'a>,
  ) -> Expression<'a> {
    match value {
      JSXAttributeValue::Element(value) => Expression::JSXElement(value.clone_in(self.allocator)),
      JSXAttributeValue::Fragment(value) => Expression::JSXFragment(value.clone_in(self.allocator)),
      JSXAttributeValue::StringLiteral(value) => {
        self
          .ast
          .expression_string_literal(value.span, value.value, value.raw)
      }
      JSXAttributeValue::ExpressionContainer(value) => {
        self
          .process_expression(value.expression.to_expression_mut())
          .0
      }
    }
  }

  pub fn process_expression(&'a self, exp: &mut Expression<'a>) -> (Expression<'a>, bool) {
    let span = exp.span();
    let mut value = if exp.is_literal() {
      exp.clone_in(self.allocator)
    } else {
      exp.take_in(self.allocator)
    };
    let mut has_ref = false;
    let should_optimize = *self.should_optimize.borrow();
    let mut has_scope_ref = !should_optimize;
    let has_scope_ref_ptr = &mut has_scope_ref as *mut _;
    let has_ref_ptr = &mut has_ref as *mut bool;
    WalkIdentifiersMut::new(
      Box::new(move |id, _| {
        if !should_optimize {
          self.add_slot_scopes(id);
        } else if self
          .options
          .identifiers
          .borrow()
          .get(id.name.as_str())
          .is_some()
        {
          *unsafe { &mut *has_scope_ref_ptr } = true;
          self.add_slot_scopes(id);
        }
        *unsafe { &mut *has_ref_ptr } = true;
        None
      }),
      self.options,
    )
    .visit(&mut value);
    self
      .reference_expressions
      .borrow_mut()
      .insert(span, has_ref);
    (value, has_scope_ref)
  }

  fn add_slot_scopes(&self, id: &IdentifierReference) {
    let slot_scopes = &mut self.options.slot_scopes.borrow_mut();
    if let Some(last_slot) = slot_scopes.last()
      && last_slot.1.identifiers.contains(&id.name.as_str())
    {
      return;
    }
    for value in slot_scopes.values_mut() {
      value.seen += 1;
    }
  }

  pub fn add_identifiers(&'a self, exp: &Option<&Expression<'a>>) -> Vec<&'a str> {
    let Some(exp) = exp else { return vec![] };
    let identifiers = self.options.identifiers.as_ptr();
    let mut ids = vec![];
    let ids_ptr = &mut ids as *mut Vec<&str>;
    WalkIdentifiers::new(
      Box::new(move |id, _, _| {
        let name = id.name.as_str();
        unsafe { &mut *ids_ptr }.push(name);
        unsafe { &mut *identifiers }
          .entry(name)
          .and_modify(|v| *v += 1)
          .or_insert(1);
      }),
      self.options,
    )
    .visit(exp);
    ids
  }

  pub fn remove_identifiers(&self, ids: Vec<&'a str>) {
    let identifiers = &mut self.options.identifiers.borrow_mut();
    for id in ids {
      if let Some(v) = identifiers.get_mut(&id)
        && *v > 1
      {
        *v -= 1;
      } else {
        identifiers.remove(&id);
      }
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

      let mut directives = Directives::default();
      let is_root = RootNode::is_root(&*node);
      if !is_root {
        let context = self as *const TransformContext;
        let parent_node = parent_node.unwrap() as *mut JSXChild;
        if let JSXChild::Element(element) = &mut *node {
          let is_component = if self.options.is_custom_element.as_ref()(
            element
              .opening_element
              .name
              .get_identifier_name()
              .map(|name| name.as_str())
              .unwrap_or_default(),
          ) {
            false
          } else {
            is_jsx_component(element)
          };
          directives = Directives::new(element, self.options);
          directives.is_component = is_component;
          if (directives.v_if.is_some()
            || directives.v_else_if.is_some()
            || directives.v_else.is_some())
            && let Some(on_exit) =
              transform_v_if(&mut directives, node, &*context, &mut *parent_node)
          {
            exit_fns.push(on_exit);
          };

          if directives.v_once.is_some()
            && let Some(on_exit) = transform_v_once(&mut directives, node, &*context)
          {
            exit_fns.push(on_exit);
          };

          if directives.v_memo.is_some()
            && let Some(on_exit) = transform_v_memo(&mut directives, node, &*context)
          {
            exit_fns.push(on_exit);
          };

          if directives.v_for.is_some()
            && let Some(on_exit) = transform_v_for(&mut directives, node, &*context)
          {
            exit_fns.push(on_exit);
          };

          if directives.v_slot.is_none()
            && let Some(on_exit) = transform_v_slots(&mut directives, node, &*context)
          {
            exit_fns.push(on_exit);
          };
        }

        if let Some(on_exit) =
          transform_element(&mut directives, node, &*context, &mut *parent_node)
        {
          exit_fns.push(on_exit);
        };

        if let Some(on_exit) = track_slot_scopes(&mut directives, node, &*context) {
          exit_fns.push(on_exit);
        };

        if let Some(on_exit) = transform_text(&directives, node, &*context) {
          exit_fns.push(on_exit);
        };
      }

      transform_children(&directives, &mut *node, self);

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
