use std::{borrow::Cow, cell::RefCell, mem, rc::Rc};

use napi::{Either, bindgen_prelude::Either3};
use oxc_allocator::{FromIn, TakeIn};
use oxc_ast::ast::{
  Expression, JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild,
  JSXElement, JSXElementName, JSXExpression,
};
use oxc_span::{GetSpan, Ident, SPAN, Span};

use crate::{
  ir::{
    component::{IRProp, IRProps, IRPropsDynamicExpression, IRPropsStatic},
    index::{
      BlockIRNode, CreateComponentIRNode, DirectiveIRNode, DynamicFlag, OperationNode,
      SetBlockKeyIRNode, SetDynamicEventsIRNode, SetDynamicPropsIRNode, SetPropIRNode,
    },
  },
  transform::{
    DirectiveTransformResult, TransformContext, transform_key::resolve_static_key,
    transform_slot_outlet::transform_slot_outlet, transform_transition::transform_transition,
    v_bind::transform_v_bind, v_html::transform_v_html, v_model::transform_v_model,
    v_on::transform_v_on, v_show::transform_v_show, v_text::transform_v_text,
  },
};

use common::{
  ast::RootNode,
  check::{
    get_directive_name, is_always_close_tag, is_block_tag, is_built_in_directive,
    is_formatting_tag, is_reserved_prop, is_template, is_void_tag,
  },
  directive::{Directives, resolve_directive, resolve_prop_name},
  dom::is_valid_html_nesting,
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  text::{camelize, get_tag_name, get_text_like_value},
};

/// # SAFETY
pub unsafe fn transform_element<'a>(
  directives: &mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_template(node)
    && (directives.v_if.is_some()
      || directives.v_else_if.is_some()
      || directives.v_else.is_some()
      || directives.v_for.is_some()
      || directives.v_slot.is_some())
  {
    return None;
  }
  let mut effect_index = context_block.effect.len() as i32;
  let get_effect_index = Rc::new(RefCell::new(Box::new(move || {
    let current = effect_index;
    effect_index += 1;
    current
  }) as Box<dyn FnMut() -> i32>));
  let mut operation_index = context_block.operation.len() as i32;
  let get_operation_index = Rc::new(RefCell::new(Box::new(move || {
    let current = operation_index;
    operation_index += 1;
    current
  }) as Box<dyn FnMut() -> i32>));

  let tag = directives.tag_name;
  let tag_span = node.opening_element.name.span();
  if tag == "slot" {
    return unsafe {
      transform_slot_outlet(
        directives,
        context_node,
        context,
        context_block,
        parent_node,
        get_effect_index,
        get_operation_index,
      )
    };
  } else if matches!(tag, "VaporTransition" | "VaporTransitionGroup") {
    transform_transition(node, context);
  }
  // treat custom elements as components because the template helper cannot
  // resolve them properly; they require creation via createElement
  let is_custom_element = directives.is_custom_element;
  let is_component = directives.is_component;

  let static_key = resolve_static_key(directives, context);

  // If the element is a component, we need to isolate its slots context.
  // This ensures that slots defined for this component are not accidentally
  // inherited by its children components.
  let mut parent_slots = None;
  if is_component {
    parent_slots = Some(context_block.slots.drain(..).collect::<Vec<_>>());
  }

  let context_block_ptr = context_block as *mut BlockIRNode;
  let props_result = build_props(
    directives,
    node,
    parent_node,
    context,
    unsafe { &mut *context_block_ptr },
    is_component,
    false,
    Rc::clone(&get_effect_index),
    Rc::clone(&get_operation_index),
  );

  let single_root = RootNode::is_single_root(parent_node);

  Some(Box::new(move || {
    if is_component {
      transform_component_element(
        tag,
        tag_span,
        props_result,
        static_key,
        single_root,
        is_custom_element,
        context,
        context_block,
        Rc::clone(&get_operation_index),
      );
    } else {
      transform_native_element(
        tag,
        props_result,
        static_key,
        single_root,
        context,
        context_block,
        parent_node,
        Rc::clone(&get_effect_index),
        Rc::clone(&get_operation_index),
      );
    }

    if let Some(parent_slots) = parent_slots {
      unsafe { &mut *context_block_ptr }.slots = parent_slots;
    }
  }))
}

