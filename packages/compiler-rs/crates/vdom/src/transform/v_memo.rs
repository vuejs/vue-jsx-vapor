use common::check::is_jsx_component;
use oxc_ast::{
  NONE,
  ast::{FormalParameterKind, JSXChild, NumberBase},
};
use oxc_span::SPAN;

use crate::{ast::NodeTypes, transform::TransformContext};
use common::directive::Directives;

/// # SAFETY
pub unsafe fn transform_v_memo<'a>(
  directives: &mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let node = unsafe { &mut *context_node };
  if let JSXChild::Element(node) = node
    && directives.v_for.is_none()
    && let Some(dir) = directives.v_memo.as_mut()
  {
    let seen = &mut context.seen.borrow_mut();
    if seen.contains(&dir.span.start) || *context.options.in_v_once.borrow() || context.options.ssr
    {
      return None;
    }
    seen.insert(dir.span.start);
    let mut value = dir.value.take()?;
    let is_component = is_jsx_component(node, context.options);
    return Some(Box::new(move || {
      let codegen_map = &mut context.codegen_map.borrow_mut();
      if let Some(NodeTypes::VNodeCall(mut codegen)) = codegen_map.remove(&node.span) {
        // non-component sub tree should be turned into a block
        if !is_component {
          codegen.is_block = true;
        }
        let ast = &context.ast;
        let codegen = NodeTypes::CacheExpression(
          ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.atom(&context.helper("withMemo"))),
            NONE,
            ast.vec_from_array([
              context.jsx_attribute_value_to_expression(&mut value).into(),
              ast
                .expression_arrow_function(
                  SPAN,
                  true,
                  false,
                  NONE,
                  ast.formal_parameters(
                    SPAN,
                    FormalParameterKind::ArrowFormalParameters,
                    ast.vec(),
                    NONE,
                  ),
                  NONE,
                  ast.function_body(
                    SPAN,
                    ast.vec(),
                    ast.vec1(
                      ast.statement_expression(SPAN, context.gen_vnode_call(codegen, codegen_map)),
                    ),
                  ),
                )
                .into(),
              ast.expression_identifier(SPAN, "_cache").into(),
              ast
                .expression_numeric_literal(
                  SPAN,
                  *context.cache_index.borrow() as f64,
                  None,
                  NumberBase::Hex,
                )
                .into(),
            ]),
            false,
          ),
        );
        *context.cache_index.borrow_mut() += 1;
        codegen_map.insert(node.span, codegen);
      }
    }));
  }
  None
}
