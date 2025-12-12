use std::collections::HashMap;

use napi::{
  Either,
  bindgen_prelude::Either3,
};
use oxc_allocator::TakeIn;
use oxc_ast::{
  AstBuilder, NONE,
  ast::{
    ArrayExpression, Expression,
    JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild, JSXElement,
    ObjectProperty, ObjectPropertyKind, PropertyKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{NodeTypes, VNodeCall},
  ir::index::{
    BlockIRNode, RootNode,
  },
  transform::{
    DirectiveTransformResult, TransformContext, cache_static::get_constant_type,
    v_bind::transform_v_bind, v_html::transform_v_html,
  },
};

use common::{
  check::{
    is_built_in_directive, is_directive, is_event, is_jsx_component, is_reserved_prop, is_template,
  },
  directive::DirectiveNode,
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  patch_flag::PatchFlags,
  text::{camelize, get_tag_name, is_empty_text, to_valid_asset_id},
};

/// # SAFETY
/// generate a JavaScript AST for this element's codegen
pub unsafe fn transform_element<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  if is_template(node) {
    return None;
  }

  let ast = &context.ast;

  let is_component = is_jsx_component(node);

  // The goal of the transform is to create a codegenNode implementing the
  // VNodeCall interface.
  let mut vnode_tag = get_tag_name(&node.opening_element.name, context.ir.borrow().source);
  if is_component && (context.options.with_fallback || vnode_tag.contains("-")) {
    context.helper("resolveComponent");
    context.components.borrow_mut().insert(vnode_tag.clone());
    vnode_tag = to_valid_asset_id(&vnode_tag, "component");
  }

  let mut should_use_block = RootNode::is_single_root(parent_node)
    || vnode_tag == "Teleport"
    || vnode_tag == "Suspense"
    || (!is_component &&
    // <svg> and <foreignObject> must be forced into blocks so that block
    // updates inside get proper isSVG flag at runtime. (#639, #643)
    // This is technically web-specific, but splitting the logic out of core
    // leads to too much unnecessary complexity.
    (vnode_tag == "svg" || vnode_tag =="foreignObject" || vnode_tag =="math"));

  let _context_block = context_block as *mut BlockIRNode;
  let _node = node as *mut oxc_allocator::Box<JSXElement>;
  let props_build_result = build_props(
    unsafe { &mut *_node },
    context,
    unsafe { &mut *_context_block },
    is_component,
  );

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

      vnode_children = Some(
        // if should_build_as_slots {
        // TODO
        // const { slots, hasDynamicSlots } = buildSlots(node, context)
        // vnodeChildren = slots
        // if (hasDynamicSlots) {
        //   patchFlag |= PatchFlags.DYNAMIC_SLOTS
        // }
        // } else
        if children.len() == 1 && vnode_tag != "Teleport" {
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
        },
      );
    }

    // patchFlag & dynamicPropNames
    let vnode_dynamic_props = if !dynamic_prop_names.is_empty() {
      Some(ast.expression_array(
        SPAN,
        ast.vec_from_iter(dynamic_prop_names.into_iter().map(|name| {
          ast
            .expression_string_literal(SPAN, ast.atom(&name), None)
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
      patch_flag: if patch_flag == 0 {
        None
      } else {
        Some(patch_flag)
      },
      dynamic_props: vnode_dynamic_props,
      directives: vnode_directives,
      is_block: should_use_block,
      disable_tracking: false,
      is_component,
      v_for: None,
      v_if: None,
      loc: SPAN,
    };
    context
      .codegen_map
      .borrow_mut()
      .insert(node.span, NodeTypes::VNodeCall(vnode_call));
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
  context_block: &'a mut BlockIRNode<'a>,
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
  let runtime_directives: Vec<DirectiveNode> = vec![];
  let has_children = !node.children.is_empty();
  let mut should_use_block = false;
  let directive_import_map: HashMap<Span, String> = HashMap::new();

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
    if *context.in_v_for.borrow() > 0 {
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

      // TODO
      // if (
      //   value.type === NodeTypes.JS_CACHE_EXPRESSION ||
      //   ((value.type === NodeTypes.SIMPLE_EXPRESSION ||
      //     value.type === NodeTypes.COMPOUND_EXPRESSION) &&
      //     getConstantType(value, context) > 0)
      // ) {
      //   // skip if the prop is a cached handler or has constant value
      //   return
      // }
      if (get_constant_type(Either::B(value), context) as i32) > 0 {
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
      has_dynamic_keys = true
    }
  };

  let properties = &mut properties;
  for prop in props {
    // static attribute
    match prop {
      JSXAttributeItem::Attribute(prop) => {
        let ast = &context.ast;
        let name = match &prop.name {
          JSXAttributeName::Identifier(name) => name.name.as_str(),
          JSXAttributeName::NamespacedName(name) => name.namespace.name.as_str(),
        }
        .split("_")
        .collect::<Vec<&str>>()[0];
        if !is_directive(name)
          && !is_event(name)
          && matches!(prop.value, Some(JSXAttributeValue::StringLiteral(_)))
          && name == "ref" {
            has_ref = prop
              .value
              .as_ref()
              .map(|value| matches!(value, JSXAttributeValue::StringLiteral(_)))
              .unwrap_or_default();
            if let Some(marker) = ref_v_for_marker() {
              properties.push(marker)
            };
          }

        let mut dir_name = if is_event(name) {
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

          if prop.name.as_identifier().is_some() {
            if let Some(value) = &mut prop.value {
              // v-on={obj} -> toHandlers(obj)
              if !properties.is_empty() {
                merge_args
                  .push(ast.expression_object(node.span, dedupe_properties(properties, ast)));
              }
              merge_args.push(jsx_attribute_value_to_expression(
                value.take_in(context.allocator),
                ast.allocator,
              ));
            } else {
              context.options.on_error.as_ref()(ErrorCodes::VOnNoExpression, prop.span);
            }
            continue;
          }
        }

        if let Some(DirectiveTransformResult {
          props,
          need_runtime,
        }) = match dir_name.as_str() {
          "bind" => {
            // #938: elements with dynamic keys should be forced into blocks
            if name == "key" {
              should_use_block = true
            }
            // force hydration for prop with .prop modifier
            if name.split("_").any(|n| n == "prop") {
              patch_flag |= PatchFlags::NeedHydration as i32;
            }
            transform_v_bind(prop, node, context)
          }
          "on" => {
            // inline before-update hooks need to force block so that it is invoked
            // before children
            if has_children && name == "onVue:before-update" {
              should_use_block = true;
            }

            None
            // return transform_v_on(prop, node, context, context_block),
          }
          // "model" => return transform_v_model(prop, node, context, context_block),
          // "show" => return transform_v_show(prop, node, context, context_block),
          "html" => transform_v_html(prop, node, context),
          // "text" => return transform_v_text(prop, node, context, context_block),
          _ => {
            if !is_built_in_directive(&dir_name) {
              let with_fallback = context.options.with_fallback;
              if with_fallback {
                let directive = &mut context.ir.borrow_mut().directive;
                directive.insert(dir_name.clone());
              } else {
                dir_name = camelize(&format!("v-{dir_name}"))
              };

              let element = context.reference(&mut context_block.dynamic);
              // context.register_operation(
              //   context_block,
              //   Either16::M(DirectiveIRNode {
              //     directive: true,
              //     element,
              //     dir: resolve_directive(prop, context.ir.borrow().source),
              //     name: dir_name,
              //     asset: Some(with_fallback),
              //     builtin: None,
              //     model_type: None,
              //   }),
              //   None,
              // )
            }
            None
          }
        } {
          if !*context.options.ssr.borrow() {
            props.iter().for_each(&mut analyze_patch_flag);
          }
          properties.extend(props);
        };
      }
      JSXAttributeItem::SpreadAttribute(prop) => {
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
            && matches!(class_prop.value, Expression::StringLiteral(_))
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
    Some(ast.array_expression(
      SPAN,
      ast.vec_from_iter(runtime_directives.into_iter().map(|dir| {
        let loc = dir.loc;
        build_directive_args(dir, context, directive_import_map.get(&loc)).into()
      })),
    ))
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
  runtime: Option<&String>,
) -> Expression<'a> {
  let ast = &context.ast;
  let mut dir_args = ast.vec();
  if let Some(runtime) = runtime {
    // built-in directive with runtime
    // dir_args.push(context.helper(&runtime));
  } else {
    // inject statement for resolving directive
    context.helper("resolveDirective");
    dir_args
      .push(ast.expression_identifier(SPAN, ast.atom(&to_valid_asset_id(&dir.name, "directive"))));
    context.ir.borrow_mut().directive.insert(dir.name);
  }
  let exp_is_none = dir.exp.is_none();
  if let Some(exp) = dir.exp
    && let Some(node) = exp.ast
  {
    dir_args.push(node.take_in(ast.allocator));
  }
  let arg_is_none = dir.arg.is_none();
  if let Some(arg) = dir.arg {
    if arg_is_none {
      dir_args.push(ast.expression_identifier(SPAN, "void 0"))
    }
    if let Some(arg_ast) = arg.ast {
      dir_args.push(arg_ast.take_in(ast.allocator))
    }
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