// keys cannot be a part of the template and need to be set dynamically
static DYNAMIC_KEYS: [&str; 1] = ["indeterminate"];

#[allow(clippy::too_many_arguments)]
pub fn transform_native_element<'a>(
  tag: &'a str,
  props_result: PropsResult<'a>,
  static_key: Option<Expression<'a>>,
  single_root: bool,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
  get_effect_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) {
  let mut template = format!("<{tag}");

  match props_result.props {
    Either::A(props) => {
      let element = context.reference(&mut context_block.dynamic);
      /* dynamic props */
      context.register_effect(
        context_block,
        false,
        OperationNode::SetDynamicProps(SetDynamicPropsIRNode {
          set_dynamic_props: true,
          props,
          element,
          tag,
        }),
        Some(get_effect_index),
        Some(Rc::clone(&get_operation_index)),
      )
    }
    Either::B(props) => {
      // tracks if previous attribute was quoted, allowing space omission
      // e.g. `class="foo"id="bar"` is valid, `class=foo id=bar` needs space
      let mut prev_was_quoted = false;
      for prop in props {
        let values = &prop.values;
        if let Expression::StringLiteral(key) = &prop.key
          && values.len() == 1
          && let Some(Expression::StringLiteral(first_value)) = values.first()
          && !DYNAMIC_KEYS.contains(&key.value.as_str())
        {
          if !prev_was_quoted {
            template += " "
          }
          let value = first_value.value;
          template += &key.value;

          if !value.is_empty() {
            // The attribute value can remain unquoted if it doesn't contain ASCII whitespace
            // or any of " ' ` = < or >.
            // https://html.spec.whatwg.org/multipage/introduction.html#intro-early-example
            prev_was_quoted = value.contains(|c: char| {
              c.is_whitespace() || matches!(c, '"' | '\'' | '`' | '=' | '<' | '>')
            });
            template += &if prev_was_quoted {
              format!(r#"="{}""#, value.replace("\"", "&quot;"))
            } else {
              format!("={}", value)
            };
          } else {
            prev_was_quoted = false;
          }
        } else {
          let element = context.reference(&mut context_block.dynamic);
          context.register_effect(
            context_block,
            context.is_operation(values.iter().collect()),
            OperationNode::SetProp(SetPropIRNode {
              set_prop: true,
              prop,
              element,
              tag,
            }),
            Some(Rc::clone(&get_effect_index)),
            Some(Rc::clone(&get_operation_index)),
          );
        }
      }
    }
  }

  template += &format!(">{}", context.children_template.borrow().join(""));
  if !is_void_tag(tag) && !can_omit_end_tag(tag, parent_node, context) {
    template += &format!("</{}>", tag)
  }

  if single_root {
    let ir = &mut context.ir.borrow_mut();
    ir.root_template_index = Some(context.options.templates.borrow().len())
  }

  if let JSXChild::Element(parent_node) = parent_node
    && let JSXElementName::Identifier(name) = &parent_node.opening_element.name
    && !is_valid_html_nesting(&name.name, tag)
  {
    let dynamic = &mut context_block.dynamic;
    context.reference(dynamic);
    dynamic.template = Some(context.push_template(template, Some(tag)));
    dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  } else {
    *context.template.borrow_mut() = format!("{}{}", context.template.borrow(), template);
  }

  if let Some(static_key) = static_key {
    let dynamic = &mut context_block.dynamic;
    let element = context.reference(dynamic);
    context.register_operation(
      context_block,
      OperationNode::SetBlockKey(SetBlockKeyIRNode {
        element,
        value: static_key,
      }),
      Some(Rc::clone(&get_operation_index)),
    );
  }
}

fn can_omit_end_tag<'a>(
  tag: &str,
  parent_node: &JSXChild<'a>,
  context: &TransformContext<'a>,
) -> bool {
  // Root-level elements generate dedicated templates
  // so closing tags can be omitted
  if RootNode::is_single_root(parent_node) {
    return true;
  }

  // Elements in the alwaysClose list cannot have their end tags omitted
  // unless they are on the rightmost path.
  if is_always_close_tag(tag) && !*context.is_on_rightmost_path.borrow() {
    return false;
  }

  // Formatting tags and same-name nested tags require explicit closing
  // unless on the rightmost path of the tree:
  // - Formatting tags: https://html.spec.whatwg.org/multipage/parsing.html#reconstruct-the-active-formatting-elements
  // - Same-name tags: parent's close tag would incorrectly close the child
  if is_formatting_tag(tag)
    || if let JSXChild::Element(parent_node) = parent_node {
      get_tag_name(parent_node, context.options) == tag
    } else {
      false
    }
  {
    return *context.is_on_rightmost_path.borrow();
  }

  // For inline element containing block element, if the inline ancestor
  // is not on rightmost path, the block must close to avoid parsing issues
  if is_block_tag(tag) && *context.has_inline_ancestor_needing_close.borrow() {
    return false;
  }

  *context.is_last_effective_child.borrow()
}

