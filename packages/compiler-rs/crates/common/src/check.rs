use oxc_ast::{
  AstKind,
  ast::{
    ArrayExpressionElement, Expression, IdentifierReference, JSXChild, JSXElement, JSXElementName,
    ObjectPropertyKind,
  },
};
use oxc_span::GetSpan;
use phf::phf_set;

use crate::expression::is_globally_allowed;

pub fn is_template<'a>(node: &'a JSXElement<'a>) -> bool {
  if let JSXElementName::Identifier(name) = &node.opening_element.name {
    name.name.eq("template")
  } else {
    false
  }
}

pub fn is_constant_node(node: &Option<&Expression>) -> bool {
  let Some(node) = node else {
    return false;
  };
  match node.without_parentheses().get_inner_expression() {
    // void 0, !true
    Expression::UnaryExpression(node) => is_constant_node(&Some(&node.argument)),
    // 1 > 2
    Expression::LogicalExpression(node) => {
      is_constant_node(&Some(&node.left)) && is_constant_node(&Some(&node.right))
    }
    // 1 + 2
    Expression::BinaryExpression(node) => {
      is_constant_node(&Some(&node.left)) && is_constant_node(&Some(&node.right))
    }
    // 1 ? 2 : 3
    Expression::ConditionalExpression(node) => {
      is_constant_node(&Some(&node.test))
        && is_constant_node(&Some(&node.consequent))
        && is_constant_node(&Some(&node.alternate))
    }
    // (1, 2)
    Expression::SequenceExpression(node) => node
      .expressions
      .iter()
      .all(|exp| is_constant_node(&Some(exp))),
    // `foo${1}`
    Expression::TemplateLiteral(node) => node
      .expressions
      .iter()
      .all(|exp| is_constant_node(&Some(exp))),
    Expression::ParenthesizedExpression(node) => is_constant_node(&Some(&node.expression)),
    Expression::NullLiteral(_)
    | Expression::BigIntLiteral(_)
    | Expression::RegExpLiteral(_)
    | Expression::StringLiteral(_)
    | Expression::BooleanLiteral(_)
    | Expression::NumericLiteral(_) => true,
    Expression::Identifier(node) => {
      let name = node.name.as_str();
      name == "undefined" || is_globally_allowed(name)
    }
    Expression::ObjectExpression(node) => {
      node.properties.iter().all(|prop| match prop {
        // { bar() {} } object methods are not considered static nodes
        ObjectPropertyKind::ObjectProperty(prop) => {
          if prop.method {
            return false;
          }
          // { foo: 1 }
          (!prop.computed || is_constant_node(&Some(prop.key.to_expression())))
            && is_constant_node(&Some(&prop.value))
        }
        ObjectPropertyKind::SpreadProperty(prop) => is_constant_node(&Some(&prop.argument)),
      })
    }
    Expression::ArrayExpression(node) => {
      node.elements.iter().all(|element| {
        // [1, , 3]
        if let ArrayExpressionElement::Elision(_) = element {
          return true;
        }
        // [1, ...[2, 3]]
        if let ArrayExpressionElement::SpreadElement(element) = element {
          return is_constant_node(&Some(&element.argument));
        }
        // [1, 2]
        is_constant_node(&Some(element.to_expression()))
      })
    }
    _ => false,
  }
}

// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
static HTML_TAGS: phf::Set<&'static str> = phf_set! {
    "html",
    "body",
    "base",
    "head",
    "link",
    "meta",
    "style",
    "title",
    "address",
    "article",
    "aside",
    "footer",
    "header",
    "hgroup",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "nav",
    "section",
    "div",
    "dd",
    "dl",
    "dt",
    "figcaption",
    "figure",
    "picture",
    "hr",
    "img",
    "li",
    "main",
    "math",
    "ol",
    "p",
    "pre",
    "ul",
    "a",
    "b",
    "abbr",
    "bdi",
    "bdo",
    "br",
    "cite",
    "code",
    "data",
    "dfn",
    "em",
    "i",
    "kbd",
    "mark",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "small",
    "span",
    "strong",
    "sub",
    "sup",
    "time",
    "u",
    "var",
    "wbr",
    "area",
    "audio",
    "map",
    "track",
    "video",
    "embed",
    "object",
    "param",
    "source",
    "canvas",
    "script",
    "noscript",
    "del",
    "ins",
    "caption",
    "col",
    "colgroup",
    "table",
    "thead",
    "tbody",
    "td",
    "th",
    "tr",
    "button",
    "datalist",
    "fieldset",
    "form",
    "input",
    "label",
    "legend",
    "meter",
    "optgroup",
    "option",
    "output",
    "progress",
    "select",
    "textarea",
    "details",
    "dialog",
    "menu",
    "summary",
    "template",
    "blockquote",
    "iframe",
    "tfoot",
};
pub fn is_html_tag(tag_name: &str) -> bool {
  HTML_TAGS.contains(tag_name)
}

// https://developer.mozilla.org/en-US/docs/Web/SVG/Element
static SVG_TAGS: phf::Set<&'static str> = phf_set! {
    "svg",
    "animate",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "defs",
    "desc",
    "discard",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "foreignObject",
    "g",
    "hatch",
    "hatchpath",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "mesh",
    "meshgradient",
    "meshpatch",
    "meshrow",
    "metadata",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "set",
    "solidcolor",
    "stop",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tspan",
    "unknown",
    "use",
    "view",
};
pub fn is_svg_tag(tag_name: &str) -> bool {
  SVG_TAGS.contains(tag_name)
}

pub fn is_jsx_component<'a>(node: &'a JSXElement<'a>) -> bool {
  match &node.opening_element.name {
    JSXElementName::Identifier(name) => !is_html_tag(&name.name) && !is_svg_tag(&name.name),
    _ => true,
  }
}

pub fn is_fragment_node(node: &JSXChild) -> bool {
  match node {
    JSXChild::Fragment(_) => true,
    JSXChild::Element(node) => is_template(node),
    _ => false,
  }
}

static VOID_TAGS: phf::Set<&'static str> = phf_set! {
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
};
pub fn is_void_tag(tag_name: &str) -> bool {
  VOID_TAGS.contains(tag_name)
}

static BUILD_IN_DIRECTIVE: phf::Set<&'static str> = phf_set! {
  "bind", "cloak", "else-if", "else", "for", "html", "if", "model", "on", "once", "pre", "show",
  "slot", "slots", "text", "memo",
};
pub fn is_built_in_directive(prop_name: &str) -> bool {
  BUILD_IN_DIRECTIVE.contains(prop_name)
}

pub fn is_simple_identifier(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }
  let first = s.chars().next().unwrap();
  if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
    return false;
  }
  for c in s.chars().skip(1) {
    if !(c.is_ascii_alphanumeric()
      || c == '_'
      || c == '$'
      || (c as u32 >= 0x00A0 && c as u32 <= 0xFFFF))
    {
      return false;
    }
  }
  true
}

// events
static DELEGATED_EVENTS: phf::Set<&'static str> = phf_set! {
    "beforeinput",
    "click",
    "dblclick",
    "contextmenu",
    "focusin",
    "focusout",
    "input",
    "keydown",
    "keyup",
    "mousedown",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "pointerdown",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "touchend",
    "touchmove",
    "touchstart"
};

pub fn is_delegated_events(s: &str) -> bool {
  DELEGATED_EVENTS.contains(s)
}

pub fn is_event_option_modifier(modifier: &str) -> bool {
  matches!(modifier, "passive" | "once" | "capture")
}

pub fn is_non_key_modifier(modifier: &str) -> bool {
  matches!(
    modifier,
    // event propagation management
    "stop" | "prevent" | "self" |
    // system modifiers + exact
    "ctrl" | "shift" | "alt" | "meta" | "exact" |
    // mouse
    "middle"
  )
}

// left & right could be mouse or key modifiers based on event type
pub fn maybe_key_modifier(modifier: &str) -> bool {
  matches!(modifier, "left" | "right")
}

pub fn is_keyboard_event(key: &str) -> bool {
  matches!(key, "keyup" | "keydown" | "keypress")
}

