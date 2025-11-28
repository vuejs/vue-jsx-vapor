use std::{collections::HashMap, ops::Deref};

use napi::bindgen_prelude::Either16;
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  NONE,
  ast::{
    Argument, ArrayExpression, AssignmentOperator, AssignmentTarget, BinaryExpression,
    BinaryOperator, BindingPatternKind, Expression, FormalParameterKind, NumberBase,
    ObjectExpression, ObjectPropertyKind, PropertyKey, Statement, VariableDeclarationKind,
  },
};
use oxc_ast_visit::Visit;
use oxc_span::{GetSpan, SPAN, Span};
use oxc_traverse::{
  Ancestor,
  ancestor::{ArrayExpressionWithoutElements, ObjectExpressionWithoutProperties},
};

use crate::{
  generate::{
    CodegenContext, block::gen_block_content, expression::gen_expression, operation::gen_operation,
  },
  ir::index::{BlockIRNode, ForIRNode, IREffect, SimpleExpressionNode},
  utils::{expression::is_globally_allowed, walk::WalkIdentifiers},
};

/**
 * Flags to optimize vapor `createFor` runtime behavior, shared between the
 * compiler and the runtime
 */
pub enum VaporVForFlags {
  /**
   * v-for is the only child of a parent container, so it can take the fast
   * path with textContent = '' when the whole list is emptied
   */
  FastRemove = 1,
  /**
   * v-for used on component - we can skip creating child scopes for each block
   * because the component itself already has a scope.
   */
  IsComponent = 1 << 1,
  /**
   * v-for inside v-ince
   */
  Once = 1 << 2,
}

