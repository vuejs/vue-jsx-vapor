use napi::{Either, bindgen_prelude::Either3};
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  NONE,
  ast::{
    AssignmentOperator, AssignmentTarget, BinaryExpression, BinaryOperator, BindingPatternKind,
    Expression, FormalParameterKind, FormalParameters, JSXAttribute, JSXAttributeValue, JSXChild,
    JSXElement, LogicalOperator, NumberBase, PropertyKind, Statement, VariableDeclarationKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{ConstantTypes, ForNode, NodeTypes, VNodeCall},
  transform::{
    TransformContext,
    cache_static::{cache_static_children, get_constant_type},
    utils::inject_prop,
  },
};
use common::{
  check::is_template,
  directive::{find_prop, find_prop_mut},
  error::ErrorCodes,
  expression::expression_to_params,
  patch_flag::PatchFlags,
  text::is_empty_text,
};

/// # SAFETY
pub unsafe fn transform_v_for<'a>(
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
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
  *context.options.in_v_for.borrow_mut() += 1;

  let is_template = is_template(unsafe { &*node });
  let memo = if let Some(memo_prop) =
    find_prop_mut(unsafe { &mut *node }, Either::A("v-memo".to_string()))
    && let Some(value) = &mut memo_prop.value
  {
    Some(context.jsx_attribute_value_to_expression(value))
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
      context.jsx_attribute_value_to_expression(&mut value.clone_in(context.allocator)),
      false,
      false,
      false,
    ))
  } else {
    None
  };

  let is_stable_fragment = (get_constant_type(
    Either::B(&source),
    context,
    &mut context.codegen_map.borrow_mut(),
  ) as i32)
    > ConstantTypes::NotConstant as i32;
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
  if let Some(NodeTypes::VNodeCall(vnode_call)) =
    context.codegen_map.borrow_mut().get_mut(&fragment_span)
  {
    vnode_call.v_for = true;
    vnode_call.patch_flag = Some(fragment_flag as i32);
    vnode_call.disable_tracking = !is_stable_fragment;
  } else {
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
        is_component: true,
        v_for: true,
        v_if: None,
        loc: node_span,
      }),
    );
  }

  Some(Box::new(move || {
    *context.options.in_v_for.borrow_mut() -= 1;
    // finish the codegen now that all children have been traversed
    let children = &mut unsafe { &mut *node }
      .children
      .iter_mut()
      .filter(|child| !is_empty_text(child))
      .collect::<Vec<_>>();

    let child_span = &children[0].span();
    let mut key_exp = None;
    // Normal element v-for. Directly use the child's codegenNode
    // but mark it as a block.
    if let NodeTypes::VNodeCall(child_block) = context
      .codegen_map
      .borrow_mut()
      .get_mut(child_span)
      .unwrap()
    {
      if is_template && let Some(key_property) = key_property {
        key_exp = Some(key_property.value.clone_in(context.allocator));
        inject_prop(child_block, key_property, context);
      }
      child_block.is_block = !is_stable_fragment;
    }
    cache_static_children(
      None,
      vec![children.get_mut(0).unwrap()],
      context,
      &mut context.codegen_map.borrow_mut(),
      false,
    );
    let child_block = context.codegen_map.borrow_mut().remove(child_span).unwrap();
    let child_block = match child_block {
      NodeTypes::VNodeCall(child_block) => {
        context.gen_vnode_call(child_block, &mut context.codegen_map.borrow_mut())
      }
      NodeTypes::CacheExpression(exp) => exp,
      _ => unreachable!(),
    };

    if let Some(memo) = memo {
      render_exp.arguments.push(
        ast
          .expression_arrow_function(
            SPAN,
            false,
            false,
            NONE,
            create_for_loop_params(
              value,
              key,
              index,
              Some(ast.expression_identifier(SPAN, "_cached")),
              context,
            ),
            NONE,
            ast.function_body(
              SPAN,
              ast.vec(),
              ast.vec_from_array([
                Statement::VariableDeclaration(ast.alloc_variable_declaration(
                  SPAN,
                  VariableDeclarationKind::Const,
                  ast.vec1(ast.variable_declarator(
                    SPAN,
                    VariableDeclarationKind::Const,
                    ast.binding_pattern(
                      ast.binding_pattern_kind_binding_identifier(SPAN, "_memo"),
                      NONE,
                      false,
                    ),
                    Some(ast.expression_parenthesized(SPAN, memo)),
                    false,
                  )),
                  false,
                )),
                Statement::IfStatement(ast.alloc_if_statement(
                  SPAN,
                  ast.expression_logical(
                    SPAN,
                    if let Some(key_exp) = key_exp {
                      ast.expression_logical(
                        SPAN,
                        ast.expression_identifier(SPAN, "_cached"),
                        LogicalOperator::And,
                        ast.expression_binary(
                          SPAN,
                          Expression::StaticMemberExpression(ast.alloc_static_member_expression(
                            SPAN,
                            ast.expression_identifier(SPAN, "_cached"),
                            ast.identifier_name(SPAN, "key"),
                            false,
                          )),
                          BinaryOperator::StrictEquality,
                          key_exp,
                        ),
                      )
                    } else {
                      ast.expression_identifier(SPAN, "_cached")
                    },
                    LogicalOperator::And,
                    ast.expression_call(
                      SPAN,
                      ast.expression_identifier(SPAN, ast.atom(&context.helper("isMemoSame"))),
                      NONE,
                      ast.vec_from_array([
                        ast.expression_identifier(SPAN, "_cached").into(),
                        ast.expression_identifier(SPAN, "_memo").into(),
                      ]),
                      false,
                    ),
                  ),
                  ast.statement_return(SPAN, Some(ast.expression_identifier(SPAN, "_cached"))),
                  None,
                )),
                Statement::VariableDeclaration(ast.alloc_variable_declaration(
                  SPAN,
                  VariableDeclarationKind::Const,
                  ast.vec1(ast.variable_declarator(
                    SPAN,
                    VariableDeclarationKind::Const,
                    ast.binding_pattern(
                      ast.binding_pattern_kind_binding_identifier(SPAN, "_item"),
                      NONE,
                      false,
                    ),
                    Some(child_block),
                    false,
                  )),
                  false,
                )),
                ast.statement_expression(
                  SPAN,
                  ast.expression_assignment(
                    SPAN,
                    AssignmentOperator::Assign,
                    AssignmentTarget::StaticMemberExpression(ast.alloc_static_member_expression(
                      SPAN,
                      ast.expression_identifier(SPAN, "_item"),
                      ast.identifier_name(SPAN, "memo"),
                      false,
                    )),
                    ast.expression_identifier(SPAN, "_memo"),
                  ),
                ),
                ast.statement_return(SPAN, Some(ast.expression_identifier(SPAN, "_item"))),
              ]),
            ),
          )
          .into(),
      );
      render_exp
        .arguments
        .push(ast.expression_identifier(SPAN, "_cache").into());
      render_exp.arguments.push(
        ast
          .expression_numeric_literal(
            SPAN,
            *context.cache_index.borrow() as f64,
            None,
            NumberBase::Hex,
          )
          .into(),
      );
      *context.cache_index.borrow_mut() += 1;
    } else {
      render_exp.arguments.push(
        ast
          .expression_arrow_function(
            SPAN,
            true,
            false,
            NONE,
            create_for_loop_params(value, key, index, None, context),
            NONE,
            ast.function_body(
              SPAN,
              ast.vec(),
              ast.vec1(ast.statement_expression(SPAN, child_block)),
            ),
          )
          .into(),
      );
    }

    if let Some(NodeTypes::VNodeCall(fragment_codegen)) =
      context.codegen_map.borrow_mut().get_mut(&fragment_span)
    {
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
  // scope management
  // inject identifiers to context
  if let Some(value) = value.as_ref() {
    context.add_identifiers(value);
  }
  if let Some(key) = key.as_ref() {
    context.add_identifiers(key);
  }
  if let Some(index) = index.as_ref() {
    context.add_identifiers(index);
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
  memo: Option<Expression<'a>>,
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
              *context.source.borrow(),
              context.allocator,
              context.options.source_type,
            )
          }
        } else if key.is_some() || index.is_some() || memo.is_some() {
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
        } else if index.is_some() || memo.is_some() {
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
        } else if memo.is_some() {
          Some(ast.formal_parameter(
            SPAN,
            ast.vec(),
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(ast.alloc_binding_identifier(SPAN, "___")),
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
        if let Some(Expression::Identifier(memo)) = memo {
          Some(ast.formal_parameter(
            SPAN,
            ast.vec(),
            ast.binding_pattern(
              BindingPatternKind::BindingIdentifier(
                ast.alloc_binding_identifier(memo.span, memo.name),
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
