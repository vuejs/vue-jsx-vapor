use std::collections::HashMap;

use common::{
  check::{get_directive_name, is_jsx_component},
  patch_flag::PatchFlags,
  text::is_empty_text,
  walk::WalkIdentifiers,
};
use napi::{Either, bindgen_prelude::Either3};
use oxc_ast::ast::{
  Expression, JSXAttributeItem, JSXAttributeValue, JSXChild, JSXElement, NumberBase,
  ObjectPropertyKind,
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{ConstantTypes, NodeTypes, VNodeCallChildren, get_vnode_block_helper},
  transform::TransformContext,
};

pub fn cache_static<'a>(
  root: &'a mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
) {
  let node = unsafe { &*(root as *mut JSXChild) };
  let children = match unsafe { &mut *(root as *mut JSXChild) } {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => unimplemented!(),
  };
  let single_root = if let JSXChild::Fragment(_) = node
    && children.len() == 1
    && let JSXChild::Element(child) = &children[0]
    && !is_jsx_component(child)
  {
    true
  } else {
    false
  };
  cache_static_children(
    Some(Either::A(node)),
    children,
    context,
    codegen_map,
    // Root node is unfortunately non-hoistable due to potential parent
    // fallthrough attributes.
    single_root,
  );
}

pub fn cache_static_children<'a>(
  node: Option<Either<&JSXChild<'a>, &mut VNodeCallChildren<'a>>>,
  children: &mut oxc_allocator::Vec<JSXChild<'a>>,
  context: &'a TransformContext<'a>,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
  do_not_hoist_node: bool,
) {
  let codegen_map_ptr = codegen_map as *mut HashMap<Span, NodeTypes>;
  let child_len = children.len();
  let mut to_cache = vec![];
  for child in children {
    let child_ptr = child as *mut JSXChild;
    let child = unsafe { &mut *child_ptr };
    let child_span = child.span();
    // only plain elements & text calls are eligible for caching.
    if let JSXChild::Element(child) = child
      && !is_jsx_component(child)
    {
      let contant_type = if do_not_hoist_node {
        ConstantTypes::NotConstant
      } else {
        get_constant_type(Either::A(unsafe { &*child_ptr }), context, codegen_map)
      };
      if (contant_type.clone() as i32) > ConstantTypes::NotConstant as i32 {
        if (contant_type.clone() as i32) >= ConstantTypes::CanCache as i32 {
          if let Some(NodeTypes::VNodeCall(codegen)) = codegen_map.get_mut(&child.span) {
            codegen.patch_flag = Some(PatchFlags::Cached as i32);
          };
          to_cache.push(unsafe { &mut *child_ptr });
          continue;
        }
      } else if let Some(codegen) = unsafe { &mut *codegen_map_ptr }.get_mut(&child_span)
        // node may contain dynamic children, but its props may be eligible for
        // hoisting.
        && let NodeTypes::VNodeCall(codegen) = codegen
      {
        let flag = codegen.patch_flag;
        if if let Some(flag) = flag {
          flag == PatchFlags::NeedPatch as i32 || flag == PatchFlags::Text as i32
        } else {
          true
        } && get_generated_props_constant_type(child, context, codegen_map) as i32
          >= ConstantTypes::CanCache as i32
          && let Some(props) = &mut codegen.props
        {
          codegen.props = Some(context.hoist(props))
        };

        if let Some(dynamic_props) = codegen.dynamic_props.as_mut() {
          codegen.dynamic_props = Some(context.hoist(dynamic_props))
        }
      }
    } else if let Some(codegen) = unsafe { &mut *codegen_map_ptr }.get_mut(&child_span)
      && let NodeTypes::TextCallNode(codegen) = codegen
    {
      let contant_type = if do_not_hoist_node {
        ConstantTypes::NotConstant
      } else {
        get_constant_type(Either::A(unsafe { &*child_ptr }), context, codegen_map)
      };
      if contant_type as i32 >= ConstantTypes::CanCache as i32 {
        if let Expression::CallExpression(codegen) = codegen
          && !codegen.arguments.is_empty()
        {
          codegen.arguments.push(
            context
              .ast
              .expression_numeric_literal(
                SPAN,
                PatchFlags::Cached as i32 as f64,
                None,
                NumberBase::Hex,
              )
              .into(),
          );
        }
        to_cache.push(unsafe { &mut *child_ptr });
        continue;
      }
    }

    // walk further
    if let JSXChild::Element(child) = unsafe { &*child_ptr } {
      let is_component = is_jsx_component(child);
      if is_component {
        *context.options.in_v_slot.borrow_mut() += 1;
      }
      cache_static_children(
        Some(Either::A(unsafe { &*child_ptr })),
        match unsafe { &mut *child_ptr } {
          JSXChild::Element(node) => &mut node.children,
          JSXChild::Fragment(node) => &mut node.children,
          _ => unimplemented!(),
        },
        context,
        codegen_map,
        false,
      );
      if is_component {
        *context.options.in_v_slot.borrow_mut() -= 1;
      }
    } else if let JSXChild::Fragment(child) = unsafe { &*child_ptr } {
      let codegen = codegen_map.get(&child.span);
      if let Some(NodeTypes::VNodeCall(codegen)) = codegen
        && codegen.v_for.is_some()
      {
        // Do not hoist v-for because it has to be a block
        cache_static_children(
          Some(Either::A(unsafe { &*child_ptr })),
          match unsafe { &mut *child_ptr } {
            JSXChild::Element(node) => &mut node.children,
            JSXChild::Fragment(node) => &mut node.children,
            _ => unimplemented!(),
          },
          context,
          codegen_map,
          true,
        )
      }
    }
  }

  if child_len > 1
    && to_cache.len() == child_len
    && let Some((children, codegen_children)) = if let Some(Either::A(node)) = node
      && let JSXChild::Element(node) = node
      && let Some(codegen) = unsafe { &mut *codegen_map_ptr }.get_mut(&node.span)
      && let NodeTypes::VNodeCall(codegen) = codegen
      && let Some(codegen_children) = codegen.children.as_mut()
      && let Either3::B(children) = codegen_children
    {
      Some((*children, codegen_children))
    } else if let Some(Either::B(codegen_children)) = node
      && let Either3::B(children) = codegen_children
    {
      Some((*children, codegen_children))
    } else {
      None
    }
  {
    // all children were hoisted - the entire children array is cacheable.
    *codegen_children = Either3::C(context.cache(
      context.gen_node_list(Either3::B(children), codegen_map),
      false,
      false,
      true,
    ));
    return;
  }

  for child in to_cache {
    let span = child.span();
    match codegen_map.remove(&span).unwrap() {
      NodeTypes::VNodeCall(codegen) => {
        unsafe { &mut *codegen_map_ptr }.insert(
          span,
          NodeTypes::CacheExpression(context.cache(
            context.gen_vnode_call(codegen, codegen_map),
            false,
            false,
            false,
          )),
        );
      }
      NodeTypes::TextCallNode(codegen) => {
        codegen_map.insert(
          span,
          NodeTypes::CacheExpression(context.cache(codegen, false, false, false)),
        );
      }
      _ => (),
    }
  }
}

