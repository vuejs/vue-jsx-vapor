use common::ast::RootNode;
use common::directive::{Directives, Modifiers};
use common::expression::get_constant_expression_text;
use common::options::Template;
pub use common::options::TransformOptions;
use napi::bindgen_prelude::Either3;
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::ast::{Expression, JSXAttributeItem, JSXChild, JSXElement, JSXExpression};
use oxc_ast::{AstBuilder, NONE};
use oxc_span::{GetSpan, SPAN, Span};
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
use crate::transform::transform_element::{
  is_keep_alive_tag, is_select_value_key, is_transition_tag, props_may_contain_select_value,
};
use crate::transform::transform_key::transform_key;
use crate::{
  ir::{
    component::IRProp,
    index::{
      BlockIRNode, DynamicFlag, IRDynamicInfo, IREffect, IfIRNode, OperationNode, RootIRNode,
    },
  },
  transform::{
    transform_children::transform_children, transform_element::transform_element,
    transform_template_ref::transform_template_ref, transform_text::transform_text,
    v_for::transform_v_for, v_if::transform_v_if, v_slot::transform_v_slot,
    v_slots::transform_v_slots,
  },
};

use common::check::{get_namespace, is_constant_node, is_native_on, is_native_tag};
use common::html_nesting::is_foreign_breakout_tag;

/// A text/interpolation-only owner renders nothing when empty, so a block it
/// owns cannot be a single root when hydrating.
fn is_text_only_owner(node: &JSXChild) -> bool {
  let children = match node {
    JSXChild::Element(element) => &element.children,
    JSXChild::Fragment(fragment) => &fragment.children,
    _ => return false,
  };
  children.iter().all(|child| match child {
    JSXChild::Text(_) => true,
    JSXChild::ExpressionContainer(exp) => {
      !matches!(exp.expression, JSXExpression::EmptyExpression(_))
    }
    _ => false,
  })
}

pub struct DirectiveTransformResult<'a> {
  pub key: Expression<'a>,
  pub value: Expression<'a>,
  pub to_display_string: bool,
  pub modifier: Option<&'a str>,
  pub runtime_camelize: bool,
  pub handler: bool,
  pub handler_modifiers: Option<Modifiers<'a>>,
  pub model: bool,
  pub model_modifiers: Option<Vec<String>>,
}

impl<'a> DirectiveTransformResult<'a> {
  pub fn new(key: Expression<'a>, value: Expression<'a>, modifier: Option<&'a str>) -> Self {
    DirectiveTransformResult {
      key,
      value,
      to_display_string: false,
      modifier,
      runtime_camelize: false,
      handler: false,
      handler_modifiers: None,
      model: false,
      model_modifiers: None,
    }
  }
}

type GetIndex<'a> = Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>;
type GetOrder<'a> = Rc<RefCell<Box<dyn FnMut() -> u64 + 'a>>>;

// A context takes one stride of registration order keys when it is created;
// the prop effects of its element use `order + 1 ... order + k`
const ORDER_STRIDE: u64 = 1 << 20;

