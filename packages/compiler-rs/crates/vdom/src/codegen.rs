use std::collections::HashMap;

use common::text::{is_empty_text, resolve_jsx_text, to_valid_asset_id};
use napi::bindgen_prelude::Either3;
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{
    Argument, BindingPatternKind, Expression, FormalParameterKind, JSXChild, NumberBase, Statement,
    VariableDeclarationKind,
  },
};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ast::{NodeTypes, VNodeCall, VNodeCallChildren, get_vnode_block_helper, get_vnode_helper},
  transform::TransformContext,
};

impl<'a> TransformContext<'a> {
  // IR -> JS codegen
  pub fn generate(&'a self) -> Expression<'a> {
    let ast = &self.ast;
    let mut statements = ast.vec();

    if *self.cache_index.borrow() > 0 {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(
            ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Const,
              ast.binding_pattern(
                ast.binding_pattern_kind_binding_identifier(SPAN, "_cache"),
                NONE,
                false,
              ),
              Some(
                ast.expression_call(
                  SPAN,
                  ast.expression_identifier(SPAN, ast.atom(&self.helper("createVNodeCache"))),
                  NONE,
                  ast.vec1(
                    ast
                      .expression_numeric_literal(
                        SPAN,
                        *self.options.cache_index.borrow() as f64,
                        None,
                        NumberBase::Hex,
                      )
                      .into(),
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
      *self.options.cache_index.borrow_mut() += 1;
    }

    for name in self.components.borrow_mut().drain() {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(
            ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Const,
              ast.binding_pattern(
                BindingPatternKind::BindingIdentifier(ast.alloc_binding_identifier(
                  SPAN,
                  ast.atom(&to_valid_asset_id(&name, "component")),
                )),
                NONE,
                false,
              ),
              Some(ast.expression_call(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&self.helper("resolveComponent"))),
                NONE,
                ast.vec_from_array([Argument::StringLiteral(ast.alloc_string_literal(
                  SPAN,
                  ast.atom(&name),
                  None,
                ))]),
                false,
              )),
              false,
            ),
          ),
          false,
        ),
      ));
    }

