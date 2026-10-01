use common::{options::TransformOptions, text::hash_string};
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

struct Component<'a> {
  local: &'a str,
  exported: &'a str,
  id: String,
  /// Plain functions are wrapped into a stable component carrying the
  /// implementation on a swappable property (see
  /// [`Self::wrap_function_components`]); `define*()` calls produce objects
  /// that Vue patches in place as-is.
  is_function: bool,
}

pub struct HmrOrSsrTransform<'a> {
  has_default_export: bool,
  components: Vec<Component<'a>>,
  options: &'a TransformOptions<'a>,
  define_component_name: Vec<&'a str>,
}

impl<'a> HmrOrSsrTransform<'a> {
  pub fn new(options: &'a TransformOptions<'a>) -> Self {
    Self {
      has_default_export: false,
      components: vec![],
      options,
      define_component_name: if let Either::B(hmr) = &options.hmr {
        hmr
          .define_component_name
          .iter()
          .map(|n| n.as_str())
          .collect()
      } else {
        vec![
          "defineComponent",
          "defineVaporComponent",
          "defineCustomElement",
          "defineVaporCustomElement",
          "_defineVaporSSRComponent",
        ]
      },
    }
  }

  fn is_define_component_call(&self, node: Option<&Expression>) -> bool {
    if let Some(Expression::CallExpression(node)) = node
      && let Expression::Identifier(id) = &node.callee
      && self.define_component_name.contains(&id.name.as_str())
    {
      true
    } else {
      false
    }
  }