#[allow(clippy::too_many_arguments)]
pub fn transform_component_element<'a>(
  tag: &'a str,
  tag_span: Span,
  props_result: PropsResult<'a>,
  static_key: Option<Expression<'a>>,
  single_root: bool,
  is_custom_element: bool,
  context: &'a TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) {
  let dynamic = &mut context_block.dynamic;
  dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  let id = context.reference(dynamic);
  dynamic.operation = Some(Box::new(OperationNode::CreateComponent(
    CreateComponentIRNode {
      create_component: true,
      id,
      tag,
      tag_span,
      props: match props_result.props {
        Either::A(props) => props,
        Either::B(props) => vec![Either3::A(props)],
      },
      asset: false,
      root: single_root,
      slots: mem::take(&mut context_block.slots),
      once: *context.in_v_once.borrow(),
      is_custom_element,
      parent: None,
      anchor: None,
      logical_index: None,
      append: false,
      last: false,
    },
  )));

  if let Some(static_key) = static_key {
    context.register_operation(
      context_block,
      OperationNode::SetBlockKey(SetBlockKeyIRNode {
        element: id,
        value: static_key,
      }),
      Some(Rc::clone(&get_operation_index)),
    );
  }
}

pub struct PropsResult<'a> {
  pub dynamic: bool,
  pub props: Either<Vec<IRProps<'a>>, IRPropsStatic<'a>>,
  pub name_prop: Option<&'a mut JSXAttribute<'a>>,
}