pub fn gen_for<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  oper: ForIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
  let ast = &context.ast;
  let ForIRNode {
    source,
    value,
    key,
    index,
    id,
    key_prop,
    mut render,
    once,
    component,
    only_child,
    ..
  } = oper;

  let (raw_key, key_span) = if let Some(key) = key {
    (Some(key.content), key.loc)
  } else {
    (None, SPAN)
  };
  let (raw_index, index_span) = if let Some(index) = index {
    (Some(index.content), index.loc)
  } else {
    (None, SPAN)
  };
  let (raw_value, value_span, value_ast) = if let Some(value) = value {
    (value.content, value.loc, value.ast)
  } else {
    (String::new(), SPAN, None)
  };

  let source_expr = ast.expression_arrow_function(
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
      ast.vec1(ast.statement_expression(SPAN, gen_expression(source, context, None, None))),
    ),
  );

  let (depth, exit_scope) = context.enter_scope();
  let mut id_map: HashMap<String, Option<Expression>> = HashMap::new();
  let item_var = format!("_for_item{depth}");
  id_map.insert(item_var.clone(), None);

  let _id_map = &mut id_map as *mut HashMap<String, Option<Expression>>;
  let _item_var = item_var.clone();
  // construct a id -> accessor path map.
  // e.g. `{ x: { y: [z] }}` -> `Map{ 'z' => '.x.y[0]' }`
  if !raw_value.is_empty() {
    if let Some(_ast) = value_ast
      && !matches!(_ast, Expression::Identifier(_))
    {
      WalkIdentifiers::new(
        context,
        Box::new(move |id, _, ancestry, _, _| {
          let mut path = ast
            .member_expression_static(
              id.span(),
              ast.expression_identifier(SPAN, ast.atom(&_item_var)),
              ast.identifier_name(SPAN, "value"),
              false,
            )
            .into();
          let mut parent_stack = ancestry.ancestors().collect::<Vec<_>>();
          parent_stack.reverse();
          for i in 0..parent_stack.len() {
            let parent = parent_stack[i];
            let child = parent_stack.get(i + 1);
            let child_is_spread = if let Some(child) = child {
              matches!(child, Ancestor::SpreadElementArgument(_))
            } else {
              false
            };

            if let Ancestor::ObjectPropertyValue(parent) = parent {
              if let PropertyKey::StringLiteral(key) = &parent.key() {
                path = ast
                  .member_expression_computed(
                    SPAN,
                    path,
                    ast.expression_identifier(SPAN, key.value),
                    false,
                  )
                  .into();
              } else if let PropertyKey::StaticIdentifier(key) = &parent.key() {
                // non-computed, can only be identifier
                path = ast
                  .member_expression_static(SPAN, path, key.deref().clone_in(ast.allocator), false)
                  .into()
              }
            } else if let Ancestor::ArrayExpressionElements(parent) = &parent {
              let elements = unsafe {
                let parent = *((parent as *const ArrayExpressionWithoutElements)
                  as *const *const ArrayExpression);
                &(*parent).elements
              };
              let index = elements
                .iter()
                .position(|element| {
                  if let Some(child) = child {
                    let span = match child {
                      Ancestor::SpreadElementArgument(e) => e.span(),
                      Ancestor::ArrayExpressionElements(e) => e.span(),
                      Ancestor::ObjectExpressionProperties(e) => e.span(),
                      _ => unimplemented!(),
                    };
                    element.span().eq(span)
                  } else {
                    element.span().eq(&id.span())
                  }
                })
                .unwrap();
              if child_is_spread {
                path = ast.expression_call(
                  SPAN,
                  ast
                    .member_expression_static(SPAN, path, ast.identifier_name(SPAN, "slice"), false)
                    .into(),
                  NONE,
                  ast.vec1(Argument::NumericLiteral(ast.alloc_numeric_literal(
                    SPAN,
                    index as f64,
                    None,
                    NumberBase::Hex,
                  ))),
                  false,
                );
              } else {
                path = ast
                  .member_expression_computed(
                    SPAN,
                    path,
                    ast.expression_numeric_literal(SPAN, index as f64, None, NumberBase::Hex),
                    false,
                  )
                  .into();
              }
            } else if let Ancestor::ObjectExpressionProperties(parent) = &parent
              && child_is_spread
            {
              let properties = unsafe {
                let parent = *((parent as *const ObjectExpressionWithoutProperties)
                  as *const *const ObjectExpression);
                &(*parent).properties
              };
              unsafe { &mut *_id_map }.insert("getRestElement".to_string(), None);
              path = ast.expression_call(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&context.helper("getRestElement"))),
                NONE,
                ast.vec_from_array([
                  path.into(),
                  ast
                    .expression_array(
                      SPAN,
                      ast.vec_from_iter(properties.iter().filter_map(|p| {
                        if let ObjectPropertyKind::ObjectProperty(p) = p {
                          Some(if let PropertyKey::StringLiteral(key) = &p.key {
                            ast.expression_string_literal(SPAN, key.value, None).into()
                          } else {
                            ast
                              .expression_string_literal(
                                SPAN,
                                ast.atom(&p.key.name().unwrap()),
                                None,
                              )
                              .into()
                          })
                        } else {
                          None
                        }
                      })),
                    )
                    .into(),
                ]),
                false,
              );
            }
          }
          unsafe { &mut *_id_map }.insert(
            id.span().source_text(context.ir.source).to_string(),
            Some(path),
          );
          None
        }),
        false,
      )
      .traverse(_ast.take_in(context.ast.allocator));
    } else {
      id_map.insert(
        raw_value.clone(),
        Some(
          ast
            .member_expression_static(
              value_span,
              ast.expression_identifier(SPAN, ast.atom(&item_var)),
              ast.identifier_name(SPAN, "value"),
              false,
            )
            .into(),
        ),
      );
    }
  }

  let mut args: Vec<String> = vec![];
  args.push(item_var);
  if let Some(raw_key) = raw_key.clone() {
    let key_var = format!("_for_key{depth}");
    args.push(key_var.clone());
    id_map.insert(
      raw_key,
      Some(
        ast
          .member_expression_static(
            key_span,
            ast.expression_identifier(SPAN, ast.atom(&key_var)),
            ast.identifier_name(SPAN, "value"),
            false,
          )
          .into(),
      ),
    );
    id_map.insert(key_var, None);
  }
  if let Some(raw_index) = raw_index.clone() {
    let index_var = format!("_for_index{depth}");
    args.push(index_var.clone());
    id_map.insert(
      raw_index,
      Some(
        ast
          .member_expression_static(
            index_span,
            ast.expression_identifier(SPAN, ast.atom(&index_var)),
            ast.identifier_name(SPAN, "value"),
            false,
          )
          .into(),
      ),
    );
    id_map.insert(index_var.to_string(), None);
  }

  let (effect_patterns, selector_patterns, key_only_binding_patterns) =
    match_patterns(&mut render, &key_prop, &id_map, context);
  let mut selector_declarations = ast.vec();
  let mut selector_setup = ast.vec();

  for (i, selector) in selector_patterns.into_iter().enumerate() {
    let selector_name = format!("_selector{id}_{i}");
    selector_declarations.push(Statement::VariableDeclaration(
      ast.alloc_variable_declaration(
        SPAN,
        VariableDeclarationKind::Let,
        ast.vec1(ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Let,
          ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(
              ast.alloc_binding_identifier(SPAN, ast.atom(&selector_name)),
            ),
            NONE,
            false,
          ),
          None,
          false,
        )),
        false,
      ),
    ));
    selector_setup.push(ast.statement_expression(
      SPAN,
      ast.expression_assignment(
        SPAN,
        AssignmentOperator::Assign,
        AssignmentTarget::AssignmentTargetIdentifier(
          ast.alloc_identifier_reference(SPAN, ast.atom(&selector_name)),
        ),
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom("createSelector")),
          NONE,
          ast.vec1(Argument::ArrowFunctionExpression(
            ast.alloc_arrow_function_expression(
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
                  ast.statement_expression(SPAN, gen_expression(selector, context, None, None)),
                ),
              ),
            ),
          )),
          false,
        ),
      ),
    ));
  }

  let block_fn = context.with_id(
    || {
      ast.expression_arrow_function(
        SPAN,
        false,
        false,
        NONE,
        ast.formal_parameters(
          SPAN,
          FormalParameterKind::ArrowFormalParameters,
          ast.vec_from_iter(args.into_iter().map(|arg| {
            ast.formal_parameter(
              SPAN,
              ast.vec(),
              ast.binding_pattern(
                BindingPatternKind::BindingIdentifier(
                  ast.alloc_binding_identifier(SPAN, ast.atom(&arg)),
                ),
                NONE,
                false,
              ),
              None,
              false,
              false,
            )
          })),
          NONE,
        ),
        NONE,
        ast.function_body(
          SPAN,
          ast.vec(),
          if !effect_patterns.is_empty() || !key_only_binding_patterns.is_empty() {
            gen_block_content(
              Some(render),
              context,
              context_block,
              false,
              Some(Box::new(move |statements, context_block| {
                for (i, effect) in effect_patterns.into_iter().enumerate() {
                  let mut body = ast.vec();
                  for oper in effect.operations {
                    let _context_block = context_block as *mut BlockIRNode;
                    gen_operation(
                      &mut body,
                      oper,
                      context,
                      unsafe { &mut *_context_block },
                      &vec![],
                    );
                  }
                  statements.push(ast.statement_expression(
                    SPAN,
                    ast.expression_call(
                      SPAN,
                      ast.expression_identifier(SPAN, ast.atom(&format!("_selector{id}_{i}"))),
                      NONE,
                      ast.vec1(Argument::ArrowFunctionExpression(
                        ast.alloc_arrow_function_expression(
                          SPAN,
                          false,
                          false,
                          NONE,
                          ast.formal_parameters(
                            SPAN,
                            FormalParameterKind::ArrowFormalParameters,
                            ast.vec(),
                            NONE,
                          ),
                          NONE,
                          ast.function_body(SPAN, ast.vec(), body),
                        ),
                      )),
                      false,
                    ),
                  ));
                }

                for effect in key_only_binding_patterns {
                  for oper in effect.operations {
                    let _context_block = context_block as *mut BlockIRNode;
                    gen_operation(
                      statements,
                      oper,
                      context,
                      unsafe { &mut *_context_block },
                      &vec![],
                    )
                  }
                }
              })),
            )
          } else {
            gen_block_content(Some(render), context, context_block, false, None)
          },
        ),
      )
    },
    id_map,
  );
  exit_scope();

  let mut flags = 0;
  if only_child {
    flags |= VaporVForFlags::FastRemove as i32;
  }
  if component {
    flags |= VaporVForFlags::IsComponent as i32;
  }
  if once {
    flags |= VaporVForFlags::Once as i32;
  }

  let gen_callback = if let Some(key_prop) = key_prop {
    let res = context.with_id(
      || gen_expression(key_prop, context, None, None),
      HashMap::new(),
    );

    Some(
      ast.expression_arrow_function(
        SPAN,
        true,
        false,
        NONE,
        ast.formal_parameters(
          SPAN,
          FormalParameterKind::ArrowFormalParameters,
          ast.vec_from_iter(
            [
              if !raw_value.is_empty() {
                Some(ast.formal_parameter(
                  SPAN,
                  ast.vec(),
                  ast.binding_pattern(
                    BindingPatternKind::BindingIdentifier(
                      ast.alloc_binding_identifier(value_span, ast.atom(&raw_value)),
                    ),
                    NONE,
                    false,
                  ),
                  None,
                  false,
                  false,
                ))
              } else if raw_key.is_some() || raw_index.is_some() {
                Some(ast.formal_parameter(
                  SPAN,
                  ast.vec(),
                  ast.binding_pattern(
                    BindingPatternKind::BindingIdentifier(
                      ast.alloc_binding_identifier(SPAN, ast.atom("_")),
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
              if let Some(raw_key) = raw_key {
                Some(ast.formal_parameter(
                  SPAN,
                  ast.vec(),
                  ast.binding_pattern(
                    BindingPatternKind::BindingIdentifier(
                      ast.alloc_binding_identifier(key_span, ast.atom(&raw_key)),
                    ),
                    NONE,
                    false,
                  ),
                  None,
                  false,
                  false,
                ))
              } else if raw_index.is_some() {
                Some(ast.formal_parameter(
                  SPAN,
                  ast.vec(),
                  ast.binding_pattern(
                    BindingPatternKind::BindingIdentifier(
                      ast.alloc_binding_identifier(SPAN, ast.atom("__")),
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
              raw_index.map(|raw_index| {
                ast.formal_parameter(
                  SPAN,
                  ast.vec(),
                  ast.binding_pattern(
                    BindingPatternKind::BindingIdentifier(
                      ast.alloc_binding_identifier(index_span, ast.atom(&raw_index)),
                    ),
                    NONE,
                    false,
                  ),
                  None,
                  false,
                  false,
                )
              }),
            ]
            .into_iter()
            .flatten(),
          ),
          NONE,
        ),
        NONE,
        ast.function_body(
          SPAN,
          ast.vec(),
          ast.vec1(ast.statement_expression(SPAN, res)),
        ),
      ),
    )
  } else if flags > 0 {
    Some(ast.expression_identifier(SPAN, "void 0"))
  } else {
    None
  };

  let ast = &context.ast;

  statements.extend(selector_declarations);

  let selector_setup_expression = if !selector_setup.is_empty() {
    Some(
      ast
        .expression_arrow_function(
          SPAN,
          false,
          false,
          NONE,
          ast.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            ast.vec1(ast.formal_parameter(
              SPAN,
              ast.vec(),
              ast.binding_pattern(
                BindingPatternKind::ObjectPattern(ast.alloc_object_pattern(
                  SPAN,
                  ast.vec1(ast.binding_property(
                    SPAN,
                    ast.property_key_static_identifier(SPAN, ast.atom("createSelector")),
                    ast.binding_pattern(
                      BindingPatternKind::BindingIdentifier(
                        ast.alloc_binding_identifier(SPAN, ast.atom("createSelector")),
                      ),
                      NONE,
                      false,
                    ),
                    true,
                    false,
                  )),
                  NONE,
                )),
                NONE,
                false,
              ),
              None,
              false,
              false,
            )),
            NONE,
          ),
          NONE,
          ast.function_body(SPAN, ast.vec(), selector_setup),
        )
        .into(),
    )
  } else {
    None
  };

  statements.push(Statement::VariableDeclaration(
    ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(
        ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern(
            BindingPatternKind::BindingIdentifier(
              ast.alloc_binding_identifier(SPAN, ast.atom(&format!("n{id}"))),
            ),
            NONE,
            false,
          ),
          Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("createFor"))),
              NONE,
              ast.vec_from_iter(
                [
                  Some(source_expr.into()),
                  Some(block_fn.into()),
                  gen_callback.map(|i| i.into()),
                  if flags > 0 {
                    Some(
                      ast
                        .expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Hex)
                        .into(),
                    )
                  } else if selector_setup_expression.is_some() {
                    Some(ast.expression_identifier(SPAN, ast.atom("void 0")).into())
                  } else {
                    None
                  },
                  selector_setup_expression,
                  // todo: hydrationNode
                ]
                .into_iter()
                .flatten(),
              ),
              false,
            ),
          ),
          false,
        ),
      ),
      false,
    ),
  ))
}

fn match_patterns<'a>(
  render: &mut BlockIRNode<'a>,
  key_prop: &Option<SimpleExpressionNode<'a>>,
  id_map: &HashMap<String, Option<Expression<'a>>>,
  context: &CodegenContext,
) -> (
  Vec<IREffect<'a>>,
  Vec<SimpleExpressionNode<'a>>,
  Vec<IREffect<'a>>,
) {
  let mut effect_patterns = vec![];
  let mut selector_patterns = vec![];
  let mut key_only_binding_patterns = vec![];

  if let Some(key_prop) = key_prop {
    let effects = &mut render.effect;
    let _effects = effects as *mut Vec<IREffect>;
    for i in 0..effects.len() {
      let effect = unsafe { &mut *_effects }.get(i).unwrap();
      if let Some(selector) = match_selector_pattern(effect, &key_prop.content, id_map, context) {
        effect_patterns.push(effects.remove(i));
        selector_patterns.push(selector);
      } else if !effect.operations.is_empty()
        && let Some(ast) = &get_expression(effect).unwrap().ast
        && key_prop
          .content
          .eq(ast.span().source_text(context.ir.source))
      {
        key_only_binding_patterns.push(effects.remove(i));
      }
    }
  }

  (
    effect_patterns,
    selector_patterns,
    key_only_binding_patterns,
  )
}

