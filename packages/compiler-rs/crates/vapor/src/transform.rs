use common::ast::RootNode;
use common::directive::{Directives, Modifiers};
use common::expression::SimpleExpressionNode;
pub use common::options::TransformOptions;
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::AstBuilder;
use oxc_ast::ast::{
  Expression, JSXChild, JSXClosingFragment, JSXExpressionContainer, JSXFragment, JSXOpeningFragment,
};
use oxc_span::{GetSpan, SPAN};
use std::{cell::RefCell, collections::HashSet, mem, rc::Rc};
pub mod transform_children;
pub mod transform_element;
pub mod transform_key;
pub mod transform_slot_outlet;
pub mod transform_template_ref;
pub mod transform_text;
pub mod transform_transition;
pub mod v_bind;
pub mod v_for;
pub mod v_html;
pub mod v_if;
pub mod v_model;
pub mod v_on;
pub mod v_show;
pub mod v_slot;
pub mod v_slots;
pub mod v_text;

use crate::generate::CodegenContext;
use crate::transform::transform_key::transform_v_key;
use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, IREffect, OperationNode, RootIRNode},
  transform::{
    transform_children::transform_children, transform_element::transform_element,
    transform_template_ref::transform_template_ref, transform_text::transform_text,
    v_for::transform_v_for, v_if::transform_v_if, v_slot::transform_v_slot,
    v_slots::transform_v_slots,
  },
};

use common::check::{is_constant_node, is_math_ml_tag, is_svg_tag, is_template};

pub struct DirectiveTransformResult<'a> {
  pub key: SimpleExpressionNode<'a>,
  pub value: SimpleExpressionNode<'a>,
  pub modifier: Option<String>,
  pub runtime_camelize: bool,
  pub handler: bool,
  pub handler_modifiers: Option<Modifiers>,
  pub model: bool,
  pub model_modifiers: Option<Vec<String>>,
}

impl<'a> DirectiveTransformResult<'a> {
  pub fn new(key: SimpleExpressionNode<'a>, value: SimpleExpressionNode<'a>) -> Self {
    DirectiveTransformResult {
      key,
      value,
      modifier: None,
      runtime_camelize: false,
      handler: false,
      handler_modifiers: None,
      model: false,
      model_modifiers: None,
    }
  }
}

type GetIndex<'a> = Option<Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>>;

pub struct TransformContext<'a> {
  pub allocator: &'a Allocator,
  pub source_text: &'a str,
  pub ast: &'a AstBuilder<'a>,
  pub index: RefCell<i32>,

  pub block: RefCell<BlockIRNode<'a>>,
  pub options: &'a TransformOptions<'a>,

  pub template: RefCell<String>,
  pub children_template: RefCell<Vec<String>>,

  pub in_v_once: RefCell<bool>,
  pub in_v_for: RefCell<i32>,

  pub seen: Rc<RefCell<HashSet<u32>>>,

  global_id: RefCell<i32>,

  pub ir: Rc<RefCell<RootIRNode>>,
  pub node: RefCell<JSXChild<'a>>,

  pub parent_dynamic: RefCell<IRDynamicInfo<'a>>,
}

impl<'a> TransformContext<'a> {
  pub fn new(options: &'a TransformOptions<'a>, ast: &'a AstBuilder<'a>) -> Self {
    let allocator = &options.allocator;
    TransformContext {
      allocator,
      source_text: *options.source_text.borrow(),
      index: RefCell::new(0),
      template: RefCell::new(String::new()),
      children_template: RefCell::new(Vec::new()),
      in_v_once: RefCell::new(*options.in_v_once.borrow()),
      in_v_for: RefCell::new(*options.in_v_for.borrow()),
      seen: Rc::new(RefCell::new(HashSet::new())),
      global_id: RefCell::new(0),
      node: RefCell::new(RootNode::new(allocator)),
      parent_dynamic: RefCell::new(IRDynamicInfo::new()),
      ir: Rc::new(RefCell::new(RootIRNode::new())),
      block: RefCell::new(BlockIRNode::new()),
      ast,
      options,
    }
  }

  pub fn transform(&'a self, expression: Expression<'a>) -> Expression<'a> {
    let allocator = self.allocator;
    let ir = RootIRNode::new();
    *self.node.borrow_mut() = RootNode::from(allocator, expression, true);
    *self.block.borrow_mut() = BlockIRNode::new();
    *self.ir.borrow_mut() = ir;
    self.transform_node(None, None);
    let generate_context: *const CodegenContext = &CodegenContext::new(self);
    (unsafe { &*generate_context }).generate()
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

