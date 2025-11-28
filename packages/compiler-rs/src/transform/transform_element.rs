use std::{cell::RefCell, rc::Rc};

use napi::{
  Either,
  bindgen_prelude::{Either3, Either16},
};
use oxc_ast::ast::{
  JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild, JSXElement,
  JSXElementName,
};
use oxc_span::SPAN;

use crate::{
  ir::{
    component::{IRProp, IRProps, IRPropsDynamicExpression, IRPropsStatic},
    index::{
      BlockIRNode, CreateComponentIRNode, DirectiveIRNode, DynamicFlag, SetDynamicEventsIRNode,
      SetDynamicPropsIRNode, SetPropIRNode, SimpleExpressionNode,
    },
  },
  transform::{
    ContextNode, DirectiveTransformResult, TransformContext, v_bind::transform_v_bind,
    v_html::transform_v_html, v_model::transform_v_model, v_on::transform_v_on,
    v_show::transform_v_show, v_text::transform_v_text,
  },
  utils::{
    check::{is_build_in_directive, is_jsx_component, is_template, is_void_tag},
    directive::resolve_directive,
    dom::is_valid_html_nesting,
    error::ErrorCodes,
    text::get_text_like_value,
    text::{camelize, get_tag_name},
  },
};

static RESERVED_PROP: [&str; 5] = ["", "key", "ref", "ref_for", "ref_key"];
pub fn is_reserved_prop(name: &str) -> bool {
  RESERVED_PROP.contains(&name)
}

pub fn is_event(s: &str) -> bool {
  s.starts_with("on")
    && s
      .chars()
      .nth(2)
      .map(|c| c.is_ascii_uppercase())
      .unwrap_or(false)
}

pub fn is_directive(s: &str) -> bool {
  s.starts_with("v-")
    && s
      .chars()
      .nth(2)
      .map(|c| c.is_ascii_lowercase())
      .unwrap_or(false)
}

/// # SAFETY
pub unsafe fn transform_element<'a>(
  context_node: *mut ContextNode<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let Either::B(JSXChild::Element(node)) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_template(node) {
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

  let tag = get_tag_name(&node.opening_element.name, context);
  let is_component = is_jsx_component(node);
  let _context_block = context_block as *mut BlockIRNode;
  let props_result = build_props(
    node,
    context,
    unsafe { &mut *_context_block },
    is_component,
    Rc::clone(&get_effect_index),
    Rc::clone(&get_operation_index),
  );

  let single_root = matches!(parent_node, Either::A(parent_node) if parent_node.is_single_root);

  Some(Box::new(move || {
    if is_component {
      transform_component_element(tag, props_result, single_root, context, context_block);
    } else {
      transform_native_element(
        tag,
        props_result,
        single_root,
        context,
        context_block,
        parent_node,
        Rc::clone(&get_effect_index),
        Rc::clone(&get_operation_index),
      );
    }
  }))
}

