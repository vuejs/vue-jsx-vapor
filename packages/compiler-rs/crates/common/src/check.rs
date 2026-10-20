use oxc_ast::{
  AstKind,
  ast::{
    ArrayExpressionElement, Expression, IdentifierReference, JSXAttributeItem, JSXAttributeName,
    JSXAttributeValue, JSXElement, JSXElementName, ObjectPropertyKind,
  },
};
use oxc_span::GetSpan;
use phf::phf_set;

use crate::expression::is_globally_allowed;

pub fn is_constant_node(node: &Expression) -> bool {
  match node.without_parentheses().get_inner_expression() {
    // void 0, !true
    Expression::UnaryExpression(node) => is_constant_node(&node.argument),
    // 1 > 2
    Expression::LogicalExpression(node) => {
      is_constant_node(&node.left) && is_constant_node(&node.right)
    }
    // 1 + 2
    Expression::BinaryExpression(node) => {
      is_constant_node(&node.left) && is_constant_node(&node.right)
    }
    // 1 ? 2 : 3
    Expression::ConditionalExpression(node) => {
      is_constant_node(&node.test)
        && is_constant_node(&node.consequent)
        && is_constant_node(&node.alternate)
    }
    // (1, 2)
    Expression::SequenceExpression(node) => node.expressions.iter().all(is_constant_node),
    // `foo${1}`
    Expression::TemplateLiteral(node) => node.expressions.iter().all(is_constant_node),
    Expression::ParenthesizedExpression(node) => is_constant_node(&node.expression),
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
          (!prop.computed || is_constant_node(prop.key.to_expression()))
            && is_constant_node(&prop.value)
        }
        ObjectPropertyKind::SpreadProperty(prop) => is_constant_node(&prop.argument),
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
          return is_constant_node(&element.argument);
        }
        // [1, 2]
        is_constant_node(element.to_expression())
      })
    }
    _ => false,
  }
}

// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
static HTML_TAGS: phf::Set<&'static str> = phf_set! {
  "html", "body", "base", "head", "link", "meta", "style", "title", "address", "article", "aside",
  "footer", "header", "hgroup", "h1", "h2", "h3", "h4", "h5", "h6", "nav", "section", "div",
  "dd", "dl", "dt", "figcaption", "figure", "picture", "hr", "img", "li", "main", "math", "ol",
  "p", "pre", "ul", "a", "b", "abbr", "bdi", "bdo", "br", "cite", "code", "data", "dfn", "em",
  "i", "kbd", "mark", "q", "rp", "rt", "ruby", "s", "samp", "small", "span", "strong", "sub", "sup",
  "time", "u", "var", "wbr", "area", "audio", "map", "track", "video", "embed", "object", "param",
  "source", "canvas", "script", "noscript", "del", "ins", "caption", "col", "colgroup",
  "table", "thead", "tbody", "td", "th", "tr", "button", "datalist", "fieldset", "form", "input",
  "label", "legend", "meter", "optgroup", "option", "output", "progress", "select", "textarea", "details",
  "dialog", "menu", "summary", "template", "blockquote", "iframe", "tfoot",
};
pub fn is_html_tag(tag_name: &str) -> bool {
  HTML_TAGS.contains(tag_name)
}

