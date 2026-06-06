use std::borrow::Cow;

use common::text::capitalize;
use common::text::get_text_like_value;
use napi::bindgen_prelude::Either3;
use oxc_allocator::CloneIn;
use oxc_allocator::TakeIn;
use oxc_ast::NONE;
use oxc_ast::ast::ArrayExpressionElement;
use oxc_ast::ast::BinaryOperator;
use oxc_ast::ast::ConditionalExpression;
use oxc_ast::ast::Expression;
use oxc_ast::ast::NumberBase;
use oxc_ast::ast::ObjectExpression;
use oxc_ast::ast::ObjectPropertyKind;
use oxc_ast::ast::PropertyKey;
use oxc_ast::ast::PropertyKind;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::component::IRProp;
use crate::ir::index::SetDynamicPropsIRNode;
use crate::ir::index::SetPropIRNode;
use common::check::is_simple_identifier;
use common::check::is_svg_tag;

pub struct HelperConfig<'a> {
  name: &'a str,
  need_key: bool,
  is_svg: bool,
}

fn helpers<'a>(name: &str, is_svg: bool) -> HelperConfig<'a> {
  match name {
    "setText" => HelperConfig {
      name: "_setText",
      need_key: false,
      is_svg,
    },
    "setHtml" => HelperConfig {
      name: "_setHtml",
      need_key: false,
      is_svg,
    },
    "setClass" => HelperConfig {
      name: "_setClass",
      need_key: false,
      is_svg,
    },
    "setClassName" => HelperConfig {
      name: "setClassName",
      need_key: false,
      is_svg,
    },
    "setStyle" => HelperConfig {
      name: "_setStyle",
      need_key: false,
      is_svg,
    },
    "setValue" => HelperConfig {
      name: "_setValue",
      need_key: false,
      is_svg,
    },
    "setAttr" => HelperConfig {
      name: "_setAttr",
      need_key: true,
      is_svg,
    },
    "setProp" => HelperConfig {
      name: "_setProp",
      need_key: true,
      is_svg,
    },
    "setDOMProp" => HelperConfig {
      name: "_setDOMProp",
      need_key: true,
      is_svg,
    },
    "setDynamicProps" => HelperConfig {
      name: "_setDynamicProps",
      need_key: true,
      is_svg,
    },
    _ => panic!("Unsupported helper name"),
  }
}

pub fn gen_set_prop<'a>(oper: SetPropIRNode<'a>, context: &'a CodegenContext<'a>) -> Statement<'a> {
  let ast = &context.ast;
  let SetPropIRNode {
    element,
    prop: IRProp {
      key,
      mut values,
      modifier,
      ..
    },
    tag,
    ..
  } = oper;

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.str(&format!("_n{}", element)))
      .into(),
  );
  let key_value = if let Expression::StringLiteral(key) = &key {
    &key.value
  } else {
    ""
  };
  let resolved_helper = get_runtime_helper(tag, key_value, modifier);
  if key_value == "class"
    && !resolved_helper.is_svg
    && resolved_helper.name == "_setClass"
    && let Some(class_name) = gen_set_class_name(element, &mut values, context)
  {
    return class_name;
  }
  if resolved_helper.need_key {
    arguments.push(gen_expression(key, context, None, false).into());
  }
  arguments.push(gen_prop_value(values, context).into());
  if resolved_helper.is_svg {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper(resolved_helper.name))),
      NONE,
      arguments,
      false,
    ),
  )
}

struct ClassNameEntry<'a> {
  class_name: Cow<'a, str>,
  condition: Option<Expression<'a>>,
  negate: bool,
  value: Option<bool>,
}

// Runtime uses signed bitwise shifts when iterating fragments, so 31 entries
// is the largest safe flag set (1 << 30).
const MAX_CLASS_NAME_ENTRIES: usize = 31;

