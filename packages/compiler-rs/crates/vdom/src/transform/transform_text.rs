use napi::Either;
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  NONE,
  ast::{ConditionalExpression, Expression, JSXChild, LogicalExpression, NumberBase, PropertyKind},
};
use oxc_span::{GetSpan, SPAN};

use crate::{
  ast::{ConstantTypes, NodeTypes},
  transform::{
    TransformContext,
    cache_static::{cache_static_children, get_constant_type},
    utils::inject_prop,
  },
};

use common::{
  check::{is_built_in_directive, is_directive, is_jsx_component, is_template},
  patch_flag::PatchFlags,
  text::{is_empty_text, resolve_jsx_text},
};

/// # SAFETY
/// Merge adjacent text nodes and expressions into a single expression
/// e.g. <div>abc {{ d }} {{ e }}</div> should have a single expression node as child.
pub unsafe fn transform_text<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let ast = &context.ast;
  let node = unsafe { &mut *context_node };

  let mut children = match unsafe { &mut *context_node } {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => return None,
  }
  .into_iter()
  .filter(|child| !is_empty_text(child))
  .collect::<Vec<_>>();

  Some(Box::new(move || {
    let has_text = children.iter().any(|child| is_text_like(child));

    // if this is a plain element with a single text child, leave it
    // as-is since the runtime has dedicated fast path for this by directly
    // setting textContent of the element.
    // for component root it's always normalized anyway.
    if !has_text
      || (children.len() == 1
        && (if let JSXChild::Element(node) = node {
          // #3756
          // custom directives can potentially add DOM elements arbitrarily,
          // we need to avoid setting textContent of the element at runtime
          // to avoid accidentally overwriting the DOM elements added
          // by the user through custom directives.
          !is_template(node)
            && !is_jsx_component(node)
            && !node.opening_element.attributes.iter().any(|p| {
              p.as_attribute()
                .map(|p| {
                  is_directive(&p.name.get_identifier().name)
                    && !is_built_in_directive(&p.name.get_identifier().name[2..])
                })
                .unwrap_or_default()
            })
            && if let Some(JSXChild::ExpressionContainer(child)) = children.first_mut() {
              matches!(child.expression.as_expression_mut(), Some(exp) if exp.is_literal())
            } else {
              true
            }
        } else {
          false
        }))
    {
      return;
    }

    // pre-convert text nodes into createTextVNode(text) calls to avoid
    // runtime normalization.
    for child in children.iter_mut() {
      if is_text_like(child) {
        let mut call_args = ast.vec();
        // createTextVNode defaults to single whitespace, so if it is a
        // single space the code could be an empty call to save bytes
        if let JSXChild::Text(child) = child {
          if !child.value.trim().is_empty() {
            call_args.push(
              ast
                .expression_string_literal(SPAN, ast.atom(&resolve_jsx_text(child)), None)
                .into(),
            )
          }
        } else if let JSXChild::ExpressionContainer(child) = child
          && let Some(exp) = child.expression.as_expression_mut()
        {
          if let Expression::ConditionalExpression(exp) = exp {
            transform_condition_expression(exp, unsafe { &mut *context_node }, context);
            continue;
          } else if let Expression::LogicalExpression(exp) = exp {
            transform_logical_expression(exp, unsafe { &mut *context_node }, context);
            continue;
          }
          call_args.push(
            context
              .jsx_expression_to_expression(child.expression.clone_in(context.allocator))
              .into(),
          )
        };
        // mark dynamic text with flag so it gets patched inside a block
        if !*context.options.ssr.borrow()
          && matches!(
            get_constant_type(
              Either::A(child),
              context,
              &mut context.codegen_map.borrow_mut()
            ),
            ConstantTypes::NotConstant
          )
        {
          call_args.push(
            ast
              .expression_numeric_literal(
                SPAN,
                PatchFlags::Text as i32 as f64,
                None,
                NumberBase::Hex,
              )
              .into(),
          )
        }
        context.codegen_map.borrow_mut().insert(
          child.span(),
          NodeTypes::TextCallNode(ast.expression_call(
            child.span(),
            ast.expression_identifier(SPAN, ast.atom(&context.helper("normalizeVNode"))),
            NONE,
            call_args,
            false,
          )),
        );
      }
    }
  }))
}

fn is_text_like(node: &JSXChild) -> bool {
  matches!(node, JSXChild::Text(_) | JSXChild::ExpressionContainer(_))
}

fn transform_condition_expression<'a>(
  node: &mut ConditionalExpression<'a>,
  parent: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) {
  let context_v_if_map = context.v_if_map.as_ptr();
  let v_if_map = unsafe { &mut *context_v_if_map }
    .entry(parent.span())
    .or_default();
  let key = v_if_map.0;
  v_if_map.0 += 2;
  transform_branch(&mut node.consequent, key, parent, context);
  transform_branch(&mut node.alternate, key + 1, parent, context);
}

fn transform_logical_expression<'a>(
  node: &mut LogicalExpression<'a>,
  parent: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) {
  let context_v_if_map = context.v_if_map.as_ptr();
  let v_if_map = unsafe { &mut *context_v_if_map }
    .entry(parent.span())
    .or_default();
  let key = v_if_map.0;
  v_if_map.0 += 1;
  transform_branch(&mut node.right, key, parent, context);
}

fn transform_branch<'a>(
  exp: &mut Expression<'a>,
  key: usize,
  parent: &mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) {
  let ast = &context.ast;
  let span = exp.span();
  let mut branch = if let Expression::JSXElement(branch) = exp {
    JSXChild::Element(branch.take_in_box(context.allocator))
  } else {
    context.wrap_fragment(exp.take_in(context.allocator), span)
  };
  unsafe {
    context.transform_node(&mut branch, Some(parent));
    let codegen_map = &mut context.codegen_map.borrow_mut();
    if let Some(NodeTypes::VNodeCall(mut vnode_call)) = codegen_map.remove(&span) {
      let key_property = ast.object_property(
        SPAN,
        PropertyKind::Init,
        ast.property_key_static_identifier(SPAN, "key"),
        ast.expression_numeric_literal(SPAN, key as f64, None, NumberBase::Hex),
        false,
        false,
        false,
      );
      vnode_call.is_block = true;
      inject_prop(&mut vnode_call, key_property, context);
      cache_static_children(None, vec![&mut branch], context, codegen_map, false);
      *exp = context.gen_vnode_call(vnode_call, codegen_map);
    }
  }
}
