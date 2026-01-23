use std::mem;

use common::options::{RootJsx, TransformOptions};
use napi::Either;
use oxc_allocator::{Allocator, FromIn, TakeIn};
use oxc_ast::{
  AstBuilder, AstKind, NONE,
  ast::{Argument, Expression, ImportOrExportKind, Program, Statement, VariableDeclarationKind},
};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_span::{Atom, GetSpan, SPAN};

use crate::hmr_or_ssr::HmrOrSsrTransform;

pub struct Transform<'a> {
  allocator: &'a Allocator,
  ast: AstBuilder<'a>,
  source_text: &'a str,
  roots: Vec<RootJsx<'a>>,
  options: &'a TransformOptions<'a>,
  parents: Vec<AstKind<'a>>,
}

impl<'a> Transform<'a> {
  pub fn new(allocator: &'a Allocator, options: &'a TransformOptions<'a>) -> Self {
    *options.on_enter_expression.borrow_mut() = Some(Box::new(|node, parents| unsafe {
      if options.ssr
        && let Expression::CallExpression(node) = &mut *node
        && let Expression::Identifier(callee) = &mut node.callee
        && callee.name.eq("defineVaporComponent")
      {
        callee.name = Atom::from_in("_defineVaporSSRComponent", allocator);
        options
          .helpers
          .borrow_mut()
          .insert("defineVaporSSRComponent".to_string());
      } else if matches!(
        &*node,
        Expression::JSXElement(_) | Expression::JSXFragment(_)
      ) {
        if !options.ssr && options.interop {
          let mut has_define_vapor_component = false;
          for parent in parents.iter().rev() {
            if let AstKind::CallExpression(parent) = parent
              && let Expression::Identifier(name) = &parent.callee
            {
              if name.name == "defineVaporComponent" {
                has_define_vapor_component = true;
                break;
              } else if name.name == "defineComponent" {
                return Some((node, true));
              }
            }
          }
          if !has_define_vapor_component {
            return Some((node, true));
          }
        }
        return Some((node, options.ssr));
      }
      None
    }));

    *options.on_exit_program.borrow_mut() = Some(Box::new(|mut roots, source| unsafe {
      for root in roots.drain(..) {
        if root.vdom {
          use vdom::transform::TransformContext;
          let transform_context: *mut TransformContext =
            &mut TransformContext::new(allocator, options);
          let source_text = &source[..root.node.span().end as usize];
          *root.node_ref = (&*transform_context).transform(root.node, source_text);
        } else {
          use vapor::transform::TransformContext;
          let transform_context: *mut TransformContext =
            &mut TransformContext::new(allocator, options);
          let source_text = &source[..root.node.span().end as usize];
          *root.node_ref = (&*transform_context).transform(root.node, source_text);
        }
      }
    }));

    Self {
      allocator,
      source_text: "",
      roots: vec![],
      ast: AstBuilder::new(allocator),
      options,
      parents: vec![],
    }
  }