/// The namespace context of an element being transformed, used to resolve the
/// namespace of its children (mirrors `compiler-dom`'s `getNamespace`).
#[derive(Clone, Copy, Default)]
pub struct NsContext<'a> {
  /// The element's tag name, or `None` for a fragment/root.
  pub tag: Option<&'a str>,
  /// The element's own namespace (0 = HTML, 1 = SVG, 2 = MathML).
  pub ns: i32,
  /// The namespace of the enclosing element, i.e. the `ns` this one was
  /// resolved from (mirrors `parent.node.ns` in `compiler-vapor`).
  pub parent_ns: i32,
  /// Whether the element is `<annotation-xml>` with an HTML `encoding`.
  pub is_html_annotation_xml: bool,
}

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
  // registration order key of this node; a block operation records it as the
  // point its own effects start from, see `finalize_block`
  pub order: RefCell<u64>,
  // root-only: the next free order key
  next_order: RefCell<u64>,

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
  // An ancestor with an explicit close tag that cannot pop an open special
  // descendant (any other end tag, formatting elements) also requires special
  // descendants in the same template to close explicitly.
  pub template_close_blocks: RefCell<bool>,
  // Whether the element is written into its parent's template and what that
  // template leaves open above it (`open_scope` bits); 0 when it starts a
  // template of its own.
  pub template_scope: RefCell<i32>,

  // Namespace (0 = HTML, 1 = SVG, 2 = MathML) context of the innermost element
  // currently being transformed. `transform_element` saves it, installs its own
  // context before visiting children and restores the parent on exit, so a
  // child resolves its namespace from the enclosing element exactly like
  // `compiler-dom`'s `getNamespace` (exit callbacks run strictly innermost-first,
  // so a single slot is enough). Components/custom elements that fall back to
  // plain elements at runtime are created in this namespace
  // (https://github.com/vuejs/core/pull/15451).
  pub ns: RefCell<NsContext<'a>>,

  global_id: RefCell<i32>,
  if_index: RefCell<i32>,
  // every `v-if` head with whether it sits in `v-once`; chains that grow past
  // IF_CHAIN_THRESHOLD get their key range reserved once the tree is done
  pub if_heads: RefCell<Vec<(*mut IfIRNode<'a>, bool)>>,
  // the latest `v-if` chain among the current parent's children: its head (with
  // whether it sits in `v-once`), and the `v-if` / `v-else-if` node a following
  // `v-else[-if]` attaches to. Replaced when a parent's children transform
  // starts, like `is_slot_root`
  pub if_head: RefCell<Option<(*mut IfIRNode<'a>, bool)>>,
  pub if_tail: RefCell<Option<*mut IfIRNode<'a>>>,

  pub ir: Rc<RefCell<RootIRNode<'a>>>,
  pub node: RefCell<JSXChild<'a>>,

  pub parent_dynamic: RefCell<IRDynamicInfo<'a>>,
  pub grandparent_node_span: RefCell<Span>,
  // whether the node currently being transformed is the single root of a slot
  // body - the direct content of a component (its default slot body, built-ins
  // aside) or of a slot template, or a v-if branch of either. Replaced when a
  // parent's children transform starts, like `grandparent_node_span`
  pub is_slot_root: RefCell<bool>,

  pub parent_directives: RefCell<Directives<'a>>,
}

impl<'a> TransformContext<'a> {
  pub fn new(
    node: Expression<'a>,
    options: &'a TransformOptions<'a>,
    ast: &'a AstBuilder<'a>,
  ) -> Self {
    let allocator = &options.allocator;
    let context = TransformContext {
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
      order: RefCell::new(0),
      next_order: RefCell::new(0),
      is_last_effective_child: RefCell::new(true),
      is_on_rightmost_path: RefCell::new(true),
      template_close_blocks: RefCell::new(false),
      template_close_tags: RefCell::new(HashSet::new()),
      template_scope: RefCell::new(0),
      ns: RefCell::default(),
      global_id: RefCell::new(0),
      if_index: RefCell::new(0),
      if_heads: RefCell::new(Vec::new()),
      if_head: RefCell::new(None),
      if_tail: RefCell::new(None),
      node: RefCell::new(RootNode::from(ast, options, node, true, None)),
      parent_dynamic: RefCell::new(IRDynamicInfo::new()),
      grandparent_node_span: RefCell::new(SPAN),
      is_slot_root: RefCell::new(false),
      ir: Rc::new(RefCell::new(RootIRNode::default())),
      block: RefCell::new(BlockIRNode::new()),
      parent_directives: RefCell::new(Directives::default()),
      ast,
      options,
    };
    *context.order.borrow_mut() = context.allocate_order();
    context
  }

  pub fn transform(self) -> Expression<'a> {
    let ir = RootIRNode::default();
    let mut block = BlockIRNode::new();
    block.root = true;
    *self.block.borrow_mut() = block;
    *self.ir.borrow_mut() = ir;
    self.transform_node(None, None);
    self.finalize_if_chains();
    self.finalize_block(&mut self.block.borrow_mut());
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

  pub fn next_if_index(&self, count: i32) -> i32 {
    let if_index = *self.if_index.borrow();
    *self.if_index.borrow_mut() += count;
    if_index
  }

  fn allocate_order(&self) -> u64 {
    let order = *self.next_order.borrow();
    *self.next_order.borrow_mut() = order + ORDER_STRIDE;
    order
  }

  // A long chain renders as one keyed fragment: its selection is frozen only by
  // `v-once` on the head, and its branch keys are consecutive, so the range is
  // reserved after every `v-else-if` has joined (indexes handed out in between
  // to nested `v-if`s are left as gaps).
  fn finalize_if_chains(&self) {
    for (head, in_v_once) in self.if_heads.borrow().iter().copied() {
      let head = unsafe { &mut *head };
      if !v_if::is_long_if_chain(head) {
        continue;
      }
      head.once = in_v_once;
      if !in_v_once {
        // one index covers two keys; the chain keys every branch plus the empty
        // state past the last one
        head.index = self.next_if_index((v_if::get_if_chain_length(head) + 2) / 2);
      }
    }
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
    get_effect_order: Option<GetOrder<'a>>,
    get_operation_index: Option<GetIndex<'a>>,
  ) {
    if is_operation {
      return self.register_operation(context_block, operation, get_operation_index);
    }

    let order = if let Some(get_effect_order) = get_effect_order {
      get_effect_order.borrow_mut()()
    } else {
      self.allocate_order()
    };
    context_block.effect.push(IREffect {
      operations: vec![operation],
      order,
    });
  }

  // Settles the order a block's operations and effects are generated in, once
  // all of its nodes are transformed. Effects sort by order key: an element's
  // prop effects are keyed from the element's own context, so they precede
  // the effects of its children although the element exits last. Built-in
  // v-model needs the initial DOM props applied first (the runtime picks the
  // text / checkbox / radio implementation from them), listeners on the same
  // element run after v-model's own so a handler for the same event sees the
  // updated value, as in vdom, and a select's value waits for its options:
  // all of them move behind every other operation and effect. The block
  // operations' boundaries then become indexes into the settled arrays.
  pub fn finalize_block(&self, block: &mut BlockIRNode<'a>) {
    block.effect.sort_by_key(|effect| effect.order);

    let model_elements: Vec<i32> = block.operation.iter().filter_map(v_model_element).collect();
    let select_value_elements = mem::take(&mut block.select_value_elements);
    let is_deferred = |operation: &OperationNode<'a>| {
      is_v_model_listener(operation, &model_elements)
        || is_select_value_operation(operation, &select_value_elements)
    };

    let mut operation_indexes = None;
    if model_elements.is_empty() && select_value_elements.is_empty() {
      block.deferred_operation_index = block.operation.len();
      block.deferred_effect_index = block.effect.len();
    } else {
      let mut immediate = Vec::with_capacity(block.operation.len());
      let mut models = Vec::new();
      let mut deferred = Vec::new();
      let mut indexes = Vec::with_capacity(block.operation.len() + 1);
      for operation in block.operation.drain(..) {
        indexes.push(immediate.len());
        if v_model_element(&operation).is_some() {
          models.push(operation);
        } else if is_deferred(&operation) {
          deferred.push(operation);
        } else {
          immediate.push(operation);
        }
      }
      indexes.push(immediate.len());
      block.deferred_operation_index = immediate.len();
      block.operation = immediate;
      block.operation.extend(models);
      block.operation.extend(deferred);
      operation_indexes = Some(indexes);

      let mut immediate = Vec::with_capacity(block.effect.len());
      let mut deferred = Vec::new();
      for effect in block.effect.drain(..) {
        if !effect.operations.is_empty() && effect.operations.iter().all(is_deferred) {
          deferred.push(effect);
        } else {
          immediate.push(effect);
        }
      }
      block.deferred_effect_index = immediate.len();
      block.effect = immediate;
      block.effect.extend(deferred);
    }

    resolve_effect_boundaries(
      &mut block.dynamic,
      &block.effect,
      block.deferred_effect_index,
      operation_indexes.as_deref(),
    );
  }

  pub fn register_operation(
    &self,
    context_block: &mut BlockIRNode<'a>,
    operation: OperationNode<'a>,
    get_operation_index: Option<GetIndex<'a>>,
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

  /// `ns` is the namespace the template's own element resolves to, which the
  /// runtime parses the string in. It is not the tag's: inside `<svg>` even a
  /// `tbody` is svg, and an html tag at an integration point is html.
  pub fn push_template(&self, content: String, ns: i32, _static: bool) -> i32 {
    let len = self.options.templates.borrow().len();
    let root = *self.template_root.borrow();
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

  pub fn register_template(&self, block: &mut BlockIRNode<'a>, ns: i32, _static: bool) -> i32 {
    let template = self.template.borrow();
    if template.is_empty() {
      return -1;
    }
    let id = self.push_template(template.clone(), ns, _static);
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
    // a `v-for` row is never the single root of a slot body: upstream's
    // structural transform wraps the host in a `<template v-for>` whose
    // children read a cut chain, so cut here where the block content starts
    let is_slot_root = if is_v_for {
      self.is_slot_root.replace(false)
    } else {
      *self.is_slot_root.borrow()
    };

    *context_block = ir;
    if is_v_for {
      *self.in_v_for.borrow_mut() += 1;
    }

    (Box::new(move || {
      // exit
      self.finalize_block(context_block);
      self.register_template(context_block, 0, false);
      let return_block = mem::take(context_block);
      *context_block = block;
      *self.template.borrow_mut() = template;
      *self.template_root.borrow_mut() = template_root;
      *self.children_template.borrow_mut() = children_template;
      *self.is_slot_root.borrow_mut() = is_slot_root;
      if is_v_for {
        *self.in_v_for.borrow_mut() -= 1;
      }
      *self.effect_index.borrow_mut() = effect_index;
      *self.operation_index.borrow_mut() = operation_index;
      return_block
    }) as Box<dyn FnOnce() -> BlockIRNode<'a>>) as _
  }

  pub fn is_single_root(&self, parent_node: &JSXChild) -> bool {
    if let JSXChild::Element(parent_node) = parent_node
      && parent_node
        .opening_element
        .name
        .get_identifier_name()
        .is_some_and(|name| is_transition_tag(&name) || is_keep_alive_tag(&name))
    {
      RootNode::is_single_root(*self.grandparent_node_span.borrow())
    } else {
      RootNode::is_single_root(parent_node.span())
    }
  }

  pub fn wrap_fragment(
    &self,
    mut node: Expression<'a>,
    parent_node: Option<&JSXChild<'a>>,
  ) -> JSXChild<'a> {
    let ast = self.ast;
    if let Expression::JSXFragment(node) = node {
      JSXChild::Fragment(node)
    } else if let Expression::JSXElement(node) = &mut node
      && node
        .opening_element
        .name
        .get_identifier_name()
        .is_some_and(|name| name == "template")
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
      let parent_span = if let Some(parent_node) = parent_node {
        if let JSXChild::Element(parent_node) = &*parent_node
          && parent_node
            .opening_element
            .name
            .get_identifier_name()
            .is_some_and(|name| is_transition_tag(&name) || is_keep_alive_tag(&name))
        {
          *self.grandparent_node_span.borrow()
        } else if RootNode::is_root(parent_node) {
          parent_node.span()
        } else {
          SPAN
        }
      } else {
        SPAN
      };
      ast.jsx_child_fragment(
        parent_span,
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
    parent_node: Option<&JSXChild<'a>>,
  ) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
    let mut block = BlockIRNode::new();
    *context_node = self.wrap_fragment(node, parent_node);
    block.node_text_only = is_text_only_owner(context_node);
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
    in_v_once: bool,
  ) -> impl FnOnce() {
    let is_on_rightmost_path = *self.is_on_rightmost_path.borrow() && is_last_effective_child;

    self.node.replace(node);
    let index = self.index.replace(index);
    let prev_in_v_once = self.in_v_once.replace(in_v_once);
    let template = self.template.replace(String::new());
    let is_last_effective_child = self
      .is_last_effective_child
      .replace(is_last_effective_child);
    let is_on_rightmost_path = self.is_on_rightmost_path.replace(is_on_rightmost_path);
    let template_close_blocks = self.template_close_blocks.take();
    let template_close_tags = self.template_close_tags.take();
    let template_scope = self.template_scope.replace(0);
    self.children_template.take();
    mem::take(&mut block.dynamic);
    let effect_index = self.effect_index.replace(block.effect.len());
    let operation_index = self.operation_index.replace(block.operation.len());
    let order = self.allocate_order();
    let order = self.order.replace(order);

    move || {
      self.index.replace(index);
      self.in_v_once.replace(prev_in_v_once);
      self.template.replace(template);
      self
        .is_last_effective_child
        .replace(is_last_effective_child);
      self.is_on_rightmost_path.replace(is_on_rightmost_path);
      self.template_close_blocks.replace(template_close_blocks);
      *self.template_close_tags.borrow_mut() = template_close_tags;
      *self.template_scope.borrow_mut() = template_scope;
      *self.effect_index.borrow_mut() = effect_index;
      *self.operation_index.borrow_mut() = operation_index;
      self.order.replace(order);
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
          // A custom element, and an html element the parser would move out of
          // foreign content, are created imperatively: the template helper
          // parses its string in the html namespace, so it cannot give such an
          // element the namespace it was written in, which vdom does.
          let parent = *self.ns.borrow();
          directives.use_create_element = directives.is_custom_element
            || (is_foreign_breakout_tag(directives.tag_name)
              && get_namespace(
                directives.tag_name,
                parent.tag,
                parent.ns,
                parent.is_html_annotation_xml,
              ) != 0);
          if directives.use_create_element {
            directives.is_component = true;
          }
          // v-once reaching this node from an ancestor or the chain head; its own
          // `v-once` is applied below and a chain branch may drop it again
          let inherited_in_v_once = *(&*context).in_v_once.borrow();
          if directives.v_once.is_some() {
            *(&*context).in_v_once.borrow_mut() = true;
          };

          if let Some(memo) = &directives.v_memo
            && (&*context).seen.borrow_mut().insert(memo.span.start)
          {
            (&*context).options.on_warn.as_ref()(
              "v-memo is not supported in Vapor mode and will be ignored.".to_string(),
              memo.span,
            );
          }

          if (directives.v_if.is_some()
            || directives.v_else_if.is_some()
            || directives.v_else.is_some())
            && !(directives.is_template && directives.v_slot.is_some())
            && let Some(on_exit) = transform_v_if(
              &mut *directives_ptr,
              node,
              &*context,
              &mut *block,
              &mut *parent_node,
              inherited_in_v_once,
            )
          {
            exit_fns.push(on_exit);
          };

          if directives.v_for.is_some()
            && !(directives.is_template && directives.v_slot.is_some())
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
            && !(directives.tag_name == "template" && directives.v_slot.is_some())
            && !*(&*context).in_v_once.borrow()
            && let Some(on_exit) = transform_key(
              &mut *directives_ptr,
              node,
              &*context,
              &mut *block,
              &*parent_node,
            )
          {
            exit_fns.push(on_exit);
          };

          if directives._ref.is_some()
            && !matches!(&*node, JSXChild::Fragment(_))
            && !directives.is_template
            && !(directives.tag_name == "template" && directives.key.is_some())
            && let Some(on_exit) =
              transform_template_ref(&mut *directives_ptr, &*context, &mut *block)
          {
            exit_fns.push(on_exit);
          }
        }

        if !directives.is_template
          && !(directives.tag_name == "template" && directives.key.is_some())
        {
          if let Some(on_exit) = transform_element(
            &mut directives,
            node,
            &*context,
            &mut *block,
            &mut *parent_node,
          ) {
            exit_fns.push(on_exit);
          };
        }

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
        directives,
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
        self.register_template(context_block, 0, false);
      }
    }
  }
}