fn gen_set_class_name<'a>(
  element: i32,
  values: &mut Vec<Expression<'a>>,
  context: &'a CodegenContext<'a>,
) -> Option<Statement<'a>> {
  let ast = context.ast;
  let Some((prefix, suffix, entries)) = resolve_class_name(values, context) else {
    return None;
  };

  let mut class_fragments: oxc_allocator::Vec<ArrayExpressionElement> = ast.vec();
  let entries_len = entries.len();
  for entry in entries.iter() {
    class_fragments.push(
      ast
        .expression_string_literal(
          SPAN,
          if prefix.is_empty() && entries_len == 1 {
            ast.str(entry.class_name.as_ref())
          } else {
            ast.str(format!(" {}", entry.class_name).as_ref())
          },
          None,
        )
        .into(),
    );
  }

  let fragments = if entries_len == 1 {
    class_fragments.remove(0).into_expression()
  } else {
    ast.expression_array(SPAN, class_fragments)
  };

  let flags = gen_class_flags(entries, context);

  Some(
    ast.statement_expression(
      SPAN,
      ast.expression_call(
        SPAN,
        ast.expression_identifier(SPAN, ast.str(context.options.helper("_setClassName"))),
        NONE,
        ast.vec_from_iter(
          [
            Some(
              ast
                .expression_identifier(SPAN, ast.str(&format!("_n{}", element)))
                .into(),
            ),
            Some(flags.into()),
            Some(fragments.into()),
            if !prefix.is_empty() {
              Some(
                ast
                  .expression_string_literal(SPAN, ast.str(&prefix), None)
                  .into(),
              )
            } else if !suffix.is_empty() {
              Some(ast.expression_string_literal(SPAN, "", None).into())
            } else {
              None
            },
            if !suffix.is_empty() {
              Some(
                ast
                  .expression_string_literal(SPAN, ast.str(&suffix), None)
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
  )
}

fn resolve_class_name<'a>(
  values: &mut Vec<Expression<'a>>,
  context: &'a CodegenContext<'a>,
) -> Option<(Cow<'a, str>, Cow<'a, str>, Vec<ClassNameEntry<'a>>)> {
  let mut prefix = Cow::Borrowed("");
  let mut suffix = Cow::Borrowed("");
  let mut entries: Vec<ClassNameEntry> = vec![];
  let mut saw_dynamic = false;
  let mut saw_suffix = false;

  for value in values.iter_mut() {
    if let Some(static_value) = get_text_like_value(value, true) {
      let normalized = normalize_class(static_value);
      if !normalized.is_empty() {
        if saw_suffix {
          suffix = append_class(suffix, normalized);
        } else if saw_dynamic {
          saw_suffix = true;
          suffix = append_class(suffix, normalized);
        } else {
          prefix = append_class(prefix, normalized)
        }
      }
      continue;
    };

    if saw_suffix {
      return None;
    }
    saw_dynamic = true;

    if let Expression::ObjectExpression(value) = value {
      if !resolve_object_class_name(value, &mut entries, context) {
        return None;
      }
    } else if let Expression::ConditionalExpression(value) = value {
      if !resolve_condition_class_name(value, &mut entries, context) {
        return None;
      }
    } else {
      return None;
    }
  }

  if entries.len() <= MAX_CLASS_NAME_ENTRIES {
    Some((prefix, suffix, entries))
  } else {
    None
  }
}

fn resolve_object_class_name<'a>(
  value: &mut ObjectExpression<'a>,
  entries: &mut Vec<ClassNameEntry<'a>>,
  context: &'a CodegenContext<'a>,
) -> bool {
  for prop in value.properties.iter_mut() {
    if let ObjectPropertyKind::ObjectProperty(prop) = prop {
      if prop.computed {
        return false;
      }

      let Some(raw_class_name) = (match &prop.key {
        PropertyKey::StaticIdentifier(key) => Some(key.name.as_str()),
        PropertyKey::StringLiteral(key) => Some(key.value.as_str()),
        _ => None,
      }) else {
        return false;
      };

      let class_name = normalize_class(Cow::Borrowed(raw_class_name));
      // Empty normalized keys contribute no class and no flag bit.
      if class_name.is_empty() {
        continue;
      }

      let mut value = None;
      let condition = if let Expression::BooleanLiteral(prop_value) = &prop.value {
        value = Some(prop_value.value);
        None
      } else {
        Some(prop.value.clone_in(context.ast.allocator))
      };
      entries.push(ClassNameEntry {
        class_name,
        condition,
        negate: false,
        value,
      })
    } else {
      return false;
    }
  }
  true
}

fn resolve_condition_class_name<'a>(
  value: &mut ConditionalExpression<'a>,
  entries: &mut Vec<ClassNameEntry<'a>>,
  context: &'a CodegenContext<'a>,
) -> bool {
  let consequent = get_text_like_value(&value.consequent, false).map(normalize_class);
  let alternate = get_text_like_value(&value.alternate, false).map(normalize_class);
  let consequent_is_empty = consequent
    .as_ref()
    .is_some_and(|consequent| consequent.is_empty());
  let alternate_is_empty = alternate
    .as_ref()
    .is_some_and(|alternate| alternate.is_empty());

  if let Some(consequent) = consequent
    && alternate_is_empty
  {
    entries.push(ClassNameEntry {
      class_name: consequent,
      condition: Some(value.test.take_in(context.ast.allocator)),
      negate: false,
      value: None,
    });
    true
  } else if let Some(alternate) = alternate
    && consequent_is_empty
  {
    entries.push(ClassNameEntry {
      class_name: alternate,
      condition: Some(value.test.take_in(context.ast.allocator)),
      negate: true,
      value: None,
    });
    true
  } else {
    false
  }
}

fn normalize_class<'a>(base: Cow<'a, str>) -> Cow<'a, str> {
  let normalized = base.split_whitespace().collect::<Vec<_>>().join(" ");

  match base {
    Cow::Borrowed(s) if normalized == s => Cow::Borrowed(s),
    Cow::Owned(s) if normalized == s => Cow::Owned(s),
    _ => Cow::Owned(normalized),
  }
}