// https://developer.mozilla.org/en-US/docs/Web/SVG/Element
static SVG_TAGS: phf::Set<&'static str> = phf_set! {
  "svg", "animate", "animateMotion", "animateTransform", "circle", "clipPath", "color-profile", "defs", "desc",
  "discard", "ellipse", "feBlend", "feColorMatrix", "feComponentTransfer",  "feComposite", "feConvolveMatrix",
  "feDiffuseLighting", "feDisplacementMap", "feDistantLight", "feDropShadow", "feFlood", "feFuncA", "feFuncB",
  "feFuncG", "feFuncR", "feGaussianBlur", "feImage", "feMerge", "feMergeNode", "feMorphology", "feOffset", "fePointLight",
  "feSpecularLighting", "feSpotLight", "feTile", "feTurbulence", "filter", "foreignObject", "g", "hatch", "hatchpath",
  "image", "line", "linearGradient", "marker", "mask", "mesh", "meshgradient", "meshpatch", "meshrow", "metadata",
  "mpath", "path", "pattern", "polygon", "polyline", "radialGradient", "rect", "set", "solidcolor", "stop",
  "switch", "symbol", "text", "textPath", "title", "tspan", "unknown", "use", "view",
};
pub fn is_svg_tag(tag_name: &str) -> bool {
  SVG_TAGS.contains(tag_name)
}

// https://www.w3.org/TR/mathml4/ (content elements excluded)
static MATH_TAGS: phf::Set<&'static str> = phf_set! {
  "annotation", "annotation-xml", "maction", "maligngroup", "malignmark", "math", "menclose",
  "merror", "mfenced", "mfrac", "mfraction", "mglyph", "mi", "mlabeledtr", "mlongdiv", "mmultiscripts",
  "mn", "mo", "mover", "mpadded", "mphantom", "mprescripts", "mrow", "ms", "mscarries", "mscarry",
  "msgroup", "msline", "mspace", "msqrt", "msrow", "mstack", "mstyle", "msub", "msubsup", "msup",
  "mtable", "mtd", "mtext", "mtr", "munder", "munderover", "none", "semantics"
};
pub fn is_math_ml_tag(tag_name: &str) -> bool {
  MATH_TAGS.contains(tag_name)
}

// Mirrors `compiler-dom`'s `getNamespace`: resolves the namespace of `tag`
// from its parent element's tag and namespace.
// https://html.spec.whatwg.org/multipage/parsing.html#tree-construction
pub fn get_namespace(
  tag: &str,
  parent_tag: Option<&str>,
  parent_ns: i32,
  parent_is_html_annotation_xml: bool,
) -> i32 {
  let mut ns = parent_ns;
  if let Some(parent_tag) = parent_tag {
    if ns == 2 {
      if parent_tag == "annotation-xml" {
        if is_svg_tag(tag) {
          return 1;
        }
        if parent_is_html_annotation_xml {
          ns = 0;
        }
      } else if matches!(parent_tag, "mi" | "mo" | "mn" | "ms" | "mtext")
        && tag != "mglyph"
        && tag != "malignmark"
      {
        ns = 0;
      }
    } else if ns == 1 && matches!(parent_tag, "foreignObject" | "desc" | "title") {
      ns = 0;
    }
  }
  if ns == 0 {
    if is_svg_tag(tag) {
      return 1;
    }
    if is_math_ml_tag(tag) {
      return 2;
    }
  }
  ns
}

pub fn is_html_annotation_xml(node: &JSXElement) -> bool {
  matches!(&node.opening_element.name, JSXElementName::Identifier(name) if name.name == "annotation-xml")
    && node.opening_element.attributes.iter().any(|attr| {
      let JSXAttributeItem::Attribute(attr) = attr else {
        return false;
      };
      matches!(&attr.name, JSXAttributeName::Identifier(name) if name.name == "encoding")
        && matches!(&attr.value, Some(JSXAttributeValue::StringLiteral(value))
          if matches!(value.value.as_str(), "text/html" | "application/xhtml+xml"))
    })
}

pub fn is_jsx_component(node: &JSXElement) -> bool {
  match &node.opening_element.name {
    JSXElementName::Identifier(name) => name
      .name
      .as_str()
      .chars()
      .next()
      .is_some_and(|c| c.is_ascii_uppercase() || !c.is_ascii() || c == '_' || c == '$'),
    JSXElementName::NamespacedName(_) => false,
    _ => true,
  }
}

