use napi::{Either, bindgen_prelude::Either3};
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  AstBuilder, NONE,
  ast::{
    ArrayExpression, Expression, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild,
    JSXElement, ObjectProperty, ObjectPropertyKind, PropertyKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{NodeTypes, RootNode, VNodeCall},
  transform::{
    DirectiveTransformResult, TransformContext, cache_static::get_constant_type,
    v_bind::transform_v_bind, v_html::transform_v_html, v_model::transform_v_model,
    v_on::transform_v_on, v_show::transform_v_show, v_slot::build_slots, v_text::transform_v_text,
  },
};

use common::{
  check::{
    is_built_in_directive, is_directive, is_event, is_jsx_component, is_reserved_prop, is_template,
  },
  directive::{DirectiveNode, find_prop, resolve_directive},
  error::ErrorCodes,
  expression::parse_expression,
  patch_flag::PatchFlags,
  text::{camelize, get_tag_name, is_empty_text, to_valid_asset_id},
};

/// # SAFETY
/// generate a JavaScript AST for this element's codegen
pub unsafe fn transform_element<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let ast = &context.ast;
  // <></> => <Fragment></Fragment>
  if let JSXChild::Fragment(node) = unsafe { &mut *context_node }
    // skip v-if / v-for generated fragment
    && node.span.end > node.span.start
  {
    let name = ast.jsx_element_name_identifier(SPAN, ast.atom(&context.helper("Fragment")));
    *unsafe { &mut *context_node } = ast.jsx_child_element(
      node.span,
      ast.jsx_opening_element(
        node.opening_fragment.span,
        name.clone_in(context.allocator),
        NONE,
        ast.vec(),
      ),
      node.children.take_in(context.allocator),
      Some(ast.jsx_closing_element(node.closing_fragment.span, name)),
    )
  };

  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_template(node)
    && find_prop(
      node,
      Either::B(vec![
        String::from("v-if"),
        String::from("v-else-if"),
        String::from("v-else"),
        String::from("v-for"),
        String::from("v-slot"),
      ]),
    )
    .is_some()
  {
    return None;
  }

  // The goal of the transform is to create a codegenNode implementing the
  // VNodeCall interface.
  let mut vnode_tag = get_tag_name(&node.opening_element.name, *context.source.borrow());
  let is_custom_element = context.options.is_custom_element.as_ref()(vnode_tag.clone());
  let is_component = is_jsx_component(node) && !is_custom_element;
  if !is_custom_element
    && (is_component
      && ((context.options.with_fallback
        && !context.options.helpers.borrow().contains(
          if let Some(vnode_tag) = vnode_tag.strip_prefix("_") {
            vnode_tag
          } else {
            &vnode_tag
          },
        ))
        || vnode_tag.contains("-")))
  {
    context.helper("resolveComponent");
    context.components.borrow_mut().insert(vnode_tag.clone());
    vnode_tag = to_valid_asset_id(&vnode_tag, "component");
  }

  let mut should_use_block = RootNode::is_single_root(parent_node)
    || RootNode::is_fragment(parent_node)
    || vnode_tag == "Teleport"
    || vnode_tag == "Suspense"
    || (!is_component
      // <svg> and <foreignObject> must be forced into blocks so that block
      // updates inside get proper isSVG flag at runtime. (#639, #643)
      // This is technically web-specific, but splitting the logic out of core
      // leads to too much unnecessary complexity.
      && (vnode_tag == "svg" || vnode_tag == "foreignObject" || vnode_tag == "math"));

  let _node = node as *mut oxc_allocator::Box<JSXElement>;
  let props_build_result = build_props(unsafe { &mut *_node }, context, is_component);

  let vnode_props = props_build_result.props;
  let mut vnode_children = None;
  let mut patch_flag = props_build_result.patch_flag;
  let dynamic_prop_names = props_build_result.dynamic_prop_names;
  let vnode_directives = props_build_result.directives;
  if props_build_result.should_use_block {
    should_use_block = true;
  }

  // perform the work on exit, after all child expressions have been
  // processed and merged.
  Some(Box::new(move || {
    // children
    let node_span = node.span;
    let children = &mut node
      .children
      .iter_mut()
      .filter(|child| !is_empty_text(child))
      .collect::<Vec<_>>();
    if !children.is_empty() {
      if vnode_tag == "KeepAlive" || vnode_tag == "keep-alive" {
        // Although a built-in component, we compile KeepAlive with raw children
        // instead of slot functions so that it can be used inside Transition
        // or other Transition-wrapping HOCs.
        // To ensure correct updates with block optimizations, we need to:
        // 1. Force keep-alive into a block. This avoids its children being
        //    collected by a parent block.
        should_use_block = true;
        // 2. Force keep-alive to always be updated, since it uses raw children.
        patch_flag |= PatchFlags::DynamicSlots as i32;
        if children.len() > 1 {
          context.options.on_error.as_ref()(
            ErrorCodes::KeepAliveInvalidChildren,
            Span::new(
              children.first().unwrap().span().start,
              children.last().unwrap().span().end,
            ),
          );
        }
      }

      let should_build_as_slots = is_component
      && vnode_tag != "Teleport" // Teleport is not a real component and has dedicated runtime handling
      && (vnode_tag != "KeepAlive" || vnode_tag != "keep-alive"); // explained above.

      if vnode_tag == "Fragment" || vnode_tag == "_Fragment" {
        patch_flag |= PatchFlags::StableFragment as i32;
      }

      vnode_children = Some(if should_build_as_slots {
        let (slots, has_dynamic_slots) = build_slots(node, context);
        if has_dynamic_slots {
          patch_flag |= PatchFlags::DynamicSlots as i32
        }
        Either3::C(slots)
      } else if children.len() == 1 && vnode_tag != "Teleport" {
        let child = children.get_mut(0).unwrap();
        // check for dynamic text children
        let has_dynamic_text_child = child.is_expression_container();
        // pass directly if the only child is a text node
        // (plain / interpolation / expression)
        if has_dynamic_text_child || matches!(child, JSXChild::Text(_)) {
          Either3::A(*child as *mut _)
        } else {
          Either3::B(&mut node.children as *mut _)
        }
      } else {
        Either3::B(&mut node.children as *mut _)
      });
    }

    // patchFlag & dynamicPropNames
    let vnode_dynamic_props = if !dynamic_prop_names.is_empty() {
      Some(ast.expression_array(
        SPAN,
        ast.vec_from_iter(dynamic_prop_names.into_iter().map(|name| {
          if name.starts_with("\"") {
            ast.expression_identifier(SPAN, ast.atom(&name))
          } else {
            ast.expression_string_literal(SPAN, ast.atom(&name), None)
          }
          .into()
        })),
      ))
    } else {
      None
    };

    let vnode_call = VNodeCall {
      tag: vnode_tag,
      props: vnode_props,
      children: vnode_children,
      patch_flag: if RootNode::is_fragment(parent_node) {
        Some(PatchFlags::StableFragment as i32)
      } else if patch_flag == 0 {
        None
      } else {
        Some(patch_flag)
      },
      dynamic_props: vnode_dynamic_props,
      directives: vnode_directives,
      is_block: should_use_block,
      disable_tracking: false,
      is_component,
      v_for: false,
      v_if: None,
      loc: SPAN,
    };
    context
      .codegen_map
      .borrow_mut()
      .insert(node_span, NodeTypes::VNodeCall(vnode_call));
  }))
}

