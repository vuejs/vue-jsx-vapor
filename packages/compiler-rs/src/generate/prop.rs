use napi::bindgen_prelude::Either3;
use oxc_ast::NONE;
use oxc_ast::ast::BinaryOperator;
use oxc_ast::ast::Expression;
use oxc_ast::ast::PropertyKey;
use oxc_ast::ast::PropertyKind;
use oxc_ast::ast::Statement;
use oxc_span::SPAN;

use crate::generate::CodegenContext;
use crate::generate::expression::gen_expression;
use crate::ir::component::IRProp;
use crate::ir::index::SetDynamicPropsIRNode;
use crate::ir::index::SetPropIRNode;
use crate::ir::index::SimpleExpressionNode;
use crate::utils::check::is_simple_identifier;
use crate::utils::check::is_svg_tag;

pub struct HelperConfig {
  name: String,
  need_key: bool,
}

fn helpers(name: &str) -> HelperConfig {
  match name {
    "setText" => HelperConfig {
      name: "setText".to_string(),
      need_key: false,
    },
    "setHtml" => HelperConfig {
      name: "setHtml".to_string(),
      need_key: false,
    },
    "setClass" => HelperConfig {
      name: "setClass".to_string(),
      need_key: false,
    },
    "setStyle" => HelperConfig {
      name: "setStyle".to_string(),
      need_key: false,
    },
    "setValue" => HelperConfig {
      name: "setValue".to_string(),
      need_key: false,
    },
    "setAttr" => HelperConfig {
      name: "setAttr".to_string(),
      need_key: true,
    },
    "setProp" => HelperConfig {
      name: "setProp".to_string(),
      need_key: true,
    },
    "setDOMProp" => HelperConfig {
      name: "setDOMProp".to_string(),
      need_key: true,
    },
    "setDynamicProps" => HelperConfig {
      name: "setDynamicProps".to_string(),
      need_key: true,
    },
    _ => panic!("Unsupported helper name"),
  }
}

pub fn gen_set_prop<'a>(oper: SetPropIRNode<'a>, context: &'a CodegenContext<'a>) -> Statement<'a> {
  let ast = &context.ast;
  let SetPropIRNode {
    prop: IRProp {
      key,
      values,
      modifier,
      ..
    },
    tag,
    ..
  } = oper;

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.atom(&format!("n{}", oper.element)))
      .into(),
  );
  let resolved_helper = get_runtime_helper(&tag, &key.content, modifier);
  if resolved_helper.need_key {
    arguments.push(gen_expression(key, context, None, None).into());
  }
  arguments.push(gen_prop_value(values, context).into());

  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(
        SPAN,
        ast.atom(&context.helper(resolved_helper.name.as_str())),
      ),
      NONE,
      arguments,
      false,
    ),
  )
}

