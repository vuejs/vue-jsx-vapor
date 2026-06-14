use std::collections::HashMap;

use common::{expression::is_globally_allowed, patch_flag::VaporVForFlags, walk::WalkIdentifiers};
use oxc_allocator::CloneIn;
use oxc_ast::{
  NONE,
  ast::{
    Argument, BinaryExpression, BinaryOperator, Expression, FormalParameterKind, NumberBase,
    Statement, VariableDeclarationKind,
  },
};
use oxc_ast_visit::Visit;
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  generate::{
    CodegenContext, block::gen_block_content, expression::gen_expression, operation::gen_operation,
  },
  ir::index::{BlockIRNode, ForIRNode, IRDynamicInfo, IREffect, OperationNode},
};

pub fn gen_for<'a>(
  statements: &mut oxc_allocator::Vec<'a, Statement<'a>>,
  oper: ForIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
  let ast = &context.ast;
  let ForIRNode {
    source,
    mut value,
    key,
    index,
    id,
    key_prop,
    mut render,
    once,
    component,
    only_child,
    slot_root,
    ..
  } = oper;

  let (raw_key, key_span) = if let Some(Expression::Identifier(key)) = key {
    let span = key.span();
    (Some(span.source_text(context.source_text)), span)
  } else {
    (None, SPAN)
  };
  let (raw_index, index_span) = if let Some(index) = index {
    let span = index.span();
    (Some(span.source_text(context.source_text)), span)
  } else {
    (None, SPAN)
  };
  let (raw_value, value_span) = if let Some(value) = &value {
    let span = value.span();
    (Some(span.source_text(context.source_text)), span)
  } else {
    (None, SPAN)
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
      ast.vec1(ast.statement_expression(SPAN, gen_expression(source, context, None, false))),
    ),
  );

  let (depth, exit_scope) = context.enter_scope();
  let item_var = format!("_for_item{depth}");
  let mut id_map = context.parse_value_destructure(
    value.as_mut(),
    ast
      .member_expression_static(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(&item_var)),
        ast.identifier_name(SPAN, "value"),
        false,
      )
      .into(),
  );

  let mut args: Vec<String> = vec![];
  args.push(item_var);
  if let Some(raw_key) = raw_key {
    let key_var = format!("_for_key{depth}");
    id_map.insert(
      raw_key,
      ast
        .member_expression_static(
          key_span,
          ast.expression_identifier(SPAN, ast.str(&key_var)),
          ast.identifier_name(SPAN, "value"),
          false,
        )
        .into(),
    );
    args.push(key_var);
  }
  if let Some(raw_index) = raw_index {
    let index_var = format!("_for_index{depth}");
    id_map.insert(
      raw_index,
      ast
        .member_expression_static(
          index_span,
          ast.expression_identifier(SPAN, ast.str(&index_var)),
          ast.identifier_name(SPAN, "value"),
          false,
        )
        .into(),
    );
    args.push(index_var);
  }

  let (effect_patterns, selector_patterns, key_only_binding_patterns) =
    match_patterns(&mut render, &key_prop, &id_map, context);
  let key_prop_for_effects = if !selector_patterns.is_empty() {
    key_prop.as_ref().map(|expr| expr.clone_in(ast.allocator))
  } else {
    None
  };
  let mut selector_declarations = ast.vec();

  let selector_patterns_len = selector_patterns.len();
  let mut on_reset_calls = ast.vec();
  for (i, selector) in selector_patterns.into_iter().enumerate() {
    let selector_name = ast.str(&if selector_patterns_len > 1 {
      format!("_selector{id}_{i}")
    } else {
      format!("_selector{id}")
    });
    selector_declarations.push(Statement::VariableDeclaration(
      ast.alloc_variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ast.vec1(ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern_binding_identifier(SPAN, selector_name),
          NONE,
          Some(ast.expression_call(
            SPAN,
            ast.expression_identifier(SPAN, ast.str(context.options.helper("_createSelector"))),
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
                    ast.statement_expression(SPAN, gen_expression(selector, context, None, false)),
                  ),
                ),
              ),
            )),
            false,
          )),
          false,
        )),
        false,
      ),
    ));
    on_reset_calls.push(
      ast.statement_expression(
        SPAN,
        ast.expression_call(
          SPAN,
          ast
            .member_expression_static(
              SPAN,
              ast.expression_identifier(SPAN, ast.str(&format!("_n{}", id))),
              ast.identifier_name(SPAN, "onReset"),
              false,
            )
            .into(),
          NONE,
          ast.vec1(
            ast
              .member_expression_static(
                SPAN,
                ast.expression_identifier(SPAN, selector_name),
                ast.identifier_name(SPAN, "reset"),
                false,
              )
              .into(),
          ),
          false,
        ),
      ),
    );
  }

  let fragment_block = is_fragment_block(&render);
  let single_node_block = !component && is_single_node_block(&render);

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
            ast.plain_formal_parameter(
              SPAN,
              ast.binding_pattern_binding_identifier(SPAN, ast.str(&arg)),
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
                  statements.push(
                    ast.statement_expression(
                      SPAN,
                      ast.expression_call(
                        SPAN,
                        ast.expression_identifier(
                          SPAN,
                          ast.str(&if selector_patterns_len > 1 {
                            format!("_selector{id}_{i}")
                          } else {
                            format!("_selector{id}")
                          }),
                        ),
                        NONE,
                        ast.vec_from_array([
                          gen_expression(
                            key_prop_for_effects
                              .as_ref()
                              .map(|i| i.clone_in(ast.allocator))
                              .unwrap(),
                            context,
                            None,
                            false,
                          )
                          .into(),
                          Argument::ArrowFunctionExpression(ast.alloc_arrow_function_expression(
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
                          )),
                        ]),
                        false,
                      ),
                    ),
                  );
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
            gen_block_content(Some(render), context, context_block, None)
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
  if fragment_block {
    flags |= VaporVForFlags::IsFragment as i32;
  }
  if single_node_block {
    flags |= VaporVForFlags::IsSingleNode as i32;
  }
  if once {
    flags |= VaporVForFlags::Once as i32;
  }
  if slot_root {
    flags |= VaporVForFlags::SlotRoot as i32;
  }

  let gen_callback =
    if let Some(key_prop) = key_prop {
      let res = context.with_id(
        || gen_expression(key_prop, context, None, false),
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
                if let Some(raw_value) = raw_value {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(value_span, ast.str(raw_value)),
                  ))
                } else if raw_key.is_some() || raw_index.is_some() {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, "_"),
                  ))
                } else {
                  None
                },
                if let Some(raw_key) = raw_key {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(key_span, ast.str(raw_key)),
                  ))
                } else if raw_index.is_some() {
                  Some(ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(SPAN, "__"),
                  ))
                } else {
                  None
                },
                raw_index.map(|raw_index| {
                  ast.plain_formal_parameter(
                    SPAN,
                    ast.binding_pattern_binding_identifier(index_span, ast.str(raw_index)),
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

  statements.extend(selector_declarations);
  statements.push(Statement::VariableDeclaration(
    ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(
        ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{id}"))),
          NONE,
          Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.str(context.options.helper("_createFor"))),
              NONE,
              ast.vec_from_iter(
                [
                  Some(source_expr.into()),
                  Some(block_fn.into()),
                  gen_callback.map(|i| i.into()),
                  if flags > 0 {
                    Some(
                      ast
                        .expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Decimal)
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
          ),
          false,
        ),
      ),
      false,
    ),
  ));
  statements.extend(on_reset_calls);
}

fn is_single_node_block(block: &BlockIRNode) -> bool {
  let Some(child) = get_single_returned_child(block) else {
    return false;
  };
  child.template.is_some()
}

fn is_fragment_block(block: &BlockIRNode) -> bool {
  let child = get_single_returned_child(block);
  let Some(operation) = child.map(|child| child.operation.as_deref()).flatten() else {
    return false;
  };
  matches!(
    operation,
    // <slot/>
    OperationNode::SlotOutlet(_) |
    // <template v-for> with a single v-for child
    OperationNode::For(_) |
    // <template v-for> with a single dynamic :key child
    OperationNode::Key(_)
  ) || // <template v-for> with a single dynamic v-if child
    matches!(operation, OperationNode::If(op) if !op.once)
}

fn get_single_returned_child<'a>(block: &'a BlockIRNode) -> Option<&'a IRDynamicInfo<'a>> {
  if block.returns.len() != 1 {
    return None;
  }
  let id = block.returns[0];
  for child in block.dynamic.children.iter() {
    if child.id.is_some_and(|i| i == id) {
      return Some(child);
    }
  }
  None
}