  pub fn push_template(&self, content: String, tag: Option<String>) -> i32 {
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
    let namespace = if let Some(tag) = tag {
      if is_svg_tag(&tag) {
        1
      } else if is_math_ml_tag(&tag) {
        2
      } else {
        0
      }
    } else {
      0
    };
    self
      .options
      .templates
      .borrow_mut()
      .push((content, root, namespace));
    len as i32
  }

  pub fn register_template(&self, dynamic: &mut IRDynamicInfo, tag: Option<String>) -> i32 {
    let template = self.template.borrow();
    if template.is_empty() {
      return -1;
    }
    let id = self.push_template(template.clone(), tag);
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
      self.register_template(&mut context_block.dynamic, None);
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

  pub fn wrap_fragment(&self, mut node: Expression<'a>) -> JSXChild<'a> {
    if let Expression::JSXFragment(node) = node {
      JSXChild::Fragment(node)
    } else if let Expression::JSXElement(node) = &mut node
      && is_template(node)
    {
      JSXChild::Element(oxc_allocator::Box::new_in(
        node.take_in(self.allocator),
        self.allocator,
      ))
    } else {
      JSXChild::Fragment(oxc_allocator::Box::new_in(
        JSXFragment {
          span: SPAN,
          opening_fragment: JSXOpeningFragment { span: SPAN },
          closing_fragment: JSXClosingFragment { span: SPAN },
          children: oxc_allocator::Vec::from_array_in(
            [match node {
              Expression::JSXElement(node) => JSXChild::Element(node),
              Expression::JSXFragment(node) => JSXChild::Fragment(node),
              _ => JSXChild::ExpressionContainer(oxc_allocator::Box::new_in(
                JSXExpressionContainer {
                  span: node.span(),
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
    is_v_for: bool,
  ) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
    let block = BlockIRNode::new();
    *context_node = self.wrap_fragment(node);
    let _context_block = context_block as *mut BlockIRNode;
    let exit_block = self.enter_block(unsafe { &mut *_context_block }, block, is_v_for);
    self.reference(&mut context_block.dynamic);
    exit_block
  }

  pub fn create(
    self: &TransformContext<'a>,
    node: JSXChild<'a>,
    index: i32,
    block: &mut BlockIRNode<'a>,
  ) -> impl FnOnce() {
    self.node.replace(node);
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

      let is_root = RootNode::is_root(&self.node.borrow());
      if !is_root {
        let context = self as *const TransformContext;
        let node = &mut *self.node.borrow_mut() as *mut _;
        let parent_node = parent_node.unwrap() as *mut _;
        let mut directives = Directives::default();
        let directives_ptr = &mut directives as *mut _;
        if let JSXChild::Element(element) = &mut *node {
          directives = Directives::new(element);
          if directives.v_once.is_some() {
            *(&*context).in_v_once.borrow_mut() = true;
          };

          if (directives.v_if.is_some()
            || directives.v_else_if.is_some()
            || directives.v_else.is_some())
            && let Some(on_exit) =
              transform_v_if(&mut *directives_ptr, node, &*context, &mut *block)
          {
            exit_fns.push(on_exit);
          };

          if directives.v_for.is_some()
            && let Some(on_exit) = transform_v_for(
              &mut *directives_ptr,
              node,
              &*context,
              &mut *block,
              &mut *parent_node,
            )
          {
            exit_fns.push(on_exit);
          } else if directives.key.is_some()
            && !*(&*context).in_v_once.borrow()
            && let Some(on_exit) =
              transform_v_key(&mut *directives_ptr, node, &*context, &mut *block)
          {
            exit_fns.push(on_exit);
          };

          if directives._ref.is_some()
            && let Some(on_exit) =
              transform_template_ref(&mut *directives_ptr, node, &*context, &mut *block)
          {
            exit_fns.push(on_exit);
          };
        }

        if let Some(on_exit) =
          transform_element(&directives, node, &*context, &mut *block, &mut *parent_node)
        {
          exit_fns.push(on_exit);
        };

        if let Some(on_exit) = transform_text(node, &*context, &mut *block, &mut *parent_node) {
          exit_fns.push(on_exit);
        };

        if directives.v_slot.is_none()
          && let Some(on_exit) =
            transform_v_slots(&mut *directives_ptr, node, &*context, &mut *block)
        {
          exit_fns.push(on_exit);
        }

        if let Some(on_exit) = transform_v_slot(
          &mut *directives_ptr,
          node,
          &*context,
          &mut *block,
          &mut *parent_node,
        ) {
          exit_fns.push(on_exit);
        }
      }

      let node = &mut self.node.borrow_mut().take_in(self.allocator);
      transform_children(node, self, &mut *block);

      let mut i = exit_fns.len();
      while i > 0 {
        i -= 1;
        let on_exit = exit_fns.pop().unwrap();
        on_exit();
      }

      if is_root {
        self.register_template(&mut context_block.dynamic, None);
      }
    }
  }
}
