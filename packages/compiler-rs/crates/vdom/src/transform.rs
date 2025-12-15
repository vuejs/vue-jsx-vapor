use common::expression::SimpleExpressionNode;
pub use common::options::TransformOptions;
use oxc_allocator::{Allocator, CloneIn, TakeIn};
use oxc_ast::ast::{
  ArrayExpressionElement, AssignmentOperator, AssignmentTarget, Expression, JSXChild,
  JSXClosingFragment, JSXExpressionContainer, JSXFragment, JSXOpeningFragment, LogicalOperator,
  ObjectPropertyKind,
};
use oxc_ast::{AstBuilder, NONE};
use oxc_span::{GetSpan, SPAN, Span};
use std::collections::HashMap;
use std::{cell::RefCell, collections::HashSet, mem, rc::Rc};
pub mod cache_static;
pub mod transform_children;
pub mod transform_element;
pub mod transform_template_ref;
pub mod transform_text;
pub mod utils;
pub mod v_bind;
pub mod v_for;
pub mod v_html;
pub mod v_if;
pub mod v_model;
pub mod v_on;
pub mod v_once;
pub mod v_show;
pub mod v_slot;
pub mod v_slots;
pub mod v_text;

use crate::ast::{ConstantTypes, NodeTypes};
use crate::transform::cache_static::cache_static;
use crate::transform::v_slot::track_slot_scopes;
use crate::{
  ir::index::{
    BlockIRNode, DynamicFlag, IRDynamicInfo, IREffect, OperationNode, RootIRNode, RootNode,
  },
  transform::{
    transform_children::transform_children, transform_element::transform_element,
    transform_text::transform_text, v_for::transform_v_for, v_if::transform_v_if,
    v_slots::transform_v_slots,
  },
};

use common::check::{is_constant_node, is_template};

pub struct DirectiveTransformResult<'a> {
  pub props: Vec<ObjectPropertyKind<'a>>,
  pub runtime: Option<Expression<'a>>,
}

// pub type ContextNode<'a> = Either<RootNode<'a>, JSXChild<'a>>;
type GetIndex<'a> = Option<Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>>;

