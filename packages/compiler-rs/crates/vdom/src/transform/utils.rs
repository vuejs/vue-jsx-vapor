use oxc_allocator::TakeIn;
use oxc_ast::{
  NONE,
  ast::{Argument, Expression, ObjectProperty, ObjectPropertyKind, PropertyKey},
};
use oxc_span::{GetSpan, SPAN};

use crate::{ast::VNodeCall, transform::TransformContext};

fn get_unnormalized_props<'a>(
  props: &mut Expression<'a>,
  call_path: &mut Vec<&mut Expression<'a>>,
) -> *mut Expression<'a> {
  let _props = props as *mut _;
  if let Expression::CallExpression(props) = props {
    let callee = if let Expression::Identifier(callee) = &props.callee {
      callee.name.to_string()
    } else {
      String::new()
    };
    if ["normalizeProps", "guardReactiveProps"].contains(&callee.as_str()) {
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
  let source_text = context.ir.borrow().source;
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
      let first = props.arguments.first_mut();
      if let Some(first) = first
        && let Argument::ObjectExpression(first) = first
      {
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
      } else {
        if props
          .callee
          .span()
          .source_text(source_text)
          .eq("toHandlers")
        {
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
          )
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
          .callee
          .span()
          .source_text(source_text)
          .eq("guardReactiveProps")
      {
        let len = call_path.len();
        parent_call = call_path.get_mut(len - 2);
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
