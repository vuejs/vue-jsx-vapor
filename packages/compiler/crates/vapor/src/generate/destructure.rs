use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use common::{expression::gen_getter, walk::WalkIdentifiers, walk_mut::WalkIdentifiersMut};
use oxc_allocator::{CloneIn, Vec};
use oxc_ast::{
  AstKind, NONE,
  ast::{
    Argument, AssignmentOperator, AssignmentTarget, BindingPattern, Expression, FormalParameter,
    NumberBase, ObjectPropertyKind, PropertyKey,
  },
};
use oxc_span::{GetSpan, SPAN};

use crate::generate::{CodegenContext, expression::gen_expression};

/// Whether `id` is a name bound by the pattern, rather than one it merely
/// references (a default value, a computed key, a member expression).
fn is_binding_pattern_id<'a>(
  id: &oxc_ast::ast::IdentifierReference<'a>,
  stack: &[AstKind<'a>],
) -> bool {
  match stack.last() {
    None => true,
    Some(AstKind::AssignmentExpression(parent)) => parent.left.span().eq(&id.span),
    Some(AstKind::ObjectProperty(parent)) => parent.value.span().eq(&id.span),
    Some(AstKind::ArrayExpression(_)) => true,
    Some(AstKind::SpreadElement(_)) => true,
    _ => false,
  }
}

impl<'a> CodegenContext<'a> {
  /// The parameter list of a `v-for` callback (`:key`), rebuilt from the loop
  /// aliases so anything they reference (a default value, a computed key) is
  /// resolved like a normal expression.
  pub fn gen_alias_params(
    &'a self,
    value: Option<&Expression<'a>>,
    key: Option<&Expression<'a>>,
    index: Option<&Expression<'a>>,
  ) -> Vec<'a, FormalParameter<'a>> {
    let ast = self.ast;
    let mut params = ast.vec();
    if let Some(value) = value {
      params.push(ast.plain_formal_parameter(SPAN, self.alias_binding_pattern(value)));
    } else if key.is_some() || index.is_some() {
      params
        .push(ast.plain_formal_parameter(SPAN, ast.binding_pattern_binding_identifier(SPAN, "_")));
    }
    if let Some(key) = key {
      params.push(ast.plain_formal_parameter(SPAN, self.alias_binding_pattern(key)));
    } else if index.is_some() {
      params
        .push(ast.plain_formal_parameter(SPAN, ast.binding_pattern_binding_identifier(SPAN, "__")));
    }
    if let Some(index) = index {
      params.push(ast.plain_formal_parameter(SPAN, self.alias_binding_pattern(index)));
    }
    params
  }

  /// Aliases are parsed as expressions, so an alias carrying a default value has
  /// to be rebuilt before it can be printed as a parameter; any other pattern is
  /// printed from source, which leaves its formatting alone.
  fn alias_binding_pattern(&'a self, alias: &Expression<'a>) -> BindingPattern<'a> {
    if let Expression::AssignmentExpression(assignment) = alias
      && assignment.operator == AssignmentOperator::Assign
      && let AssignmentTarget::AssignmentTargetIdentifier(id) = &assignment.left
    {
      return self.ast.binding_pattern_assignment_pattern(
        assignment.span,
        self
          .ast
          .binding_pattern_binding_identifier(id.span, self.ast.str(&id.name)),
        gen_expression(
          assignment.right.clone_in(self.ast.allocator),
          self,
          None,
          false,
        ),
      );
    }
    let span = alias.span();
    self
      .ast
      .binding_pattern_binding_identifier(span, self.ast.str(span.source_text(self.source_text)))
  }

  // construct a id -> accessor path map.
  // e.g. `{ x: { y: [z] }}` -> `Map{ 'z' => '.x.y[0]' }`
  pub fn parse_value_destructure(
    &'a self,
    value_ast: Option<&mut Expression<'a>>,
    path: Expression<'a>,
  ) -> HashMap<&'a str, Expression<'a>> {
    let id_map: Rc<RefCell<HashMap<&'a str, Expression>>> = Rc::new(RefCell::new(HashMap::new()));
    let Some(value_ast) = value_ast else {
      return id_map.take();
    };

    let ast = self.ast;
    let id_map_clone = id_map.clone();
    WalkIdentifiers::new(
      Box::new(move |id, _, parent_stack| {
        // the walk also reports identifiers an alias references (a default
        // value, a computed key) - only the ones it binds belong in the map.
        if !is_binding_pattern_id(id, parent_stack) {
          return;
        }
        let mut path = path.clone_in(ast.allocator);
        let mut default_value: Option<Expression> = None;
        for i in 0..parent_stack.len() {
          let parent = parent_stack[i];
          let child = parent_stack.get(i + 1);
          let child_is_spread = if let Some(child) = child {
            matches!(child, AstKind::SpreadElement(_))
          } else {
            false
          };

          if let AstKind::ObjectProperty(parent) = parent {
            if let PropertyKey::StringLiteral(key) = &parent.key {
              path = ast
                .member_expression_computed(
                  SPAN,
                  path,
                  ast.expression_identifier(SPAN, key.value),
                  false,
                )
                .into();
            } else if parent.computed
              && let Some(key) = parent.key.as_expression()
            {
              let mut key = key.clone_in(ast.allocator);
              // use empty SPAN to prevent infinite loop
              WalkIdentifiersMut::new(
                Box::new(|id, _| {
                  id.span = SPAN;
                  None
                }),
                self.options,
              )
              .visit(&mut key);
              path = ast
                .member_expression_computed(SPAN, path, key, false)
                .into();
            } else if let PropertyKey::StaticIdentifier(key) = &parent.key {
              // non-computed, can only be identifier
              path = ast
                .member_expression_static(SPAN, path, ast.identifier_name(SPAN, key.name), false)
                .into()
            }
          } else if let AstKind::ArrayExpression(parent) = &parent {
            let index = parent
              .elements
              .iter()
              .position(|element| {
                if let Some(child) = child {
                  element.span().eq(&child.span())
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
                  NumberBase::Decimal,
                ))),
                false,
              );
            } else {
              path = ast
                .member_expression_computed(
                  SPAN,
                  path,
                  ast.expression_numeric_literal(SPAN, index as f64, None, NumberBase::Decimal),
                  false,
                )
                .into();
            }
          } else if let AstKind::ObjectExpression(parent) = &parent
            && child_is_spread
          {
            let properties = &parent.properties;
            path = ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.str(self.options.helper("_getRestElement"))),
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
                        } else if let Some(key) = p.key.as_expression() {
                          key.clone_in(ast.allocator).into()
                        } else {
                          ast
                            .expression_string_literal(
                              SPAN,
                              ast.str(&p.key.name().unwrap_or(Cow::from(""))),
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
          } else if let AstKind::AssignmentExpression(parent) = &parent
            && parent.left.span().eq(&id.span)
          {
            // default value, either inside a pattern (`[a = 1]`) or on the
            // alias itself (`(item, index = 0)`) - both parse as assignment
            // expressions here, as aliases are parsed as expressions.
            default_value = Some(parent.right.clone_in(ast.allocator));
          }
        }
        if let Some(default_value) = default_value {
          path = ast
            .expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.str(self.options.helper("_getDefaultValue"))),
              NONE,
              ast.vec_from_array([path.into(), gen_getter(default_value, ast).into()]),
              false,
            )
            .into();
        }
        id_map_clone
          .borrow_mut()
          .insert(id.span().source_text(self.source_text), path);
      }),
      self.options,
    )
    .visit(value_ast);
    id_map.take()
  }
}
