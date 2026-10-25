// The start tags that close an element still open above them in the same
// template string, transcribed from the "in body" insertion mode of
// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody for
// the native html tags the compiler knows. Complements
// `dom::is_valid_html_nesting`, which only covers what the direct
// parent itself can hold (table, row, select and raw-text content models, void
// parents): `<p><span><div>` is valid at every step, yet the parser closes the
// `<p>` at the `<div>`.

use phf::phf_set;

use crate::dom::is_valid_html_nesting;

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

// Special elements, which end the search for an open `<li>`, `<dd>` or `<dt>`
// (the parser exempts address, div and p).
static SPECIAL_TAGS: phf::Set<&'static str> = phf_set! {
  "area", "article", "aside", "base", "blockquote", "body", "br", "button", "caption", "col",
  "colgroup", "dd", "details", "dl", "dt", "embed", "fieldset", "figcaption", "figure", "footer",
  "form", "h1", "h2", "h3", "h4", "h5", "h6", "head", "header", "hgroup", "hr", "html", "iframe",
  "img", "input", "li", "link", "main", "menu", "meta", "nav", "noscript", "object", "ol",
  "param", "pre", "script", "section", "select", "source", "style", "summary", "table", "tbody",
  "td", "template", "textarea", "tfoot", "th", "thead", "title", "tr", "track", "ul", "wbr",
};

// Elements whose end tag the parser implies before an `<rt>` or `<rp>`.
static IMPLIED_END_TAGS: phf::Set<&'static str> =
  phf_set! { "dd", "dt", "li", "optgroup", "option", "p", "rp", "rt" };

fn ends_list_item_search(tag: &str) -> bool {
  SPECIAL_TAGS.contains(tag) && !matches!(tag, "address" | "div" | "p")
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
/// that both can share a template string. Two layers: `is_valid_html_nesting`
/// rejects what the parent itself cannot hold (table, row, select and raw-text
/// content models, void parents), `closes_open_ancestor` the start tags that
/// end an element still open above. The end-tag omission in `transform_element`
/// assumes only elements passing both share a template.
pub fn keeps_in_template(tag: &str, ns: i32, parent_tag: &str, scope: i32) -> bool {
  is_valid_html_nesting(parent_tag, tag) && !closes_open_ancestor(tag, ns, parent_tag, scope)
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
