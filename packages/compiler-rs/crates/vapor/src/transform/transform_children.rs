use std::{borrow::Cow, mem};

use oxc_allocator::{CloneIn, TakeIn};
use oxc_ast::ast::{Expression, JSXAttributeItem, JSXChild};
use oxc_span::{GetSpan, SPAN, Span};

use crate::{
  ir::index::{
    BlockIRNode, DynamicFlag, IRDynamicInfo, InsertNodeIRNode,
    OperationNode::{self},
  },
  transform::{
    TransformContext,
    transform_element::get_child_template_close_tags,
    transform_element::{is_keep_alive_tag, is_transition_tag},
    transform_text::mark_non_template,
  },
};

use common::{
  ast::RootNode,
  check::get_namespace,
  directive::Directives,
  html_nesting::{enter_open_scope, keeps_in_template, open_scope},
  text::{get_tag_name, is_empty_text},
};

/// # SAFETY
pub unsafe fn transform_children<'a>(
  directives: Directives<'a>,
  node: &mut JSXChild<'a>,
  context: &TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
  parent_node: Option<&JSXChild<'a>>,
) -> Option<Box<dyn FnOnce() + 'a>> {
  let is_fragment_or_component = RootNode::is_root(node)
    || matches!(node, JSXChild::Fragment(_))
    || directives.is_template
    || (directives.tag_name == "template" && directives.key.is_some())
    || directives.is_component;

  if !matches!(&node, JSXChild::Element(_)) && !is_fragment_or_component {
    return None;
  }

  let _node = node as *mut _;
  let parent_tag_name = directives.tag_name;
  // a plain element writes its children into its template string; a `<template>`
  // keeps them in `.content`, and a component or fragment registers each child
  // as a template of its own
  let hosts_template = !is_fragment_or_component && parent_tag_name != "template";
  let (child_template_close_tags, child_template_close_blocks) = if hosts_template {
    get_child_template_close_tags(parent_tag_name, parent_node, context)
  } else {
    Default::default()
  };
  // what this template leaves open above its children
  let child_scope = if hosts_template {
    let parent_ns = *context.ns.borrow();
    enter_open_scope(
      *context.template_scope.borrow(),
      parent_tag_name,
      parent_ns.ns,
    )
  } else {
    0
  };
  let children = match node {
    JSXChild::Element(node) => &mut node.children,
    JSXChild::Fragment(node) => &mut node.children,
    _ => unreachable!(),
  };
  // Slot content is executed by the child component, which re-runs it on its
  // own updates (vdom parity), so v-once does not reach into it.
  let child_in_v_once = *context.in_v_once.borrow() && !directives.is_component;
  let children_ptr = children as *mut oxc_allocator::Vec<JSXChild>;
  let mut parent_children_template = context.children_template.take();
  let grand_parent_dynamic = context
    .parent_dynamic
    .replace(mem::take(&mut context_block.dynamic));
  let grand_parent_directives = context.parent_directives.replace(directives);
  let grandparent_node_span = context.grandparent_node_span.replace(
    // A Transition / KeepAlive parent swaps the span its children read for a
    // single-root one: its child is a root template wherever the transition
    // sits, provided it keeps a single child. Only such a parent pays for the
    // scan, and a ternary child is one child to begin with - the scan only
    // collapses a `v-if` / `v-else` directive chain into one.
    if (is_transition_tag(parent_tag_name) || is_keep_alive_tag(parent_tag_name))
      && has_single_root_child(children)
    {
      Span::new(3, 0)
    } else {
      parent_node.map_or(SPAN, |node| node.span())
    },
  );
  let _context_block = context_block as *mut BlockIRNode;
  let mut i = 0;
  if let Some(last) = children.last()
    && is_empty_text(last)
  {
    children.pop();
  }

  let mut children_len = children.len();
  while let Some(child) = children.get_mut(i) {
    if is_empty_text(child) {
      children.remove(i);
      children_len -= 1;
      continue;
    } else if let JSXChild::Fragment(child) = child {
      children_len += child.children.len();
      unsafe { &mut *children_ptr }.splice(i..i + 1, child.children.take_in(context.allocator));
      continue;
    }
    let mut tag = "";
    let mut child_ns = 0;
    let mut next_is_interpolation = false;
    let is_text_child = matches!(&child, JSXChild::Text(_));
    let exit_context = context.create(
      if is_text_child
        && let Some(next) = unsafe { &mut *children_ptr }.get_mut(i + 1)
        && let JSXChild::ExpressionContainer(exp) = next
        && let Some(expression) = exp.expression.as_expression()
        && !matches!(
          expression.get_inner_expression(),
          Expression::ConditionalExpression(_)
        )
      {
        next_is_interpolation = true;
        child.clone_in(context.allocator)
      } else {
        if let JSXChild::Element(child) = child {
          tag = get_tag_name(child, context.options);
          // the namespace the child resolves to, which decides whether the
          // parser would keep it where it is written
          let parent_ns = *context.ns.borrow();
          child_ns = get_namespace(
            tag,
            parent_ns.tag,
            parent_ns.ns,
            parent_ns.is_html_annotation_xml,
          );
        }
        child.take_in(context.allocator)
      },
      i as i32,
      if !parent_tag_name.is_empty() {
        children_len == i + 1
      } else {
        true
      },
      unsafe { &mut *_context_block },
      child_in_v_once,
    );
    // The text belongs to the interpolation that follows it, so it must not be
    // materialized on its own. Marking it here rather than in the parent
    // pre-pass covers the root (which never runs the pre-pass) and nested
    // fragments (flattened into their parent after the pre-pass ran).
    if next_is_interpolation {
      mark_non_template(child, &mut context.seen.borrow_mut());
    }
    let is_same_template =
      hosts_template && keeps_in_template(tag, child_ns, parent_tag_name, child_scope);
    if is_same_template {
      *context.template_close_tags.borrow_mut() = child_template_close_tags.clone();
      *context.template_close_blocks.borrow_mut() = child_template_close_blocks;
      *context.template_scope.borrow_mut() = child_scope | open_scope::SHARED;
    } else {
      context.template_close_tags.borrow_mut().clear();
      *context.template_close_blocks.borrow_mut() = false;
      *context.template_scope.borrow_mut() = 0;
    }
    context.transform_node(
      Some(unsafe { &mut *_context_block }),
      Some(unsafe { &mut *_node }),
    );

    let mut parent_dynamic = context.parent_dynamic.borrow_mut();
    let flags = context_block.dynamic.flags;
    if is_fragment_or_component {
      if !next_is_interpolation {
        context.register_template(
          context_block,
          child_ns,
          is_text_child || context.can_use_static_template(context_block, tag),
        );
        context.reference(&mut context_block.dynamic);
        if flags & DynamicFlag::NonTemplate as i32 == 0 || flags & DynamicFlag::Insert as i32 != 0 {
          context_block
            .returns
            .push(context_block.dynamic.id.unwrap());
        }
      }
    } else {
      parent_children_template.push(Cow::Owned(context.template.take()));
    }

    if context_block.dynamic.has_dynamic_child
      || context_block.dynamic.id.is_some()
      || flags & DynamicFlag::NonTemplate as i32 != 0
      || flags & DynamicFlag::Insert as i32 != 0
    {
      parent_dynamic.has_dynamic_child = true;
    }

    parent_dynamic
      .children
      .insert(i, mem::take(&mut context_block.dynamic));

    exit_context();
    i += 1;
  }
  *context.children_template.borrow_mut() = parent_children_template;
  *context.grandparent_node_span.borrow_mut() = grandparent_node_span;
  context_block.dynamic = context.parent_dynamic.replace(grand_parent_dynamic);
  *context.parent_directives.borrow_mut() = grand_parent_directives;

  if !is_fragment_or_component {
    process_dynamic_children(context, context_block);
  }

  None
}

