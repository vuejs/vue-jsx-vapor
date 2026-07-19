use common::ast::RootNode;
use common::directive::{Directives, Modifiers};
use common::expression::get_constant_expression_text;
use common::options::Template;
pub use common::options::TransformOptions;
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::ast::{Expression, JSXAttributeItem, JSXChild, JSXElement};
use oxc_ast::{AstBuilder, NONE};
use oxc_span::{GetSpan, SPAN};
use std::borrow::Cow;
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
use crate::transform::transform_key::transform_key;
use crate::{
  ir::index::{BlockIRNode, DynamicFlag, IRDynamicInfo, IREffect, OperationNode, RootIRNode},
  transform::{
    transform_children::transform_children, transform_element::transform_element,
    transform_template_ref::transform_template_ref, transform_text::transform_text,
    v_for::transform_v_for, v_if::transform_v_if, v_slot::transform_v_slot,
    v_slots::transform_v_slots,
  },
};

use common::check::{is_constant_node, is_math_ml_tag, is_native_tag, is_svg_tag, is_template};

pub struct DirectiveTransformResult<'a> {
  pub key: Expression<'a>,
  pub value: Expression<'a>,
  pub modifier: Option<&'a str>,
  pub runtime_camelize: bool,
  pub handler: bool,
  pub handler_modifiers: Option<Modifiers<'a>>,
  pub model: bool,
  pub model_modifiers: Option<Vec<String>>,
}

impl<'a> DirectiveTransformResult<'a> {
  pub fn new(key: Expression<'a>, value: Expression<'a>) -> Self {
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
  pub template_root: RefCell<bool>,
  pub children_template: RefCell<Vec<Cow<'a, str>>>,

  pub in_v_once: RefCell<bool>,
  pub in_v_for: RefCell<i32>,

  pub seen: Rc<RefCell<HashSet<u32>>>,

  pub effect_index: RefCell<usize>,
  pub operation_index: RefCell<usize>,

  // whether this node is the last effective child of its parent
  // (all siblings after it are components, which don't appear in HTML template)
  pub is_last_effective_child: RefCell<bool>,
  // whether this node is on the rightmost path of the tree
  // (all ancestors are also last effective children)
  pub is_on_rightmost_path: RefCell<bool>,
  // If an ancestor in the same template must close explicitly, descendants
  // with matching tags must also close so the browser doesn't consume the
  // ancestor close tag for the descendant.
  pub template_close_tags: RefCell<HashSet<&'a str>>,
  // Inline ancestors with explicit close tags also require block descendants
  // in the same template to close explicitly.
  pub template_close_blocks: RefCell<bool>,

  global_id: RefCell<i32>,
  if_index: RefCell<i32>,

  pub ir: Rc<RefCell<RootIRNode<'a>>>,
  pub node: RefCell<JSXChild<'a>>,

  pub parent_dynamic: RefCell<IRDynamicInfo<'a>>,
}

impl<'a> TransformContext<'a> {
  pub fn new(
    node: Expression<'a>,
    options: &'a TransformOptions<'a>,
    ast: &'a AstBuilder<'a>,
  ) -> Self {
    let allocator = &options.allocator;
    TransformContext {
      allocator,
      source_text: *options.source_text.borrow(),
      index: RefCell::new(0),
      template: RefCell::new(String::new()),
      template_root: RefCell::new(false),
      children_template: RefCell::new(Vec::new()),
      in_v_once: RefCell::new(*options.in_v_once.borrow()),
      in_v_for: RefCell::new(*options.in_v_for.borrow()),
      seen: Rc::new(RefCell::new(HashSet::new())),
      effect_index: RefCell::new(0),
      operation_index: RefCell::new(0),
      is_last_effective_child: RefCell::new(true),
      is_on_rightmost_path: RefCell::new(true),
      template_close_blocks: RefCell::new(false),
      template_close_tags: RefCell::new(HashSet::new()),
      global_id: RefCell::new(0),
      if_index: RefCell::new(0),
      node: RefCell::new(RootNode::from(ast, options, node, true, None)),
      parent_dynamic: RefCell::new(IRDynamicInfo::new()),
      ir: Rc::new(RefCell::new(RootIRNode::default())),
      block: RefCell::new(BlockIRNode::new()),
      ast,
      options,
    }
  }

  pub fn transform(self) -> Expression<'a> {
    let ir = RootIRNode::default();
    let mut block = BlockIRNode::new();
    block.root = true;
    *self.block.borrow_mut() = block;
    *self.ir.borrow_mut() = ir;
    self.transform_node(None, None);
    CodegenContext::new(self).generate()
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

  pub fn next_if_index(&self) -> i32 {
    let if_index = *self.if_index.borrow();
    *self.if_index.borrow_mut() += 1;
    if_index
  }

