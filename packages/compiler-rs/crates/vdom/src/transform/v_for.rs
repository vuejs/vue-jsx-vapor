use napi::{Either, bindgen_prelude::Either3};
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  NONE,
  ast::{
    BinaryExpression, BindingPatternKind, Expression, FormalParameterKind, FormalParameters,
    JSXAttribute, JSXAttributeValue, JSXChild, JSXElement, ObjectPropertyKind, PropertyKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{ConstantTypes, ForNode, NodeTypes, VNodeCall},
  ir::index::BlockIRNode,
  transform::{TransformContext, cache_static::get_constant_type, utils::inject_prop},
};
use common::{
  check::is_template,
  directive::{find_prop, find_prop_mut},
  error::ErrorCodes,
  expression::{expression_to_params, jsx_attribute_value_to_expression},
  patch_flag::PatchFlags,
  text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_for<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  _: &'a mut BlockIRNode<'a>,
  _: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let node = node as *mut oxc_allocator::Box<JSXElement>;
  if is_template(unsafe { &*node })
    && find_prop(unsafe { &*node }, Either::A("v-slot".to_string())).is_some()
  {
    return None;
  }

  let dir = find_prop_mut(unsafe { &mut *node }, Either::A("v-for".to_string()))?;
  let seen = &mut context.seen.borrow_mut();
  let span = dir.span;
  if seen.contains(&span.start) {
    return None;
  }
  seen.insert(span.start);

  let ForNode {
    value,
    key,
    index,
    source,
  } = get_for_parse_result(dir, context)?;

  let Some(source) = source else {
    context.options.on_error.as_ref()(ErrorCodes::VForMalformedExpression, span);
    return None;
  };

  let ast = &context.ast;

  // bookkeeping
  *context.in_v_for.borrow_mut() += 1;

  let is_template = is_template(unsafe { &*node });
  let memo = if let Some(memo_prop) =
    find_prop_mut(unsafe { &mut *node }, Either::A("memo".to_string()))
    && let Some(value) = &mut memo_prop.value
  {
    Some(jsx_attribute_value_to_expression(
      value.take_in(context.allocator),
      context.allocator,
    ))
  } else {
    None
  };
  let key_property = if let Some(key_prop) =
    find_prop_mut(unsafe { &mut *node }, Either::A("key".to_string()))
    && let Some(value) = &mut key_prop.value
  {
    Some(ast.object_property(
      SPAN,
      PropertyKind::Init,
      ast.property_key_static_identifier(SPAN, ast.atom("key")),
      jsx_attribute_value_to_expression(value.clone_in(context.allocator), context.allocator),
      false,
      false,
      false,
    ))
  } else {
    None
  };

  let is_stable_fragment =
    (get_constant_type(Either::B(&source), context) as i32) > ConstantTypes::NotConstant as i32;
  let fragment_flag = if is_stable_fragment {
    PatchFlags::StableFragment
  } else if key_property.is_some() {
    PatchFlags::KeyedFragment
  } else {
    PatchFlags::UnkeyedFragment
  };

  // create the loop render function expression now, and add the
  // iterator on exit after all children have been traversed
  let mut render_exp = ast.call_expression(
    SPAN,
    ast.expression_identifier(SPAN, ast.atom(&context.helper("renderList"))),
    NONE,
    ast.vec1(source.into()),
    false,
  );

  let node_span = unsafe { &*node }.span;
  let fragment_span = Span::new(node_span.end, node_span.start);
  *unsafe { &mut *context_node } = context.wrap_fragment(
    Expression::JSXElement(unsafe { &mut *node }.take_in_box(context.allocator)),
    fragment_span,
  );
  context.codegen_map.borrow_mut().insert(
    fragment_span,
    NodeTypes::VNodeCall(VNodeCall {
      tag: context.helper("Fragment"),
      props: None,
      children: None,
      patch_flag: Some(fragment_flag as i32),
      dynamic_props: None,
      directives: None,
      is_block: true,
      disable_tracking: !is_stable_fragment,
      is_component: false,
      v_for: Some(true),
      v_if: None,
      loc: node_span,
    }),
  );

  Some(Box::new(move || {
    *context.in_v_for.borrow_mut() -= 1;
    // finish the codegen now that all children have been traversed
    let children = &mut unsafe { &mut *node }
      .children
      .iter()
      .filter(|child| {
        // check <template v-for> key placement
        if is_template
          && let JSXChild::Element(child) = child
          && let Some(key) = find_prop(child, Either::A(String::from("key")))
        {
          context.options.on_error.as_ref()(ErrorCodes::VForTemplateKeyPlacement, key.span);
          true
        } else {
          !is_empty_text(child)
        }
      })
      .collect::<Vec<_>>();

    let need_fragment_wrapper = children.len() != 1 || !matches!(children[0], JSXChild::Element(_));
    let child_block = if need_fragment_wrapper {
      // <template v-for="..."> with text or multi-elements
      // should generate a fragment block for each loop
      VNodeCall {
        tag: context.helper("Fragment"),
        props: key_property.map(|key_property| {
          ast.expression_object(
            SPAN,
            ast.vec1(ObjectPropertyKind::ObjectProperty(ast.alloc(key_property))),
          )
        }),
        children: Some(Either3::B(&mut unsafe { &mut *node }.children)),
        patch_flag: Some(PatchFlags::StableFragment as i32),
        dynamic_props: None,
        directives: None,
        is_block: true,
        disable_tracking: false,
        is_component: false,
        v_for: None,
        v_if: None,
        loc: SPAN,
      }
    } else {
      // Normal element v-for. Directly use the child's codegenNode
      // but mark it as a block.
      let NodeTypes::VNodeCall(mut child_block) = context
        .codegen_map
        .borrow_mut()
        .remove(&children[0].span())
        .unwrap()
      else {
        unreachable!()
      };
      if is_template && let Some(key_property) = key_property {
        inject_prop(&mut child_block, key_property, context);
      }
      child_block.is_block = !is_stable_fragment;
      child_block
    };

    render_exp.arguments.push(
      ast
        .expression_arrow_function(
          SPAN,
          true,
          false,
          NONE,
          create_for_loop_params(value, key, index, context),
          NONE,
          ast.function_body(
            SPAN,
            ast.vec(),
            ast.vec1(ast.statement_expression(
              SPAN,
              context.gen_vnode_call(child_block, &mut context.codegen_map.borrow_mut()),
            )),
          ),
        )
        .into(),
    );

    if let Some(NodeTypes::VNodeCall(fragment_codegen)) =
      context.codegen_map.borrow_mut().get_mut(&fragment_span)
    {
      if need_fragment_wrapper {
        fragment_codegen.v_for = Some(false);
      }
      fragment_codegen.children = Some(Either3::C(Expression::CallExpression(
        ast.alloc(render_exp),
      )));
    };
  }))
}