#[allow(clippy::too_many_arguments)]
pub fn transform_native_element<'a>(
  tag: String,
  props_result: PropsResult<'a>,
  single_root: bool,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut ContextNode<'a>,
  get_effect_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) {
  let mut template = format!("<{tag}");

  let mut dynamic_props = vec![];

  match props_result.props {
    Either::A(props) => {
      let element = context.reference(&mut context_block.dynamic);
      /* dynamic props */
      context.register_effect(
        context_block,
        false,
        Either16::E(SetDynamicPropsIRNode {
          set_dynamic_props: true,
          props,
          element,
          root: single_root,
        }),
        Some(get_effect_index),
        Some(get_operation_index),
      )
    }
    Either::B(props) => {
      for prop in props {
        let key = &prop.key;
        let values = &prop.values;
        if key.is_static && values.len() == 1 && values[0].is_static {
          template += &format!(" {}", key.content);
          if !values[0].content.is_empty() {
            template += &format!("=\"{}\"", values[0].content);
          }
        } else {
          dynamic_props.push(key.content.clone());

          let element = context.reference(&mut context_block.dynamic);
          context.register_effect(
            context_block,
            context.is_operation(values.iter().collect::<Vec<&SimpleExpressionNode>>()),
            Either16::D(SetPropIRNode {
              set_prop: true,
              prop,
              element,
              tag: tag.clone(),
              root: single_root,
            }),
            Some(Rc::clone(&get_effect_index)),
            Some(Rc::clone(&get_operation_index)),
          );
        }
      }
    }
  }

  template += &format!(">{}", context.children_template.borrow().join(""));
  // TODO remove unnecessary close tag, e.g. if it's the last element of the template
  if !is_void_tag(&tag) {
    template += &format!("</{}>", tag)
  }

  if single_root {
    let ir = &mut context.ir.borrow_mut();
    ir.root_template_index = Some(context.options.templates.borrow().len())
  }

  if let Either::B(JSXChild::Element(parent_node)) = parent_node
    && let JSXElementName::Identifier(name) = &parent_node.opening_element.name
    && !is_valid_html_nesting(&name.name, &tag)
  {
    let dynamic = &mut context_block.dynamic;
    context.reference(dynamic);
    dynamic.template = Some(context.push_template(template));
    dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  } else {
    *context.template.borrow_mut() = format!("{}{}", context.template.borrow(), template);
  }
}

pub fn transform_component_element<'a>(
  mut tag: String,
  props_result: PropsResult<'a>,
  single_root: bool,
  context: &'a TransformContext<'a>,
  context_block: &mut BlockIRNode<'a>,
) {
  let mut asset = context.options.with_fallback;

  if let Some(dot_index) = tag.find('.') {
    let ns = tag[0..dot_index].to_string();
    if !ns.is_empty() {
      tag = ns + &tag[dot_index..];
    }
  }

  if tag.contains("-") {
    asset = true
  }

  if asset {
    let component = &mut context.ir.borrow_mut().component;
    component.insert(tag.clone());
  }

  let dynamic = &mut context_block.dynamic;
  dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;

  dynamic.operation = Some(Box::new(Either16::N(CreateComponentIRNode {
    create_component: true,
    id: context.reference(dynamic),
    tag,
    props: match props_result.props {
      Either::A(props) => props,
      Either::B(props) => vec![Either3::A(props)],
    },
    asset,
    root: single_root && *context.in_v_for.borrow() == 0,
    slots: context.slots.take(),
    once: *context.in_v_once.borrow(),
    parent: None,
    anchor: None,
    dynamic: None,
  })));
}

pub struct PropsResult<'a> {
  pub dynamic: bool,
  pub props: Either<Vec<IRProps<'a>>, IRPropsStatic<'a>>,
}

