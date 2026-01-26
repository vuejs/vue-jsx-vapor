use common::{
  directive::Directives,
  error::ErrorCodes,
  text::{get_tag_name, is_empty_text},
};
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{Expression, JSXChild, JSXElement},
};
use oxc_span::{GetSpan, SPAN};

use crate::{
  ast::{NodeTypes, VNodeCallChildren},
  transform::{
    TransformContext,
    cache_static::cache_static_children,
    transform_children::transform_children,
    transform_element::{PropsResult, build_props},
  },
};

pub fn transform_slot_outlet<'a>(
  directives: &mut Directives<'a>,
  node: &mut JSXElement<'a>,
  context: &'a TransformContext<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let tag = get_tag_name(&node.opening_element.name, context.source_text);
  if tag != "slot" {
    return None;
  }

  for value in &mut context.options.slot_identifiers.borrow_mut().values_mut() {
    value.2 = true;
  }

  let ast = context.ast;
  let node_span = node.span;
  let (slot_name, slot_props) =
    process_slot_outlet(directives, unsafe { &mut *(node as *mut _) }, context);

  let expression = NodeTypes::CacheExpression(
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("renderSlot"))),
      NONE,
      ast.vec_from_iter(
        [
          Some(
            ast
              .expression_identifier(SPAN, {
                *context.has_slot.borrow_mut() = true;
                "_slots"
              })
              .into(),
          ),
          Some(slot_name.into()),
          if let Some(slot_props) = slot_props {
            Some(slot_props.into())
          } else if !node.children.is_empty() {
            Some(ast.expression_object(SPAN, ast.vec()).into())
          } else {
            None
          },
          if node
            .children
            .iter()
            .filter(|child| !is_empty_text(child))
            .count()
            != 0
          {
            let mut fragment = ast.jsx_child_fragment(
              node_span,
              ast.jsx_opening_fragment(SPAN),
              node.children.take_in(context.allocator),
              ast.jsx_closing_fragment(SPAN),
            );
            unsafe {
              transform_children(&mut fragment, context);
            }
            let mut children = if let JSXChild::Fragment(fragment) = &mut fragment {
              fragment.children.take_in(context.allocator)
            } else {
              ast.vec()
            };
            let codegen_map = &mut context.codegen_map.borrow_mut();
            cache_static_children(None, &mut children, context, codegen_map, false);
            Some(
              ast
                .expression_arrow_function(
                  node.span,
                  true,
                  false,
                  NONE,
                  ast.formal_parameters(
                    SPAN,
                    oxc_ast::ast::FormalParameterKind::ArrowFormalParameters,
                    ast.vec(),
                    NONE,
                  ),
                  NONE,
                  ast.function_body(
                    SPAN,
                    ast.vec(),
                    ast.vec1(ast.statement_expression(
                      SPAN,
                      context.gen_node_list(VNodeCallChildren::B(&mut children), codegen_map),
                    )),
                  ),
                )
                .into(),
            )
          } else {
            None
          },
        ]
        .into_iter()
        .flatten(),
      ),
      false,
    ),
  );

  context
    .codegen_map
    .borrow_mut()
    .insert(node_span, expression);
  None
}

fn process_slot_outlet<'a>(
  directives: &mut Directives<'a>,
  node: &'a mut JSXElement<'a>,
  context: &'a TransformContext<'a>,
) -> (Expression<'a>, Option<Expression<'a>>) {
  let ast = context.ast;
  let mut slot_name = ast.expression_string_literal(SPAN, "default", None);
  let mut slot_props = None;

  let props = &mut node.opening_element.attributes;
  if !props.is_empty() {
    let PropsResult {
      props,
      directives,
      mut name_prop,
      ..
    } = build_props(directives, node, context, false, true);
    if let Some(name_prop) = &mut name_prop
      && let Some(value) = &mut name_prop.value
    {
      slot_name = context.jsx_attribute_value_to_expression(value);
    }
    slot_props = props;

    if let Some(directives) = directives
      && !directives.elements.is_empty()
    {
      context.options.on_error.as_ref()(
        ErrorCodes::VSlotUnexpectedDirectiveOnSlotOutlet,
        directives.elements.first().unwrap().span(),
      );
    }
  }

  return (slot_name, slot_props);
}