pub struct PropsResult<'a> {
  pub props: Option<Expression<'a>>,
  pub directives: Option<ArrayExpression<'a>>,
  pub patch_flag: i32,
  pub dynamic_prop_names: Vec<String>,
  pub should_use_block: bool,
}

pub fn build_props<'a>(
  node: &'a mut JSXElement<'a>,
  context: &'a TransformContext<'a>,
  is_component: bool,
) -> PropsResult<'a> {
  let ast = &context.ast;
  let _node = node as *mut JSXElement;
  let props = &mut (unsafe { &mut *_node }).opening_element.attributes;
  if props.is_empty() {
    return PropsResult {
      props: None,
      directives: None,
      patch_flag: 0,
      dynamic_prop_names: vec![],
      should_use_block: false,
    };
  }

  let mut properties: oxc_allocator::Vec<ObjectPropertyKind> = ast.vec();
  let mut merge_args: oxc_allocator::Vec<Expression> = ast.vec();
  let mut runtime_directives = vec![];
  let has_children = !node.children.is_empty();
  let mut should_use_block = false;

  // patchFlag analysis
  let mut patch_flag = 0;
  let mut has_ref = false;
  let mut has_class_binding = false;
  let mut has_style_binding = false;
  let mut has_hydration_event_binding = false;
  let mut has_dynamic_keys = false;
  let mut has_vnode_hook = false;
  let mut dynamic_prop_names = vec![];

  // mark template ref on v-for
  let ref_v_for_marker = || -> Option<ObjectPropertyKind> {
    if *context.options.in_v_for.borrow() > 0 {
      Some(ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, "ref_for"),
        ast.expression_boolean_literal(SPAN, true),
        false,
        false,
        false,
      ))
    } else {
      None
    }
  };

  let _has_ref = &mut has_ref as *mut _;
  let _has_dynamic_keys = &mut has_dynamic_keys as *mut _;
  let mut analyze_patch_flag = |prop: &ObjectPropertyKind| {
    let ObjectPropertyKind::ObjectProperty(prop) = prop else {
      return;
    };
    let ObjectProperty { key, computed, .. } = prop.as_ref();
    let mut value = &prop.value;
    if !computed {
      let name = key.name().map(|name| name.to_string()).unwrap_or_default();
      let is_event_handler = is_event(&name);
      if is_event_handler && !is_component &&
      // omit the flag for click handlers because hydration gives click
      // dedicated fast path.
      name.to_lowercase() != "onclick" &&
      // omit v-model handlers
      name != "onUpdate:modelValue" &&
      // omit onVnodeXXX hooks
      !is_reserved_prop(&name)
      {
        has_hydration_event_binding = true
      }

      if is_event_handler && is_reserved_prop(&name) {
        has_vnode_hook = true
      }

      if is_event_handler
        && let Expression::CallExpression(call_expr) = value
        && let Some(arg) = call_expr.arguments.first()
      {
        // handler wrapped with internal helper e.g. withModifiers(fn)
        // extract the actual expression
        value = arg.to_expression();
      }

      if matches!(value, Expression::LogicalExpression(value) if value.span() == SPAN)
        || (get_constant_type(
          Either::B(value),
          context,
          &mut context.codegen_map.borrow_mut(),
        ) as i32)
          > 0
      {
        // skip if the prop is a cached handler or has constant values
        return;
      }

      if name == "ref" {
        *unsafe { &mut *_has_ref } = true;
      } else if name == "class" {
        has_class_binding = true;
      } else if name == "style" {
        has_style_binding = true;
      } else if name != "key" && !dynamic_prop_names.contains(&name) {
        dynamic_prop_names.push(name.clone());
      }

      // treat the dynamic class and style binding of the component as dynamic props
      if is_component && (name == "class" || name == "style") && !dynamic_prop_names.contains(&name)
      {
        dynamic_prop_names.push(name);
      }
    } else {
      *unsafe { &mut *_has_dynamic_keys } = true;
    }
  };

  let properties = &mut properties;
  for prop in props {
    // static attribute
    match prop {
      JSXAttributeItem::Attribute(prop) => {
        let ast = &context.ast;
        let name_splited = match &prop.name {
          JSXAttributeName::Identifier(name) => name.name.as_str(),
          JSXAttributeName::NamespacedName(name) => name.namespace.name.as_str(),
        }
        .split("_")
        .collect::<Vec<_>>();
        let Some((name, modifiers)) = name_splited.split_first() else {
          unreachable!()
        };
        if !is_directive(name) && !is_event(name) && *name == "ref" {
          has_ref = true;
          if let Some(marker) = ref_v_for_marker() {
            properties.push(marker)
          };
        }

        let dir_name = if is_event(name) {
          "on".to_string()
        } else if is_directive(name) {
          name[2..].to_string()
        } else {
          "bind".to_string()
        };

        if dir_name == "on" {
          // skip v-on in SSR compilation
          if *context.options.ssr.borrow() {
            continue;
          }

          if prop.name.get_identifier().name == "v-on" {
            has_dynamic_keys = true;
            if let Some(value) = &mut prop.value {
              // v-on={obj} -> toHandlers(obj)
              if !properties.is_empty() {
                merge_args
                  .push(ast.expression_object(node.span, dedupe_properties(properties, ast)));
              }
              let mut args = ast.vec1(context.jsx_attribute_value_to_expression(value).into());
              if !is_component {
                args.push(ast.expression_boolean_literal(SPAN, true).into())
              }
              merge_args.push(ast.expression_call(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&context.helper("toHandlers"))),
                NONE,
                args,
                false,
              ));
            } else {
              context.options.on_error.as_ref()(ErrorCodes::VOnNoExpression, prop.span);
            }
            continue;
          }
        }

        if let Some(DirectiveTransformResult { props, runtime }) = match dir_name.as_str() {
          "bind" => {
            // #938: elements with dynamic keys should be forced into blocks
            if *name == "key"
              && !prop
                .value
                .as_ref()
                .map(|value| matches!(value, JSXAttributeValue::StringLiteral(_)))
                .unwrap_or_default()
            {
              should_use_block = true
            }
            // force hydration for prop with .prop modifier
            if modifiers.contains(&"prop") {
              patch_flag |= PatchFlags::NeedHydration as i32;
            }
            transform_v_bind(prop, node, context)
          }
          "on" => {
            // inline before-update hooks need to force block so that it is invoked
            // before children
            if has_children && *name == "onVue:beforeUpdate" {
              should_use_block = true;
            }
            transform_v_on(prop, node, context)
          }
          "model" => transform_v_model(prop, node, context),
          "show" => transform_v_show(prop, context),
          "html" => transform_v_html(prop, node, context),
          "text" => transform_v_text(prop, node, context),
          _ => {
            if !is_built_in_directive(&dir_name) {
              let runtime = if context.options.with_fallback {
                // inject statement for resolving directive
                context.helper("resolveDirective");
                context.directives.borrow_mut().insert(dir_name.clone());
                to_valid_asset_id(&dir_name, "directive")
              } else {
                camelize(&format!("v-{dir_name}"))
              };
              runtime_directives.push(
                build_directive_args(
                  resolve_directive(prop, *context.source.borrow()),
                  context,
                  &runtime,
                )
                .into(),
              );
              // custom dirs may use beforeUpdate so they need to force blocks
              // to ensure before-update gets called before children update
              if has_children {
                should_use_block = true;
              }
            }
            None
          }
        } {
          if !*context.options.ssr.borrow() {
            props.iter().for_each(&mut analyze_patch_flag);
          }
          properties.extend(props);
          if let Some(runtime) = runtime {
            runtime_directives.push(runtime.into());
          }
        };
      }
      JSXAttributeItem::SpreadAttribute(prop) => {
        has_dynamic_keys = true;
        // #10696 in case a {...obj} object contains ref
        if let Some(marker) = ref_v_for_marker() {
          properties.push(marker)
        };
        if !properties.is_empty() {
          merge_args.push(ast.expression_object(node.span, dedupe_properties(properties, ast)));
        }
        merge_args.push(prop.argument.take_in(ast.allocator));
      }
    }
  }

  // has {...object} or v-on={object}, wrap with mergeProps
  let mut props_expression = if !merge_args.is_empty() {
    // close up any not-yet-merged props
    if !properties.is_empty() {
      merge_args.push(ast.expression_object(node.span, dedupe_properties(properties, ast)));
    }
    if merge_args.len() > 1 {
      Some(ast.expression_call(
        node.span,
        ast.expression_identifier(SPAN, ast.atom(&context.helper("mergeProps"))),
        NONE,
        ast.vec_from_iter(merge_args.into_iter().map(|arg| arg.into())),
        false,
      ))
    } else {
      // no need for a mergeProps call
      Some(merge_args.remove(0))
    }
  } else if !properties.is_empty() {
    Some(ast.expression_object(node.span, dedupe_properties(properties, ast)))
  } else {
    None
  };

  // patchFlag analysis
  if has_dynamic_keys {
    patch_flag |= PatchFlags::FullProps as i32;
  } else {
    if has_class_binding && !is_component {
      patch_flag |= PatchFlags::Class as i32;
    }
    if has_style_binding && !is_component {
      patch_flag |= PatchFlags::Style as i32;
    }
    if !dynamic_prop_names.is_empty() {
      patch_flag |= PatchFlags::Props as i32;
    }
    if has_hydration_event_binding {
      patch_flag |= PatchFlags::NeedHydration as i32;
    }
  }
  if !should_use_block
    && (patch_flag == 0 || patch_flag == PatchFlags::NeedHydration as i32)
    && (has_ref || has_vnode_hook || !runtime_directives.is_empty())
  {
    patch_flag |= PatchFlags::NeedPatch as i32;
  }

  // pre-normalize props, SSR is skipped for now
  if !context.options.in_ssr
    && let Some(props_expression) = &mut props_expression
  {
    match props_expression {
      Expression::ObjectExpression(object_expression) => {
        // means that there is no v-bind,
        // but still need to deal with dynamic key binding
        let mut class_prop = None;
        let mut style_prop = None;
        let mut has_dynamic_key = false;

        for property in &mut object_expression.properties {
          if let ObjectPropertyKind::ObjectProperty(property) = property {
            let key = &property.key;
            let name = key.name().map(|n| n.to_string()).unwrap_or(String::new());
            if !property.computed {
              if name == "class" {
                class_prop = Some(property);
              } else if name == "style" {
                style_prop = Some(property);
              }
            } else if !is_event(&name) {
              has_dynamic_key = true;
            }
          }
        }

        // no dynamic key
        if !has_dynamic_key {
          if let Some(class_prop) = class_prop
            && !matches!(class_prop.value, Expression::StringLiteral(_))
          {
            class_prop.value = ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("normalizeClass"))),
              NONE,
              ast.vec1(class_prop.value.take_in(ast.allocator).into()),
              false,
            )
          }
          if let Some(style_prop) = style_prop
            // the static style is compiled into an object,
            // so use `hasStyleBinding` to ensure that it is a dynamic style binding
            && (has_style_binding || matches!(style_prop.value, Expression::ArrayExpression(_)))
          {
            style_prop.value = ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("normalizeStyle"))),
              NONE,
              ast.vec1(style_prop.value.take_in(ast.allocator).into()),
              false,
            )
          }
        } else {
          // dynamic key binding, wrap with `normalizeProps`
          *props_expression = ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&context.helper("normalizeProps"))),
            NONE,
            ast.vec1(props_expression.take_in(ast.allocator).into()),
            false,
          )
        }
      }
      // mergeProps call, do nothing
      Expression::CallExpression(_) => (),
      // single v-bind
      _ => {
        *props_expression = ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("normalizeProps"))),
          NONE,
          ast.vec1(
            ast
              .expression_call(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&context.helper("guardReactiveProps"))),
                NONE,
                ast.vec1(props_expression.take_in(ast.allocator).into()),
                false,
              )
              .into(),
          ),
          false,
        )
      }
    }
  }

  let directives = if !runtime_directives.is_empty() {
    Some(ast.array_expression(SPAN, ast.vec_from_iter(runtime_directives)))
  } else {
    None
  };

  PropsResult {
    props: props_expression,
    directives,
    patch_flag,
    dynamic_prop_names,
    should_use_block,
  }
}