pub struct TransformContext<'a> {
  pub allocator: &'a Allocator,
  pub index: RefCell<i32>,

  pub block: RefCell<BlockIRNode<'a>>,
  pub options: &'a TransformOptions<'a>,

  pub template: RefCell<String>,
  pub children_template: RefCell<Vec<String>>,

  pub in_v_once: RefCell<bool>,
  pub in_v_for: RefCell<i32>,
  pub in_v_slot: RefCell<i32>,

  pub seen: Rc<RefCell<HashSet<u32>>>,

  global_id: RefCell<i32>,

  pub ir: Rc<RefCell<RootIRNode<'a>>>,
  pub root_node: RefCell<JSXChild<'a>>,

  pub parent_dynamic: RefCell<IRDynamicInfo<'a>>,

  pub vdom: RefCell<bool>,
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
      global_id: RefCell::new(0),
      root_node: RefCell::new(RootNode::new(allocator)),
      parent_dynamic: RefCell::new(IRDynamicInfo::new()),
      ir: Rc::new(RefCell::new(RootIRNode::new(""))),
      block: RefCell::new(BlockIRNode::new()),
      vdom: RefCell::new(false),
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
    let ir = RootIRNode::new(source);
    *self.root_node.borrow_mut() = RootNode::from(allocator, expression);
    *self.block.borrow_mut() = BlockIRNode::new();
    *self.ir.borrow_mut() = ir;
    self.transform_node(self.root_node.as_ptr(), None, None);
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
    need_array_spread: bool,
  ) -> Expression<'a> {
    let ast = &self.ast;
    let cache = ast.alloc_computed_member_expression(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom("_cache")),
      ast.expression_identifier(SPAN, ast.atom(&self.cache_index.borrow().to_string())),
      false,
    );
    let exp = ast.expression_logical(
      SPAN,
      Expression::ComputedMemberExpression(cache.clone_in(ast.allocator)),
      LogicalOperator::Or,
      ast.expression_parenthesized(
        SPAN,
        ast.expression_assignment(
          SPAN,
          AssignmentOperator::Assign,
          AssignmentTarget::ComputedMemberExpression(cache),
          value,
        ),
      ),
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

  pub fn increase_id(&self) -> i32 {
    let current = *self.global_id.borrow();
    *self.global_id.borrow_mut() += 1;
    current
  }

  pub fn reference(&self, dynamic: &mut IRDynamicInfo) -> i32 {
    if let Some(id) = dynamic.id {
      return id;
    }
    dynamic.flags |= DynamicFlag::Referenced as i32;
    let id = self.increase_id();
    dynamic.id = Some(id);
    id
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

  pub fn register_effect(
    &self,
    context_block: &mut BlockIRNode<'a>,
    is_operation: bool,
    operation: OperationNode<'a>,
    get_effect_index: GetIndex<'a>,
    get_operation_index: GetIndex<'a>,
  ) {
    if is_operation {
      return self.register_operation(context_block, operation, get_operation_index);
    }

    let index = if let Some(get_effect_index) = get_effect_index {
      get_effect_index.borrow_mut()() as usize
    } else {
      context_block.effect.len()
    };
    context_block.effect.insert(
      index,
      IREffect {
        expressions: vec![],
        operations: vec![operation],
      },
    );
  }

  pub fn register_operation(
    &self,
    context_block: &mut BlockIRNode<'a>,
    operation: OperationNode<'a>,
    get_operation_index: GetIndex<'a>,
  ) {
    let index = if let Some(get_operation_index) = get_operation_index {
      get_operation_index.borrow_mut()() as usize
    } else {
      context_block.operation.len()
    };
    context_block.operation.insert(index, operation);
  }

  pub fn push_template(&self, content: String) -> i32 {
    let ir = self.ir.borrow_mut();
    let root_template_index = ir.root_template_index;
    let len = self.options.templates.borrow().len();
    let root = root_template_index.map(|i| i.eq(&len)).unwrap_or(false);
    let existing = self
      .options
      .templates
      .borrow()
      .iter()
      .position(|i| i.0.eq(&content) && i.1.eq(&root));
    if let Some(existing) = existing {
      return existing as i32;
    }
    self.options.templates.borrow_mut().push((content, root));
    len as i32
  }

  pub fn register_template(&self, dynamic: &mut IRDynamicInfo) -> i32 {
    let template = self.template.borrow();
    if template.is_empty() {
      return -1;
    }
    let id = self.push_template(template.clone());
    dynamic.template = Some(id);
    id
  }

  pub fn enter_block(
    self: &'a TransformContext<'a>,
    context_block: &'a mut BlockIRNode<'a>,
    ir: BlockIRNode<'a>,
    is_v_for: bool,
  ) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
    let block = mem::take(&mut *context_block);
    let template = mem::take(&mut *self.template.borrow_mut());
    let children_template = mem::take(&mut *self.children_template.borrow_mut());

    *context_block = ir;
    if is_v_for {
      *self.in_v_for.borrow_mut() += 1;
    }

    (Box::new(move || {
      // exit
      self.register_template(&mut context_block.dynamic);
      let return_block = mem::take(context_block);
      *context_block = block;
      *self.template.borrow_mut() = template;
      *self.children_template.borrow_mut() = children_template;
      if is_v_for {
        *self.in_v_for.borrow_mut() -= 1;
      }
      return_block
    }) as Box<dyn FnOnce() -> BlockIRNode<'a>>) as _
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
            SPAN,
            ast.jsx_opening_element(
              node.opening_element.span,
              name.clone_in(ast.allocator),
              NONE,
              ast.vec(),
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

  pub fn create_block(
    &'a self,
    context_node: &mut JSXChild<'a>,
    context_block: &'a mut BlockIRNode<'a>,
    node: Expression<'a>,
    is_v_for: Option<bool>,
  ) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
    let block = BlockIRNode::new();
    *context_node = self.wrap_fragment(node, SPAN);
    let _context_block = context_block as *mut BlockIRNode;
    let exit_block = self.enter_block(
      unsafe { &mut *_context_block },
      block,
      is_v_for.unwrap_or(false),
    );
    self.reference(&mut context_block.dynamic);
    exit_block
  }

  pub fn create(
    self: &TransformContext<'a>,
    context_node: &mut JSXChild<'a>,
    node: JSXChild<'a>,
    index: i32,
    block: &mut BlockIRNode<'a>,
  ) -> impl FnOnce() {
    *context_node = node;
    let index = self.index.replace(index);
    let in_v_once = *self.in_v_once.borrow();
    let template = self.template.replace(String::new());
    self.children_template.take();
    mem::take(&mut block.dynamic);

    move || {
      self.index.replace(index);
      self.in_v_once.replace(in_v_once);
      self.template.replace(template);
      self.index.replace(index);
    }
  }

  pub fn transform_node(
    self: &TransformContext<'a>,
    node: *mut JSXChild<'a>,
    context_block: Option<&'a mut BlockIRNode<'a>>,
    parent_node: Option<&mut JSXChild<'a>>,
  ) {
    unsafe {
      let context_block = if let Some(context_block) = context_block {
        context_block
      } else {
        &mut self.block.borrow_mut()
      };

      let block = context_block as *mut BlockIRNode;
      let mut exit_fns = vec![];

      let is_root = RootNode::is_root(&*node);
      if !is_root {
        let context = self as *const TransformContext;
        let parent_node = parent_node.unwrap() as *mut JSXChild;
        for node_transform in [
          // transform_v_once,
          transform_v_if,
          transform_v_for,
          // transform_template_ref,
          transform_v_slots,
          transform_element,
          track_slot_scopes,
          transform_text,
        ] {
          let on_exit = node_transform(node, &*context, &mut *block, &mut *parent_node);
          if let Some(on_exit) = on_exit {
            exit_fns.push(on_exit);
          }
        }
      }

      // if is_root {
      transform_children(&mut *node, self, &mut *block);
      // }

      let mut i = exit_fns.len();
      while i > 0 {
        i -= 1;
        let on_exit = exit_fns.pop().unwrap();
        on_exit();
      }

      if is_root {
        self.register_template(&mut context_block.dynamic);
        cache_static(&mut *node, self, &mut self.codegen_map.borrow_mut());
      }
    }
  }
}