    for name in self.directives.borrow_mut().drain() {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(
            ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Const,
              ast.binding_pattern(
                BindingPatternKind::BindingIdentifier(ast.alloc_binding_identifier(
                  SPAN,
                  ast.atom(&to_valid_asset_id(&name, "directive")),
                )),
                NONE,
                false,
              ),
              Some(ast.expression_call(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&self.helper("resolveDirective"))),
                NONE,
                ast.vec1(Argument::StringLiteral(ast.alloc_string_literal(
                  SPAN,
                  ast.atom(&name),
                  None,
                ))),
                false,
              )),
              false,
            ),
          ),
          false,
        ),
      ))
    }

    if let JSXChild::Fragment(node) = &mut *self.root_node.borrow_mut()
      && let Some(child) = &node.children.first()
    {
      let codegen_map = &mut self.codegen_map.borrow_mut();
      if let Some(codegen) = codegen_map.remove(&child.span()) {
        statements.push(ast.statement_return(
          SPAN,
          Some(match codegen {
            NodeTypes::VNodeCall(vnode_call) => self.gen_vnode_call(vnode_call, codegen_map),
            NodeTypes::TextCallNode(exp) => exp,
            NodeTypes::CacheExpression(exp) => exp,
          }),
        ))
      }
    }

    ast.expression_call(
      SPAN,
      ast.expression_parenthesized(
        SPAN,
        ast.expression_arrow_function(
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
          ast.function_body(SPAN, ast.vec(), statements),
        ),
      ),
      NONE,
      ast.vec(),
      false,
    )
  }

  pub fn gen_node_list(
    &'a self,
    children: VNodeCallChildren<'a>,
    codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
  ) -> Expression<'a> {
    let ast = &self.ast;

    match children {
      Either3::A(children) => self
        .gen_node(
          unsafe { &mut *children }.take_in(ast.allocator),
          codegen_map,
        )
        .unwrap(),
      Either3::B(children) => ast.expression_array(
        SPAN,
        ast.vec_from_iter(unsafe { &mut *children }.into_iter().filter_map(|child| {
          if is_empty_text(child) {
            None
          } else {
            self
              .gen_node(child.take_in(self.allocator), codegen_map)
              .map(|node| node.into())
          }
        })),
      ),
      Either3::C(children) => children,
    }
  }

  pub fn gen_node(
    &'a self,
    node: JSXChild<'a>,
    codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
  ) -> Option<Expression<'a>> {
    let ast = &self.ast;
    if let Some(codegen) = codegen_map.remove(&node.span()) {
      Some(match codegen {
        NodeTypes::VNodeCall(codegen) => self.gen_vnode_call(codegen, codegen_map),
        NodeTypes::TextCallNode(codegen) => codegen,
        NodeTypes::CacheExpression(codegen) => codegen,
      })
    } else {
      match node {
        JSXChild::Text(node) => {
          Some(ast.expression_string_literal(node.span, ast.atom(&resolve_jsx_text(&node)), None))
        }
        JSXChild::ExpressionContainer(mut node) => {
          Some(node.expression.to_expression_mut().take_in(self.allocator))
        }
        _ => None,
      }
    }
  }

  pub fn gen_vnode_call(
    &'a self,
    node: VNodeCall<'a>,
    codegen_map: &mut HashMap<Span, NodeTypes<'a>>,
  ) -> Expression<'a> {
    let ast = &self.ast;
    let VNodeCall {
      tag,
      props,
      children,
      patch_flag,
      dynamic_props,
      directives,
      is_block,
      disable_tracking,
      is_component,
      ..
    } = node;

    // skip v-if / else-if generated fragment
    if tag.is_empty()
      && let Some(children) = children
    {
      return self.gen_node_list(children, codegen_map);
    }

    let call_helper = if is_block {
      get_vnode_block_helper(self.options.in_ssr, is_component)
    } else {
      get_vnode_helper(self.options.in_ssr, is_component)
    };
    let mut result = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&self.helper(&call_helper))),
      NONE,
      ast.vec_from_iter(
        [
          Some(
            if is_component {
              ast.expression_identifier(SPAN, ast.atom(&tag))
            } else {
              ast.expression_string_literal(SPAN, ast.atom(&tag), None)
            }
            .into(),
          ),
          if let Some(props) = props {
            Some(props.into())
          } else if children.is_some() || patch_flag.is_some() || dynamic_props.is_some() {
            Some(ast.expression_null_literal(SPAN).into())
          } else {
            None
          },
          if let Some(children) = children {
            Some(self.gen_node_list(children, codegen_map).into())
          } else if patch_flag.is_some() || dynamic_props.is_some() {
            Some(ast.expression_null_literal(SPAN).into())
          } else {
            None
          },
          if let Some(patch_flag) = patch_flag {
            Some(
              ast
                .expression_numeric_literal(SPAN, patch_flag as f64, None, NumberBase::Hex)
                .into(),
            )
          } else if dynamic_props.is_some() {
            Some(ast.expression_null_literal(SPAN).into())
          } else {
            None
          },
          dynamic_props.map(|dynamic_props| dynamic_props.into()),
        ]
        .into_iter()
        .flatten(),
      ),
      false,
    );
    if is_block {
      result = ast.expression_parenthesized(
        SPAN,
        ast.expression_sequence(
          SPAN,
          ast.vec_from_array([
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&self.helper("openBlock"))),
              NONE,
              if disable_tracking {
                ast.vec1(ast.expression_boolean_literal(SPAN, true).into())
              } else {
                ast.vec()
              },
              false,
            ),
            result,
          ]),
        ),
      )
    }

    if let Some(directives) = directives {
      result = ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.atom(&self.helper("withDirectives"))),
        NONE,
        ast.vec_from_array([
          result.into(),
          Argument::ArrayExpression(ast.alloc(directives)),
        ]),
        false,
      )
    }

    result
  }
}