// Dedupe props in an object literal.
// Literal duplicated attributes would have been warned during the parse phase,
// however, it's possible to encounter duplicated `onXXX` handlers with different
// modifiers. We also need to merge static and dynamic class / style attributes.
pub fn dedupe_properties<'a>(
  properties: &mut oxc_allocator::Vec<'a, ObjectPropertyKind<'a>>,
  ast: &'a AstBuilder<'a>,
) -> oxc_allocator::Vec<'a, ObjectPropertyKind<'a>> {
  let mut deduped = ast.vec();

  for mut property in properties.drain(..) {
    match &mut property {
      // dynamic keys are always allowed
      ObjectPropertyKind::SpreadProperty(_) => deduped.push(property),
      ObjectPropertyKind::ObjectProperty(prop) => {
        if prop.computed {
          deduped.push(property);
        } else if let Some(name) = prop.key.name() {
          let name = name.to_string();
          if let Some(existing) = deduped.iter_mut().find(|i| match i {
            ObjectPropertyKind::ObjectProperty(i) => i
              .key
              .name()
              .map(|key_name| key_name.eq(name.as_str()))
              .unwrap_or_default(),
            ObjectPropertyKind::SpreadProperty(_) => false,
          }) && let ObjectPropertyKind::ObjectProperty(existing) = existing
          {
            if name == "style" || name == "class" || is_event(&name) {
              if let Expression::ArrayExpression(value) = &mut existing.value {
                value
                  .elements
                  .push(prop.value.take_in(ast.allocator).into());
              } else {
                existing.value = ast.expression_array(
                  existing.span(),
                  ast.vec_from_array([
                    existing.value.take_in(ast.allocator).into(),
                    prop.value.take_in(ast.allocator).into(),
                  ]),
                )
              }
            }
            // unexpected duplicate, should have emitted error during parse
          } else {
            deduped.push(property.take_in(ast.allocator));
          };
        }
      }
    }
  }

  deduped
}