#[allow(clippy::too_many_arguments)]
pub fn build_props<'a>(
  directives: &Directives<'a>,
  node: &'a mut JSXElement<'a>,
  parent_node: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  is_component: bool,
  collect_name: bool,
  get_effect_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) -> PropsResult<'a> {
  let node = node as *mut JSXElement;
  let props = &mut (unsafe { &mut *node }).opening_element.attributes;
  let mut name_prop = None;
  if props.is_empty() {
    return PropsResult {
      dynamic: false,
      props: Either::B(vec![]),
      name_prop: None,
    };
  }

  let mut dynamic_args: Vec<IRProps> = vec![];
  let mut results: Vec<DirectiveTransformResult> = vec![];

  for prop in props {
    match prop {
      JSXAttributeItem::SpreadAttribute(prop) => {
        let value = prop.argument.take_in(context.allocator);
        if !results.is_empty() {
          dynamic_args.push(Either3::A(dedupe_properties(results)));
          results = vec![];
        }
        dynamic_args.push(Either3::C(IRPropsDynamicExpression {
          value,
          handler: false,
        }));
        continue;
      }
      JSXAttributeItem::Attribute(prop) => {
        if let Some(JSXAttributeValue::ExpressionContainer(value)) = &prop.value
          && matches!(value.expression, JSXExpression::EmptyExpression(_))
        {
          continue;
        }
        let span = prop.span;
        let prop_name = prop.name.get_identifier().name;
        if prop_name.eq("v-on") {
          // v-on={obj}
          if let Some(prop_value) = &mut prop.value {
            let value = jsx_attribute_value_to_expression(prop_value, context.ast);
            if is_component {
              if !results.is_empty() {
                dynamic_args.push(Either3::A(dedupe_properties(results)));
                results = vec![];
              }
              dynamic_args.push(Either3::C(IRPropsDynamicExpression {
                value,
                handler: true,
              }))
            } else {
              let element = context.reference(&mut context_block.dynamic);
              context.register_effect(
                context_block,
                context.is_operation(vec![&value]),
                OperationNode::SetDynamicEvents(SetDynamicEventsIRNode {
                  set_dynamic_events: true,
                  element,
                  value,
                }),
                Some(Rc::clone(&get_effect_index)),
                Some(Rc::clone(&get_operation_index)),
              );
            }
          } else {
            context.options.on_error.as_ref()(ErrorCodes::VOnNoExpression, span);
          }
          continue;
        } else if collect_name && prop_name == "name" {
          name_prop = Some(prop.as_mut());
          continue;
        }

        let context_block = context_block as *mut BlockIRNode;
        if let Some(prop) = transform_prop(
          directives,
          prop,
          unsafe { &mut *node },
          parent_node,
          is_component,
          context,
          unsafe { &mut *context_block },
          Rc::clone(&get_operation_index),
        ) {
          if is_component && !matches!(&prop.key, Expression::StringLiteral(_)) {
            // v-model:$name$="value"
            if !results.is_empty() {
              dynamic_args.push(Either3::A(dedupe_properties(results)));
              results = vec![];
            }
            dynamic_args.push(Either3::B(IRProp {
              key: prop.key,
              modifier: prop.modifier,
              runtime_camelize: prop.runtime_camelize,
              handler: prop.handler,
              handler_modifiers: prop.handler_modifiers,
              model: prop.model,
              model_modifiers: prop.model_modifiers,
              values: vec![prop.value],
              dynamic: true,
            }));
          } else {
            // other static props
            results.push(prop)
          }
        }
      }
    }
  }

  // has dynamic key or {...obj}
  if !dynamic_args.is_empty()
    || results
      .iter()
      .any(|prop| !matches!(&prop.key, Expression::StringLiteral(_)))
  {
    // take rest of props as dynamic props
    if !results.is_empty() {
      dynamic_args.push(Either3::A(dedupe_properties(results)));
    }
    return PropsResult {
      dynamic: true,
      props: Either::A(dynamic_args),
      name_prop,
    };
  }

  PropsResult {
    dynamic: false,
    props: Either::B(dedupe_properties(results)),
    name_prop,
  }
}