fn process_dynamic_children<'a>(
  context: &TransformContext<'a>,
  context_block: &'a mut BlockIRNode<'a>,
) {
  let children = &mut context_block.dynamic.children as *mut Vec<IRDynamicInfo>;

  // The index of the last child that materializes in the parent template.
  // Dynamic children before it are anchored by their own `<!>` placeholder;
  // dynamic children after it are appends and need no placeholder.
  let mut last_template_index = -1;
  for (i, child) in unsafe { &mut *children }.iter_mut().enumerate().rev() {
    let flags = child.flags;
    if flags & DynamicFlag::NonTemplate as i32 == 0 {
      last_template_index = i as i32;
      break;
    }
  }

  // Logical unit counter. Each template child (placeholders included) and
  // each trailing dynamic block occupies one SSR logical unit; for template
  // children the unit index equals the CSR element index by construction,
  // which is what lets hydration reuse the CSR locators unchanged.
  let mut unit_index = 0;

  for (index, child) in unsafe { &mut *children }.iter_mut().enumerate() {
    if child.flags & DynamicFlag::Insert as i32 != 0 {
      let mut anchor = None;
      if (index as i32) < last_template_index {
        // anchored insert: own `<!>` placeholder in the parent template,
        // located at runtime and passed as the insertion anchor
        context.children_template.borrow_mut()[index] = Cow::Borrowed("<!>");
        child.flags =
          (child.flags - DynamicFlag::NonTemplate as i32) | DynamicFlag::Referenced as i32;
        child.anchor = Some(context.increase_id());
        anchor = child.anchor;
      }
      if child.template.is_some()
        && let Some(id) = child.id
      {
        // template node due to invalid nesting - generate actual insertion,
        // appended when no anchor was assigned
        child.operation = Some(Box::new(OperationNode::InsertNode(InsertNodeIRNode {
          elements: vec![id],
          parent: context.reference(&mut context_block.dynamic),
          anchor,
        })));
      } else if let Some(operation) = match child.operation.as_deref_mut() {
        Some(OperationNode::If(operation)) => Some((
          &mut operation.parent,
          &mut operation.anchor,
          &mut operation.append_index,
        )),
        Some(OperationNode::For(operation)) => Some((
          &mut operation.parent,
          &mut operation.anchor,
          &mut operation.append_index,
        )),
        Some(OperationNode::Key(operation)) => Some((
          &mut operation.parent,
          &mut operation.anchor,
          &mut operation.append_index,
        )),
        Some(OperationNode::CreateComponent(operation)) => Some((
          &mut operation.parent,
          &mut operation.anchor,
          &mut operation.append_index,
        )),
        Some(OperationNode::SlotOutlet(operation)) => Some((
          &mut operation.parent,
          &mut operation.anchor,
          &mut operation.append_index,
        )),
        _ => None,
      } {
        *operation.0 = Some(context.reference(&mut context_block.dynamic));
        if let Some(anchor) = anchor {
          *operation.1 = Some(anchor);
        } else {
          // append: the block's SSR output starts at logical unit `unitIndex`
          *operation.2 = Some(unit_index);
        }
      }
      unit_index += 1;
    } else if child.flags & DynamicFlag::NonTemplate as i32 == 0 {
      unit_index += 1;
    }
  }
}