fn gen_class_flags<'a>(
  entries: Vec<ClassNameEntry<'a>>,
  context: &'a CodegenContext<'a>,
) -> Expression<'a> {
  let ast = context.ast;
  let mut values = ast.vec();

  for (index, entry) in entries.into_iter().enumerate() {
    let bit = 1 << index;
    if let Some(value) = entry.value {
      values.push(if value == true {
        ast.expression_numeric_literal(SPAN, bit as f64, None, NumberBase::Hex)
      } else {
        ast.number_0()
      });
      continue;
    }

    values.push(ast.expression_parenthesized(
      SPAN,
      ast.expression_conditional(
        SPAN,
        gen_expression(entry.condition.unwrap(), context, None, false),
        if entry.negate {
          ast.number_0()
        } else {
          ast.expression_numeric_literal(SPAN, bit as f64, None, NumberBase::Hex)
        },
        if entry.negate {
          ast.expression_numeric_literal(SPAN, bit as f64, None, NumberBase::Hex)
        } else {
          ast.number_0()
        },
      ),
    ));
  }

  let values_len = values.len();
  if values.len() > 1 {
    let mut result = values.remove(0);
    for value in values {
      result = ast.expression_binary(SPAN, result, BinaryOperator::BitwiseOR, value);
    }
    result
  } else if values_len == 1 {
    values.remove(0)
  } else {
    ast.number_0()
  }
}

fn append_class<'a>(base: Cow<'a, str>, value: Cow<'a, str>) -> Cow<'a, str> {
  if !base.is_empty() {
    if !value.is_empty() {
      Cow::Owned(format!("{} {}", base, value))
    } else {
      base
    }
  } else {
    value
  }
}

fn get_runtime_helper<'a>(tag: &str, key: &str, modifier: Option<&str>) -> HelperConfig<'a> {
  let tag_name = tag.to_uppercase();
  if let Some(modifier) = modifier {
    return if modifier.eq(".") {
      if let Some(result) = get_special_helper(key, &tag_name) {
        result
      } else {
        helpers("setDOMProp", false)
      }
    } else {
      helpers("setAttr", false)
    };
  }

  // 1. SVG: always attribute
  if is_svg_tag(tag) {
    return helpers("setAttr", true);
  }

  // 2. special handling for value / style / class / textContent /  innerHTML
  if let Some(helper) = get_special_helper(key, &tag_name) {
    return helper;
  };

  // 3. Aria DOM properties shared between all Elements in
  //    https://developer.mozilla.org/en-US/docs/Web/API/Element
  if key.starts_with("aria")
    && key
      .chars()
      .nth(4)
      .map(|c| c.is_ascii_uppercase())
      .unwrap_or(false)
  {
    return helpers("setDOMProp", false);
  }

  // 4. respect shouldSetAsAttr used in vdom and setDynamicProp for consistency
  //    also fast path for presence of hyphen (covers data-* and aria-*)
  if should_set_as_attr(&tag_name, key) || key.contains("-") {
    return helpers("setAttr", false);
  }

  // 5. Fallback to setDOMProp, which has a runtime `key in el` check to
  // ensure behavior consistency with vdom
  helpers("setProp", false)
}

// The following attributes must be set as attribute
fn should_set_as_attr(tag_name: &str, key: &str) -> bool {
  // these are enumerated attrs, however their corresponding DOM properties
  // are actually booleans - this leads to setting it with a string "false"
  // value leading it to be coerced to `true`, so we need to always treat
  // them as attributes.
  // Note that `contentEditable` doesn't have this problem: its DOM
  // property is also enumerated string values.
  if key == "spellcheck" || key == "draggable" || key == "translate" || key == "autocorrect" {
    return true;
  }

  // #1787, #2840 form property on form elements is readonly and must be set as attribute.
  if key == "form" {
    return true;
  }

  // #1526 <input list> must be set as attribute
  if key == "list" && tag_name == "INPUT" {
    return true;
  }

  // #8780 the width or height of embedded tags must be set as attribute
  if (key == "width" || key == "height")
    && (tag_name == "IMG" || tag_name == "VIDEO" || tag_name == "CANVAS" || tag_name == "SOURCE")
  {
    return true;
  }

  false
}

