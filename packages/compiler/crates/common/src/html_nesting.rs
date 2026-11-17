// The start tags that close an element still open above them in the same
// template string, transcribed from the "in body" insertion mode of
// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody for
// the native html tags the compiler knows. Complements
// `dom::is_valid_html_nesting`, which only covers what the direct
// parent itself can hold (table, row, select and raw-text content models):
// `<p><span><div>` is valid at every step, yet the parser closes the `<p>` at
// the `<div>`.

use phf::phf_set;

use crate::{check::is_void_tag, dom::is_valid_html_nesting};

// Start tags that close an open `<p>` in button scope.
static P_CLOSING_TAGS: phf::Set<&'static str> = phf_set! {
  "address", "article", "aside", "blockquote", "dd", "details", "dialog", "div", "dl", "dt",
  "fieldset", "figcaption", "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6",
  "header", "hgroup", "hr", "li", "main", "menu", "nav", "ol", "p", "pre", "section", "summary",
  "table", "ul",
};

// "has an element in scope": the native html tags that end the search.
static SCOPE_BOUNDARY_TAGS: phf::Set<&'static str> =
  phf_set! { "caption", "object", "select", "table", "td", "th" };

// Markers in the list of active formatting elements.
static FORMATTING_MARKER_TAGS: phf::Set<&'static str> =
  phf_set! { "caption", "object", "td", "th" };

// Special elements: they end the search for an open `<li>`, `<dd>` or `<dt>`
// (the parser exempts address, div and p there), and an "any other end tag"
// that reaches one while still open is ignored.
static SPECIAL_TAGS: phf::Set<&'static str> = phf_set! {
  "address", "area", "article", "aside", "base", "blockquote", "body", "br", "button", "caption",
  "col", "colgroup", "dd", "details", "div", "dl", "dt", "embed", "fieldset", "figcaption",
  "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "head", "header", "hgroup", "hr",
  "html", "iframe", "img", "input", "li", "link", "main", "menu", "meta", "nav", "noscript",
  "object", "ol", "p", "param", "pre", "script", "section", "select", "source", "style", "summary",
  "table", "tbody", "td", "template", "textarea", "tfoot", "th", "thead", "title", "tr", "track",
  "ul", "wbr",
};

/// Whether the html parser counts `tag` as special (`SPECIAL_TAGS`), which is
/// what an "any other end tag" is ignored at.
pub fn is_special_tag(tag: &str) -> bool {
  SPECIAL_TAGS.contains(tag)
}

// Elements whose end tag the parser implies before an `<rt>` or `<rp>`.
static IMPLIED_END_TAGS: phf::Set<&'static str> =
  phf_set! { "dd", "dt", "li", "optgroup", "option", "p", "rp", "rt" };

fn ends_list_item_search(tag: &str) -> bool {
  SPECIAL_TAGS.contains(tag) && !matches!(tag, "address" | "div" | "p")
}

// End tags with handling of their own pop whatever is still open below them.
// Any other end tag is ignored when it meets a special element on its way up
// the stack, and a formatting end tag runs the adoption agency around one, so
// under those an open special descendant has to close itself.
static POPS_OPEN_DESCENDANTS_TAGS: phf::Set<&'static str> = phf_set! {
  "address", "article", "aside", "blockquote", "button", "caption", "colgroup", "dd", "details",
  "dialog", "div", "dl", "dt", "fieldset", "figcaption", "figure", "footer", "h1", "h2", "h3",
  "h4", "h5", "h6", "header", "hgroup", "li", "main", "menu", "nav", "object", "ol", "p", "pre",
  "section", "summary", "table", "tbody", "td", "tfoot", "th", "thead", "tr", "ul",
};

/// Whether the end tag of `tag` pops whatever is still open below it, so that a
/// special descendant need not close itself.
pub fn pops_open_descendants(tag: &str) -> bool {
  POPS_OPEN_DESCENDANTS_TAGS.contains(tag)
}

// Html start tags that end foreign content: the parser pops the open svg or
// mathml elements and inserts these in the html namespace, while vdom creates
// them in the foreign namespace their ast node carries. Only at html
// integration points (`foreignObject`, `mi`, ...) do html children stay.
static FOREIGN_BREAKOUT_TAGS: phf::Set<&'static str> = phf_set! {
  "b", "blockquote", "body", "br", "code", "dd", "div", "dl", "dt", "em", "embed", "h1", "h2",
  "h3", "h4", "h5", "h6", "head", "hr", "i", "img", "li", "menu", "meta", "ol", "p", "pre", "ruby",
  "s", "small", "span", "strong", "sub", "sup", "table", "u", "ul", "var",
};

/// Whether the html parser moves `tag` out of the foreign content it was
/// written in, so that it cannot share a template string with its parent.
pub fn is_foreign_breakout_tag(tag: &str) -> bool {
  FOREIGN_BREAKOUT_TAGS.contains(tag)
}

// Table parts whose text content other than whitespace the parser
// foster-parents in front of the table.
static FOSTER_PARENTING_TAGS: phf::Set<&'static str> =
  phf_set! { "colgroup", "table", "tbody", "tfoot", "thead", "tr" };