  /// Collects the components declared by a variable declaration statement,
  /// together with whether they are created by a `define*()` call.
  fn parse_component_decls(&self, node: &VariableDeclaration<'a>) -> Vec<(&'a str, bool)> {
    let mut decls = vec![];
    for decl in &node.declarations {
      if let BindingPattern::BindingIdentifier(id) = &decl.id
        && let Some(init) = &decl.init
        && (init.is_function() || self.is_define_component_call(Some(init)))
      {
        decls.push((id.name.as_str(), self.is_define_component_call(Some(init))));
      }
    }
    decls
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
        declared_components.push((id.name.as_str(), false))
      } else if let Statement::ExportNamedDeclaration(node) = node {
        if let Some(Declaration::VariableDeclaration(declaration)) = &node.declaration {
          self.components.extend(
            self
              .parse_component_decls(declaration)
              .into_iter()
              .map(|(name, is_define)| Component {
                local: name,
                exported: name,
                id: hash_string(&format!("{}{}", self.options.filename, name)),
                is_function: !is_define,
              })
              .collect::<Vec<_>>(),
          )
        } else if let Some(Declaration::FunctionDeclaration(declaration)) = &node.declaration
          && let Some(id) = &declaration.id
        {
          self.components.push(Component {
            local: id.name.as_str(),
            exported: id.name.as_str(),
            id: hash_string(&format!("{}{}", self.options.filename, &id.name)),
            is_function: true,
          });
        } else {
          for spec in &node.specifiers {
            // match the local binding: `export { Comp as Alias }` exports `Comp`
            if let Some(name) = spec.exported.identifier_name()
              && let Some((_, is_define)) = declared_components
                .iter()
                .find(|(declared, _)| *declared == spec.local.name().as_str())
            {
              self.components.push(Component {
                local: spec.local.name().as_str(),
                exported: name.as_str(),
                id: hash_string(&format!("{}{}", self.options.filename, &name)),
                is_function: !*is_define,
              })
            }
          }
        }
      } else if let Statement::ExportDefaultDeclaration(node) = node {
        if let ExportDefaultDeclarationKind::Identifier(id) = &node.declaration {
          let _name = id.name.as_str();
          if let Some((_, is_define)) = declared_components
            .iter()
            .find(|(declared, _)| declared == &_name)
          {
            self.components.push(Component {
              local: _name,
              exported: "default",
              id: hash_string(&format!("{}{}", self.options.filename, "default")),
              is_function: !*is_define,
            })
          }
        } else if let ExportDefaultDeclarationKind::FunctionDeclaration(declaration) =
          &node.declaration
        {
          self.has_default_export = declaration.id.is_none();
          self.components.push(Component {
            local: if let Some(id) = &declaration.id {
              id.name.as_str()
            } else {
              "__default__"
            },
            exported: "default",
            id: hash_string(&format!("{}{}", self.options.filename, "default")),
            is_function: true,
          })
        } else if self.is_define_component_call(node.declaration.as_expression())
          || node
            .declaration
            .as_expression()
            .map(|e| e.is_function())
            .unwrap_or_default()
        {
          let is_define = self.is_define_component_call(node.declaration.as_expression());
          self.has_default_export = true;
          self.components.push(Component {
            local: if let ExportDefaultDeclarationKind::Identifier(id) = &node.declaration {
              self.has_default_export = false;
              id.name.as_str()
            } else {
              "__default__"
            },
            exported: "default",
            id: hash_string(&format!("{}{}", self.options.filename, "default")),
            is_function: !is_define,
          })
        }
        default_declaration_index = index;
      }
    }

    if !self.components.is_empty() {
      if let Some(default_declaration) = program.body.get_mut(default_declaration_index)
        && let Statement::ExportDefaultDeclaration(default_declaration) = default_declaration
        && self.has_default_export
        && self
          .components
          .iter()
          .any(|component| component.exported == "default")
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
              Some(ast.expression_string_literal(SPAN, ast.str(self.options.filename), None)),
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
              ast.str(
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
                Argument::Identifier(ast.alloc_identifier_reference(SPAN, ast.str(local))),
                Argument::Identifier(ast.alloc_identifier_reference(SPAN, "__moduleId")),
              ]),
              false,
            ),
          ))
        }
      } else if !self.options.filename.contains("?vue&type=script") {
        // Plain function components cannot be patched in place by Vue: wrap
        // them into a stable component (`defineVaporHmrComponent` /
        // `defineHmrComponent`) that carries the implementation on a property
        // which an update copies over.
        self.wrap_function_components(ast, program);
        // only the wrapped components reference the helper
        if self.components.iter().any(should_wrap_component) {
          program.body.insert(0, hmr_import(ast, self.options));
        }

        let mut statements = ast.vec();
        let mut callbacks = ast.vec();
        // Fetching the updated module can fail (a syntax error, for example);
        // Vite logs the failure and still invokes this callback with
        // `mod === undefined`, so bail out instead of throwing on each access.
        callbacks.push(ast.statement_if(
          SPAN,
          ast.expression_unary(
            SPAN,
            UnaryOperator::LogicalNot,
            ast.expression_identifier(SPAN, "mod"),
          ),
          ast.statement_return(SPAN, None),
          None,
        ));
        let member = |object: Expression<'a>, name: &str| {
          Expression::StaticMemberExpression(ast.alloc_static_member_expression(
            SPAN,
            object,
            ast.identifier_name(SPAN, ast.str(name)),
            false,
          ))
        };

        for Component {
          local,
          exported,
          id,
          ..
        } in self.components.drain(..)
        {
          statements.push(
            ast.statement_expression(
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
                  // the assignment expression evaluates to the id
                  ast
                    .expression_assignment(
                      SPAN,
                      AssignmentOperator::Assign,
                      AssignmentTarget::StaticMemberExpression(ast.alloc_static_member_expression(
                        SPAN,
                        ast.expression_identifier(SPAN, ast.str(local)),
                        ast.identifier_name(SPAN, "__hmrId"),
                        false,
                      )),
                      ast.expression_string_literal(SPAN, ast.str(&id), None),
                    )
                    .into(),
                  Argument::Identifier(ast.alloc_identifier_reference(SPAN, ast.str(local))),
                ]),
                false,
              ),
            ),
          );

          // Vite runs the accept callback registered by the PREVIOUS evaluation
          // (it snapshots callbacks before importing the update), so every
          // component is read from the new namespace under its export name — a
          // bare local binding would be stale. Reload every surviving component
          // so the newest module code always applies; components declared only
          // locally are covered by the reloaded exported component that
          // renders them.
          let component_expression = member(ast.expression_identifier(SPAN, "mod"), exported);

          let component_render_expression =
            member(component_expression.clone_in(ast.allocator), "render");
          let hmr_call = ast.expression_call(
            SPAN,
            Expression::ComputedMemberExpression(ast.alloc_computed_member_expression(
              SPAN,
              ast.expression_identifier(SPAN, "__VUE_HMR_RUNTIME__"),
              ast.expression_conditional(
                SPAN,
                component_render_expression.clone_in(ast.allocator),
                ast.expression_string_literal(SPAN, "rerender", None),
                ast.expression_string_literal(SPAN, "reload", None),
              ),
              false,
            )),
            NONE,
            ast.vec_from_array([
              member(component_expression.clone_in(ast.allocator), "__hmrId").into(),
              ast
                .expression_logical(
                  SPAN,
                  component_render_expression,
                  LogicalOperator::Or,
                  component_expression.clone_in(ast.allocator),
                )
                .into(),
            ]),
            false,
          );
          // A removed or renamed export must not throw (a throw aborts every
          // later patch) — the guard skips it without blocking the others.
          callbacks.push(ast.statement_if(
            SPAN,
            component_expression,
            Statement::ExpressionStatement(ast.alloc_expression_statement(SPAN, hmr_call)),
            None,
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
        if callbacks.is_empty() {
          // nothing registered — nothing to accept
          return;
        }

        statements.push(
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

        // Leave modules outside a Vue realm (e.g. a web worker) to a plain
        // full reload instead of a no-op HMR accept.
        program.body.push(ast.statement_if(
          SPAN,
          ast.expression_binary(
            SPAN,
            ast.expression_unary(
              SPAN,
              UnaryOperator::Typeof,
              ast.expression_identifier(SPAN, "__VUE_HMR_RUNTIME__"),
            ),
            BinaryOperator::StrictInequality,
            ast.expression_string_literal(SPAN, ast.str("undefined"), None),
          ),
          ast.statement_block(SPAN, statements),
          None,
        ));
      }
    }
  }

  /// Wraps every exported plain function component into a stable component:
  ///
  /// ```js
  /// export const Comp = __defineVaporHmrComponent((props, instance) => <div />);
  /// export function Comp() {}
  /// Comp = __defineVaporHmrComponent(Comp); // keeps the hoisted binding kind
  /// ```
  ///
  /// The binding identity never changes, so parents holding it stay valid, and
  /// the implementation rides on a property that an update swaps in place (see
  /// the runtime helpers). Only component-like bindings are wrapped
  /// ([`should_wrap_component`]), so a lower case helper stays a plain function.
  fn wrap_function_components(&self, ast: &AstBuilder<'a>, program: &mut Program<'a>) {
    let local_alias = if self.options.interop {
      "_defineHmrComponent"
    } else {
      "_defineVaporHmrComponent"
    };
    let functions = self
      .components
      .iter()
      .filter_map(|comp| {
        if should_wrap_component(comp) {
          Some(comp.local)
        } else {
          None
        }
      })
      .collect::<Vec<_>>();
    if functions.is_empty() {
      return;
    }
    let is_function = |name: &str| functions.iter().any(|function| *function == name);

    let mut wrapped = vec![];
    for (index, node) in program.body.iter_mut().enumerate() {
      let mut statements = ast.vec();
      match node {
        // `const Comp = () => {}`
        Statement::VariableDeclaration(declaration) => {
          wrap_variable_declaration(ast, declaration, &is_function, local_alias);
        }
        // `function Comp() {}` (referenced by `export { Comp }`)
        Statement::FunctionDeclaration(declaration) => {
          if let Some(name) = declaration
            .id
            .as_ref()
            .map(|id| id.name.as_str())
            .filter(|name| is_function(name))
          {
            statements.push(reassign_declaration(ast, name, local_alias));
          }
        }
        Statement::ExportNamedDeclaration(export) => match export.declaration.take() {
          // `export const Comp = () => {}`
          Some(Declaration::VariableDeclaration(mut declaration)) => {
            wrap_variable_declaration(ast, &mut declaration, &is_function, local_alias);
            export.declaration = Some(Declaration::VariableDeclaration(declaration));
          }
          // `export function Comp() {}`
          Some(Declaration::FunctionDeclaration(declaration)) => {
            if let Some(name) = declaration
              .id
              .as_ref()
              .map(|id| id.name.as_str())
              .filter(|name| is_function(name))
            {
              export.declaration = Some(Declaration::FunctionDeclaration(declaration));
              statements.push(reassign_declaration(ast, name, local_alias));
            } else {
              export.declaration = Some(Declaration::FunctionDeclaration(declaration));
            }
          }
          declaration => export.declaration = declaration,
        },
        // `export default function Comp() {}`
        Statement::ExportDefaultDeclaration(export) => {
          if let ExportDefaultDeclarationKind::FunctionDeclaration(declaration) =
            &export.declaration
            && let Some(name) = declaration
              .id
              .as_ref()
              .map(|id| id.name.as_str())
              .filter(|name| is_function(name))
          {
            statements.push(reassign_declaration(ast, name, local_alias));
          }
        }
        _ => {}
      }
      if !statements.is_empty() {
        wrapped.push((index, statements));
      }
    }

    // insert from the back, so that the collected indices stay valid
    for (index, statements) in wrapped.into_iter().rev() {
      for (offset, statement) in statements.into_iter().enumerate() {
        program.body.insert(index + 1 + offset, statement);
      }
    }
  }
}