/// Upstream `hasSingleRootChild`: one meaningful child, or a complete
/// `v-if` / `v-else-if` / `v-else` chain counting as one.
pub(crate) fn has_single_root_child(children: &[JSXChild]) -> bool {
  let mut non_comment_children = 0usize;
  let mut has_encountered_if = false;
  let mut is_single_if_block = true;
  for child in children {
    if is_empty_text(child) {
      continue;
    }
    non_comment_children += 1;
    let dir_name = if let JSXChild::Element(element) = child {
      element.opening_element.attributes.iter().find_map(|attr| {
        if let JSXAttributeItem::Attribute(attr) = attr
          && let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name
          && matches!(name.name.as_str(), "v-if" | "v-else-if" | "v-else")
        {
          Some(name.name.as_str())
        } else {
          None
        }
      })
    } else {
      None
    };
    if dir_name == Some("v-if") {
      if has_encountered_if {
        // a second `v-if` means two meaningful children already
        return false;
      }
      has_encountered_if = true;
    } else if !has_encountered_if || (dir_name != Some("v-else") && dir_name != Some("v-else-if")) {
      // a lone child may still be the single root; a second one that does
      // not close the chain never is
      if non_comment_children > 1 {
        return false;
      }
      is_single_if_block = false;
    }
  }
  non_comment_children == 1 || is_single_if_block
}
