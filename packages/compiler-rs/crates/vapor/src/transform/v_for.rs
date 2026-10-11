use oxc_allocator::TakeIn;
use oxc_ast::ast::{
  Argument, BinaryExpression, BinaryOperator, BindingPattern, Expression, JSXAttribute,
  JSXAttributeName, JSXAttributeValue, JSXChild, JSXElement, Statement,
};
use oxc_ast_visit::{Visit, walk};
use oxc_span::SPAN;

use crate::{
  ir::index::{BlockIRNode, DynamicFlag, ForIRNode, IRFor, OperationNode},
  transform::{TransformContext, transform_element::is_transition_tag},
};
use common::{
  ast::RootNode,
  check::{is_constant_node, is_custom_element, is_jsx_component},
  directive::Directives,
  error::ErrorCodes,
  expression::jsx_attribute_value_to_expression,
  text::is_empty_text,
};

/// Lower direct keyed JSX maps before text runs are collected, so they use the
/// same reconciliation, insertion and alias handling as a `v-for` directive.
pub fn transform_map_children<'a>(node: &mut JSXChild<'a>, context: &TransformContext<'a>) {
  let children = match node {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => return,
  };
  for child in children {
    lower_map_child(child, context);
  }
}

fn lower_map_child<'a>(child: &mut JSXChild<'a>, context: &TransformContext<'a>) -> Option<()> {
  let JSXChild::ExpressionContainer(container) = child else {
    return None;
  };
  let span = container.span;
  let Expression::CallExpression(call) = container
    .expression
    .as_expression_mut()?
    .get_inner_expression_mut()
  else {
    return None;
  };
  let call = call.as_mut();
  let Expression::StaticMemberExpression(member) = &mut call.callee else {
    return None;
  };
  if call.optional || member.optional || member.property.name != "map" || call.arguments.len() != 1
  {
    return None;
  }
  let Argument::ArrowFunctionExpression(callback) = &mut call.arguments[0] else {
    return None;
  };
  let callback = callback.as_mut();
  let semantic = context.options.semantic.borrow();
  // The third map argument is the source array, not v-for's ordinal index.
  // Mutated aliases and complex callbacks need ordinary JS evaluation.
  if callback.r#async
    || callback.params.rest.is_some()
    || !(1..=2).contains(&callback.params.items.len())
    || !callback.params.items.iter().all(|param| {
      let BindingPattern::BindingIdentifier(id) = &param.pattern else {
        return false;
      };
      param.initializer.is_none()
        && id
          .symbol_id
          .get()
          .is_some_and(|symbol| !semantic.scoping().symbol_is_mutated(symbol))
    })
    || !callback.body.directives.is_empty()
    || callback.body.statements.len() != 1
  {
    return None;
  }
  let row = match &mut callback.body.statements[0] {
    Statement::ExpressionStatement(statement) if callback.expression => &mut statement.expression,
    Statement::ReturnStatement(statement) if !callback.expression => statement.argument.as_mut()?,
    _ => return None,
  };
  let Expression::JSXElement(row) = row.get_inner_expression_mut() else {
    return None;
  };
  // A structural directive on the row has its own scope/precedence. Keep that
  // expression on the existing path rather than changing its meaning.
  if row.opening_element.attributes.iter().any(|attribute| {
    attribute.as_attribute().is_some_and(|attribute| {
      matches!(&attribute.name, JSXAttributeName::Identifier(name)
        if matches!(name.name.as_str(), "v-if" | "v-else-if" | "v-else" | "v-for" | "v-slot" | "v-slots"))
        || matches!(&attribute.name, JSXAttributeName::NamespacedName(name) if name.namespace.name == "v-slot")
    })
  }) || !row.opening_element.attributes.iter().any(|attribute| {
    attribute.as_attribute().is_some_and(|attribute| attribute.is_key() && attribute.value.is_some())
  }) {
    return None;
  }

  let mut refs = TemplateRefVisitor(false);
  refs.visit_jsx_element(row);
  if refs.0 {
    return None;
  }

  let ast = context.ast;
  let aliases = ast.vec_from_iter(callback.params.items.iter().map(|param| {
    let BindingPattern::BindingIdentifier(id) = &param.pattern else {
      unreachable!();
    };
    ast.expression_identifier(id.span, id.name)
  }));
  let alias = if aliases.len() == 1 {
    aliases.into_iter().next().unwrap()
  } else {
    ast.expression_sequence(callback.params.span, aliases)
  };
  let source = member.object.take_in(context.allocator);
  row.opening_element.attributes.push(
    ast.jsx_attribute_item_attribute(
      span,
      ast.jsx_attribute_name_identifier(span, "v-for"),
      Some(
        ast.jsx_attribute_value_expression_container(
          span,
          ast
            .expression_binary(span, alias, BinaryOperator::In, source)
            .into(),
        ),
      ),
    ),
  );
  *child = JSXChild::Element(row.take_in_box(context.allocator));
  context.mapped_for_spans.borrow_mut().insert(span.start);
  Some(())
}

