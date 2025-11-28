use napi::{Either, Env};
use napi_derive::napi;
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::ast::{
  Expression, JSXChild, JSXClosingFragment, JSXExpressionContainer, JSXFragment, JSXOpeningFragment,
};
use oxc_codegen::{Codegen, CodegenReturn, IndentChar};
use oxc_parser::Parser;
use oxc_span::{SPAN, SourceType, Span};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::{cell::RefCell, collections::HashSet, mem, rc::Rc};
pub mod transform_children;
pub mod transform_element;
pub mod transform_template_ref;
pub mod transform_text;
pub mod v_bind;
pub mod v_for;
pub mod v_html;
pub mod v_if;
pub mod v_model;
pub mod v_on;
mod v_once;
pub mod v_show;
pub mod v_slot;
pub mod v_slots;
pub mod v_text;

use crate::compile::CompilerOptions;
use crate::compile::Template;
use crate::generate::CodegenContext;
use crate::traverse::jsx::JsxTraverse;
use crate::{
  ir::{
    component::IRSlots,
    index::{
      BlockIRNode, DynamicFlag, IRDynamicInfo, IREffect, Modifiers, OperationNode, RootIRNode,
      RootNode, SimpleExpressionNode,
    },
  },
  transform::{
    transform_children::transform_children, transform_element::transform_element,
    transform_template_ref::transform_template_ref, transform_text::transform_text,
    v_for::transform_v_for, v_if::transform_v_if, v_once::transform_v_once,
    v_slot::transform_v_slot, v_slots::transform_v_slots,
  },
  utils::{
    check::{is_constant_node, is_template},
    error::ErrorCodes,
  },
};

pub struct TransformOptions<'a> {
  pub templates: RefCell<Vec<Template>>,
  pub helpers: RefCell<BTreeSet<String>>,
  pub delegates: RefCell<BTreeSet<String>>,
  pub with_fallback: bool,
  pub is_custom_element: Box<dyn Fn(String) -> bool + 'a>,
  pub on_error: Box<dyn Fn(ErrorCodes, Span) + 'a>,
  pub source_map: bool,
  pub filename: &'a str,
  pub source_type: SourceType,
  pub interop: bool,
  pub hmr: bool,
  pub ssr: bool,
}
impl<'a> Default for TransformOptions<'a> {
  fn default() -> Self {
    TransformOptions {
      filename: "index.jsx",
      source_type: SourceType::jsx(),
      templates: RefCell::new(vec![]),
      helpers: RefCell::new(BTreeSet::new()),
      delegates: RefCell::new(BTreeSet::new()),
      source_map: false,
      with_fallback: false,
      is_custom_element: Box::new(|_| false),
      on_error: Box::new(|_, _| {}),
      interop: false,
      hmr: false,
      ssr: false,
    }
  }
}

pub struct DirectiveTransformResult<'a> {
  pub key: SimpleExpressionNode<'a>,
  pub value: SimpleExpressionNode<'a>,
  pub modifier: Option<String>,
  pub runtime_camelize: Option<bool>,
  pub handler: Option<bool>,
  pub handler_modifiers: Option<Modifiers>,
  pub model: Option<bool>,
  pub model_modifiers: Option<Vec<String>>,
}

impl<'a> DirectiveTransformResult<'a> {
  pub fn new(key: SimpleExpressionNode<'a>, value: SimpleExpressionNode<'a>) -> Self {
    DirectiveTransformResult {
      key,
      value,
      modifier: None,
      runtime_camelize: None,
      handler: None,
      handler_modifiers: None,
      model: None,
      model_modifiers: None,
    }
  }
}

pub type ContextNode<'a> = Either<RootNode<'a>, JSXChild<'a>>;
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

  pub slots: RefCell<Vec<IRSlots<'a>>>,

  pub seen: Rc<RefCell<HashSet<u32>>>,

  global_id: RefCell<i32>,

  pub ir: Rc<RefCell<RootIRNode<'a>>>,
  pub node: RefCell<ContextNode<'a>>,

  pub parent_dynamic: RefCell<IRDynamicInfo<'a>>,
}

impl<'a> TransformContext<'a> {
  pub fn new(allocator: &'a Allocator, options: &'a TransformOptions<'a>) -> Self {
    TransformContext {
      allocator,
      index: RefCell::new(0),
      template: RefCell::new(String::new()),
      children_template: RefCell::new(Vec::new()),
      in_v_once: RefCell::new(false),
      in_v_for: RefCell::new(0),
      slots: RefCell::new(Vec::new()),
      seen: Rc::new(RefCell::new(HashSet::new())),
      global_id: RefCell::new(0),
      node: RefCell::new(Either::A(RootNode::new(allocator))),
      parent_dynamic: RefCell::new(IRDynamicInfo::new()),
      ir: Rc::new(RefCell::new(RootIRNode::new(""))),
      block: RefCell::new(BlockIRNode::new()),
      options,
    }
  }