pub fn get_for_parse_result<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<ForNode<'a>> {
  let mut value = None;
  let mut index = None;
  let mut key = None;
  let mut source = None;
  if let Some(dir_value) = &mut dir.value {
    let expression = if let JSXAttributeValue::ExpressionContainer(dir_value) = dir_value {
      Some(
        dir_value
          .expression
          .to_expression_mut()
          .without_parentheses_mut()
          .get_inner_expression_mut(),
      )
    } else {
      None
    };
    if let Some(expression) = expression
      && let Expression::BinaryExpression(expression) = expression
    {
      let expression = expression as *mut oxc_allocator::Box<BinaryExpression>;
      let left = unsafe { &mut *expression }
        .left
        .without_parentheses_mut()
        .get_inner_expression_mut();
      if let Expression::SequenceExpression(left) = left {
        let expressions = &mut left.expressions as *mut oxc_allocator::Vec<Expression>;
        value = unsafe { &mut *expressions }
          .get_mut(0)
          .map(|e| e.take_in(context.allocator));
        key = unsafe { &mut *expressions }
          .get_mut(1)
          .map(|e| e.take_in(context.allocator));
        index = unsafe { &mut *expressions }
          .get_mut(2)
          .map(|e| e.take_in(context.allocator));
      } else {
        value = Some(left.take_in(context.allocator));
      };
      source = Some(unsafe { &mut *expression }.right.take_in(context.allocator));
    }
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VForNoExpression, dir.span);
    return None;
  }
  Some(ForNode {
    value,
    index,
    key,
    source,
  })
}

pub fn create_for_loop_params<'a>(
  value: Option<Expression<'a>>,
  key: Option<Expression<'a>>,
  index: Option<Expression<'a>>,
  context: &TransformContext<'a>,
) -> FormalParameters<'a> {
  let ast = &context.ast;
  ast.formal_parameters(
    SPAN,
    FormalParameterKind::ArrowFormalParameters,
    ast.vec_from_iter(
      [
        if let Some(value) = value {
          if let Expression::Identifier(value) = value {
            Some(ast.formal_parameter(
              SPAN,
              ast.vec(),
              ast.binding_pattern(
                BindingPatternKind::BindingIdentifier(
                  ast.alloc_binding_identifier(value.span, value.name),
                ),
                NONE,
                false,
              ),
              None,
              false,
              false,
            ))
          } else {
            expression_to_params(
              &value,
              context.ir.borrow().source,
              context.allocator,
              context.options.source_type,
            )
          }
        } else if key.is_some() || index.is_some() {
          Some(ast.formal_parameter(
            SPAN,
            ast.vec(),
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(ast.alloc_binding_identifier(SPAN, "_")),
              NONE,
              false,
            ),
            None,
            false,
            false,
          ))
        } else {
          None
        },
        if let Some(Expression::Identifier(key)) = key {
          Some(ast.formal_parameter(
            SPAN,
            ast.vec(),
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(
                ast.alloc_binding_identifier(key.span, key.name),
              ),
              NONE,
              false,
            ),
            None,
            false,
            false,
          ))
        } else if index.is_some() {
          Some(ast.formal_parameter(
            SPAN,
            ast.vec(),
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(ast.alloc_binding_identifier(SPAN, "__")),
              NONE,
              false,
            ),
            None,
            false,
            false,
          ))
        } else {
          None
        },
        if let Some(Expression::Identifier(index)) = index {
          Some(ast.formal_parameter(
            SPAN,
            ast.vec(),
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(
                ast.alloc_binding_identifier(index.span, index.name),
              ),
              NONE,
              false,
            ),
            None,
            false,
            false,
          ))
        } else {
          None
        },
      ]
      .into_iter()
      .flatten(),
    ),
    NONE,
  )
}