pub fn build_props<'a>(
  node: &'a mut JSXElement<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  is_component: bool,
  get_effect_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) -> PropsResult<'a> {
  let node = node as *mut JSXElement;
  let props = &mut (unsafe { &mut *node }).opening_element.attributes;
  if props.is_empty() {
    return PropsResult {
      dynamic: false,
      props: Either::B(vec![]),
    };
  }

  let mut dynamic_args: Vec<IRProps> = vec![];
  let mut results: Vec<DirectiveTransformResult> = vec![];

  for prop in props {
    match prop {
      JSXAttributeItem::SpreadAttribute(prop) => {
        let value = SimpleExpressionNode::new(Either3::A(&mut prop.argument), context);
        if !results.is_empty() {
          dynamic_args.push(Either3::A(dedupe_properties(results)));
          results = vec![];
        }
        dynamic_args.push(Either3::C(IRPropsDynamicExpression {
          value,
          handler: None,
        }));
        continue;
      }
      JSXAttributeItem::Attribute(prop) => {
        let span = prop.span;
        if prop.name.get_identifier().name.eq("v-on") {
          // v-on={obj}
          if let Some(prop_value) = &mut prop.value {
            let value = SimpleExpressionNode::new(Either3::C(prop_value), context);
            if is_component {
              if !results.is_empty() {
                dynamic_args.push(Either3::A(dedupe_properties(results)));
                results = vec![];
              }
              dynamic_args.push(Either3::C(IRPropsDynamicExpression {
                value,
                handler: Some(true),
              }))
            } else {
              let element = context.reference(&mut context_block.dynamic);
              context.register_effect(
                context_block,
                context.is_operation(vec![&value]),
                Either16::F(SetDynamicEventsIRNode {
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
        }

        let context_block = context_block as *mut BlockIRNode;
        if let Some(prop) = transform_prop(
          prop,
          unsafe { &mut *node },
          is_component,
          context,
          unsafe { &mut *context_block },
          Rc::clone(&get_operation_index),
        ) {
          if is_component && !prop.key.is_static {
            // v-model:&name&="value"
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
  if !dynamic_args.is_empty() || results.iter().any(|prop| !prop.key.is_static) {
    // take rest of props as dynamic props
    if !results.is_empty() {
      dynamic_args.push(Either3::A(dedupe_properties(results)));
    }
    return PropsResult {
      dynamic: true,
      props: Either::A(dynamic_args),
    };
  }

  PropsResult {
    dynamic: false,
    props: Either::B(dedupe_properties(results)),
  }
}

pub fn transform_prop<'a>(
  prop: &'a mut JSXAttribute<'a>,
  node: &'a mut JSXElement<'a>,
  is_component: bool,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  get_operation_index: Rc<RefCell<Box<dyn FnMut() -> i32 + 'a>>>,
) -> Option<DirectiveTransformResult<'a>> {
  let name = match &prop.name {
    JSXAttributeName::Identifier(name) => name.name.as_str(),
    JSXAttributeName::NamespacedName(name) => name.namespace.name.as_str(),
  }
  .split("_")
  .collect::<Vec<&str>>()[0];
  let value = if let Some(value) = &prop.value {
    match value {
      JSXAttributeValue::ExpressionContainer(value) => {
        get_text_like_value(value.expression.to_expression(), Some(is_component))
      }
      JSXAttributeValue::StringLiteral(value) => Some(value.value.to_string()),
      _ => None,
    }
  } else {
    None
  };
  if !is_directive(name) && !is_event(name) && (prop.value.is_none() || value.is_some()) {
    if is_reserved_prop(name) {
      return None;
    }
    return Some(DirectiveTransformResult::new(
      SimpleExpressionNode {
        content: name.to_string(),
        is_static: true,
        ast: None,
        loc: SPAN,
      },
      if let Some(value) = value {
        SimpleExpressionNode {
          content: value,
          is_static: true,
          ast: None,
          loc: SPAN,
        }
      } else {
        SimpleExpressionNode {
          content: "true".to_string(),
          is_static: false,
          ast: None,
          loc: SPAN,
        }
      },
    ));
  }

  let mut name = if is_event(name) {
    "on".to_string()
  } else if is_directive(name) {
    name[2..].to_string()
  } else {
    "bind".to_string()
  };

  match name.as_str() {
    "bind" => return transform_v_bind(prop, node, context, context_block),
    "on" => return transform_v_on(prop, node, context, context_block),
    "model" => return transform_v_model(prop, node, context, context_block),
    "show" => return transform_v_show(prop, node, context, context_block),
    "html" => return transform_v_html(prop, node, context, context_block),
    "text" => return transform_v_text(prop, node, context, context_block),
    _ => (),
  };

  if !is_build_in_directive(&name) {
    let with_fallback = context.options.with_fallback;
    if with_fallback {
      let directive = &mut context.ir.borrow_mut().directive;
      directive.insert(name.clone());
    } else {
      name = camelize(&format!("v-{name}"))
    };

    let element = context.reference(&mut context_block.dynamic);
    context.register_operation(
      context_block,
      Either16::M(DirectiveIRNode {
        directive: true,
        element,
        dir: resolve_directive(prop, context),
        name,
        asset: Some(with_fallback),
        builtin: None,
        model_type: None,
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
    if !prop.key.is_static {
      deduped.push(prop);
      continue;
    }
    let name = prop.key.content.as_str();
    let existing = deduped.iter_mut().find(|i| i.key.content == name);
    if let Some(existing) = existing {
      if name == "style" || name == "class" {
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
