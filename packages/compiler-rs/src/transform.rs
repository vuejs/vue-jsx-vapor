use common::{
  options::{RootJsx, TransformOptions},
  patch_flag::TemplateFlags,
};
use napi::Either;
use oxc_allocator::TakeIn;
use oxc_ast::{
  AstBuilder, NONE,
  ast::{
    Argument, Expression, ImportOrExportKind, NumberBase, Program, Statement,
    VariableDeclarationKind,
  },
};
use oxc_ast_visit::{
  VisitMut,
  walk_mut::{
    walk_expression, walk_for_in_statement, walk_for_of_statement, walk_for_statement,
    walk_function, walk_statement,
  },
};
use oxc_span::{GetSpan, SPAN};

use crate::hmr_or_ssr::HmrOrSsrTransform;

pub struct Transform<'a> {
  ast: AstBuilder<'a>,
  source_text: &'a str,
  roots: Vec<RootJsx<'a>>,
  options: &'a TransformOptions<'a>,
}

impl<'a> Transform<'a> {
  pub fn new(options: &'a TransformOptions<'a>) -> Self {
    let ast = AstBuilder::new(&options.allocator);
    let ast_ptr = &ast as *const _;
    *options.on_enter_expression.borrow_mut() = Some(Box::new(|node| unsafe {
      if let Expression::CallExpression(node) = &mut *node
        && let Expression::Identifier(callee) = &mut node.callee
        && matches!(
          callee.name.as_str(),
          "defineVaporComponent" | "defineVaporCustomElement"
        )
      {
        if options.ssr {
          callee.name = "_defineVaporSSRComponent".into();
          options
            .helpers
            .borrow_mut()
            .insert("defineVaporSSRComponent");
        }
        *options.in_vapor.borrow_mut() += 1;
      } else if matches!(
        &*node,
        Expression::JSXElement(_) | Expression::JSXFragment(_)
      ) {
        if !options.ssr && options.interop && *options.in_vapor.borrow() < 1 {
          return Some((node, true));
        }
        return Some((node, options.ssr));
      }
      None
    }));

    *options.on_leave_expression.borrow_mut() = Some(Box::new(|node| {
      if let Expression::CallExpression(node) = node
        && node
          .callee_name()
          .is_some_and(|name| matches!(name, "defineVaporComponent" | "defineVaporCustomElement"))
      {
        *options.in_vapor.borrow_mut() -= 1;
      } else if let Expression::ArrowFunctionExpression(node) = node
        && options.interop
        && let Some(map) = options
          .scope_identifiers_map
          .borrow_mut()
          .remove(&node.span)
      {
        options.remove_identifiers(map.1);
      }
    }));

    *options.create_root_jsx.borrow_mut() = Some(Box::new(move |node_ptr, vdom| unsafe {
      let node = (&mut *node_ptr).take_in(&options.allocator);
      let expression = if vdom {
        use vdom::transform::TransformContext;
        let transform_context: *const TransformContext = &TransformContext::new(options, &*ast_ptr);
        (&*transform_context).transform(node)
      } else {
        use vapor::transform::TransformContext;
        let transform_context: *const TransformContext = &TransformContext::new(options, &*ast_ptr);
        (&*transform_context).transform(node)
      };
      RootJsx {
        node_ptr,
        expression,
      }
    }));

    Self {
      ast,
      source_text: "",
      roots: vec![],
      options,
    }
  }