static RESERVED_PROP: phf::Set<&str> = phf_set!(
  "",
  "key",
  "ref",
  "ref_for",
  "ref_key",
  "onVnodeBeforeMount",
  "onVnodeMounted",
  "onVnodeBeforeUpdate",
  "onVnodeUpdated",
  "onVnodeBeforeUnmount",
  "onVnodeUnmounted",
);
pub fn is_reserved_prop(name: &str) -> bool {
  RESERVED_PROP.contains(name)
}

pub fn is_event(s: &str) -> bool {
  s.starts_with("on")
    && s
      .chars()
      .nth(2)
      .map(|c| c.is_ascii_uppercase())
      .unwrap_or(false)
}

pub fn is_directive(s: &str) -> bool {
  s.starts_with("v-")
    && s
      .chars()
      .nth(2)
      .map(|c| c.is_ascii_lowercase())
      .unwrap_or(false)
}

// Checks if the input `node` is a reference to a bound variable.
//
// Copied from https://github.com/babel/babel/blob/main/packages/babel-types/src/validators/isReferenced.ts
//
// @param node - The node to check.
// @param parent - The parent node of the input `node`.
// @param grandparent - The grandparent node of the input `node`.
// @returns True if the input `node` is a reference to a bound variable, false otherwise.
fn is_referenced(
  node: &IdentifierReference,
  parent: &AstKind,
  grandparent: Option<&AstKind>,
) -> bool {
  match parent {
    // yes: PARENT[NODE]
    // yes: NODE.child
    // no: parent.NODE
    AstKind::StaticMemberExpression(parent) => parent.object.span() == node.span,

    AstKind::ComputedMemberExpression(parent) => {
      if parent.expression.span().eq(&node.span) {
        true
      } else {
        parent.object.span() == node.span
      }
    }

    AstKind::JSXMemberExpression(parent) => parent.object.span() == node.span,

    // no: let NODE = init;
    // yes: let id = NODE;
    AstKind::VariableDeclarator(parent) => parent
      .init
      .as_ref()
      .map(|init| init.span().eq(&node.span))
      .unwrap_or(false),

    // yes: () => NODE
    // no: (NODE) => {}
    AstKind::ArrowFunctionExpression(parent) => parent.body.span.eq(&node.span),

    // no: class { #NODE; }
    // no: class { get #NODE() {} }
    // no: class { #NODE() {} }
    // no: class { fn() { return this.#NODE; } }
    AstKind::PrivateIdentifier(_)
    | AstKind::PrivateInExpression(_)
    | AstKind::PrivateFieldExpression(_) => false,

    // no: class { NODE() {} }
    // yes: class { [NODE]() {} }
    // no: class { foo(NODE) {} }
    AstKind::MethodDefinition(parent) => parent.key.span().eq(&node.span) && parent.computed,

    // yes: { [NODE]: "" }
    // no: { NODE: "" }
    // depends: { NODE }
    // depends: { key: NODE }
    AstKind::ObjectProperty(parent) => {
      if parent.computed && parent.key.span().eq(&node.span) {
        true
      } else if let Some(grandparent) = grandparent {
        !matches!(grandparent, AstKind::ObjectPattern(_))
      } else {
        true
      }
    }
    // no: class { NODE = value; }
    // yes: class { [NODE] = value; }
    // yes: class { key = NODE; }
    AstKind::PropertyDefinition(parent) => {
      if parent.key.span().eq(&node.span) {
        parent.computed
      } else {
        true
      }
    }
    AstKind::AccessorProperty(parent) => parent.key.span() != node.span,

    // no: class NODE {}
    // yes: class Foo extends NODE {}
    AstKind::Class(parent) => {
      if let Some(super_class) = &parent.super_class {
        super_class.span().eq(&node.span)
      } else {
        false
      }
    }

    // yes: left = NODE;
    // no: NODE = right;
    AstKind::AssignmentExpression(parent) => parent.right.span().eq(&node.span),

    // no: [NODE = foo] = [];
    // yes: [foo = NODE] = [];
    AstKind::AssignmentPattern(parent) => parent.right.span().eq(&node.span),

    // no: NODE: for (;;) {}
    AstKind::LabeledStatement(_) => false,

    // no: try {} catch (NODE) {}
    AstKind::CatchClause(_) => false,

    // no: function foo(...NODE) {}
    AstKind::BindingRestElement(_) => false,

    // no: break;
    // no: continue;
    AstKind::BreakStatement(_) | AstKind::ContinueStatement(_) => false,

    // no: function NODE() {}
    // no: function foo(NODE) {}
    AstKind::Function(_) | AstKind::FunctionBody(_) => false,

    // no: export NODE from "foo";
    // no: export * as NODE from "foo";
    //
    // don't support in oxc
    // case 'ExportDefaultSpecifier':
    AstKind::ExportAllDeclaration(_) | AstKind::ExportDefaultDeclaration(_) => false,

    // no: export { foo as NODE };
    // yes: export { NODE as foo };
    // no: export { NODE as foo } from "foo";
    AstKind::ExportSpecifier(parent) => {
      if matches!(grandparent, Some(AstKind::ExportNamedDeclaration(_))) {
        return false;
      }
      parent.local.span().eq(&node.span)
    }

    // no: import NODE from "foo";
    // no: import * as NODE from "foo";
    // no: import { NODE as foo } from "foo";
    // no: import { foo as NODE } from "foo";
    // no: import NODE from "bar";
    AstKind::ImportDefaultSpecifier(_)
    | AstKind::ImportNamespaceSpecifier(_)
    | AstKind::ImportSpecifier(_) => false,

    // no: import "foo" assert { NODE: "json" }
    AstKind::ImportAttribute(_) => false,

    // no: <div NODE="foo" />
    // no: <div foo:NODE="foo" />
    AstKind::JSXAttribute(_) | AstKind::JSXNamespacedName(_) => false,

    // no: [NODE] = [];
    // no: ({ NODE }) = [];
    AstKind::ObjectPattern(_) | AstKind::ArrayPattern(_) => false,

    // no: new.NODE
    // no: NODE.target
    AstKind::MetaProperty(_) => false,

    // yes: enum X { Foo = NODE }
    // no: enum X { NODE }
    AstKind::TSEnumMember(parent) => !parent.id.span().eq(&node.span),

    // yes: { [NODE]: value }
    // no: { NODE: value }
    AstKind::TSPropertySignature(parent) => {
      if parent.key.span().eq(&node.span) {
        parent.computed
      } else {
        true
      }
    }
    _ => true,
  }
}