fn match_selector_pattern<'a>(
  effect: &'a IREffect<'a>,
  key: &str,
  id_map: &HashMap<String, Option<Expression<'a>>>,
  context: &CodegenContext,
) -> Option<SimpleExpressionNode<'a>> {
  let source = context.ir.source;
  if effect.operations.len() != 1 {
    return None;
  }
  let expression = get_expression(effect)?;
  let Some(ast) = &expression.ast else {
    return None;
  };

  let offset = ast.span().start;

  let mut matcheds: Vec<(Span, Span)> = vec![];

  BinaryExpressionVisitor {
    on_binary_expression: Box::new(|ast| {
      if matches!(
        ast.operator,
        BinaryOperator::Equality | BinaryOperator::StrictEquality
      ) {
        let left = &ast.left;
        let right = &ast.right;
        let left_is_key = key.eq(&source[left.span().start as usize..left.span().end as usize]);
        let right_is_key = key.eq(&source[right.span().start as usize..right.span().end as usize]);
        if left_is_key
          && !right_is_key
          && analyze_variable_scopes(right, id_map, context).is_empty()
        {
          matcheds.push((left.span(), right.span()));
        } else if right_is_key
          && !left_is_key
          && analyze_variable_scopes(left, id_map, context).is_empty()
        {
          matcheds.push((right.span(), left.span()));
        }
      }
    }),
  }
  .visit_expression(ast);

  if matcheds.len() == 1 {
    let (key, selector) = matcheds[0];

    let mut has_extra_id = false;
    WalkIdentifiers::new(
      context,
      Box::new(|id, _, _, _, _| {
        let start = id.span.start;
        if start != key.start && start != selector.start {
          has_extra_id = true
        }
        None
      }),
      false,
    )
    .traverse(ast.clone_in(context.ast.allocator));

    if !has_extra_id {
      let content = expression.content
        [(selector.start - offset) as usize..(selector.end - offset) as usize]
        .to_string();
      return Some(SimpleExpressionNode {
        content,
        ast: None,
        loc: SPAN,
        is_static: false,
      });
    }
  }
  None
}

