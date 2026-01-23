use std::hash::{DefaultHasher, Hash, Hasher};

use common::options::TransformOptions;
use napi::Either;
use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::{
  AstBuilder, NONE,
  ast::{
    Argument, AssignmentOperator, AssignmentTarget, BinaryOperator, BindingPattern, Declaration,
    ExportDefaultDeclarationKind, Expression, FormalParameterKind, ImportOrExportKind,
    LogicalOperator, Program, Statement, UnaryOperator, VariableDeclaration,
    VariableDeclarationKind,
  },
};
use oxc_span::{GetSpan, SPAN};

struct Component {
  local: String,
  exported: String,
  id: String,
}

pub struct HmrOrSsrTransform<'a> {
  has_default_export: bool,
  components: Vec<Component>,
  options: &'a TransformOptions<'a>,
  define_component_name: Vec<String>,
}

impl<'a> HmrOrSsrTransform<'a> {
  pub fn new(options: &'a TransformOptions<'a>) -> Self {
    Self {
      has_default_export: false,
      components: vec![],
      options,
      define_component_name: if let Either::B(hmr) = &options.hmr {
        hmr.define_component_name.clone()
      } else {
        vec![
          String::from("defineComponent"),
          String::from("defineVaporComponent"),
          String::from("_defineVaporSSRComponent"),
        ]
      },
    }
  }

  fn is_define_component_call(&self, node: Option<&Expression>) -> bool {
    if let Some(Expression::CallExpression(node)) = node
      && let Expression::Identifier(id) = &node.callee
      && self.define_component_name.contains(&id.name.to_string())
    {
      true
    } else {
      false
    }
  }

  fn parse_component_decls(&self, node: &VariableDeclaration) -> Vec<String> {
    let mut names = vec![];
    for decl in &node.declarations {
      if let BindingPattern::BindingIdentifier(id) = &decl.id
        && let Some(init) = &decl.init
        && (init.is_function() || self.is_define_component_call(Some(init)))
      {
        names.push(id.name.to_string());
      }
    }
    names
  }

  fn hash_string(&self, s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    format!("{}{}", self.options.filename, s).hash(&mut hasher);
    format!("{:x}", hasher.finish())
  }