fn can_set_value_directly(tag_name: &str) -> bool {
  tag_name != "PROGRESS" &&
    // custom elements may use _value internally
    !tag_name.contains("-")
}

fn get_special_helper<'a>(key_name: &str, tag_name: &str) -> Option<HelperConfig<'a>> {
  // special case for 'value' property
  match key_name {
    "value" if can_set_value_directly(tag_name) => Some(helpers("setValue", false)),
    "class" => Some(helpers("setClass", false)),
    "style" => Some(helpers("setStyle", false)),
    "innerHTML" => Some(helpers("setHtml", false)),
    "textContent" => Some(helpers("setText", false)),
    _ => None,
  }
}

// dynamic key props and {...obj} will reach here
pub fn gen_dynamic_props<'a>(
  oper: SetDynamicPropsIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let is_svg = is_svg_tag(oper.tag);
  let values = oper.props.into_iter().map(|props| {
    match props {
      Either3::A(props) => gen_literal_object_props(props, context).into(),
      Either3::B(prop) => gen_literal_object_props(vec![prop], context).into(),
      Either3::C(props) => gen_expression(props.value, context, None, false).into(), // {...obj}
    }
  });

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.str(&format!("_n{}", oper.element)))
      .into(),
  );
  arguments.push(ast.expression_array(SPAN, ast.vec_from_iter(values)).into());
  if is_svg {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }
  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_setDynamicProps"))),
      NONE,
      arguments,
      false,
    ),
  )
}

fn gen_literal_object_props<'a>(
  props: Vec<IRProp<'a>>,
  context: &'a CodegenContext<'a>,
) -> Expression<'a> {
  let ast = context.ast;
  ast.expression_object(
    SPAN,
    ast.vec_from_iter(props.into_iter().map(|prop| {
      ast.object_property_kind_object_property(
        SPAN,
        PropertyKind::Init,
        gen_prop_key(
          prop.key,
          prop.runtime_camelize,
          prop.modifier,
          prop.handler,
          prop
            .handler_modifiers
            .map(|i| i.options)
            .unwrap_or_default(),
          context,
        ),
        gen_prop_value(prop.values, context),
        false,
        false,
        false,
      )
    })),
  )
}

pub fn gen_prop_key<'a>(
  node: Expression<'a>,
  runtime_camelize: bool,
  modifier: Option<&str>,
  handler: bool,
  options: Vec<Cow<'a, str>>,
  context: &'a CodegenContext<'a>,
) -> PropertyKey<'a> {
  let ast = &context.ast;

  let handler_modifier_postfix = if !options.is_empty() {
    options
      .into_iter()
      .map(capitalize)
      .collect::<Vec<_>>()
      .join("")
  } else {
    String::new()
  };
  // static arg was transformed by v-bind transformer
  if let Expression::StringLiteral(node) = node {
    // only quote keys if necessary
    let key_name = if handler {
      format!(
        "on{}{}{}",
        node.value[0..1].to_uppercase(),
        &node.value[1..],
        &handler_modifier_postfix
      )
    } else {
      format!("{}{}", node.value, &handler_modifier_postfix)
    };
    let key_name = if is_simple_identifier(&key_name) {
      &key_name
    } else {
      &format!("\"{}\"", key_name)
    };
    return ast.property_key_static_identifier(node.span, ast.str(key_name));
  }

  let mut key = gen_expression(node, context, None, false);
  if runtime_camelize {
    key = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_camelize"))),
      NONE,
      ast.vec1(key.into()),
      false,
    )
  }
  if handler {
    key = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.str(context.options.helper("_toHandlerKey"))),
      NONE,
      ast.vec1(key.into()),
      false,
    )
  }

  if let Some(modifier) = modifier {
    let left = ast.expression_binary(
      SPAN,
      ast.expression_string_literal(SPAN, ast.str(&format!("\"{}\" + ", modifier)), None),
      BinaryOperator::Addition,
      key,
    );
    if !handler_modifier_postfix.is_empty() {
      ast
        .expression_binary(
          SPAN,
          left,
          BinaryOperator::Addition,
          ast.expression_string_literal(
            SPAN,
            ast.str(&format!("\"{}\" + ", handler_modifier_postfix)),
            None,
          ),
        )
        .into()
    } else {
      left.into()
    }
  } else {
    key.into()
  }
}

pub fn gen_prop_value<'a>(
  mut values: Vec<Expression<'a>>,
  context: &'a CodegenContext<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  if values.len() == 1 {
    return gen_expression(values.remove(0), context, None, false);
  }
  ast.expression_array(
    SPAN,
    ast.vec_from_iter(
      values
        .into_iter()
        .map(|value| gen_expression(value, context, None, false).into()),
    ),
  )
}