pub fn is_custom_element(node: &JSXElement) -> bool {
  match &node.opening_element.name {
    JSXElementName::Identifier(name) => {
      let tag_name = name.name.as_str();
      tag_name
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_lowercase())
        && tag_name.contains("-")
    }
    _ => false,
  }
}

pub fn is_native_tag(tag: &str) -> bool {
  !tag.contains("-")
    && tag
      .chars()
      .next()
      .is_some_and(|c| !(c.is_ascii_uppercase() || !c.is_ascii() || c == '_' || c == '$'))
}

static VOID_TAGS: phf::Set<&'static str> = phf_set! {
  "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
  "track", "wbr",
};
pub fn is_void_tag(tag_name: &str) -> bool {
  VOID_TAGS.contains(tag_name)
}

// https://html.spec.whatwg.org/multipage/parsing.html#formatting
static FORMATTING_TAGS: phf::Set<&'static str> = phf_set! {
  "a", "b", "big", "code", "em", "font", "i", "nobr", "s", "small", "strike", "strong",
  "tt", "u",
};
pub fn is_formatting_tag(tag_name: &str) -> bool {
  FORMATTING_TAGS.contains(tag_name)
}

// Elements that always require explicit closing tags due to HTML parsing rules.
// These include:
// - Formatting elements (a, b, i, etc.) - handled by FORMATTING_TAGS
// - Elements with special parsing rules
// - Scope boundary elements
static ALWAYS_CLOSE_TAGS: phf::Set<&'static str> = phf_set! {
  "title", "style", "script", "noscript", "template", // raw text / special parsing
  "object", "table", "button", "textarea", "select", "iframe", "fieldset", "form", // scope boundary / form elements
  "foreignObject", "desc", "mi", "mo", "mn", "ms", "mtext", "annotation-xml", // foreign scope boundary
};
pub fn is_always_close_tag(tag_name: &str) -> bool {
  ALWAYS_CLOSE_TAGS.contains(tag_name)
}

// Tags for which the HTML parser drops the first newline after the start tag.
// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody
pub fn is_ignore_newline_tag(tag_name: &str) -> bool {
  matches!(tag_name, "pre" | "textarea")
}

// Inline elements
static INLINE_TAGS: phf::Set<&'static str> = phf_set! {
  "a", "abbr", "acronym", "b", "bdi", "bdo", "big", "br", "button" , "canvas", "cite", "code", "data", "datalist",
  "del", "dfn", "em", "embed", "i", "iframe", "img", "input", "ins", "kbd", "label", "map", "mark", "meter",
  "noscript", "object", "output", "picture", "progress", "q", "ruby", "s", "samp", "script", "select",
  "small", "span", "strong", "sub", "sup", "svg", "textarea", "time", "u", "tt", "var", "video",
};
pub fn is_inline_tag(tag_name: &str) -> bool {
  INLINE_TAGS.contains(tag_name)
}

// Block elements
static BLOCK_TAGS: phf::Set<&'static str> = phf_set! {
  "address", "article", "aside", "blockquote", "dd", "details", "dialog", "div", "dl" , "dt", "fieldset",
  "figcaption", "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "header", "hgroup", "hr", "li",
  "main", "menu", "nav", "ol", "p", "pre", "section", "table", "ul",
};
pub fn is_block_tag(tag_name: &str) -> bool {
  BLOCK_TAGS.contains(tag_name)
}

static BUILD_IN_DIRECTIVE: phf::Set<&'static str> = phf_set! {
  "bind", "cloak", "else-if", "else", "for", "html", "if", "model", "on", "once", "pre", "show",
  "slot", "slots", "text", "memo",
};
pub fn is_built_in_directive(prop_name: &str) -> bool {
  BUILD_IN_DIRECTIVE.contains(prop_name)
}