struct TemplateRefVisitor(bool);

impl<'a> Visit<'a> for TemplateRefVisitor {
  fn visit_jsx_attribute(&mut self, attribute: &JSXAttribute<'a>) {
    self.0 |= matches!(&attribute.name, JSXAttributeName::Identifier(name) if name.name == "ref");
    walk::walk_jsx_attribute(self, attribute);
  }
}

/// # SAFETY
pub unsafe fn transform_v_for<'a>(
  directives: &'a mut Directives<'a>,
  context_node: *mut JSXChild<'a>,
  context: &'a TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: &'a mut JSXChild<'a>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let JSXChild::Element(node) = (unsafe { &mut *context_node }) else {
    return None;
  };
  let node_ptr = node as *mut oxc_allocator::Box<JSXElement>;

  let dir = directives.v_for.as_mut()?;
  let seen = &mut context.seen.borrow_mut();
  let span = dir.span;
  if seen.contains(&span.start) {
    return None;
  }
  seen.insert(span.start);

  let IRFor {
    value,
    index,
    key,
    source,
  } = get_for_parse_result(dir, context)?;

  let Some(source) = source else {
    context.options.on_error.as_ref()(ErrorCodes::VForMalformedExpression.message(), span);
    return None;
  };

  let key_prop = if let Some(key_prop) = directives.key.as_mut()
    && let Some(value) = &mut key_prop.value
  {
    seen.insert(key_prop.span.start);
    jsx_attribute_value_to_expression(value, context.ast)
  } else {
    None
  };

  let is_component =
    directives.is_component || (directives.is_template && is_template_with_single_component(node));
  // mirrors compiler-ssr: a template row that is not a single element renders
  // as a fragment, except under a Transition, whose children render without
  // nested fragment markers. A v-if or v-for on the child turns it into an
  // if/for node by the time the runtime decides, so it counts.
  let wrapped_rows = directives.is_template
    && !matches!(parent_node, JSXChild::Element(parent) if is_transition_tag(parent.opening_element.name.get_identifier_name().as_deref().unwrap_or_default()))
    && (node.children.len() != 1
      || !matches!(&node.children[0], JSXChild::Element(child)
        if !has_row_fragment_directive(child)));
  let dynamic = &mut context_block.dynamic;
  let id = context.reference(dynamic);
  dynamic.flags = dynamic.flags | DynamicFlag::NonTemplate as i32 | DynamicFlag::Insert as i32;
  let block = context_block as *mut BlockIRNode;
  let exit_block = context.create_block(
    unsafe { &mut *context_node },
    unsafe { &mut *block },
    Expression::JSXElement(oxc_allocator::Box::new_in(
      unsafe { &mut *node_ptr }.take_in(context.allocator),
      context.allocator,
    )),
    true,
    None,
  );

  // if v-for is the only child of a parent element, it can go the fast path
  // when the entire list is emptied
  let mut only_child = false;
  let parent_directives = &context.parent_directives.borrow();
  if let JSXChild::Element(parent_node) = parent_node
    && !(parent_directives.is_component || parent_directives.is_custom_element)
    && !parent_directives.is_template
  {
    let index = *context.index.borrow() as usize;
    for (i, child) in parent_node.children.iter().enumerate() {
      let child = if index == i {
        if RootNode::is_root(unsafe { &*context_node }) {
          child
        } else {
          unsafe { &*context_node }
        }
      } else {
        child
      };
      if !is_empty_text(child) || matches!(child, JSXChild::Text(text) if text.span == SPAN) {
        if only_child {
          only_child = false;
          break;
        }
        only_child = true;
      }
    }
  };

  Some(Box::new(move || {
    let block = exit_block();

    context_block.dynamic.operation = Some(Box::new(OperationNode::For(ForIRNode {
      mapped: context.mapped_for_spans.borrow().contains(&span.start),
      id,
      value,
      key,
      index,
      key_prop,
      render: block,
      once: *context.in_v_once.borrow() || is_constant_node(&source),
      source,
      component: is_component,
      only_child,
      wrapped_rows,
      parent: None,
      anchor: None,
      append_index: None,
      operation_index: Some(*context.operation_index.borrow()),
      effect_index: Some(*context.effect_index.borrow()),
      slot_root: false,
    })));
  }))
}