  pub fn is_operation(&self, expressions: Vec<&Expression<'a>>) -> bool {
    if self.in_v_once.borrow().eq(&true) {
      return true;
    }
    let expressions: Vec<_> = expressions
      .into_iter()
      .filter(|exp| get_constant_expression_text(exp, self.options).is_none())
      .collect();
    if expressions.is_empty() {
      return true;
    }
    expressions.iter().all(|exp| is_constant_node(exp))
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

    let mut has_get_effect_index = false;
    let index = if let Some(get_effect_index) = get_effect_index {
      has_get_effect_index = true;
      get_effect_index.borrow_mut()() as usize
    } else {
      context_block.effect.len()
    };
    context_block.effect.insert(
      index,
      IREffect {
        operations: vec![operation],
      },
    );
    if has_get_effect_index {
      self.shift_effect_boundaries(index, &mut context_block.dynamic);
    }
  }

  fn shift_effect_boundaries(&self, index: usize, dynamic: &mut IRDynamicInfo) {
    if let Some(operation) = dynamic.operation.as_mut()
      && let Some(effect_index) = match operation.as_mut() {
        OperationNode::If(operation) => operation.effect_index.as_mut(),
        OperationNode::For(operation) => operation.effect_index.as_mut(),
        OperationNode::CreateComponent(operation) => operation.effect_index.as_mut(),
        OperationNode::SlotOutlet(operation) => operation.effect_index.as_mut(),
        OperationNode::Key(operation) => operation.effect_index.as_mut(),
        _ => None,
      }
      && *effect_index >= index
    {
      *effect_index += 1;
    };

    for child in dynamic.children.iter_mut() {
      self.shift_effect_boundaries(index, child);
    }
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

  pub fn can_use_static_template(&self, block: &BlockIRNode, tag: &str) -> bool {
    if self.template.borrow().is_empty() {
      return false;
    }
    if *self.in_v_for.borrow() > 0 {
      return false;
    }
    if block.dynamic.has_dynamic_child {
      return false;
    }
    if block.effect.len() != *self.effect_index.borrow() {
      return false;
    }
    if block.operation.len() != *self.operation_index.borrow() {
      return false;
    }
    if tag == "template" {
      return false;
    }

    is_native_tag(tag)
  }

  pub fn push_template(&self, content: String, tag: Option<&str>, _static: bool) -> i32 {
    let len = self.options.templates.borrow().len();
    let root = *self.template_root.borrow();
    // root_template_index.map(|i| i.eq(&len)).unwrap_or(false);
    let ns = if let Some(tag) = tag {
      if is_svg_tag(tag) {
        1
      } else if is_math_ml_tag(tag) {
        2
      } else {
        0
      }
    } else {
      0
    };
    let existing = self.options.templates.borrow().iter().position(|i| {
      i.content.eq(&content) && i.root.eq(&root) && i._static.eq(&_static) && i.ns.eq(&ns)
    });
    if let Some(existing) = existing {
      return existing as i32;
    }
    self.options.templates.borrow_mut().push(Template {
      content,
      root,
      ns,
      _static,
    });
    len as i32
  }

  pub fn register_template(
    &self,
    block: &mut BlockIRNode<'a>,
    tag: Option<&str>,
    _static: bool,
  ) -> i32 {
    let template = self.template.borrow();
    if template.is_empty() {
      return -1;
    }
    let id = self.push_template(template.clone(), tag, _static);
    block.dynamic.template = Some(id);
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
    let template_root = self.template_root.replace(false);
    let effect_index = *self.effect_index.borrow();
    *self.effect_index.borrow_mut() = ir.effect.len();
    let operation_index = *self.operation_index.borrow();
    *self.operation_index.borrow_mut() = ir.operation.len();
    let children_template = mem::take(&mut *self.children_template.borrow_mut());

    *context_block = ir;
    if is_v_for {
      *self.in_v_for.borrow_mut() += 1;
    }

    (Box::new(move || {
      // exit
      self.register_template(context_block, None, false);
      let return_block = mem::take(context_block);
      *context_block = block;
      *self.template.borrow_mut() = template;
      *self.template_root.borrow_mut() = template_root;
      *self.children_template.borrow_mut() = children_template;
      if is_v_for {
        *self.in_v_for.borrow_mut() -= 1;
      }
      *self.effect_index.borrow_mut() = effect_index;
      *self.operation_index.borrow_mut() = operation_index;
      return_block
    }) as Box<dyn FnOnce() -> BlockIRNode<'a>>) as _
  }

  pub fn wrap_fragment(&self, mut node: Expression<'a>) -> JSXChild<'a> {
    let ast = self.ast;
    if let Expression::JSXFragment(node) = node {
      JSXChild::Fragment(node)
    } else if let Expression::JSXElement(node) = &mut node
      && is_template(node)
    {
      let node_ptr = node.as_mut() as *mut JSXElement;
      let mut directives = Directives::new(unsafe { &mut *node_ptr }, self.options);
      if (directives.v_if.is_some()
        || directives.v_else_if.is_some()
        || directives.v_else.is_some())
        && (directives.v_for.is_some() || directives.key.is_some())
      {
        if let Some(dir) = directives
          .v_if
          .as_mut()
          .or(directives.v_else_if.as_mut().or(directives.v_else.as_mut()))
        {
          node.opening_element.attributes =
            ast.vec1(JSXAttributeItem::Attribute(dir.take_in_box(self.allocator)));
        }

        node.children = ast.vec1(
          self.ast.jsx_child_element(
            SPAN,
            ast.jsx_opening_element(
              SPAN,
              ast.jsx_element_name_identifier(SPAN, "template"),
              NONE,
              ast.vec_from_iter(
                [
                  directives
                    .v_for
                    .map(|v_for| JSXAttributeItem::Attribute(v_for.take_in_box(self.allocator))),
                  directives
                    .key
                    .map(|key| JSXAttributeItem::Attribute(key.take_in_box(self.allocator))),
                ]
                .into_iter()
                .flatten(),
              ),
            ),
            node.children.take_in(self.allocator),
            Some(ast.jsx_closing_element(SPAN, ast.jsx_element_name_identifier(SPAN, "template"))),
          ),
        );
      }
      JSXChild::Element(node.take_in_box(self.allocator))
    } else {
      ast.jsx_child_fragment(
        SPAN,
        ast.jsx_opening_fragment(SPAN),
        ast.vec1(match node {
          Expression::JSXElement(node) => JSXChild::Element(node),
          Expression::JSXFragment(node) => JSXChild::Fragment(node),
          _ => ast.jsx_child_expression_container(node.span(), node.into()),
        }),
        ast.jsx_closing_fragment(SPAN),
      )
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
    is_last_effective_child: bool,
    block: &mut BlockIRNode<'a>,
  ) -> impl FnOnce() {
    let is_on_rightmost_path = *self.is_on_rightmost_path.borrow() && is_last_effective_child;

    self.node.replace(node);
    let index = self.index.replace(index);
    let in_v_once = *self.in_v_once.borrow();
    let template = self.template.replace(String::new());
    let is_last_effective_child = self
      .is_last_effective_child
      .replace(is_last_effective_child);
    let is_on_rightmost_path = self.is_on_rightmost_path.replace(is_on_rightmost_path);
    let template_close_blocks = self.template_close_blocks.take();
    let template_close_tags = self.template_close_tags.take();
    self.children_template.take();
    mem::take(&mut block.dynamic);
    let effect_index = self.effect_index.replace(block.effect.len());
    let operation_index = self.operation_index.replace(block.operation.len());

    move || {
      self.index.replace(index);
      self.in_v_once.replace(in_v_once);
      self.template.replace(template);
      self
        .is_last_effective_child
        .replace(is_last_effective_child);
      self.is_on_rightmost_path.replace(is_on_rightmost_path);
      self.template_close_blocks.replace(template_close_blocks);
      *self.template_close_tags.borrow_mut() = template_close_tags;
      *self.effect_index.borrow_mut() = effect_index;
      *self.operation_index.borrow_mut() = operation_index;
    }
  }

  pub fn transform_node(
    self: &TransformContext<'a>,
    context_block: Option<&'a mut BlockIRNode<'a>>,
    parent_node: Option<&mut JSXChild<'a>>,
  ) {
    let parent_ptr = parent_node.map(|node| node as *mut _);
    unsafe {
      let context_block = if let Some(context_block) = context_block {
        context_block
      } else {
        &mut self.block.borrow_mut()
      };

      let block = context_block as *mut BlockIRNode;
      let mut exit_fns = vec![];

      let mut directives = Directives::default();
      let is_root = RootNode::is_root(&self.node.borrow());
      if !is_root {
        let context = self as *const TransformContext;
        let node = &mut *self.node.borrow_mut() as *mut _;
        let parent_node = parent_ptr.unwrap();
        let directives_ptr = &mut directives as *mut _;
        if let JSXChild::Element(element) = &mut *node {
          directives = Directives::new(element, self.options);
          if directives.is_custom_element {
            directives.is_component = true;
          }
          if directives.v_once.is_some() {
            *(&*context).in_v_once.borrow_mut() = true;
          };

          if (directives.v_if.is_some()
            || directives.v_else_if.is_some()
            || directives.v_else.is_some())
            && let Some(on_exit) = transform_v_if(
              &mut *directives_ptr,
              node,
              &*context,
              &mut *block,
              &mut *parent_node,
            )
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
            && let Some(on_exit) = transform_key(&mut *directives_ptr, node, &*context, &mut *block)
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

        if let Some(on_exit) = transform_element(
          &mut directives,
          node,
          &*context,
          &mut *block,
          &mut *parent_node,
        ) {
          exit_fns.push(on_exit);
        };

        if let Some(on_exit) =
          transform_text(&directives, node, &*context, &mut *block, &mut *parent_node)
        {
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
      transform_children(
        &directives,
        node,
        self,
        &mut *block,
        parent_ptr.map(|node| &*node),
      );

      let mut i = exit_fns.len();
      while i > 0 {
        i -= 1;
        let on_exit = exit_fns.pop().unwrap();
        on_exit();
      }

      if is_root {
        self.register_template(context_block, None, false);
      }
    }
  }
}
