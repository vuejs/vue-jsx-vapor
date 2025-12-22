use common::text::is_empty_text;
use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{
    Argument, CallExpression, Expression, JSXChild, ObjectProperty, ObjectPropertyKind, PropertyKey,
  },
};
use oxc_span::SPAN;

use crate::{ast::VNodeCall, transform::TransformContext};

fn get_unnormalized_props<'a>(
  props: &mut Expression<'a>,
  call_path: &mut Vec<&mut Expression<'a>>,
) -> *mut Expression<'a> {
  let _props = props as *mut _;
  if let Expression::CallExpression(props) = props {
    if ["_normalizeProps", "_guardReactiveProps"].contains(&props.callee_name().unwrap_or_default())
    {
      call_path.push(unsafe { &mut *_props });
      return get_unnormalized_props(
        props.arguments.get_mut(0).unwrap().to_expression_mut(),
        call_path,
      );
    }
  }
  props
}

pub fn inject_prop<'a>(
  node: &mut VNodeCall<'a>,
  prop: ObjectProperty<'a>,
  context: &TransformContext<'a>,
) {
  let ast = &context.ast;
  let mut props_with_injection = None;
  // 1. mergeProps(...)
  // 2. toHandlers(...)
  // 3. normalizeProps(...)
  // 4. normalizeProps(guardReactiveProps(...))
  //
  // we need to get the real props before normalization
  let mut call_path = vec![];
  let mut parent_call = None;
  if let Some(mut props) = node.props.as_mut() {
    if let Expression::CallExpression(_) = props {
      props = unsafe { &mut *get_unnormalized_props(props, &mut call_path) };
      parent_call = call_path.last_mut();
    }
    if let Expression::CallExpression(props) = props {
      // merged props... add ours
      // only inject key to object literal if it's the first argument so that
      // if doesn't override user provided keys
      let callee_name = unsafe { &*(props as *mut oxc_allocator::Box<CallExpression>) }
        .callee_name()
        .unwrap_or_default();
      let first = props.arguments.first_mut();
      if let Some(first) = first
        && let Argument::ObjectExpression(first) = first
      {
        if callee_name.eq("_toHandlers") {
          // #2366
          props_with_injection = Some(
            ast.expression_call(
              SPAN,
              ast.expression_identifier(SPAN, ast.atom(&context.helper("mergeProps"))),
              NONE,
              ast.vec_from_array([
                ast
                  .expression_object(
                    SPAN,
                    ast.vec1(ObjectPropertyKind::ObjectProperty(ast.alloc(prop))),
                  )
                  .into(),
                Expression::CallExpression(props.take_in_box(context.allocator)).into(),
              ]),
              false,
            ),
          );
        } else {
          // #6631
          if !first.properties.iter().any(|p| {
            p.as_property()
              .map(|p| {
                if let PropertyKey::StaticIdentifier(key) = &p.key
                  && let PropertyKey::StaticIdentifier(prop_key) = &prop.key
                {
                  key.name == prop_key.name
                } else {
                  false
                }
              })
              .unwrap_or_default()
          }) {
            first
              .properties
              .insert(0, ObjectPropertyKind::ObjectProperty(ast.alloc(prop)));
          }
        }
      } else {
        props.arguments.insert(
          0,
          ast
            .expression_object(
              SPAN,
              ast.vec1(ObjectPropertyKind::ObjectProperty(ast.alloc(prop))),
            )
            .into(),
        )
      }
      if props_with_injection.is_none() {
        props_with_injection = Some(Expression::CallExpression(
          props.take_in_box(context.allocator),
        ));
      }
    } else if let Expression::ObjectExpression(props) = props {
      if !props.properties.iter().any(|p| {
        p.as_property()
          .map(|p| {
            if let PropertyKey::StaticIdentifier(key) = &p.key
              && let PropertyKey::StaticIdentifier(prop_key) = &prop.key
            {
              key.name == prop_key.name
            } else {
              false
            }
          })
          .unwrap_or_default()
      }) {
        props
          .properties
          .insert(0, ObjectPropertyKind::ObjectProperty(ast.alloc(prop)));
      }
      props_with_injection = Some(Expression::ObjectExpression(
        props.take_in_box(context.allocator),
      ));
    } else {
      // single v-bind with expression, return a merged replacement
      props_with_injection = Some(
        ast.expression_call(
          SPAN,
          ast.expression_identifier(SPAN, ast.atom(&context.helper("mergeProps"))),
          NONE,
          ast.vec_from_array([
            ast
              .expression_object(
                SPAN,
                ast.vec1(ObjectPropertyKind::ObjectProperty(ast.alloc(prop))),
              )
              .into(),
            props.take_in(context.allocator).into(),
          ]),
          false,
        ),
      );
      // in the case of nested helper call, e.g. `normalizeProps(guardReactiveProps(props))`,
      // it will be rewritten as `normalizeProps(mergeProps({ key: 0 }, props))`,
      // the `guardReactiveProps` will no longer be needed
      if let Some(Expression::CallExpression(parent)) = parent_call
        && parent
          .callee_name()
          .unwrap_or_default()
          .eq("_guardReactiveProps")
      {
        let len = call_path.len();
        call_path.remove(len - 1);
        parent_call = None;
      }
    }
  } else {
    props_with_injection = Some(ast.expression_object(
      SPAN,
      ast.vec1(ObjectPropertyKind::ObjectProperty(ast.alloc(prop))),
    ));
  };

  if let Some(Expression::CallExpression(parent_call)) = parent_call {
    parent_call
      .arguments
      .insert(0, props_with_injection.unwrap().into());
  } else {
    node.props = props_with_injection
  }
}

pub fn get_children<'a>(node: &'a mut JSXChild<'a>) -> Vec<&'a mut JSXChild<'a>> {
  match node {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => unimplemented!(),
  }
  .into_iter()
  .filter(|child| !is_empty_text(child))
  .collect::<Vec<_>>()
}