  pub fn visit(&mut self, program: &mut Program<'a>) {
    self.source_text = program.source_text;

    self.visit_program(program);

    if self.options.ssr || !matches!(self.options.hmr, Either::A(false)) {
      HmrOrSsrTransform::new(self.options).visit(&self.ast, program);
    }

    if let Some(on_exit_program) = self.options.on_exit_program.borrow().as_ref() {
      on_exit_program(mem::take(&mut self.roots), self.source_text);
    }

    let ast = &self.ast;
    let mut statements = vec![];
    let delegates = self.options.delegates.take();
    if !delegates.is_empty() {
      statements.push(ast.statement_expression(
        SPAN,
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom("_delegateEvents")),
          NONE,
          oxc_allocator::Vec::from_iter_in(
            delegates.iter().map(|delegate| {
              Argument::StringLiteral(ast.alloc(ast.string_literal(SPAN, ast.atom(delegate), None)))
            }),
            ast.allocator,
          ),
          false,
        ),
      ));
    }

    let mut helpers = self.options.helpers.take();
    if !helpers.is_empty() {
      let vdom_helpers = vec!["createVNodeCache", "normalizeVNode"]
        .into_iter()
        .filter(|helper| {
          if helpers.contains(*helper) {
            helpers.remove(*helper);
            true
          } else {
            false
          }
        })
        .collect::<Vec<_>>();
      if !vdom_helpers.is_empty() {
        statements.push(Statement::ImportDeclaration(ast.alloc_import_declaration(
          SPAN,
          Some(ast.vec_from_iter(vdom_helpers.into_iter().map(|helper| {
            ast.import_declaration_specifier_import_specifier(
              SPAN,
              ast.module_export_name_identifier_name(SPAN, ast.atom(helper)),
              ast.binding_identifier(SPAN, ast.atom(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(
            SPAN,
            ast.atom(
              if let Some(runtime_module_name) = &self.options.runtime_module_name {
                runtime_module_name.as_str()
              } else {
                "/vue-jsx-vapor/vdom"
              },
            ),
            None,
          ),
          None,
          NONE,
          ImportOrExportKind::Value,
        )))
      }

      let vapor_helpers = vec![
        "setNodes",
        "createNodes",
        "createComponent",
        "createComponentWithFallback",
        "defineVaporSSRComponent",
      ]
      .into_iter()
      .filter(|helper| {
        if helpers.contains(*helper) {
          helpers.remove(*helper);
          true
        } else {
          false
        }
      })
      .collect::<Vec<_>>();
      if !vapor_helpers.is_empty() {
        statements.push(Statement::ImportDeclaration(ast.alloc_import_declaration(
          SPAN,
          Some(ast.vec_from_iter(vapor_helpers.into_iter().map(|helper| {
            ast.import_declaration_specifier_import_specifier(
              SPAN,
              ast.module_export_name_identifier_name(SPAN, ast.atom(helper)),
              ast.binding_identifier(SPAN, ast.atom(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(
            SPAN,
            ast.atom(
              if let Some(runtime_module_name) = &self.options.runtime_module_name {
                runtime_module_name.as_str()
              } else {
                "/vue-jsx-vapor/vapor"
              },
            ),
            None,
          ),
          None,
          NONE,
          ImportOrExportKind::Value,
        )))
      }

      if !helpers.is_empty() {
        statements.push(Statement::ImportDeclaration(ast.alloc_import_declaration(
          SPAN,
          Some(ast.vec_from_iter(helpers.iter().map(|helper| {
            ast.import_declaration_specifier_import_specifier(
              SPAN,
              ast.module_export_name_identifier_name(SPAN, ast.atom(helper)),
              ast.binding_identifier(SPAN, ast.atom(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(SPAN, ast.atom("vue"), None),
          None,
          NONE,
          ImportOrExportKind::Value,
        )))
      }
    }

    let templates = self.options.templates.take();
    let template_len = templates.len();
    if template_len > 0 {
      let template_statements = templates
        .iter()
        .enumerate()
        .map(|(index, template)| {
          let template_literal =
            Argument::StringLiteral(ast.alloc_string_literal(SPAN, ast.atom(&template.0), None));

          Statement::VariableDeclaration(ast.alloc_variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            ast.vec1(ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Const,
              ast.binding_pattern_binding_identifier(SPAN, ast.atom(&format!("t{index}"))),
              NONE,
              Some(ast.expression_call(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom("_template")),
                NONE,
                if template.1 {
                  ast.vec_from_array([
                    template_literal,
                    Argument::BooleanLiteral(ast.alloc_boolean_literal(SPAN, template.1)),
                  ])
                } else {
                  oxc_allocator::Vec::from_array_in([template_literal], ast.allocator)
                },
                false,
              )),
              false,
            )),
            false,
          ))
        })
        .collect::<Vec<_>>();
      statements.extend(template_statements);
    }

    for (i, exp) in self.options.hoists.borrow_mut().drain(..).enumerate() {
      statements.push(Statement::VariableDeclaration(
        ast.alloc_variable_declaration(
          SPAN,
          VariableDeclarationKind::Const,
          ast.vec1(ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            ast.binding_pattern_binding_identifier(SPAN, ast.atom(&format!("_hoisted_{}", i + 1))),
            NONE,
            Some(exp),
            false,
          )),
          false,
        ),
      ))
    }

    if !statements.is_empty() {
      // Insert statements before the first non-import statement.
      let index = program
        .body
        .iter()
        .position(|stmt| !matches!(stmt, Statement::ImportDeclaration(_)))
        .unwrap_or(program.body.len());
      program.body.splice(index..index, statements);
    }
  }
}

impl<'a> VisitMut<'a> for Transform<'a> {
  fn visit_expression(&mut self, node: &mut Expression<'a>) {
    if let Some(on_enter_expression) = self.options.on_enter_expression.borrow().as_ref()
      && let Some((node_ref, vdom)) = on_enter_expression(node, &self.parents)
    {
      self.roots.push(RootJsx {
        node_ref,
        node: unsafe { &mut *node_ref }.take_in(self.allocator),
        vdom,
      });
    }
    walk_mut::walk_expression(self, node);
  }

  fn enter_node(&mut self, kind: AstKind<'a>) {
    self.parents.push(kind);
  }
  fn leave_node(&mut self, _: oxc_ast::AstKind<'a>) {
    self.parents.pop();
  }
}