fn get_runtime_helper(tag: &str, key: &str, modifier: Option<String>) -> HelperConfig {
  let tag_name = tag.to_uppercase();
  if let Some(modifier) = modifier {
    return if modifier.eq(".") {
      if let Some(result) = get_special_helper(key, &tag_name) {
        result
      } else {
        helpers("setDOMProp")
      }
    } else {
      helpers("setAttr")
    };
  }

  // 1. special handling for value / style / class / textContent /  innerHTML
  if let Some(helper) = get_special_helper(key, &tag_name) {
    return helper;
  };

  // 2. Aria DOM properties shared between all Elements in
  //    https://developer.mozilla.org/en-US/docs/Web/API/Element
  if key.starts_with("aria")
    && key
      .chars()
      .nth(4)
      .map(|c| c.is_ascii_uppercase())
      .unwrap_or(false)
  {
    return helpers("setDOMProp");
  }

  // 3. SVG: always attribute
  if is_svg_tag(tag) {
    // TODO pass svg flag
    return helpers("setAttr");
  }

  // 4. respect shouldSetAsAttr used in vdom and setDynamicProp for consistency
  //    also fast path for presence of hyphen (covers data-* and aria-*)
  if should_set_as_attr(&tag_name, key) || key.contains("-") {
    return helpers("setAttr");
  }

  // 5. Fallback to setDOMProp, which has a runtime `key in el` check to
  // ensure behavior consistency with vdom
  helpers("setProp")
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

fn get_special_helper(key_name: &str, tag_name: &str) -> Option<HelperConfig> {
  // special case for 'value' property
  match key_name {
    "value" if can_set_value_directly(tag_name) => Some(helpers("setValue")),
    "class" => Some(helpers("setClass")),
    "style" => Some(helpers("setStyle")),
    "innerHTML" => Some(helpers("setHtml")),
    "textContent" => Some(helpers("setText")),
    _ => None,
  }
}

// dynamic key props and {...obj} will reach here
pub fn gen_dynamic_props<'a>(
  oper: SetDynamicPropsIRNode<'a>,
  context: &'a CodegenContext<'a>,
) -> Statement<'a> {
  let ast = &context.ast;
  let values = oper.props.into_iter().map(|props| {
    match props {
      Either3::A(props) => gen_literal_object_props(props, context).into(),
      Either3::B(prop) => gen_literal_object_props(vec![prop], context).into(),
      Either3::C(props) => gen_expression(props.value, context, None, None).into(), // {...obj}
    }
  });

  let mut arguments = ast.vec();
  arguments.push(
    ast
      .expression_identifier(SPAN, ast.atom(&format!("n{}", oper.element)))
      .into(),
  );
  arguments.push(ast.expression_array(SPAN, ast.vec_from_iter(values)).into());
  if oper.root {
    arguments.push(ast.expression_boolean_literal(SPAN, true).into());
  }
  ast.statement_expression(
    SPAN,
    ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("setDynamicProps"))),
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
          prop.handler.unwrap_or(false),
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
  node: SimpleExpressionNode<'a>,
  runtime_camelize: Option<bool>,
  modifier: Option<String>,
  handler: bool,
  options: Vec<String>,
  context: &'a CodegenContext<'a>,
) -> PropertyKey<'a> {
  let ast = &context.ast;

  let handler_modifier_postfix = if !options.is_empty() {
    options
      .into_iter()
      .map(|option| option[..1].to_string().to_uppercase() + &option[1..])
      .collect::<Vec<_>>()
      .join("")
  } else {
    String::new()
  };
  // static arg was transformed by v-bind transformer
  if node.is_static {
    // only quote keys if necessary
    let key_name = (if handler {
      format!(
        "on{}",
        node.content[0..1].to_string().to_uppercase() + &node.content[1..]
      )
    } else {
      node.content
    }) + &handler_modifier_postfix;
    let key_name = if is_simple_identifier(&key_name) {
      &key_name
    } else {
      &format!("\"{}\"", key_name)
    };
    return ast.property_key_static_identifier(node.loc, ast.atom(key_name));
  }

  let mut key = gen_expression(node, context, None, None);
  if runtime_camelize.unwrap_or_default() {
    key = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("camelize"))),
      NONE,
      ast.vec1(key.into()),
      false,
    )
  }
  if handler {
    key = ast.expression_call(
      SPAN,
      ast.expression_identifier(SPAN, ast.atom(&context.helper("toHandlerKey"))),
      NONE,
      ast.vec1(key.into()),
      false,
    )
  }

  if let Some(modifier) = modifier {
    let left = ast.expression_binary(
      SPAN,
      ast.expression_string_literal(SPAN, ast.atom(&format!("\"{}\" + ", modifier)), None),
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
            ast.atom(&format!("\"{}\" + ", handler_modifier_postfix)),
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
  mut values: Vec<SimpleExpressionNode<'a>>,
  context: &'a CodegenContext<'a>,
) -> Expression<'a> {
  let ast = &context.ast;
  if values.len() == 1 {
    return gen_expression(values.remove(0), context, None, None);
  }
  ast.expression_array(
    SPAN,
    ast.vec_from_iter(
      values
        .into_iter()
        .map(|value| gen_expression(value, context, None, None).into()),
    ),
  )
}
