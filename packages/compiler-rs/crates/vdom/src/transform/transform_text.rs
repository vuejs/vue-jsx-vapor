use napi::Either;
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{JSXChild, NumberBase},
};
use oxc_span::{GetSpan, SPAN};

use crate::{
  ast::{ConstantTypes, NodeTypes, TextCallNode},
  ir::index::BlockIRNode,
  transform::{TransformContext, cache_static::get_constant_type},
};

use common::{
  check::{is_built_in_directive, is_directive, is_jsx_component, is_template},
  patch_flag::PatchFlags,
  text::{get_text_like_value, is_empty_text, is_text_like, resolve_jsx_text},
};

/// # SAFETY
/// Merge adjacent text nodes and expressions into a single expression
/// e.g. <div>abc {{ d }} {{ e }}</div> should have a single expression node as child.
pub unsafe fn transform_text<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  _: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let ast = &context.ast;
  let node = unsafe { &mut *context_node };

  Some(Box::new(move || {
    let mut children = match unsafe { &mut *context_node } {
      JSXChild::Element(node) if !is_jsx_component(node) => &mut node.children,
      JSXChild::Fragment(node) => &mut node.children,
      _ => return,
    }
    .into_iter()
    .filter(|child| !is_empty_text(child))
    .collect::<Vec<_>>();

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
                    && !is_built_in_directive(&p.name.get_identifier().name)
                })
                .unwrap_or_default()
            })
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
          if !child.value.eq(" ") {
            call_args.push(
              ast
                .expression_string_literal(SPAN, ast.atom(&resolve_jsx_text(child)), None)
                .into(),
            )
          }
        } else if let JSXChild::ExpressionContainer(child) = child {
          if let Some(value) = get_text_like_value(child.expression.to_expression(), false) {
            call_args.push(
              child
                .expression
                .to_expression_mut()
                .take_in(ast.allocator)
                .into(),
            );
          } else {
            continue;
          }
        };
        // mark dynamic text with flag so it gets patched inside a block
        if !*context.options.ssr.borrow()
          && matches!(
            get_constant_type(Either::A(child), context),
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
            ast.expression_identifier(SPAN, ast.atom(&context.helper("createTextVNode"))),
            NONE,
            call_args,
            false,
          )),
        );
      }
    }
  }))
}