static BOOLEAN_ATTRS: phf::Set<&'static str> = phf_set! {
  "itemscope", "allowfullscreen", "formnovalidate", "ismap", "nomodule", "novalidate", "readonly",
  "async", "autofocus", "autoplay", "controls", "default", "defer", "disabled", "hidden",
  "inert", "loop", "open", "required", "reversed", "scoped", "seamless",
  "checked", "muted", "multiple", "selected"
};
pub fn is_boolean_attr(prop_name: &str) -> bool {
  BOOLEAN_ATTRS.contains(prop_name)
}

// Boolean attributes whose name differs from their dom property; `setAttr`
// inspects the raw value before `setAttribute`.
static SPECIAL_BOOLEAN_ATTRS: phf::Set<&'static str> = phf_set! {
  "itemscope", "allowfullscreen", "formnovalidate", "ismap", "nomodule", "novalidate", "readonly"
};
pub fn is_special_boolean_attr(prop_name: &str) -> bool {
  SPECIAL_BOOLEAN_ATTRS.contains(prop_name)
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

static DELEGATED_EVENTS: phf::Set<&'static str> = phf_set! {
  "beforeinput", "click", "dblclick", "contextmenu", "focusin", "focusout", "input",
  "keydown", "keyup", "mousedown", "mousemove", "mouseout", "mouseover", "mouseup",
  "pointerdown", "pointermove", "pointerout", "pointerover", "pointerup",
  "touchend", "touchmove", "touchstart",
};

pub fn is_delegated_event(event: &str) -> bool {
  DELEGATED_EVENTS.contains(event)
}

pub fn is_event_option_modifier(modifier: &str) -> bool {
  matches!(modifier, "passive" | "once" | "capture")
}

// React-style camelCase modifier suffixes on `on*` props, mapped to the
// same modifiers the `_` syntax accepts, e.g. `onClickCapture` is
// equivalent to `onClick_capture`.
static CAMEL_EVENT_MODIFIERS: &[(&str, &str)] = &[
  ("Capture", "capture"),
  ("Once", "once"),
  ("Passive", "passive"),
  ("Prevent", "prevent"),
  ("Stop", "stop"),
  ("Self", "self"),
  ("Ctrl", "ctrl"),
  ("Shift", "shift"),
  ("Alt", "alt"),
  ("Meta", "meta"),
  ("Exact", "exact"),
  ("Middle", "middle"),
  ("Left", "left"),
  ("Right", "right"),
  ("Enter", "enter"),
  ("Tab", "tab"),
  ("Delete", "delete"),
  ("Esc", "esc"),
  ("Space", "space"),
  ("Up", "up"),
  ("Down", "down"),
];

// key-name modifiers — only meaningful on keyboard events
fn is_key_name(modifier: &str) -> bool {
  matches!(
    modifier,
    "enter" | "tab" | "delete" | "esc" | "space" | "up" | "down"
  )
}

// Splits trailing camelCase modifiers off an event name, e.g.
// `clickCaptureOnce` -> (`click`, ["capture", "once"]).
fn split_camel_event_modifiers(mut name: &str) -> (&str, Vec<&'static str>) {
  let mut modifiers = vec![];
  while let Some((stripped, modifier)) =
    CAMEL_EVENT_MODIFIERS.iter().find_map(|(suffix, modifier)| {
      name
        .strip_suffix(suffix)
        .filter(|stripped| !stripped.is_empty())
        .map(|stripped| (stripped, *modifier))
    })
  {
    name = stripped;
    modifiers.push(modifier);
  }
  modifiers.reverse();
  (name, modifiers)
}