/// Whether the html parser foster-parents the text of `tag` out of it, so that
/// such text cannot be written into the same template string.
pub fn is_foster_parenting_tag(tag: &str) -> bool {
  FOSTER_PARENTING_TAGS.contains(tag)
}

// html elements sit under a foreign one only at an integration point, which is
// in every scope list and special, but not a formatting marker
fn is_foreign(ns: i32) -> bool {
  ns != 0
}

/// What is still open above an element in its template, as far as the start
/// tags that close an ancestor can see: a bit per target, set when the element
/// is entered and cleared at the boundary that ends its search. Passed down
/// from parent to child, so that no element has to walk its ancestors.
pub mod open_scope {
  /// a `<p>` in button scope
  pub const P: i32 = 1 << 0;
  /// a `<button>` in scope
  pub const BUTTON: i32 = 1 << 1;
  /// a `<ruby>` in scope
  pub const RUBY: i32 = 1 << 2;
  /// an `<a>` above the last formatting element marker
  pub const A: i32 = 1 << 3;
  /// a `<form>`: the form element pointer, which no scope resets
  pub const FORM: i32 = 1 << 4;
  /// an `<li>`, or a `<dd>` or `<dt>`, within the list item search, which a
  /// special element other than address, div and p ends
  pub const LI: i32 = 1 << 5;
  pub const DD_DT: i32 = 1 << 6;
  /// not a scope: the element is written into its parent's template (unset
  /// when it starts one).
  pub const SHARED: i32 = 1 << 7;
}

// the targets grouped by the kind of scope that ends their search
const IN_BUTTON_SCOPE: i32 = open_scope::P;
const IN_SCOPE: i32 = open_scope::BUTTON | open_scope::RUBY;
const BEFORE_MARKER: i32 = open_scope::A;
const IN_LIST_ITEM_SEARCH: i32 = open_scope::LI | open_scope::DD_DT;

/// The scope the children of `tag` see, given the scope `tag` itself was
/// written in (0 when it starts a template).
pub fn enter_open_scope(scope: i32, tag: &str, ns: i32) -> i32 {
  let mut scope = scope;
  if is_foreign(ns) {
    return scope & !(IN_BUTTON_SCOPE | IN_SCOPE | IN_LIST_ITEM_SEARCH);
  }
  if SCOPE_BOUNDARY_TAGS.contains(tag) {
    scope &= !(IN_BUTTON_SCOPE | IN_SCOPE);
  }
  if FORMATTING_MARKER_TAGS.contains(tag) {
    scope &= !BEFORE_MARKER;
  }
  if ends_list_item_search(tag) {
    scope &= !IN_LIST_ITEM_SEARCH;
  }
  match tag {
    "p" => scope | open_scope::P,
    // button scope ends at a button as well
    "button" => (scope & !IN_BUTTON_SCOPE) | open_scope::BUTTON,
    "ruby" => scope | open_scope::RUBY,
    "a" => scope | open_scope::A,
    "form" => scope | open_scope::FORM,
    "li" => scope | open_scope::LI,
    "dd" | "dt" => scope | open_scope::DD_DT,
    _ => scope,
  }
}

/// Whether the html parser places an element written under `parent` there, so
/// that both can share a template string. Three layers: a void parent holds no
/// children at all, `is_valid_html_nesting` rejects what the parent itself
/// cannot hold (table, row, select and raw-text content models), and
/// `closes_open_ancestor` the start tags that end an element still open above.
/// The end-tag omission in `transform_element` assumes only elements passing all
/// three share a template.
pub fn keeps_in_template(tag: &str, ns: i32, parent_tag: &str, scope: i32) -> bool {
  !is_void_tag(parent_tag)
    && is_valid_html_nesting(parent_tag, tag)
    && !closes_open_ancestor(tag, ns, parent_tag, scope)
}

/// Whether the html parser, on the start tag of `tag`, closes an element that
/// is still open above it in the same template - the `<p>` of
/// `<p><span><div>`, the `<li>` of `<li><span><li>`, a link inside a link. The
/// element would then end up elsewhere than the compiler expects, so it has to
/// go into a template of its own. `scope` is what the parent leaves open
/// (`enter_open_scope`).
pub fn closes_open_ancestor(tag: &str, ns: i32, parent_tag: &str, scope: i32) -> bool {
  if is_foreign(ns) {
    return false;
  }
  if P_CLOSING_TAGS.contains(tag) && scope & open_scope::P != 0 {
    return true;
  }
  match tag {
    "li" => scope & open_scope::LI != 0,
    "dd" | "dt" => scope & open_scope::DD_DT != 0,
    "button" => scope & open_scope::BUTTON != 0,
    "a" => scope & open_scope::A != 0,
    "form" => scope & open_scope::FORM != 0,
    "rt" | "rp" => scope & open_scope::RUBY != 0 && IMPLIED_END_TAGS.contains(parent_tag),
    _ => false,
  }
}