  pub fn visit(&mut self, ast: &AstBuilder<'a>, program: &mut Program<'a>) {
    let mut declared_components = vec![];
    let mut default_declaration_index = 0;

    for (index, node) in program.body.iter_mut().enumerate() {
      if let Statement::VariableDeclaration(node) = node {
        declared_components.extend(self.parse_component_decls(node));
      } else if let Statement::FunctionDeclaration(node) = node
        && let Some(id) = &node.id
      {
        declared_components.push(id.name.to_string())
      } else if let Statement::ExportNamedDeclaration(node) = node {
        if let Some(Declaration::VariableDeclaration(declaration)) = &node.declaration {
          self.components.extend(
            self
              .parse_component_decls(declaration)
              .into_iter()
              .map(|name| Component {
                local: name.clone(),
                exported: name.clone(),
                id: self.hash_string(&name),
              })
              .collect::<Vec<_>>(),
          )
        } else if let Some(Declaration::FunctionDeclaration(declaration)) = &node.declaration
          && let Some(id) = &declaration.id
        {
          self.components.push(Component {
            local: id.name.to_string(),
            exported: id.name.to_string(),
            id: self.hash_string(&id.name),
          });
        } else {
          for spec in &node.specifiers {
            if let Some(name) = spec.exported.identifier_name()
              && declared_components.iter().any(|n| name.eq(n.as_str()))
            {
              self.components.push(Component {
                local: spec.local.name().to_string(),
                exported: name.to_string(),
                id: self.hash_string(&name),
              })
            }
          }
        }
      } else if let Statement::ExportDefaultDeclaration(node) = node {
        if let ExportDefaultDeclarationKind::Identifier(id) = &node.declaration {
          let _name = id.name.as_str();
          if declared_components.iter().any(|name| name.eq(_name)) {
            self.components.push(Component {
              local: _name.to_string(),
              exported: String::from("default"),
              id: self.hash_string("default"),
            })
          }
        } else if let ExportDefaultDeclarationKind::FunctionDeclaration(declaration) =
          &node.declaration
        {
          self.has_default_export = declaration.id.is_none();
          self.components.push(Component {
            local: if let Some(id) = &declaration.id {
              id.name.to_string()
            } else {
              String::from("__default__")
            },
            exported: String::from("default"),
            id: self.hash_string("default"),
          })
        } else if self.is_define_component_call(node.declaration.as_expression())
          || node
            .declaration
            .as_expression()
            .map(|e| e.is_function())
            .unwrap_or_default()
        {
          self.has_default_export = true;
          self.components.push(Component {
            local: if let ExportDefaultDeclarationKind::Identifier(id) = &node.declaration {
              self.has_default_export = false;
              id.name.to_string()
            } else {
              String::from("__default__")
            },
            exported: String::from("default"),
            id: self.hash_string("default"),
          })
        }
        default_declaration_index = index;
      }
    }

    if !self.components.is_empty() {
      if let Some(default_declaration) = program.body.get_mut(default_declaration_index)
        && let Statement::ExportDefaultDeclaration(default_declaration) = default_declaration
        && self.has_default_export
      {
        let mut declaration = default_declaration.declaration.take_in(ast.allocator);
        default_declaration.declaration = ExportDefaultDeclarationKind::Identifier(
          ast.alloc_identifier_reference(declaration.span(), "__default__"),
        );
        program.body.insert(
          default_declaration_index,
          Statement::VariableDeclaration(
            ast.alloc_variable_declaration(
              SPAN,
              VariableDeclarationKind::Const,
              ast.vec1(
                ast.variable_declarator(
                  SPAN,
                  VariableDeclarationKind::Const,
                  ast.binding_pattern_binding_identifier(SPAN, "__default__"),
                  NONE,
                  Some(match declaration {
                    ExportDefaultDeclarationKind::FunctionDeclaration(e) => {
                      Expression::FunctionExpression(e)
                    }
                    ExportDefaultDeclarationKind::ClassDeclaration(e) => {
                      Expression::ClassExpression(e)
                    }
                    _ => declaration
                      .as_expression_mut()
                      .unwrap()
                      .take_in(ast.allocator),
                  }),
                  false,
                ),
              ),
              false,
            ),
          ),
        );
      }

      if self.options.ssr && !self.components.is_empty() {
        program.body.insert(
          0,
          Statement::VariableDeclaration(ast.alloc_variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            ast.vec1(ast.variable_declarator(
              SPAN,
              VariableDeclarationKind::Const,
              ast.binding_pattern_binding_identifier(SPAN, "__moduleId"),
              NONE,
              Some(ast.expression_string_literal(SPAN, ast.atom(self.options.filename), None)),
              false,
            )),
            false,
          )),
        );
        program.body.insert(
          0,
          Statement::ImportDeclaration(ast.alloc_import_declaration(
            SPAN,
            Some(ast.vec1(ast.import_declaration_specifier_import_specifier(
              SPAN,
              ast.module_export_name_identifier_reference(SPAN, "ssrRegisterHelper"),
              ast.binding_identifier(SPAN, "ssrRegisterHelper"),
              ImportOrExportKind::Value,
            ))),
            ast.string_literal(
              SPAN,
              ast.atom(
                if let Some(runtime_module_name) = &self.options.runtime_module_name {
                  runtime_module_name.as_str()
                } else {
                  "/vue-jsx-vapor/ssr"
                },
              ),
              None,
            ),
            None,
            NONE,
            ImportOrExportKind::Value,
          )),
        );

        for Component { local, .. } in self.components.drain(..) {
          program.body.push(ast.statement_expression(
            SPAN,
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, "ssrRegisterHelper"),
              NONE,
              ast.vec_from_array([
                Argument::Identifier(ast.alloc_identifier_reference(SPAN, ast.atom(&local))),
                Argument::Identifier(ast.alloc_identifier_reference(SPAN, "__moduleId")),
              ]),
              false,
            ),
          ))
        }
      } else if !self.options.filename.contains("?vue&type=script") {
        let mut callbacks = ast.vec();

        for Component {
          local,
          exported,
          id,
        } in self.components.drain(..)
        {
          program.body.push(ast.statement_expression(
            SPAN,
            ast.expression_assignment(
              SPAN,
              AssignmentOperator::Assign,
              AssignmentTarget::StaticMemberExpression(ast.alloc_static_member_expression(
                SPAN,
                ast.expression_identifier(SPAN, ast.atom(&local)),
                ast.identifier_name(SPAN, "__hmrId"),
                false,
              )),
              ast.expression_string_literal(SPAN, ast.atom(&id), None),
            ),
          ));
          program.body.push(ast.statement_expression(
            SPAN,
            ast.expression_call(
              SPAN,
              Expression::StaticMemberExpression(ast.alloc_static_member_expression(
                SPAN,
                ast.expression_identifier(SPAN, "__VUE_HMR_RUNTIME__"),
                ast.identifier_name(SPAN, "createRecord"),
                false,
              )),
              NONE,
              ast.vec_from_array([
                Argument::StringLiteral(ast.alloc_string_literal(SPAN, ast.atom(&id), None)),
                Argument::Identifier(ast.alloc_identifier_reference(SPAN, ast.atom(&local))),
              ]),
              false,
            ),
          ));

          let exported_expression =
            Expression::StaticMemberExpression(ast.alloc_static_member_expression(
              SPAN,
              ast.expression_identifier(SPAN, "mod"),
              ast.identifier_name(SPAN, ast.atom(&exported)),
              false,
            ));
          let exported_expression_render =
            Expression::StaticMemberExpression(ast.alloc_static_member_expression(
              SPAN,
              Expression::StaticMemberExpression(ast.alloc_static_member_expression(
                SPAN,
                ast.expression_identifier(SPAN, "mod"),
                ast.identifier_name(SPAN, ast.atom(&exported)),
                false,
              )),
              ast.identifier_name(SPAN, ast.atom("render")),
              false,
            ));
          callbacks.push(Statement::ExpressionStatement(
            ast.alloc_expression_statement(
              SPAN,
              ast.expression_call(
                SPAN,
                Expression::ComputedMemberExpression(ast.alloc_computed_member_expression(
                  SPAN,
                  ast.expression_identifier(SPAN, "__VUE_HMR_RUNTIME__"),
                  ast.expression_conditional(
                    SPAN,
                    ast.expression_logical(
                      SPAN,
                      exported_expression_render.clone_in(ast.allocator),
                      LogicalOperator::Or,
                      ast.expression_binary(
                        SPAN,
                        ast.expression_unary(
                          SPAN,
                          UnaryOperator::Typeof,
                          exported_expression.clone_in(ast.allocator),
                        ),
                        BinaryOperator::StrictEquality,
                        ast.expression_string_literal(SPAN, "function", None),
                      ),
                    ),
                    ast.expression_string_literal(SPAN, "rerender", None),
                    ast.expression_string_literal(SPAN, "reload", None),
                  ),
                  false,
                )),
                NONE,
                ast.vec_from_array([
                  Expression::StaticMemberExpression(ast.alloc_static_member_expression(
                    SPAN,
                    exported_expression.clone_in(ast.allocator),
                    ast.identifier_name(SPAN, "__hmrId"),
                    false,
                  ))
                  .into(),
                  ast
                    .expression_logical(
                      SPAN,
                      exported_expression_render,
                      LogicalOperator::Or,
                      exported_expression,
                    )
                    .into(),
                ]),
                false,
              ),
            ),
          ));
        }

        let import_meta_hot = ast.member_expression_static(
          SPAN,
          ast
            .member_expression_static(
              SPAN,
              ast.expression_identifier(SPAN, "import"),
              ast.identifier_name(SPAN, "meta"),
              false,
            )
            .into(),
          ast.identifier_name(SPAN, "hot"),
          false,
        );
        program.body.push(
          ast.statement_if(
            SPAN,
            import_meta_hot.clone_in(ast.allocator).into(),
            ast.statement_expression(
              SPAN,
              ast.expression_call(
                SPAN,
                Expression::StaticMemberExpression(ast.alloc_static_member_expression(
                  SPAN,
                  import_meta_hot.into(),
                  ast.identifier_name(SPAN, "accept"),
                  false,
                )),
                NONE,
                ast.vec1(
                  ast
                    .expression_arrow_function(
                      SPAN,
                      false,
                      false,
                      NONE,
                      ast.formal_parameters(
                        SPAN,
                        FormalParameterKind::ArrowFormalParameters,
                        ast.vec1(ast.plain_formal_parameter(
                          SPAN,
                          ast.binding_pattern_binding_identifier(SPAN, "mod"),
                        )),
                        NONE,
                      ),
                      NONE,
                      ast.function_body(SPAN, ast.vec(), callbacks),
                    )
                    .into(),
                ),
                false,
              ),
            ),
            None,
          ),
        );
      }
    }
  }
}