pub fn build_directive_args<'a>(
  dir: DirectiveNode<'a>,
  context: &'a TransformContext<'a>,
  runtime: &str,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut dir_args = ast.vec();
  // built-in directive with runtime
  dir_args.push(ast.expression_identifier(SPAN, ast.atom(runtime)));
  let exp_is_none = dir.exp.is_none();
  if let Some(exp) = dir.exp {
    dir_args.push(if let Some(node) = exp.ast {
      node.take_in(ast.allocator)
    } else {
      ast.expression_string_literal(exp.loc, ast.atom(&exp.content), None)
    });
  }
  let arg_is_none = dir.arg.is_none();
  if let Some(arg) = dir.arg {
    if arg_is_none {
      dir_args.push(ast.expression_identifier(SPAN, "void 0"))
    }
    dir_args.push(if !arg.content.contains(".") {
      context
        .ast
        .expression_identifier(SPAN, ast.atom(&arg.content))
    } else {
      parse_expression(
        &arg.content,
        arg.loc,
        context.allocator,
        context.options.source_type,
      )
      .unwrap()
    })
  }
  if !dir.modifiers.is_empty() {
    if arg_is_none {
      if exp_is_none {
        dir_args.push(ast.expression_identifier(SPAN, "void 0"));
      }
      dir_args.push(ast.expression_identifier(SPAN, "void 0"));
    }
    dir_args.push(ast.expression_object(
      dir.loc,
      ast.vec_from_iter(dir.modifiers.into_iter().map(|modifier| {
        ast.object_property_kind_object_property(
          modifier.loc,
          PropertyKind::Init,
          ast.property_key_static_identifier(SPAN, ast.atom(&modifier.content)),
          ast.expression_boolean_literal(SPAN, true),
          false,
          false,
          false,
        )
      })),
    ))
  }

  ast.expression_array(
    dir.loc,
    ast.vec_from_iter(dir_args.into_iter().map(|arg| arg.into())),
  )
}