// Settles the block operations' boundaries into indexes of the finalized
// arrays: the effect boundary is the first effect keyed at or after the
// operation's node, and the operation boundary skips the operations that moved
// behind the deferred index.
fn resolve_effect_boundaries<'a>(
  dynamic: &mut IRDynamicInfo<'a>,
  effect: &[IREffect<'a>],
  deferred_effect_index: usize,
  operation_indexes: Option<&[usize]>,
) {
  if let Some(operation) = &mut dynamic.operation
    && let Some((operation_index, effect_index)) = operation.as_mut().block_boundary()
    && let Some(order) = *effect_index
  {
    *effect_index =
      Some(effect[..deferred_effect_index].partition_point(|effect| effect.order < order) as u64);
    if let Some(operation_indexes) = operation_indexes {
      *operation_index = operation_index.map(|index| operation_indexes[index]);
    }
  }

  for child in &mut dynamic.children {
    resolve_effect_boundaries(child, effect, deferred_effect_index, operation_indexes);
  }
}

// The element a built-in v-model applies to, `None` for every other operation.
fn v_model_element(operation: &OperationNode<'_>) -> Option<i32> {
  match operation {
    OperationNode::Directive(operation) if operation.builtin && operation.name == "model" => {
      Some(operation.element)
    }
    _ => None,
  }
}