fn match_patterns<'a>(
  render: &mut BlockIRNode<'a>,
  key_prop: &Option<Expression<'a>>,
  id_map: &HashMap<&'a str, Expression<'a>>,
  context: &'a CodegenContext<'a>,
) -> (Vec<IREffect<'a>>, Vec<Expression<'a>>, Vec<IREffect<'a>>) {
  let mut effect_patterns = vec![];
  let mut selector_patterns = vec![];
  let mut key_only_binding_patterns = vec![];
  let mut removed_effect_indexes = vec![];

  if let Some(key_prop) = key_prop {
    let key_content = key_prop.span().source_text(context.source_text);
    let effects = &mut render.effect;
    let mut kept = Vec::with_capacity(effects.len());
    let mut old = std::mem::take(effects);
    for (index, effect) in old.drain(..).enumerate() {
      let effect_ptr = &effect as *const _;
      if let Some(selector) =
        match_selector_pattern(unsafe { &*effect_ptr }, key_content, id_map, context)
      {
        effect_patterns.push(effect);
        selector_patterns.push(selector);
        removed_effect_indexes.push(index);
      } else if !effect.operations.is_empty()
        && let Some(ast) = get_expression(unsafe { &*effect_ptr })
        && key_content.eq(ast.span().source_text(context.source_text))
      {
        key_only_binding_patterns.push(effect);
        removed_effect_indexes.push(index);
      } else {
        kept.push(effect)
      }
    }
    *effects = kept;
  }

  if !removed_effect_indexes.is_empty() {
    shift_effect_boundaries(&mut render.dynamic, &mut removed_effect_indexes);
  }

  (
    effect_patterns,
    selector_patterns,
    key_only_binding_patterns,
  )
}