pub fn get_constant_type<'a>(
  node: Either<&JSXChild<'a>, &Expression<'a>>,
  context: &'a TransformContext<'a>,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
) -> ConstantTypes {
  let codegen_map_ptr = codegen_map as *mut HashMap<Span, NodeTypes>;
  let node_span = match node {
    Either::A(node) => node.span(),
    Either::B(node) => node.span(),
  };
  match &node {
    Either::A(node) => match node {
      JSXChild::Element(node) => {
        if is_jsx_component(node) {
          return ConstantTypes::NotConstant;
        }
        if let Some(cached) = context.constant_cache.borrow().get(&node_span) {
          return cached.clone();
        }
        if let Some(codegen) = unsafe { &mut *codegen_map_ptr }.get_mut(&node_span) {
          let NodeTypes::VNodeCall(codegen) = codegen else {
            return ConstantTypes::NotConstant;
          };
          if codegen.v_for.is_some() || codegen.v_if.is_some() {
            return ConstantTypes::NotConstant;
          }
          let tag = node.opening_element.name.to_string();
          if codegen.is_block && tag != "svg" && tag != "foreignObject" && tag != "math" {
            return ConstantTypes::NotConstant;
          }
          if codegen.patch_flag.is_none() {
            let mut return_type = ConstantTypes::CanStringify;

            // Element itself has no patch flag. However we still need to check:

            // 1. Even for a node with no patch flag, it is possible for it to contain
            // non-hoistable expressions that refers to scope variables, e.g. compiler
            // injected keys or cached event handlers. Therefore we need to always
            // check the codegenNode's props to be sure.
            let generated_props_type =
              get_generated_props_constant_type(node, context, codegen_map);
            if matches!(generated_props_type, ConstantTypes::NotConstant) {
              context
                .constant_cache
                .borrow_mut()
                .insert(node_span, ConstantTypes::NotConstant);
              return ConstantTypes::NotConstant;
            };
            if (generated_props_type.clone() as i32) < return_type.clone() as i32 {
              return_type = generated_props_type;
            }

            // 2. its children.
            for child in node.children.iter() {
              if is_empty_text(child) {
                continue;
              }
              let child_type = get_constant_type(Either::A(child), context, codegen_map);
              if matches!(child_type, ConstantTypes::NotConstant) {
                context
                  .constant_cache
                  .borrow_mut()
                  .insert(node_span, ConstantTypes::NotConstant);
                return ConstantTypes::NotConstant;
              }
              if (child_type.clone() as i32) < return_type.clone() as i32 {
                return_type = child_type;
              }
            }

            // 3. if the type is not already CAN_SKIP_PATCH which is the lowest non-0
            // type, check if any of the props can cause the type to be lowered
            // we can skip can_patch because it's guaranteed by the absence of a
            // patchFlag.
            if (return_type.clone() as i32) > ConstantTypes::CanSkipPatch as i32 {
              for p in node.opening_element.attributes.iter() {
                let JSXAttributeItem::Attribute(p) = p else {
                  continue;
                };
                let name = &p.name.get_identifier().name;
                if get_directive_name(name).is_none()
                  && let Some(JSXAttributeValue::ExpressionContainer(value)) = p.value.as_ref()
                  && value.expression.span() != SPAN
                {
                  let exp_type = get_constant_type(
                    Either::B(value.expression.to_expression()),
                    context,
                    codegen_map,
                  );
                  if matches!(exp_type, ConstantTypes::NotConstant) {
                    context
                      .constant_cache
                      .borrow_mut()
                      .insert(node_span, ConstantTypes::NotConstant);
                    return ConstantTypes::NotConstant;
                  }
                  if (exp_type.clone() as i32) < return_type.clone() as i32 {
                    return_type = exp_type;
                  }
                }
              }
            }

            // only svg/foreignObject could be block here, however if they are
            // static then they don't need to be blocks since there will be no
            // nested updates.
            if codegen.is_block {
              // except set custom directives.
              for p in node.opening_element.attributes.iter() {
                if let JSXAttributeItem::Attribute(p) = p
                  && let Some(JSXAttributeValue::ExpressionContainer(_)) = p.value
                {
                  context
                    .constant_cache
                    .borrow_mut()
                    .insert(node_span, ConstantTypes::NotConstant);
                  return ConstantTypes::NotConstant;
                }
              }

              codegen.is_block = false;
              context.helper(&get_vnode_block_helper(
                context.options.ssr,
                is_jsx_component(node),
              ));
            }

            context
              .constant_cache
              .borrow_mut()
              .insert(node_span, return_type.clone());
            return return_type;
          } else {
            context
              .constant_cache
              .borrow_mut()
              .insert(node_span, ConstantTypes::NotConstant);
            return ConstantTypes::NotConstant;
          }
        }
        ConstantTypes::NotConstant
      }
      JSXChild::ExpressionContainer(node) => {
        let node = &node.expression;
        if node.span() == SPAN || !node.to_expression().is_literal() {
          ConstantTypes::NotConstant
        } else {
          ConstantTypes::CanStringify
        }
      }
      JSXChild::Text(_) => ConstantTypes::CanStringify,
      _ => ConstantTypes::NotConstant,
    },
    Either::B(node) => {
      let node_span = node.span();
      if let Some(has_ref) = context.reference_expressions.borrow().get(&node_span) {
        if *has_ref {
          ConstantTypes::NotConstant
        } else {
          ConstantTypes::CanStringify
        }
      } else if node.is_literal() {
        ConstantTypes::CanStringify
      } else {
        let mut has_ref = false;
        if node.is_identifier_reference() || matches!(node, Expression::StaticMemberExpression(_)) {
          has_ref = true;
        } else {
          let has_ref_ptr = &mut has_ref as *mut bool;
          WalkIdentifiers::new(
            Box::new(move |_, _, _, _, _| {
              *unsafe { &mut *has_ref_ptr } = true;
            }),
            &context.ast,
            *context.source.borrow(),
            context.options,
          )
          .visit(node);
        }
        if has_ref {
          ConstantTypes::NotConstant
        } else {
          ConstantTypes::CanStringify
        }
      }
    }
  }
}

fn get_generated_props_constant_type<'a>(
  node: &JSXElement<'a>,
  context: &'a TransformContext<'a>,
  codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
) -> ConstantTypes {
  let mut return_type = ConstantTypes::CanStringify;
  if let Some(NodeTypes::VNodeCall(codegen)) =
    (unsafe { &*(codegen_map as *mut HashMap<Span, NodeTypes>) }).get(&node.span)
    && let Some(props) = &codegen.props
    && let Expression::ObjectExpression(props) = props
  {
    for prop in props.properties.iter() {
      match prop {
        ObjectPropertyKind::ObjectProperty(prop) => {
          let value_type = get_constant_type(Either::B(&prop.value), context, codegen_map);
          if let ConstantTypes::NotConstant = value_type {
            return value_type;
          } else if (value_type.clone() as i32) < (return_type.clone() as i32) {
            return_type = value_type
          }
        }
        ObjectPropertyKind::SpreadProperty(_) => return ConstantTypes::NotConstant,
      }
    }
  }
  return_type
}
