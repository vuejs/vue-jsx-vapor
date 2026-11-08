use common::expression::gen_getter;
use napi::bindgen_prelude::Either3;
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression, PropertyKind, Statement, VariableDeclarationKind};
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::block::{gen_block, mark_slot_root_operations};
use crate::generate::component::gen_raw_props;
use crate::generate::expression::gen_expression;
use crate::generate::prop::gen_literal_object_props;
use crate::ir::component::IRProps;
use crate::ir::index::{BlockIRNode, SlotOutletIRNode};

pub fn gen_slot_outlet<'a>(
  oper: SlotOutletIRNode<'a>,
  context: &'a CodegenContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let SlotOutletIRNode {
    id,
    name,
    fallback,
    props,
    flags,
    ..
  } = oper;

  Statement::VariableDeclaration(
    ast.alloc_variable_declaration(
      SPAN,
      VariableDeclarationKind::Const,
      ast.vec1(
        ast.variable_declarator(
          SPAN,
          VariableDeclarationKind::Const,
          ast.binding_pattern_binding_identifier(SPAN, ast.str(&format!("_n{}", id))),
          NONE,
          Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.str(context.options.helper("_createSlot"))),
              NONE,
              ast.vec_from_iter(
                [
                  if matches!(&name, Expression::StringLiteral(name) if name.value == "default")
                    && props.is_empty()
                    && fallback.is_none()
                    && flags == 0
                  {
                    None
                  } else if let Expression::StringLiteral(name) = name {
                    Some(ast.expression_string_literal(SPAN, name.value, None).into())
                  } else {
                    Some(gen_getter(gen_expression(name, context, None, false), ast).into())
                  },
                  if !props.is_empty()
                    && let Some(props) = gen_slot_props(props, context)
                  {
                    Some(props.into())
                  } else if fallback.is_some() || flags > 0 {
                    Some(ast.expression_null_literal(SPAN).into())
                  } else {
                    None
                  },
                  if let Some(mut fallback) = fallback {
                    if *context.in_slot_block.borrow() {
                      // Forwarded fallback validity affects the owning slot's exposed branch;
                      mark_slot_root_operations(&mut fallback, context);
                    }
                    Some(gen_block(fallback, context, context_block, ast.vec()).into())
                  } else if flags > 0 {
                    Some(ast.expression_null_literal(SPAN).into())
                  } else {
                    None
                  },
                  if flags > 0 {
                    Some(
                      ast
                        .expression_numeric_literal(
                          SPAN,
                          flags as f64,
                          None,
                          oxc_ast::ast::NumberBase::Decimal,
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
}

fn gen_slot_props<'a>(
  props: Vec<IRProps<'a>>,
  context: &'a CodegenContext<'a>,
) -> Option<Expression<'a>> {
  if props.is_empty() {
    return None;
  }
  // one group of static keys keeps the plain object shape; every other grouping
  // has to survive as its own merge argument
  if props.len() == 1
    && let Either3::A(group) = &props[0]
    && group
      .iter()
      .all(|prop| matches!(&prop.key, Expression::StringLiteral(_)))
  {
    return gen_raw_props(props, context);
  }

  let ast = &context.ast;
  let call = |name: &'a str, args: Vec<Argument<'a>>| {
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper(name))),
      NONE,
      ast.vec_from_iter(args),
      false,
    )
  };
  let handler_only = props.len() == 1
    && matches!(&props[0], Either3::C(dynamic) if dynamic.handler);
  let mut groups = Vec::with_capacity(props.len());
  for group in props {
    groups.push(match group {
      Either3::A(group) => gen_literal_object_props(group, context, true),
      Either3::B(prop) => gen_literal_object_props(vec![prop], context, true),
      Either3::C(dynamic) => {
        let value = gen_expression(dynamic.value, context, None, false);
        if dynamic.handler {
          call("_toHandlers", vec![value.into()])
        } else {
          value
        }
      }
    });
  }
  let value = if groups.len() > 1 {
    // mergeProps normalizes class and style itself
    call("_mergeProps", groups.into_iter().map(Into::into).collect())
  } else if handler_only {
    groups.remove(0)
  } else {
    call(
      "_normalizeProps",
      vec![call("_guardReactiveProps", vec![groups.remove(0).into()]).into()],
    )
  };

  let mut properties = ast.vec();
  properties.push(ast.object_property_kind_object_property(
    SPAN,
    PropertyKind::Init,
    ast.property_key_static_identifier(SPAN, ast.str("$")),
    ast.expression_array(SPAN, ast.vec1(gen_getter(value, ast).into())),
    false,
    false,
    false,
  ));
  Some(ast.expression_object(SPAN, properties))
}