fn is_native_on_prop(prop: &IRProp<'_>) -> bool {
  prop.modifier.is_none()
    && matches!(
      &prop.key,
      Expression::StringLiteral(key) if is_native_on(&key.value)
    )
}

// A select's value binding is applied after the children: the value is a no-op
// while the options it selects among are still missing (#1318).
fn is_select_value_operation<'a>(
  operation: &OperationNode<'a>,
  select_value_elements: &[i32],
) -> bool {
  if select_value_elements.is_empty() {
    return false;
  }
  match operation {
    OperationNode::SetProp(operation) => {
      select_value_elements.contains(&operation.element)
        && is_select_value_key(&operation.prop.key, operation.prop.modifier)
    }
    OperationNode::SetDynamicProps(operation) => {
      select_value_elements.contains(&operation.element)
        && props_may_contain_select_value(&operation.props)
    }
    _ => false,
  }
}

// Listeners on a v-model element are deferred until after the model
// application, so their handlers see the value the model already updated, as
// in vdom where the directive `created` hook runs before the props.
fn is_v_model_listener<'a>(operation: &OperationNode<'a>, model_elements: &[i32]) -> bool {
  if model_elements.is_empty() {
    return false;
  }
  match operation {
    OperationNode::SetEvent(operation) => model_elements.contains(&operation.element),
    OperationNode::SetDynamicEvents(operation) => model_elements.contains(&operation.element),
    OperationNode::SetProp(operation) => {
      // this dialect's v_bind props carry lowercase native event names
      // (`onclick`), so any of them can collide with the model event
      model_elements.contains(&operation.element)
        && operation.prop.modifier.is_none()
        && matches!(
          &operation.prop.key,
          Expression::StringLiteral(key) if is_native_on(&key.value)
        )
    }
    OperationNode::SetDynamicProps(operation) => {
      model_elements.contains(&operation.element)
        && operation.props.iter().any(|props| match props {
          Either3::A(props) => props.iter().any(is_native_on_prop),
          Either3::B(prop) => is_native_on_prop(prop),
          // a spread's keys are unknown - upstream resolves those by
          // listener merging rather than deferral
          Either3::C(_) => false,
        })
    }
    _ => false,
  }
}