  pub fn visit(&mut self, program: &mut Program<'a>) {
    self.source_text = program.source_text;

    self.visit_program(program);
    let ast = &self.ast;

    if self.options.ssr || !matches!(self.options.hmr, Either::A(false)) {
      HmrOrSsrTransform::new(self.options).visit(ast, program);
    }

    for root in self.roots.drain(..) {
      unsafe {
        *root.node_ptr = root.expression;
      }
    }

    let mut statements = vec![];
    let mut helpers = self.options.helpers.take();
    if !helpers.is_empty() {
      if helpers.contains("defineVaporSSRComponent") {
        program.body.retain_mut(|stmt| {
          if let Statement::ImportDeclaration(import) = stmt
            && let Some(specifiers) = &mut import.specifiers
            && let Some(index) = specifiers
              .iter()
              .position(|spec| spec.local().name.eq("defineVaporComponent"))
          {
            if specifiers.len() == 1 {
              return false;
            } else {
              specifiers.remove(index);
            }
          }
          true
        });
      };

      let vdom_helpers = vec!["createVNodeCache", "normalizeVNode", "normalizeSlot"]
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
              ast.module_export_name_identifier_name(SPAN, ast.str(helper)),
              ast.binding_identifier(SPAN, ast.str(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(
            SPAN,
            ast.str(
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
              ast.module_export_name_identifier_name(SPAN, ast.str(helper)),
              ast.binding_identifier(SPAN, ast.str(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(
            SPAN,
            ast.str(
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
              ast.module_export_name_identifier_name(SPAN, ast.str(helper)),
              ast.binding_identifier(SPAN, ast.str(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(SPAN, ast.str("vue"), None),
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
          let template_literal = Argument::StringLiteral(ast.alloc_string_literal(
            SPAN,
            ast.str(&template.content),
            None,
          ));

          let flags = if template.root {
            TemplateFlags::ROOT as i32
          } else {
            0
          } | if template._static {
            TemplateFlags::STATIC as i32
          } else {
            0
          };

          Statement::VariableDeclaration(
            ast.alloc_variable_declaration(
              SPAN,
              VariableDeclarationKind::Const,
              ast.vec1(
                ast.variable_declarator(
                  SPAN,
                  VariableDeclarationKind::Const,
                  ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_t{index}"))),
                  NONE,
                  Some(
                    ast.expression_call(
                      SPAN,
                      ast.expression_identifier(SPAN, ast.str("_template")),
                      NONE,
                      ast.vec_from_iter(
                        [
                          Some(template_literal),
                          if flags > 0 {
                            Some(
                              ast
                                .expression_numeric_literal(
                                  SPAN,
                                  flags as f64,
                                  None,
                                  NumberBase::Decimal,
                                )
                                .into(),
                            )
                          } else if template.ns > 0 {
                            Some(ast.expression_boolean_literal(SPAN, false).into())
                          } else {
                            None
                          },
                          if template.ns > 0 {
                            Some(
                              ast
                                .expression_numeric_literal(
                                  SPAN,
                                  template.ns as f64,
                                  None,
                                  NumberBase::Decimal,
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
                  ),
                  false,
                ),
              ),
              false,
            ),
          )
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
            ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_hoisted_{}", i + 1))),
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
      && let Some((node_ptr, vdom)) = on_enter_expression(node)
    {
      let root = self.options.create_root_jsx.borrow().as_ref().unwrap()(node_ptr, vdom);
      self.roots.push(root);
    }
    walk_expression(self, node);
    if let Some(on_leave_expression) = self.options.on_leave_expression.borrow().as_ref() {
      on_leave_expression(node)
    }
  }
  fn visit_function(
    &mut self,
    node: &mut oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    walk_function(self, node, flags);
    if self.options.interop
      && let Some(map) = self
        .options
        .scope_identifiers_map
        .borrow_mut()
        .remove(&node.span)
    {
      self.options.remove_identifiers(map.1);
    }
  }
  fn visit_for_statement(&mut self, node: &mut oxc_ast::ast::ForStatement<'a>) {
    walk_for_statement(self, node);
    if self.options.interop
      && let Some(map) = self
        .options
        .scope_identifiers_map
        .borrow_mut()
        .remove(&node.span)
    {
      self.options.remove_identifiers(map.1);
    }
  }
  fn visit_for_in_statement(&mut self, node: &mut oxc_ast::ast::ForInStatement<'a>) {
    walk_for_in_statement(self, node);
    if self.options.interop
      && let Some(map) = self
        .options
        .scope_identifiers_map
        .borrow_mut()
        .remove(&node.span)
    {
      self.options.remove_identifiers(map.1);
    }
  }
  fn visit_for_of_statement(&mut self, node: &mut oxc_ast::ast::ForOfStatement<'a>) {
    walk_for_of_statement(self, node);
    if self.options.interop
      && let Some(map) = self
        .options
        .scope_identifiers_map
        .borrow_mut()
        .remove(&node.span)
    {
      self.options.remove_identifiers(map.1);
    }
  }
  fn visit_statement(&mut self, node: &mut Statement<'a>) {
    walk_statement(self, node);
    if self.options.interop
      && let Some(map) = self
        .options
        .scope_identifiers_map
        .borrow_mut()
        .remove(&node.span())
    {
      self.options.remove_identifiers(map.1);
    }
  }
}