fn analyze_variable_scopes(
  ast: &Expression,
  id_map: &HashMap<String, Option<Expression>>,
  context: &CodegenContext,
) -> Vec<String> {
  let mut locals = vec![];
  WalkIdentifiers::new(
    context,
    Box::new(|id, _, _, _, _| {
      let name = id.name.to_string();
      if !is_globally_allowed(&name) && id_map.get(&name).is_some() {
        locals.push(name);
      }
      None
    }),
    false,
  )
  .traverse(ast.clone_in(context.ast.allocator));

  locals
}

fn get_expression<'a>(effect: &'a IREffect<'a>) -> Option<&'a SimpleExpressionNode<'a>> {
  let operation = effect.operations.first();
  match operation.as_ref().unwrap() {
    Either16::C(operation) => operation.values.first(),
    Either16::G(operation) => operation.values.first(),
    Either16::K(operation) => operation.values.first(),
    Either16::I(operation) => Some(&operation.value),
    Either16::H(operation) => operation.value.as_ref(),
    Either16::F(operation) => Some(&operation.value),
    Either16::J(operation) => Some(&operation.value),
    Either16::D(operation) => operation.prop.values.first(),
    _ => None,
  }
}

struct BinaryExpressionVisitor<'a> {
  on_binary_expression: Box<dyn FnMut(&BinaryExpression) + 'a>,
}
impl<'a> Visit<'a> for BinaryExpressionVisitor<'a> {
  fn visit_binary_expression(&mut self, node: &BinaryExpression) {
    self.on_binary_expression.as_mut()(node)
  }
}
