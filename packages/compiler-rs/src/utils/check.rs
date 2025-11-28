use oxc_ast::ast::{
  ArrayExpressionElement, Expression, IdentifierReference, JSXChild, JSXElement, JSXElementName,
  ObjectPropertyKind, PropertyKey,
};
use oxc_span::GetSpan;
use oxc_traverse::{Ancestor, TraverseAncestry};
use phf::phf_set;

use crate::{ir::index::SimpleExpressionNode, utils::expression::is_globally_allowed};

pub fn is_member_expression(exp: &SimpleExpressionNode) -> bool {
  let Some(ast) = &exp.ast else { return false };
  let ret = ast.without_parentheses().get_inner_expression();
  match ret {
    Expression::StaticMemberExpression(_) => true,
    Expression::Identifier(_) => !ret.is_undefined(),
    _ => false,
  }
}

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

static RESERVED_PROP: [&str; 4] = ["key", "ref", "ref_for", "ref_key"];
pub fn is_reserved_prop(prop_name: &str) -> bool {
  RESERVED_PROP.contains(&prop_name)
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
pub fn is_build_in_directive(prop_name: &str) -> bool {
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

// Checks if the input `node` is a reference to a bound variable.
//
// Copied from https://github.com/babel/babel/blob/main/packages/babel-types/src/validators/isReferenced.ts
//
// @param node - The node to check.
// @param parent - The parent node of the input `node`.
// @param grandparent - The grandparent node of the input `node`.
// @returns True if the input `node` is a reference to a bound variable, false otherwise.
pub fn is_referenced(
  node: &IdentifierReference,
  parent: &Ancestor,
  grandparent: &Ancestor,
) -> bool {
  match parent {
    // yes: PARENT[NODE]
    // yes: NODE.child
    // no: parent.NODE
    Ancestor::StaticMemberExpressionObject(parent) => !parent.property().span.eq(&node.span),
    Ancestor::StaticMemberExpressionProperty(parent) => parent.object().span().eq(&node.span),

    Ancestor::ComputedMemberExpressionObject(parent) => parent.expression().span().eq(&node.span),
    Ancestor::ComputedMemberExpressionExpression(parent) => parent.object().span().eq(&node.span),

    Ancestor::JSXMemberExpressionProperty(parent) => parent.object().span().eq(&node.span),
    Ancestor::JSXMemberExpressionObject(_) => false,

    // no: let NODE = init;
    // yes: let id = NODE;
    Ancestor::VariableDeclaratorId(parent) => parent
      .init()
      .as_ref()
      .map(|init| init.span().eq(&node.span))
      .unwrap_or(false),
    Ancestor::VariableDeclaratorInit(_) => false,

    // yes: () => NODE
    // no: (NODE) => {}
    Ancestor::ArrowFunctionExpressionParams(parent) => parent.body().span.eq(&node.span),
    Ancestor::ArrowFunctionExpressionBody(_)
    | Ancestor::ArrowFunctionExpressionReturnType(_)
    | Ancestor::ArrowFunctionExpressionTypeParameters(_) => false,

    // no: class { #NODE; }
    // no: class { get #NODE() {} }
    // no: class { #NODE() {} }
    // no: class { fn() { return this.#NODE; } }
    Ancestor::PrivateFieldExpressionField(_)
    | Ancestor::PrivateFieldExpressionObject(_)
    | Ancestor::PrivateInExpressionLeft(_)
    | Ancestor::PrivateInExpressionRight(_) => false,

    // no: class { NODE() {} }
    // yes: class { [NODE]() {} }
    // no: class { foo(NODE) {} }
    Ancestor::MethodDefinitionValue(parent) => {
      parent.key().span().eq(&node.span) && *parent.computed()
    }
    Ancestor::MethodDefinitionKey(_) | Ancestor::MethodDefinitionDecorators(_) => false,

    // yes: { [NODE]: "" }
    // no: { NODE: "" }
    // depends: { NODE }
    // depends: { key: NODE }
    Ancestor::ObjectPropertyValue(parent) => {
      if let PropertyKey::PrivateIdentifier(key) = &parent.key() {
        key.span.eq(&node.span)
      } else if *parent.computed() && parent.key().span().eq(&node.span()) {
        true
      } else if !matches!(grandparent, Ancestor::None) {
        !grandparent.is_object_pattern()
      } else {
        true
      }
    }
    Ancestor::ObjectPropertyKey(_) => false,
    // no: class { NODE = value; }
    // yes: class { [NODE] = value; }
    // yes: class { key = NODE; }
    Ancestor::AccessorPropertyValue(parent) => {
      if let PropertyKey::PrivateIdentifier(key) = &parent.key() {
        key.span.eq(&node.span)
      } else if parent.key().span().eq(&node.span()) {
        *parent.computed()
      } else {
        true
      }
    }
    Ancestor::AccessorPropertyKey(_)
    | Ancestor::AccessorPropertyTypeAnnotation(_)
    | Ancestor::AccessorPropertyDecorators(_) => false,

    // no: class NODE {}
    // yes: class Foo extends NODE {}
    Ancestor::ClassDecorators(parent) => {
      if let Some(super_class) = &parent.super_class() {
        super_class.span().eq(parent.span())
      } else {
        false
      }
    }
    Ancestor::ClassId(parent) => {
      if let Some(super_class) = &parent.super_class() {
        super_class.span().eq(parent.span())
      } else {
        false
      }
    }
    Ancestor::ClassTypeParameters(parent) => {
      if let Some(super_class) = &parent.super_class() {
        super_class.span().eq(parent.span())
      } else {
        false
      }
    }
    Ancestor::ClassSuperTypeArguments(parent) => {
      if let Some(super_class) = &parent.super_class() {
        super_class.span().eq(parent.span())
      } else {
        false
      }
    }
    Ancestor::ClassImplements(parent) => {
      if let Some(super_class) = &parent.super_class() {
        super_class.span().eq(parent.span())
      } else {
        false
      }
    }
    Ancestor::ClassBody(parent) => {
      if let Some(super_class) = &parent.super_class() {
        super_class.span().eq(parent.span())
      } else {
        false
      }
    }
    Ancestor::ClassSuperClass(_) => false,

    // yes: left = NODE;
    // no: NODE = right;
    Ancestor::AssignmentExpressionLeft(parent) => parent.right().span().eq(&node.span),
    Ancestor::AssignmentExpressionRight(_) => false,

    // no: [NODE = foo] = [];
    // yes: [foo = NODE] = [];
    Ancestor::AssignmentPatternLeft(parent) => parent.right().span().eq(&node.span),
    Ancestor::AssignmentPatternRight(_) => false,

    // no: NODE: for (;;) {}
    Ancestor::LabeledStatementLabel(_) | Ancestor::LabeledStatementBody(_) => false,

    // no: try {} catch (NODE) {}
    Ancestor::CatchClauseParam(_) | Ancestor::CatchClauseBody(_) => false,

    // no: function foo(...NODE) {}
    Ancestor::BindingRestElementArgument(_) => false,

    // no: break;
    // no: continue;
    Ancestor::BreakStatementLabel(_) | Ancestor::ContinueStatementLabel(_) => false,

    // no: function NODE() {}
    // no: function foo(NODE) {}
    Ancestor::FunctionId(_)
    | Ancestor::FunctionTypeParameters(_)
    | Ancestor::FunctionThisParam(_)
    | Ancestor::FunctionParams(_)
    | Ancestor::FunctionReturnType(_)
    | Ancestor::FunctionBody(_) => false,

    // no: export NODE from "foo";
    // no: export * as NODE from "foo";
    //
    // don't support in oxc
    // case 'ExportDefaultSpecifier':
    Ancestor::ExportAllDeclarationExported(_)
    | Ancestor::ExportAllDeclarationSource(_)
    | Ancestor::ExportAllDeclarationWithClause(_) => false,

    // no: export { foo as NODE };
    // yes: export { NODE as foo };
    // no: export { NODE as foo } from "foo";
    Ancestor::ExportSpecifierLocal(_) => !matches!(
      grandparent,
      Ancestor::ExportNamedDeclarationDeclaration(_)
        | Ancestor::ExportNamedDeclarationSpecifiers(_)
        | Ancestor::ExportNamedDeclarationWithClause(_)
    ),
    Ancestor::ExportSpecifierExported(parent) => {
      if matches!(
        grandparent,
        Ancestor::ExportNamedDeclarationDeclaration(_)
          | Ancestor::ExportNamedDeclarationSpecifiers(_)
          | Ancestor::ExportNamedDeclarationWithClause(_)
      ) {
        return false;
      }
      parent.local().span().eq(&node.span)
    }

    // no: import NODE from "foo";
    // no: import * as NODE from "foo";
    // no: import { NODE as foo } from "foo";
    // no: import { foo as NODE } from "foo";
    // no: import NODE from "bar";
    Ancestor::ImportDefaultSpecifierLocal(_)
    | Ancestor::ImportNamespaceSpecifierLocal(_)
    | Ancestor::ImportSpecifierImported(_)
    | Ancestor::ImportSpecifierLocal(_) => false,

    // no: import "foo" assert { NODE: "json" }
    Ancestor::ImportAttributeKey(_) | Ancestor::ImportAttributeValue(_) => false,

    // no: <div NODE="foo" />
    // no: <div foo:NODE="foo" />
    Ancestor::JSXAttributeName(_)
    | Ancestor::JSXAttributeValue(_)
    | Ancestor::JSXNamespacedNameName(_)
    | Ancestor::JSXNamespacedNameNamespace(_) => false,

    // no: [NODE] = [];
    // no: ({ NODE }) = [];
    Ancestor::ObjectPatternProperties(_)
    | Ancestor::ObjectPatternRest(_)
    | Ancestor::ArrayPatternElements(_)
    | Ancestor::ArrayPatternRest(_) => false,

    // no: new.NODE
    // no: NODE.target
    Ancestor::MetaPropertyMeta(_) | Ancestor::MetaPropertyProperty(_) => false,

    // yes: enum X { Foo = NODE }
    // no: enum X { NODE }
    Ancestor::TSEnumMemberInitializer(parent) => !parent.id().span().eq(&node.span),
    Ancestor::TSEnumMemberId(_) => false,

    // yes: { [NODE]: value }
    // no: { NODE: value }
    Ancestor::TSPropertySignatureTypeAnnotation(parent) => {
      if parent.key().span().eq(&node.span) {
        *parent.computed()
      } else {
        true
      }
    }
    Ancestor::TSPropertySignatureKey(_) => false,
    _ => true,
  }
}

pub fn is_referenced_identifier(id: &IdentifierReference, ancestry: &TraverseAncestry) -> bool {
  if let Ancestor::None = ancestry.parent() {
    return true;
  };
  let parent = ancestry.parent();

  // is a special keyword but parsed as identifier
  if id.name.eq("arguments") {
    return false;
  }

  if is_referenced(id, &parent, &ancestry.ancestor(1)) {
    return true;
  }

  // babel's isReferenced check returns false for ids being assigned to, so we
  // need to cover those cases here
  if parent.is_assignment_expression() || parent.is_assignment_pattern() {
    true
  } else if let Ancestor::ObjectPropertyValue(_parent) = parent {
    _parent.key().span().eq(&id.span) && is_in_desctructure_assignment(&parent, ancestry)
  } else {
    false
  }
}

fn is_in_desctructure_assignment(parent: &Ancestor, ancestry: &TraverseAncestry) -> bool {
  if parent.is_object_property() || parent.is_array_pattern() {
    for p in ancestry.ancestors() {
      if p.is_assignment_expression() {
        return true;
      } else if !(p.is_binding_property() && p.is_object_pattern() || p.is_array_pattern()) {
        break;
      }
    }
  }
  false
}