#[allow(clippy::too_many_arguments)]
pub fn transform_prop<'a>(
  directives: &Directives<'a>,
  prop: &'a mut JSXAttribute<'a>,
  node: &'a mut JSXElement<'a>,
  parent_node: &mut JSXChild<'a>,
  is_component: bool,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) -> Option<DirectiveTransformResult<'a>> {
  let name = resolve_prop_name(match &prop.name {
    JSXAttributeName::Identifier(name) => name.name.as_str(),
    JSXAttributeName::NamespacedName(name) => name.namespace.name.as_str(),
  })[0];
  let value = if let Some(value) = &prop.value {
    match value {
      JSXAttributeValue::ExpressionContainer(value) => {
        get_text_like_value(value.expression.to_expression(), is_component)
      }
      JSXAttributeValue::StringLiteral(value) => Some(value.value.into()),
      _ => None,
    }
  } else {
    None
  };

  let dir_name_raw = get_directive_name(name);
  if dir_name_raw == "bind" && (prop.value.is_none() || value.is_some()) {
    if is_reserved_prop(name) {
      return None;
    }
    let ast = context.ast;
    return Some(DirectiveTransformResult::new(
      ast.expression_string_literal(SPAN, ast.atom(name), None),
      if let Some(value) = value {
        ast.expression_string_literal(SPAN, ast.atom(&value), None)
      } else {
        ast.expression_boolean_literal(SPAN, true)
      },
    ));
  }

  match dir_name_raw {
    "bind" => return transform_v_bind(prop, context),
    "on" => return transform_v_on(directives, prop, node, context, context_block),
    "model" => return transform_v_model(directives, prop, node, context, context_block),
    "show" => return transform_v_show(prop, context, context_block, parent_node),
    "html" => return transform_v_html(directives, prop, node, context, context_block),
    "text" => return transform_v_text(directives, prop, node, context, context_block),
    _ => (),
  };

  if !is_built_in_directive(dir_name_raw) {
    let mut dir_name = Cow::Borrowed(dir_name_raw);
    let asset = if dir_name_raw
      .chars()
      .nth(1)
      .map(|c| c.is_uppercase())
      .unwrap_or_default()
    {
      false
    } else {
      let semantic = &context.options.semantic.borrow();
      let scope_id = semantic.nodes().get_node(node.node_id()).scope_id();
      let camelize_name = camelize(Cow::Borrowed(name));
      if semantic
        .scoping()
        .find_binding(
          scope_id,
          Ident::from_in(camelize_name.as_ref(), context.allocator),
        )
        .is_some()
      {
        dir_name = camelize_name;
        false
      } else {
        let directives = &mut context.ir.borrow_mut().directives;
        directives.insert(dir_name_raw);
        true
      }
    };

    let element = context.reference(&mut context_block.dynamic);
    context.register_operation(
      context_block,
      OperationNode::Directive(DirectiveIRNode {
        directive: true,
        element,
        dir: resolve_directive(prop, context.ast),
        name: dir_name,
        asset,
        builtin: false,
        model_type: None,
        deferred: false,
      }),
      Some(Rc::clone(&get_operation_index)),
    )
  }
  None
}

// Dedupe props in an object literal.
// Literal duplicated attributes would have been warned during the parse phase,
// however, it's possible to encounter duplicated `onXXX` handlers with different
// modifiers. We also need to merge static and dynamic class / style attributes.
pub fn dedupe_properties(results: Vec<DirectiveTransformResult>) -> Vec<IRProp> {
  let mut deduped = vec![];

  for result in results {
    let prop = IRProp {
      key: result.key,
      modifier: result.modifier,
      runtime_camelize: result.runtime_camelize,
      handler: result.handler,
      handler_modifiers: result.handler_modifiers,
      model: result.model,
      model_modifiers: result.model_modifiers,
      values: vec![result.value],
      dynamic: false,
    };
    // dynamic keys are always allowed
    let Expression::StringLiteral(key) = &prop.key else {
      deduped.push(prop);
      continue;
    };
    let name = &key.value;
    let existing = deduped.iter_mut().find(|i| {
      if let Expression::StringLiteral(key) = &i.key {
        key.value == name
      } else {
        false
      }
    });
    // prop names and event handler names can be the same but serve different purposes
    // e.g. `:appear="true"` is a prop while `@appear="handler"` is an event handler
    if let Some(existing) = existing
      && existing.handler.eq(&prop.handler)
    {
      if name == "style" || name == "class" || prop.handler {
        for value in prop.values {
          existing.values.push(value)
        }
      }
    // unexpected duplicate, should have emitted error during parse
    } else {
      deduped.push(prop);
    }
  }
  deduped
}