/// Whether a component binding is wrapped by the runtime HMR helper. Vue only
/// treats a binding starting with uppercase, `_` or `$` as a component — a
/// lower case name is a helper, even as a named default export. An anonymous
/// default export has no name to check and still wraps.
fn should_wrap_component(component: &Component) -> bool {
  component.is_function
    && (is_component_name(component.local) || is_component_name(component.exported))
}

/// Whether a binding name follows Vue's component naming convention.
fn is_component_name(name: &str) -> bool {
  name.chars().next().is_some_and(|first| {
    first.is_ascii_uppercase() || !first.is_ascii() || matches!(first, '_' | '$')
  })
}

/// Rewrites `const Comp = <fn>` so that `Comp` becomes the wrapper created by
/// the runtime HMR helper, keeping the original function as its implementation.
fn wrap_variable_declaration<'a>(
  ast: &AstBuilder<'a>,
  declaration: &mut VariableDeclaration<'a>,
  is_function: &impl Fn(&str) -> bool,
  local_alias: &str,
) {
  for declarator in &mut declaration.declarations {
    let BindingPattern::BindingIdentifier(identifier) = &declarator.id else {
      continue;
    };
    if !is_function(identifier.name.as_str()) {
      continue;
    }
    let Some(init) = declarator.init.take() else {
      continue;
    };
    // `define*()` calls create objects, Vue can patch those in place
    declarator.init = Some(if init.is_function() {
      ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(local_alias)),
        NONE,
        ast.vec1(init.into()),
        false,
      )
    } else {
      init
    });
  }
}