// Standard DOM event names, used to recognize camelCase modifier aliases
// (`onClickCapture`) without hijacking custom events (`onMoveLeft`).
static DOM_EVENTS: phf::Set<&'static str> = phf_set! {
  // mouse
  "click", "contextmenu", "dblclick", "auxclick", "mousedown", "mouseenter",
  "mouseleave", "mousemove", "mouseout", "mouseover", "mouseup",
  // pointer
  "pointerover", "pointerenter", "pointerdown", "pointermove", "pointerup",
  "pointercancel", "pointerout", "pointerleave", "pointerrawupdate",
  "gotpointercapture", "lostpointercapture",
  // touch
  "touchstart", "touchend", "touchmove", "touchcancel",
  // keyboard
  "keydown", "keypress", "keyup",
  // input and form
  "beforeinput", "input", "change", "invalid", "submit", "reset", "formdata",
  "select", "selectstart", "selectionchange", "search", "beforematch",
  // focus
  "focus", "blur", "focusin", "focusout",
  // drag
  "drag", "dragend", "dragenter", "dragexit", "dragleave", "dragover", "dragstart", "drop",
  // wheel and scroll
  "wheel", "mousewheel", "scroll", "scrollend", "scrollsnapchange",
  "scrollsnapchanging",
  // clipboard
  "copy", "cut", "paste",
  // composition
  "compositionstart", "compositionupdate", "compositionend",
  // media
  "abort", "canplay", "canplaythrough", "cuechange", "durationchange",
  "emptied", "encrypted", "ended", "error", "loadeddata", "loadedmetadata",
  "loadstart", "loadend", "pause", "play", "playing", "progress", "ratechange",
  "seeked", "seeking", "stalled", "suspend", "timeupdate", "volumechange",
  "waiting", "waitingforkey", "enterpictureinpicture", "leavepictureinpicture",
  // animation and transition
  "animationstart", "animationend", "animationiteration", "animationcancel",
  "transitionstart", "transitionend", "transitionrun", "transitioncancel",
  // document and window
  "load", "unload", "beforeunload", "resize", "domcontentloaded",
  "readystatechange", "visibilitychange", "hashchange", "popstate", "pageshow",
  "pagehide", "online", "offline", "message", "messageerror", "storage",
  "languagechange", "rejectionhandled", "unhandledrejection",
  "prerenderingchange", "contextrestored",
  // element lifecycle and misc
  "toggle", "beforetoggle", "close", "cancel", "command", "slotchange",
  "fullscreenchange", "fullscreenerror", "pointerlockchange",
  "pointerlockerror", "securitypolicyviolation",
  "contentvisibilityautostatechange", "devicemotion", "deviceorientation",
  "deviceorientationabsolute", "gamepadconnected", "gamepaddisconnected",
  "beforeinstallprompt", "appinstalled", "orientationchange",
};
fn is_dom_event(name: &str) -> bool {
  DOM_EVENTS.contains(name)
}

// Splits an `on*` prop name into event name and `_`-separated modifiers.
// CamelCase modifier aliases (`onClickCapture`) are normalized for native DOM
// events only — components and custom events (`onMoveLeft`) keep their names
// verbatim so emit() lookups still resolve. Key names only alias on keyboard
// events, so `onDragEnter` stays `dragEnter` rather than `drag` + `enter`.
pub fn split_event_name<'a>(replaced: &'a str, is_component: bool) -> Vec<&'a str> {
  let mut splited = replaced.split("_").collect::<Vec<_>>();
  if !is_component {
    let (name, camel_modifiers) = split_camel_event_modifiers(splited[0]);
    if !camel_modifiers.is_empty()
      && is_dom_event(name)
      && (is_keyboard_event(name) || camel_modifiers.iter().all(|m| !is_key_name(m)))
    {
      splited[0] = name;
      splited.splice(1..1, camel_modifiers);
    }
  }
  splited
}

// `click` listeners with `right`/`middle` modifiers fire `contextmenu` and
// `mouseup` respectively — the click event itself never fires for them.
pub fn normalized_event_name<'a>(name: &'a str, modifiers: &[impl AsRef<str>]) -> &'a str {
  if name == "click" && modifiers.iter().any(|m| m.as_ref() == "right") {
    "contextmenu"
  } else if name == "click" && modifiers.iter().any(|m| m.as_ref() == "middle") {
    "mouseup"
  } else {
    name
  }
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

