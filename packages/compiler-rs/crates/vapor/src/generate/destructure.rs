use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use common::{expression::gen_getter, walk::WalkIdentifiers, walk_mut::WalkIdentifiersMut};
use oxc_allocator::CloneIn;
use oxc_ast::{
  AstKind, NONE,
  ast::{Argument, Expression, NumberBase, ObjectPropertyKind, PropertyKey},
};
use oxc_span::{GetSpan, SPAN};

use crate::generate::CodegenContext;

impl<'a> CodegenContext<'a> {
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
