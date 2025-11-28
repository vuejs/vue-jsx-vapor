use crate::{transform::TransformContext, traverse::hmr_or_ssr::HmrOrSsrTraverse};
use oxc_allocator::{Allocator, TakeIn};
use oxc_ast::{
  NONE,
  ast::{
    Argument, BindingPatternKind, Expression, ImportOrExportKind, Program, Statement,
    VariableDeclarationKind,
  },
};
use oxc_semantic::SemanticBuilder;
use oxc_span::{GetSpan, SPAN};
use oxc_traverse::{Ancestor, Traverse, TraverseCtx, traverse_mut};

pub struct JsxTraverse<'a, 'ctx> {
  allocator: &'a Allocator,
  source_text: &'a str,
  roots: Vec<*mut Expression<'a>>,
  context: &'ctx TransformContext<'a>,
}

impl<'a, 'ctx: 'a> JsxTraverse<'a, 'ctx> {
  pub fn new(allocator: &'a Allocator, context: &'ctx TransformContext<'a>) -> Self {
    Self {
      allocator,
      source_text: "",
      roots: vec![],
      context,
    }
  }

  pub fn traverse(mut self, program: &mut Program<'a>) {
    let allocator = self.allocator;

    self.source_text = program.source_text;

    traverse_mut(
      &mut self,
      allocator,
      program,
      SemanticBuilder::new()
        .build(program)
        .semantic
        .into_scoping(),
      (),
    );
  }
}

impl<'a, 'ctx: 'a> Traverse<'a, ()> for JsxTraverse<'a, 'ctx> {
  fn enter_expression(
    &mut self,
    node: &mut Expression<'a>,
    ctx: &mut oxc_traverse::TraverseCtx<'a, ()>,
  ) {
    if !matches!(node, Expression::JSXElement(_) | Expression::JSXFragment(_)) {
      return;
    }
    if self.context.options.interop {
      for node in ctx.ancestors() {
        if let Ancestor::CallExpressionArguments(node) = node {
          let name = node.callee().span().source_text(self.source_text);
          if name == "defineVaporComponent" {
            break;
          } else if name == "defineComponent" {
            return;
          }
        }
      }
    }
    self.roots.push(node as *mut Expression);
  }
  fn exit_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a, ()>) {
    if self.context.options.ssr || self.context.options.hmr {
      HmrOrSsrTraverse::new(self.context.options).exit_program(program, ctx);
    }

    let allocator = ctx.ast.allocator;
    unsafe {
      for root in self.roots.drain(..) {
        let root = &mut *root;
        let source = &self.source_text[..root.span().end as usize];
        *root = self.context.transform(root.take_in(allocator), source);
      }
    }

    let ast = &ctx.ast;
    let mut statements = vec![];
    let delegates = self.context.options.delegates.take();
    if !delegates.is_empty() {
      statements.push(ast.statement_expression(
        SPAN,
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom("_delegateEvents")),
          NONE,
          oxc_allocator::Vec::from_iter_in(
            delegates.iter().map(|delegate| {
              Argument::StringLiteral(ctx.alloc(ast.string_literal(SPAN, ast.atom(delegate), None)))
            }),
            ast.allocator,
          ),
          false,
        ),
      ));
    }

    let mut helpers = self.context.options.helpers.take();
    if !helpers.is_empty() {
      let jsx_helpers = vec![
        "setNodes",
        "createNodes",
        "createComponent",
        "createComponentWithFallback",
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
      if !jsx_helpers.is_empty() {
        statements.push(Statement::ImportDeclaration(ast.alloc_import_declaration(
          SPAN,
          Some(ast.vec_from_iter(jsx_helpers.into_iter().map(|helper| {
            ast.import_declaration_specifier_import_specifier(
              SPAN,
              ast.module_export_name_identifier_name(SPAN, ast.atom(helper)),
              ast.binding_identifier(SPAN, ast.atom(format!("_{}", helper).as_str())),
              ImportOrExportKind::Value,
            )
          }))),
          ast.string_literal(SPAN, ast.atom("vue-jsx-vapor"), None),
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

    let templates = self.context.options.templates.take();
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
              ast.binding_pattern(
                BindingPatternKind::BindingIdentifier(
                  ast.alloc_binding_identifier(SPAN, ast.atom(&format!("t{index}"))),
                ),
                NONE,
                false,
              ),
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