pub fn is_event(mut s: &str) -> bool {
  if s.starts_with("\"") {
    s = &s[1..];
  }
  s.starts_with("on")
    && s
      .chars()
      .nth(2)
      .map(|c| c.is_ascii_uppercase())
      .unwrap_or(false)
}

// vdom's `isNativeOn` (`/^on[a-z]/`): native event properties like `onclick`
// that are set by value (dom property or attribute) rather than through
// `addEventListener`.
pub fn is_native_on(s: &str) -> bool {
  s.starts_with("on")
    && s
      .as_bytes()
      .get(2)
      .map(|c| c.is_ascii_lowercase())
      .unwrap_or(false)
}

pub fn get_directive_name(s: &str) -> &str {
  if is_event(s) {
    "on"
  } else if let Some(name) = s.strip_prefix("v-") {
    name
  } else {
    "bind"
  }
}

// Checks if the input `node` is a reference to a bound variable.
//
// Copied from https://github.com/babel/babel/blob/main/packages/babel-types/src/validators/isReferenced.ts
//
// @param node - The node to check.
// @param parent - The parent node of the input `node`.
// @param grandparent - The grandparent node of the input `node`.
// @returns True if the input `node` is a reference to a bound variable, false otherwise.
fn is_referenced(node: &IdentifierReference, parent: AstKind) -> bool {
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
    AstKind::ObjectProperty(_) => true,
    // no: class { NODE = value; }
    // yes: class { [NODE] = value; }
    // yes: class { key = NODE; }
    AstKind::PropertyDefinition(parent) if parent.key.span().eq(&node.span) => parent.computed,
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
    // yes: NODE = right;
    AstKind::AssignmentExpression(_) => {
      // babel's isReferenced check returns false for ids being assigned to, so we
      // need to cover those cases here
      // parent.right.span().eq(&node.span)
      true
    }

    // yes: [NODE = foo] = [];
    // yes: [foo = NODE] = [];
    AstKind::AssignmentPattern(_) => true,

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
    AstKind::ExportSpecifier(parent) => parent.local.span().eq(&node.span),

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
    AstKind::TSPropertySignature(parent) if parent.key.span().eq(&node.span) => parent.computed,
    _ => true,
  }
}

pub fn is_referenced_identifier<'a>(id: &IdentifierReference, parent: Option<AstKind<'a>>) -> bool {
  let Some(parent) = parent else { return true };

  // is a special keyword but parsed as identifier
  if id.name.eq("arguments") {
    return false;
  }

  is_referenced(id, parent)
}

pub fn is_slots_component(name: &str) -> bool {
  name.starts_with("slots.") || name.starts_with("$slots.") || name.starts_with("this.$slots.")
}

pub fn is_slots_expression(exp: &Expression) -> Option<bool> {
  match exp.get_inner_expression() {
    Expression::ObjectExpression(_)
    | Expression::FunctionExpression(_)
    | Expression::ArrowFunctionExpression(_) => Some(false),
    Expression::Identifier(_) => Some(true),
    Expression::StaticMemberExpression(exp)
      if matches!(exp.object, Expression::ThisExpression(_)) && exp.property.name == "$slots" =>
    {
      Some(true)
    }
    Expression::LogicalExpression(exp) => {
      if let Some(result) = is_slots_expression(&exp.right)
        && !result
      {
        // prevent render false text (e.g., `createVNode(Comp, null, false)`)
        Some(true)
      } else {
        None
      }
    }
    Expression::ConditionalExpression(exp) => {
      if let Some(result) = is_slots_expression(&exp.consequent)
        && !result
      {
        Some(true)
      } else if let Some(result) = is_slots_expression(&exp.alternate)
        && !result
      {
        Some(true)
      } else {
        None
      }
    }
    _ => None,
  }
}