pub fn is_referenced_identifier<'a>(id: &IdentifierReference, parents: &Vec<AstKind<'a>>) -> bool {
  let Some(parent) = parents.last() else {
    return true;
  };

  // is a special keyword but parsed as identifier
  if id.name.eq("arguments") {
    return false;
  }

  if is_referenced(
    id,
    parent,
    if parents.len() > 1 {
      parents.get(parents.len() - 2)
    } else {
      None
    },
  ) {
    return true;
  }

  // babel's isReferenced check returns false for ids being assigned to, so we
  // need to cover those cases here
  if matches!(
    parent,
    AstKind::AssignmentExpression(_) | AstKind::AssignmentPattern(_)
  ) {
    true
  } else if let AstKind::ObjectProperty(_parent) = parent {
    _parent.key.span().eq(&id.span) && is_in_desctructure_assignment(&parent, parents)
  } else {
    false
  }
}

fn is_in_desctructure_assignment(parent: &AstKind, parents: &Vec<AstKind>) -> bool {
  if matches!(
    parent,
    AstKind::ObjectProperty(_) | AstKind::ArrayPattern(_)
  ) {
    for p in parents.iter().rev() {
      if matches!(p, AstKind::AssignmentExpression(_)) {
        return true;
      } else if !(matches!(
        p,
        AstKind::ObjectProperty(_) | AstKind::ObjectPattern(_) | AstKind::ArrayPattern(_)
      )) {
        break;
      }
    }
  }
  false
}