fn has_row_fragment_directive(node: &JSXElement) -> bool {
  node.opening_element.attributes.iter().any(|attribute| {
    let oxc_ast::ast::JSXAttributeItem::Attribute(attribute) = attribute else {
      return false;
    };
    matches!(
      &attribute.name,
      oxc_ast::ast::JSXAttributeName::Identifier(name)
        if matches!(name.name.as_str(), "v-if" | "v-for")
    )
  })
}

pub fn get_for_parse_result<'a>(
  dir: &'a mut JSXAttribute<'a>,
  context: &'a TransformContext<'a>,
) -> Option<IRFor<'a>> {
  let mut value: Option<Expression> = None;
  let mut index: Option<Expression> = None;
  let mut key: Option<Expression> = None;
  let mut source: Option<Expression> = None;
  if let Some(dir_value) = &mut dir.value {
    let expression = if let JSXAttributeValue::ExpressionContainer(dir_value) = dir_value {
      Some(
        dir_value
          .expression
          .to_expression_mut()
          .without_parentheses_mut()
          .get_inner_expression_mut(),
      )
    } else {
      None
    };
    if let Some(expression) = expression
      && let Expression::BinaryExpression(expression) = expression
    {
      let expression = expression as *mut oxc_allocator::Box<BinaryExpression>;
      let left = unsafe { &mut *expression }
        .left
        .without_parentheses_mut()
        .get_inner_expression_mut();
      if let Expression::SequenceExpression(left) = left {
        let expressions = &mut left.expressions as *mut oxc_allocator::Vec<Expression>;
        value = unsafe { &mut *expressions }
          .get_mut(0)
          .map(|e| e.take_in(context.allocator));
        key = unsafe { &mut *expressions }
          .get_mut(1)
          .map(|e| e.take_in(context.allocator));
        index = unsafe { &mut *expressions }
          .get_mut(2)
          .map(|e| e.take_in(context.allocator));
      } else {
        value = Some(left.take_in(context.allocator));
      };
      source = Some((unsafe { &mut *expression }.right).take_in(context.allocator));
    }
  } else {
    context.options.on_error.as_ref()(ErrorCodes::VForNoExpression.message(), dir.span);
    return None;
  }
  Some(IRFor {
    value,
    index,
    key,
    source,
  })
}

fn is_template_with_single_component<'a>(node: &'a JSXElement<'a>) -> bool {
  let non_comment_children = node
    .children
    .iter()
    .filter(|c| !is_empty_text(c))
    .collect::<Vec<_>>();

  non_comment_children.len() == 1
    && matches!(non_comment_children[0], JSXChild::Element(child)
      if is_jsx_component(child) || is_custom_element(child)
    )
}
