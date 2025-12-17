use common::directive::find_prop;
use napi::Either;
use oxc_ast::ast::JSXChild;

use crate::{ast::NodeTypes, transform::TransformContext};

/// # SAFETY
pub unsafe fn transform_v_once<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let node = unsafe { &*context_node };

  if let JSXChild::Element(node) = &node
    && let Some(dir) = find_prop(node, Either::A(String::from("v-once")))
  {
    let seen = &mut context.seen.borrow_mut();
    if seen.contains(&dir.span.start) || *context.in_v_once.borrow() || context.options.in_ssr {
      return None;
    }
    seen.insert(dir.span.start);
    *context.options.in_v_once.borrow_mut() = true;
    return Some(Box::new(move || {
      let codegen_map = &mut context.codegen_map.borrow_mut();
      if let Some(NodeTypes::VNodeCall(mut codegen)) = codegen_map.remove(&node.span) {
        codegen.is_block = false;
        let codegen = NodeTypes::CacheExpression(context.cache(
          context.gen_vnode_call(codegen, codegen_map),
          true,
          true,
          false,
        ));
        codegen_map.insert(node.span, codegen);
      }
    }));
  }
  None
}