fn shift_effect_boundaries(dynamic: &mut IRDynamicInfo, removed_effect_indexes: &mut Vec<usize>) {
  if let Some(operation) = &mut dynamic.operation
    && let Some(effect_index) = match operation.as_mut() {
      OperationNode::If(operation) => operation.effect_index.as_mut(),
      OperationNode::For(operation) => operation.effect_index.as_mut(),
      OperationNode::CreateComponent(operation) => operation.effect_index.as_mut(),
      OperationNode::SlotOutlet(operation) => operation.effect_index.as_mut(),
      OperationNode::Key(operation) => operation.effect_index.as_mut(),
      _ => None,
    }
  {
    let mut offset = 0;
    for removed_index in removed_effect_indexes.iter() {
      if removed_index < effect_index {
        offset += 1;
      } else {
        break;
      }
    }
    *effect_index -= offset;
  }

  for child in dynamic.children.iter_mut() {
    shift_effect_boundaries(child, removed_effect_indexes);
  }
}

fn match_selector_pattern<'a>(
  effect: &'a IREffect<'a>,
  key: &str,
  id_map: &HashMap<&'a str, Expression<'a>>,
  context: &'a CodegenContext<'a>,
) -> Option<Expression<'a>> {
  if effect.operations.len() != 1 {
    return None;
  }
  let expression = get_expression(effect)?;

  let mut matcheds: Vec<(Span, Span)> = vec![];

  BinaryExpressionVisitor {
    on_binary_expression: Box::new(|ast| {
      if matches!(
        ast.operator,
        BinaryOperator::Equality | BinaryOperator::StrictEquality
      ) {
        let left = &ast.left;
        let right = &ast.right;
        let left_is_key =
          key.eq(&context.source_text[left.span().start as usize..left.span().end as usize]);
        let right_is_key =
          key.eq(&context.source_text[right.span().start as usize..right.span().end as usize]);
        if left_is_key && !right_is_key && !analyze_variable_scopes(right, id_map, context) {
          matcheds.push((left.span(), right.span()));
        } else if right_is_key && !left_is_key && !analyze_variable_scopes(left, id_map, context) {
          matcheds.push((right.span(), left.span()));
        }
      }
    }),
  }
  .visit_expression(expression);

  if matcheds.len() == 1 {
    let (key, selector) = matcheds[0];

    let mut has_extra_id = false;
    WalkIdentifiers::new(
      Box::new(|id, _, _| {
        let start = id.span.start;
        if start != key.start && start != selector.start {
          has_extra_id = true
        }
      }),
      context.options,
    )
    .visit(expression);

    if !has_extra_id {
      let content = selector.span().source_text(context.source_text);
      return Some(context.ast.expression_identifier(SPAN, content));
    }
  }
  None
}

fn analyze_variable_scopes<'a>(
  ast: &Expression,
  id_map: &HashMap<&'a str, Expression<'a>>,
  context: &CodegenContext<'a>,
) -> bool {
  let mut has_local = false;
  WalkIdentifiers::new(
    Box::new(|id, _, _| {
      let name = id.name.as_str();
      if !is_globally_allowed(name) && id_map.get(name).is_some() {
        has_local = true;
      }
    }),
    context.options,
  )
  .visit(&ast.clone_in(context.ast.allocator));
  has_local
}

fn get_expression<'a>(effect: &'a IREffect<'a>) -> Option<&'a Expression<'a>> {
  let operation = effect.operations.first();
  match operation.as_ref().unwrap() {
    OperationNode::SetText(operation) => operation.values.first(),
    // OperationNode::SetNodes(operation) => operation.values.first(),
    // OperationNode::CreateNodes(operation) => operation.values.first(),
    OperationNode::SetHtml(operation) => Some(&operation.value),
    OperationNode::SetEvent(operation) => Some(&operation.value),
    // OperationNode::SetDynamicEvents(operation) => Some(&operation.value),
    // OperationNode::SetTemplateRef(operation) => Some(&operation.value),
    OperationNode::SetProp(operation) => operation.prop.values.first(),
    OperationNode::Key(operation) => Some(&operation.value),
    _ => None,
  }
}

struct BinaryExpressionVisitor<'a> {
  on_binary_expression: Box<dyn FnMut(&BinaryExpression<'a>) + 'a>,
}
impl<'a> Visit<'a> for BinaryExpressionVisitor<'a> {
  fn visit_binary_expression(&mut self, node: &BinaryExpression<'a>) {
    self.on_binary_expression.as_mut()(node)
  }
}
