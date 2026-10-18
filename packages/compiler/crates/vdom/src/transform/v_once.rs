use oxc_ast::ast::JSXChild;
use oxc_span::Span;

use crate::{ast::NodeTypes, transform::TransformContext};
use common::directive::Directives;

/// # SAFETY
pub unsafe fn transform_v_once<'a>(
  directives: &mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let node = unsafe { &*context_node };

  if let JSXChild::Element(node) = &node
    && let Some(dir) = directives.v_once.as_ref()
  {
    let seen = &mut context.seen.borrow_mut();
    if seen.contains(&dir.span.start) || *context.options.in_v_once.borrow() || context.options.ssr
    {
      return None;
    }
    seen.insert(dir.span.start);
    let node_span = if directives.v_for.is_some() {
      Span::new(node.span.end, node.span.start)
    } else {
      node.span
    };
    *context.options.in_v_once.borrow_mut() = true;
    return Some(Box::new(move || {
      *context.options.in_v_once.borrow_mut() = false;
      let codegen_map = &mut context.codegen_map.borrow_mut();
      if let Some(mut codegen) = codegen_map.remove(&node_span) {
        if let NodeTypes::VNodeCall(mut vnode_call) = codegen {
          vnode_call.is_block = false;
          codegen = NodeTypes::CacheExpression(context.cache(
            context.gen_vnode_call(vnode_call, codegen_map),
            true,
            true,
            false,
          ));
        } else if let NodeTypes::CacheExpression(exp) = codegen {
          codegen = NodeTypes::CacheExpression(context.cache(exp, true, true, false));
        }
        codegen_map.insert(node_span, codegen);
      }
    }));
  }
  None
}
