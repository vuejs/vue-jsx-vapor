// Modified from https://github.com/MananTank/validate-html-nesting
// with ISC license
//
// To avoid runtime dependency on validate-html-nesting
// This file should not change very often in the original repo
// but we may need to keep it up-to-date from time to time.

use std::{collections::HashMap, sync::LazyLock};

// returns true if given parent-child nesting is valid HTML
pub fn is_valid_html_nesting(parent: &str, child: &str) -> bool {
  //if the parent is a template, it can have any child
  if parent == "template" {
    return true;
  }

  // if we know the list of children that are the only valid children for the given parent
  if let Some(parent) = ONLY_VALID_CHILDREN.get(parent) {
    return parent.contains(&child);
  }

  // if we know the list of parents that are the only valid parents for the given child
  if let Some(child) = ONLY_VALID_PARENTS.get(child) {
    return child.contains(&parent);
  }

  // if we know the list of children that are NOT valid for the given parent
  if let Some(parent) = KNOWN_INVALID_CHILDREN.get(parent) && // check if the child is in the list of invalid children
  // if so, return false
  parent.contains(&child)
  {
    return false;
  }

  // if we know the list of parents that are NOT valid for the given child
  if let Some(child) = KNOWN_INVALID_PARENTS.get(child) && // check if the parent is in the list of invalid parents
  // if so, return false
  child.contains(&parent)
  {
    return false;
  }

  true
}

static ONLY_VALID_CHILDREN: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
  HashMap::from([
    (
      "head",
      vec![
        "base",
        "basefront",
        "bgsound",
        "link",
        "meta",
        "title",
        "noscript",
        "noframes",
        "style",
        "script",
        "template",
      ],
    ),
    ("optgroup", vec!["option"]),
    ("select", vec!["optgroup", "option", "hr"]),
    // table
    (
      "table",
      vec!["caption", "colgroup", "tbody", "tfoot", "thead"],
    ),
    ("tr", vec!["td", "th"]),
    ("colgroup", vec!["col"]),
    ("tbody", vec!["tr"]),
    ("thead", vec!["tr"]),
    ("tfoot", vec!["tr"]),
    // these elements can not have any children elements
    ("script", vec![]),
    ("iframe", vec![]),
    ("option", vec![]),
    ("textarea", vec![]),
    ("style", vec![]),
    ("title", vec![]),
  ])
});

// maps elements to set of elements which can be it's parent, no other
static ONLY_VALID_PARENTS: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
  HashMap::from([
    // sections
    ("html", vec![]),
    ("body", vec!["html"]),
    ("head", vec!["html"]),
    // table
    ("td", vec!["tr"]),
    ("colgroup", vec!["table"]),
    ("caption", vec!["table"]),
    ("tbody", vec!["table"]),
    ("tfoot", vec!["table"]),
    ("col", vec!["colgroup"]),
    ("th", vec!["tr"]),
    ("thead", vec!["table"]),
    ("tr", vec!["tbody", "thead", "tfoot"]),
    // data list
    ("dd", vec!["dl", "div"]),
    ("dt", vec!["dl", "div"]),
    // other
    ("figcaption", vec!["figure"]),
    // li: new Set(["ul", "ol"]),
    ("summary", vec!["details"]),
    ("area", vec!["map"]),
  ])
});

static KNOWN_INVALID_CHILDREN: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
  HashMap::from([
    (
      "p",
      vec![
        "address",
        "article",
        "aside",
        "blockquote",
        "center",
        "details",
        "dialog",
        "dir",
        "div",
        "dl",
        "fieldset",
        "figure",
        "footer",
        "form",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "header",
        "hgroup",
        "hr",
        "li",
        "main",
        "nav",
        "menu",
        "ol",
        "p",
        "pre",
        "section",
        "table",
        "ul",
      ],
    ),
    (
      "svg",
      vec![
        "b",
        "blockquote",
        "br",
        "code",
        "dd",
        "div",
        "dl",
        "dt",
        "em",
        "embed",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "hr",
        "i",
        "img",
        "li",
        "menu",
        "meta",
        "ol",
        "p",
        "pre",
        "ruby",
        "s",
        "small",
        "span",
        "strong",
        "sub",
        "sup",
        "table",
        "u",
        "ul",
        "var",
      ],
    ),
  ])
});

static KNOWN_INVALID_PARENTS: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
  HashMap::from([
    ("a", vec!["a"]),
    ("button", vec!["button"]),
    ("dd", vec!["dd", "dt"]),
    ("dt", vec!["dd", "dt"]),
    ("form", vec!["form"]),
    ("li", vec!["li"]),
    ("h1", vec!["h1", "h2", "h3", "h4", "h5", "h6"]),
    ("h2", vec!["h1", "h2", "h3", "h4", "h5", "h6"]),
    ("h3", vec!["h1", "h2", "h3", "h4", "h5", "h6"]),
    ("h4", vec!["h1", "h2", "h3", "h4", "h5", "h6"]),
    ("h5", vec!["h1", "h2", "h3", "h4", "h5", "h6"]),
    ("h6", vec!["h1", "h2", "h3", "h4", "h5", "h6"]),
  ])
});