  pub fn transform(&'a self, expression: Expression<'a>, source: &'a str) -> Expression<'a> {
    let allocator = self.allocator;
    let mut ir = RootIRNode::new(source);
    *self.node.borrow_mut() = Either::A(RootNode::from(allocator, expression));
    *self.block.borrow_mut() = mem::take(&mut ir.block);
    *self.ir.borrow_mut() = ir;
    *self.index.borrow_mut() = 0;
    *self.slots.borrow_mut() = vec![];
    *self.template.borrow_mut() = String::new();
    *self.children_template.borrow_mut() = vec![];
    *self.in_v_once.borrow_mut() = false;
    *self.in_v_for.borrow_mut() = 0;
    *self.parent_dynamic.borrow_mut() = IRDynamicInfo::new();
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
    exclude_slots: bool,
  ) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
    let block = mem::take(&mut *context_block);
    let template = mem::take(&mut *self.template.borrow_mut());
    let children_template = mem::take(&mut *self.children_template.borrow_mut());
    let mut slots = None;

    *context_block = ir;
    if !exclude_slots {
      slots = Some(mem::take(&mut *self.slots.borrow_mut()));
    }

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
      if !exclude_slots && let Some(slots) = slots {
        *self.slots.borrow_mut() = slots;
      }
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
    context_node: &mut ContextNode<'a>,
    context_block: &'a mut BlockIRNode<'a>,
    node: Expression<'a>,
    is_v_for: Option<bool>,
  ) -> Box<dyn FnOnce() -> BlockIRNode<'a> + 'a> {
    let block = BlockIRNode::new();
    *context_node = Either::B(self.wrap_fragment(node));
    let _context_block = context_block as *mut BlockIRNode;
    let exit_block = self.enter_block(
      unsafe { &mut *_context_block },
      block,
      is_v_for.unwrap_or(false),
      false,
    );
    self.reference(&mut context_block.dynamic);
    exit_block
  }

  pub fn create(
    self: &TransformContext<'a>,
    node: JSXChild<'a>,
    index: i32,
    block: &mut BlockIRNode<'a>,
  ) -> impl FnOnce() {
    self.node.replace(Either::B(node));
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
    parent_node: Option<&mut ContextNode<'a>>,
  ) {
    unsafe {
      let context_block = if let Some(context_block) = context_block {
        context_block
      } else {
        &mut self.block.borrow_mut()
      };

      let block = context_block as *mut BlockIRNode;
      let mut exit_fns = vec![];

      let is_root = matches!(&*self.node.borrow(), Either::A(_));
      if !is_root {
        let context = self as *const TransformContext;
        let node = &mut *self.node.borrow_mut() as *mut _;
        let parent_node = parent_node.unwrap() as *mut ContextNode;
        for node_transform in [
          transform_v_once,
          transform_v_if,
          transform_v_for,
          transform_template_ref,
          transform_element,
          transform_text,
          transform_v_slots,
          transform_v_slot,
        ] {
          let on_exit = node_transform(node, &*context, &mut *block, &mut *parent_node);
          if let Some(on_exit) = on_exit {
            exit_fns.push(on_exit);
          }
        }
      }

      transform_children(
        &mut self.node.replace(Either::A(RootNode::new(self.allocator))),
        self,
        &mut *block,
      );

      let mut i = exit_fns.len();
      while i > 0 {
        i -= 1;
        let on_exit = exit_fns.pop().unwrap();
        on_exit();
      }

      if is_root {
        self.register_template(&mut context_block.dynamic);
      }
    }
  }
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct TransformReturn {
  pub code: String,
  pub map: Option<String>,
}

#[cfg(feature = "napi")]
#[napi]
pub fn _transform(env: Env, source: String, options: Option<CompilerOptions>) -> TransformReturn {
  use crate::utils::error::ErrorCodes;
  let options = options.unwrap_or_default();
  let filename = &options.filename.unwrap_or("index.jsx".to_string());
  let CodegenReturn { code, map, .. } = transform(
    &source,
    Some(TransformOptions {
      filename,
      source_type: SourceType::from_path(filename).unwrap(),
      templates: RefCell::new(vec![]),
      helpers: RefCell::new(BTreeSet::new()),
      delegates: RefCell::new(BTreeSet::new()),
      source_map: options.source_map.unwrap_or(false),
      with_fallback: options.with_fallback.unwrap_or(false),
      interop: options.interop.unwrap_or(false),
      hmr: options.hmr.unwrap_or(false),
      ssr: options.ssr.unwrap_or(false),
      is_custom_element: if let Some(is_custom_element) = options.is_custom_element {
        Box::new(move |tag: String| is_custom_element.call(tag).unwrap())
          as Box<dyn Fn(String) -> bool>
      } else {
        Box::new(|_: String| false) as Box<dyn Fn(String) -> bool>
      },
      on_error: if let Some(on_error) = options.on_error {
        use crate::utils::error::create_compiler_error;

        Box::new(move |code: ErrorCodes, span: Span| {
          let compiler_error = create_compiler_error(&env, code, span).unwrap();
          on_error.call(compiler_error).unwrap();
        }) as Box<dyn Fn(ErrorCodes, Span)>
      } else {
        Box::new(|_: ErrorCodes, _: Span| {}) as Box<dyn Fn(ErrorCodes, Span)>
      },
    }),
  );
  TransformReturn {
    code,
    map: map.map(|m| m.to_json_string()),
  }
}

pub fn transform(source: &str, options: Option<TransformOptions>) -> CodegenReturn {
  use oxc_codegen::CodegenOptions;
  let options = options.unwrap_or_default();
  let filename = options.filename;
  let source_map = options.source_map;
  let source_type = options.source_type;
  let allocator = Allocator::default();
  let mut program = Parser::new(&allocator, source, source_type).parse().program;
  let context = TransformContext::new(&allocator, &options);
  JsxTraverse::new(&allocator, &context).traverse(&mut program);
  Codegen::new()
    .with_options(CodegenOptions {
      source_map_path: if source_map {
        Some(PathBuf::from(&filename))
      } else {
        None
      },
      indent_width: 2,
      indent_char: IndentChar::Space,
      ..CodegenOptions::default()
    })
    .build(&program)
}