/// `Comp = __defineVaporHmrComponent(Comp);` — a function declaration creates a
/// hoisted, mutable binding: keeping the declaration intact preserves calls and
/// circular imports that run before it, and the reassignment swaps the binding
/// to the wrapper once the body reaches it.
fn reassign_declaration<'a>(ast: &AstBuilder<'a>, name: &str, local_alias: &str) -> Statement<'a> {
  ast.statement_expression(
    SPAN,
    ast.expression_assignment(
      SPAN,
      AssignmentOperator::Assign,
      AssignmentTarget::AssignmentTargetIdentifier(
        ast.alloc_identifier_reference(SPAN, ast.str(name)),
      ),
      ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(local_alias)),
        NONE,
        ast.vec1(ast.expression_identifier(SPAN, ast.str(name)).into()),
        false,
      ),
    ),
  )
}

/// `import { defineHmrComponent as _defineHmrComponent } from "<runtime>";`
fn hmr_import<'a>(ast: &AstBuilder<'a>, options: &TransformOptions<'a>) -> Statement<'a> {
  let (exported, local_alias, module_path) = if options.interop {
    (
      "defineHmrComponent",
      "_defineHmrComponent",
      "/vue-jsx-vapor/vdom",
    )
  } else {
    (
      "defineVaporHmrComponent",
      "_defineVaporHmrComponent",
      "/vue-jsx-vapor/vapor",
    )
  };
  Statement::ImportDeclaration(
    ast.alloc_import_declaration(
      SPAN,
      Some(ast.vec1(ast.import_declaration_specifier_import_specifier(
        SPAN,
        ast.module_export_name_identifier_name(SPAN, exported),
        ast.binding_identifier(SPAN, local_alias),
        ImportOrExportKind::Value,
      ))),
      ast.string_literal(
        SPAN,
        ast.str(
          options
            .runtime_module_name
            .as_deref()
            .unwrap_or(module_path),
        ),
        None,
      ),
      None,
      NONE,
      ImportOrExportKind::Value,
    ),
  )
}
