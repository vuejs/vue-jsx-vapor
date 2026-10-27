use compiler::{TransformOptions, transform};
use insta::assert_snapshot;

// basic - last child can omit closing tag
#[test]
fn template_abbreviation() {
  let code = transform(
    "<div>hello</div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>hello", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn template_abbreviation1() {
  let code = transform(
    "<div><div>hello</div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>hello", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// non-last child needs closing tag
#[test]
fn template_abbreviation2() {
  let code = transform(
    "<div><span>foo</span><span></span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span>foo</span><span>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn template_abbreviation3() {
  let code = transform(
    "<div><hr/><div></div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><hr><div>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn template_abbreviation4() {
  let code = transform(
    "<div><div></div><hr/></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div></div><hr>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// multi-root: each root generates its own template
#[test]
fn template_abbreviation5() {
  let code = transform(
    "<><span></span>hello</>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<span>", 2);
  const _t1 = _template("hello", 2);
  (() => {
  	const _n0 = _t0();
  	const _n1 = _t1();
  	return [_n0, _n1];
  })();
  "#);
}

// formatting tags on rightmost path can omit closing tag
#[test]
fn formatting_tags() {
  let code = transform(
    "<div><b>bold</b></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><b>bold", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn formatting_tags1() {
  let code = transform(
    "<div><i><b>text</b></i></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><i><b>text", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// formatting tags NOT on rightmost path need closing tag
#[test]
fn formatting_tags2() {
  let code = transform(
    "<div><b>bold</b><span></span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><b>bold</b><span>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn formatting_tags3() {
  let code = transform(
    "<div><b>1</b><b>2</b></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><b>1</b><b>2", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// same-name on rightmost path can omit
#[test]
fn same_name_nested_tags() {
  let code = transform(
    "<div><div>inner</div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>inner", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// same-name NOT on rightmost path needs closing tag
#[test]
fn same_name_nested_tags1() {
  let code = transform(
    "<div><div>a</div><div>b</div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>a</div><div>b", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn same_name_nested_tags2() {
  let code = transform(
    "<span><span>1</span><span>2</span></span>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<span><span>1</span><span>2", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn same_name_descendant_before_an_ancestor_close() {
  let code = transform(
    "<div><div><section><div>x</div></section></div><p>after</p></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><section><div>x</div></div><p>after", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn same_name_boundary_does_not_cross_component_templates() {
  let code = transform(
    "<main><div><Comp><div><section><div>x</div></section></div></Comp></div><p>after</p></main>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx/vapor";
  import { child as _child, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<div><section><div>x", 2);
  const _t1 = _template("<main><div></div><p>after", 1);
  (() => {
  	const _n3 = _t1();
  	const _n2 = _child(_n3);
  	_setInsertionState(_n2);
  	const _n1 = _createComponent(Comp, null, () => {
  		const _n0 = _t0();
  		return _n0;
  	});
  	return _n3;
  })();
  "#);
}

#[test]
fn same_name_boundary_does_not_cross_invalid_nesting_templates() {
  let code = transform(
    "<main><div><p><div>x</div></p></div><section>after</section></main>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<div>x");
  const _t1 = _template("<main><div><p></div><section>after", 1);
  (() => {
  	const _n2 = _t1();
  	let _p0 = _child(_n2);
  	const _n1 = _child(_p0);
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

// void tags never need closing tags
#[test]
fn void_tags() {
  let code = transform(
    "<div><br/></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><br>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn void_tags1() {
  let code = transform(
    "<div><hr/></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><hr>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn void_tags2() {
  let code = transform(
    "<div><input/></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><input>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn void_tags3() {
  let code = transform(
    "<div><img/></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><img>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn deeply_nested() {
  let code = transform(
    "<div><div><div><span>deep</span></div></div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><div><span>deep", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn deeply_nested1() {
  let code = transform(
    "<div><div><span>a</span><span>b</span></div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><span>a</span><span>b", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// button always needs closing tag unless on rightmost path
#[test]
fn always_close_tags() {
  let code = transform(
    "<div><button>click</button></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><button>click", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags1() {
  let code = transform(
    "<div><button>click</button><span>sibling</span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><button>click</button><span>sibling", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// select always needs closing tag unless rightmost
#[test]
fn always_close_tags2() {
  let code = transform(
    "<div><select></select></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><select>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags3() {
  let code = transform(
    "<div><select></select><span>sibling</span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><select></select><span>sibling", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// table always needs closing tag unless rightmost
#[test]
fn always_close_tags4() {
  let code = transform(
    "<div><table></table></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><table>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags5() {
  let code = transform(
    "<div><table></table><span>sibling</span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><table></table><span>sibling", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// textarea always needs closing tag unless rightmost
#[test]
fn always_close_tags6() {
  let code = transform(
    "<div><textarea></textarea></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><textarea>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags7() {
  let code = transform(
    "<div><textarea></textarea><span>sibling</span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><textarea></textarea><span>sibling", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// template always needs closing tag unless rightmost
#[test]
fn always_close_tags8() {
  let code = transform(
    "<div><template></template></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><template>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags9() {
  let code = transform(
    "<div><template></template><span>sibling</span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><template></template><span>sibling", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// script always needs closing tag unless rightmost
#[test]
fn always_close_tags10() {
  let code = transform(
    "<div><script></script></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><script>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags11() {
  let code = transform(
    "<div><script></script><span>sibling</span></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><script><\/script><span>sibling", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// without always-close elements, normal abbreviation should work
#[test]
fn always_close_tags12() {
  let code = transform(
    "<div><form><input/></form></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><form><input>", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Inline element containing block element with sibling after inline
// The block element must close because inline ancestor needs to close
#[test]
fn inline_block_ancestor_relationships() {
  let code = transform(
    "<div><span><div>text</div></span><p>after</p></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><div>text</div></span><p>after", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Same situation but deeper nesting
#[test]
fn inline_block_ancestor_relationships1() {
  let code = transform(
    "<div>
      <span>
        <p>text</p>
      </span>
      <span>after</span>
    </div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><p>text</p></span><span>after", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Inline containing block on rightmost path - can omit
#[test]
fn inline_block_ancestor_relationships2() {
  let code = transform(
    "<div>
      <span>
        <div>text</div>
      </span>
    </div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><div>text", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Normal case - no inline/block issue
#[test]
fn inline_block_ancestor_relationships3() {
  let code = transform(
    "<div><p>text</p></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><p>text", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Sibling after parent but no inline/block issue
#[test]
fn inline_block_ancestor_relationship4() {
  let code = transform(
    "<div>
      <div>
        <p>text</p>
      </div>
      <span>after</span>
    </div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><p>text</div><span>after", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Multi-level inline nesting with block inside
// Outer span is not rightmost -> Needs close -> Inner block needs close
#[test]
fn inline_block_ancestor_relationships5() {
  let code = transform(
    "<div>
      <span>
        <b>
          <div>text</div>
        </b>
      </span>
      <p>after</p>
    </div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><b><div>text</div></b></span><p>after", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Mixed nesting: div > span > div > span > div
// The middle div is inside a span that needs closing (because of outer structure)
// Both inner divs need closing because they are inside spans that need closing
#[test]
fn inline_block_ancestor_relationships6() {
  let code = transform(
    "<div>
      <span>
        <div>
          <span>
            <div>text</div>
          </span>
        </div>
      </span>
      <p>after</p>
    </div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><div><span><div>text</div></span></div></span><p>after", 3);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// upstream: `form end tag`. `</form>` removes only the form element, so a
// descendant left open would swallow the form's next sibling
#[test]
fn form_end_tag() {
  let code = transform(
    "<div><form><div>x</div></form><p>y</p></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<div><form><div>x</div></form><p>y\""),
    "{code}"
  );

  let code = transform(
    "<div><form><div><b>x</b></div></form><p>y</p></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<div><form><div><b>x</b></div></form><p>y\""),
    "{code}"
  );

  // a form closed by its parent's end tag leaves the form element pointer set,
  // so the next `<form>` start tag is ignored
  let code = transform(
    "<div><div><form><input /></form></div><div><form><input /></form></div></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<div><div><form><input></form></div><div><form><input>\""),
    "{code}"
  );

  // a form on the rightmost path can still omit
  let code = transform(
    "<div><p>y</p><form><div>x</div></form></div>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<div><p>y</p><form><div>x\""),
    "{code}"
  );
}

// upstream: `foreign scope boundary elements`. The end tag of a foreign parent
// does not close a child in another namespace, and an ancestor's end tag does
// not close a foreign scope boundary element, so both have to close themselves.
#[test]
fn foreign_scope_boundary_elements() {
  for (source, template) in [
    (
      "<svg><foreignObject><div>text</div></foreignObject><rect /></svg>",
      "<svg><foreignObject><div>text</div></foreignObject><rect>",
    ),
    (
      "<svg><g><foreignObject><div><p>text</p></div></foreignObject></g><rect /></svg>",
      "<svg><g><foreignObject><div><p>text</div></foreignObject></g><rect>",
    ),
    (
      "<math><mi><span>x</span></mi><mo>+</mo></math>",
      "<math><mi><span>x</span></mi><mo>+",
    ),
    (
      "<svg><g><desc><div>text</div></desc></g><rect /></svg>",
      "<svg><g><desc><div>text</div></desc></g><rect>",
    ),
    (
      "<svg><foreignObject><math><mi>x</mi></math></foreignObject><rect /></svg>",
      "<svg><foreignObject><math><mi>x</mi></math></foreignObject><rect>",
    ),
    // an ancestor's end tag does not close a foreign scope boundary element
    (
      "<div><p><math><mi>x</mi></math></p><p>next</p></div>",
      "<div><p><math><mi>x</mi></p><p>next",
    ),
    (
      "<div><div><svg><foreignObject>text</foreignObject></svg></div><p>next</p></div>",
      "<div><div><svg><foreignObject>text</foreignObject></div><p>next",
    ),
    // rightmost path can still omit
    (
      "<svg><foreignObject><div>text</div></foreignObject></svg>",
      "<svg><foreignObject><div>text",
    ),
    ("<p><math><mi>x</mi></math></p>", "<p><math><mi>x"),
  ] {
    let code = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert!(
      code.contains(&format!("_template(\"{template}\"")),
      "{source}\n{code}"
    );
  }
}

// upstream: `nested list end tag`. `</li>` is ignored while a nested `<ul>` or
// `<ol>` is still open, so the next item would land in the nested list
#[test]
fn nested_list_end_tag() {
  let code = transform(
    "<ul><li>a<ul><li>b</li></ul></li><li>c</li></ul>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<ul><li>a<ul><li>b</li></ul></li><li>c\""),
    "{code}"
  );

  let code = transform(
    "<ol><li><div>a<ol><li>b</li></ol></div></li><li>c</li></ol>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<ol><li><div>a<ol><li>b</li></ol></li><li>c\""),
    "{code}"
  );

  // a list item on the rightmost path can still omit
  let code = transform(
    "<ul><li>a</li><li>b<ul><li>c</li></ul></li></ul>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(
    code.contains("_template(\"<ul><li>a</li><li>b<ul><li>c\""),
    "{code}"
  );
}

// A start tag the parser answers by closing an element still open above it
// cannot share that ancestor's template: the template string is parsed again
// at runtime, so the descendant would land outside the element the compiler
// expects and `parseTemplate`'s firstChild would drop the rest.
#[test]
fn descendant_closing_an_open_p() {
  let code = transform(
    "<p><span><div>x</div></span></p>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<div>x");
  const _t1 = _template("<p><span>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_n2);
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

#[test]
fn descendant_closing_an_open_p_across_deeper_ancestors() {
  let code = transform(
    "<p><span><div><section>x</section></div></span></p>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<div><section>x");
  const _t1 = _template("<p><span>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_n2);
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

// the dynamic child keeps the placeholder of its own template
#[test]
fn descendant_closing_an_open_p_with_dynamic_content() {
  let code = transform(
    "<p><span><div>{msg}</div></span></p>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx/vapor";
  import { child as _child, insert as _insert, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  const _t1 = _template("<p><span>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_n2);
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => msg);
  	return _n2;
  })();
  "#);
}

#[test]
fn descendant_closing_an_open_link() {
  let code = transform(
    r##"<a href="#a"><div><a href="#b">x</a></div></a>"##,
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<a href=#b>x");
  const _t1 = _template("<a href=#a><div>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_n2);
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

#[test]
fn descendant_closing_an_open_list_item() {
  let code = transform(
    "<ul><li><span><li>x</li></span></li></ul>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<li>x");
  const _t1 = _template("<ul><li><span>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_child(_n2));
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

#[test]
fn descendant_closing_an_open_data_term() {
  let code = transform(
    "<dl><dt><div><dd>x</dd></div></dt></dl>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<dd>x");
  const _t1 = _template("<dl><dt><div>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_child(_n2));
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

// the form element pointer is not a scope, so a `<table>` does not reset it
#[test]
fn descendant_closing_an_open_form_through_a_table() {
  let code = transform(
    "<form><table><tbody><tr><td><form>x</form></td></tr></tbody></table></form>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<form>x");
  const _t1 = _template("<form><table><tbody><tr><td>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_child(_child(_child(_n2))));
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

#[test]
fn descendant_closing_an_open_ruby_text() {
  let code = transform(
    "<ruby><rt>a</rt><rt><rt>x</rt></rt></ruby>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, next as _next, template as _template } from "vue";
  const _t0 = _template("<rt>x");
  const _t1 = _template("<ruby><rt>a</rt><rt>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _next(_child(_n2));
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

// a link in an integration point closes the one in the html ancestor
#[test]
fn descendant_closing_an_open_link_inside_foreign_object() {
  let code = transform(
    "<a><svg><foreignObject><a>x</a></foreignObject></svg></a>",
    Some(TransformOptions {
      vapor: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { child as _child, insert as _insert, template as _template } from "vue";
  const _t0 = _template("<a>x");
  const _t1 = _template("<a><svg><foreignObject>", 1);
  (() => {
  	const _n2 = _t1();
  	const _n1 = _child(_child(_n2));
  	const _n0 = _t0();
  	_insert(_n0, _n1);
  	return _n2;
  })();
  "#);
}

// `enter_open_scope` clears what the descendants can still reach: a `<button>`
// ends the button scope of the `<p>`, a `<section>` ends the list item search,
// and a foreign element ends every html scope.
#[test]
fn keeps_a_descendant_the_parser_would_not_move() {
  for (src, expected) in [
    ("<p><button><div>x</div></button></p>", "<p><button><div>x"),
    (
      "<ul><li><section><li>x</li></section></li></ul>",
      "<ul><li><section><li>x",
    ),
    (
      "<p><svg><foreignObject><div>x</div></foreignObject></svg></p>",
      "<p><svg><foreignObject><div>x",
    ),
    ("<dl><dd><dl><dd>x</dd></dl></dd></dl>", "<dl><dd><dl><dd>x"),
    (
      "<a><table><tbody><tr><td><a>x</a></td></tr></tbody></table></a>",
      "<a><table><tbody><tr><td><a>x",
    ),
    ("<ruby><span><rt>x</rt></span></ruby>", "<ruby><span><rt>x"),
  ] {
    let code = transform(
      src,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(code.matches("_template(").count(), 1, "{src}\n{code}");
    assert!(
      code.contains(&format!("_template(\"{expected}\"")),
      "{src}\n{code}"
    );
  }
}

// An end tag the parser cannot use to pop a special descendant ("any other end
// tag" is ignored at one, a formatting end tag runs the adoption agency around
// one) makes that descendant close itself, or the next sibling lands inside it.
#[test]
fn keeps_the_end_tag_of_a_special_child_its_parent_cannot_close() {
  for (src, expected) in [
    // "any other end tag" (legend, audio, rt, ...) walks the open elements and
    // is ignored at a special one, so a special child has to close itself or
    // the next sibling lands inside it
    (
      "<main><legend><div>x</div></legend><p>y</p></main>",
      "<main><legend><div>x</div></legend><p>y",
    ),
    (
      "<div><audio><p>a</p></audio><p>b</p></div>",
      "<div><audio><p>a</p></audio><p>b",
    ),
    (
      "<ruby><rt><div>x</div></rt><rt>y</rt></ruby>",
      "<ruby><rt><div>x</div></rt><rt>y",
    ),
    // an end tag with handling of its own pops its open descendants
    (
      "<main><button><div>x</div></button><p>y</p></main>",
      "<main><button><div>x</button><p>y",
    ),
    (
      "<main><object><div>x</div></object><p>y</p></main>",
      "<main><object><div>x</object><p>y",
    ),
    // a non-special child is closed along the way
    (
      "<main><legend><span>x</span></legend><p>y</p></main>",
      "<main><legend><span>x</legend><p>y",
    ),
    // the special child's own end tag pops whatever is still open below it
    (
      "<main><span><div><p>x</p></div></span><i>y</i></main>",
      "<main><span><div><p>x</div></span><i>y",
    ),
    // the requirement passes through a child that omits its own end tag
    (
      "<main><legend><span><div>x</div></span></legend><p>y</p></main>",
      "<main><legend><span><div>x</div></legend><p>y",
    ),
  ] {
    let code = transform(
      src,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(code.matches("_template(").count(), 1, "{src}\n{code}");
    assert!(
      code.contains(&format!("_template(\"{expected}\"")),
      "{src}\n{code}"
    );
  }
}

// A void element holds no children: the browser's parser puts them after it,
// while the compiler's tree has them inside. Each one therefore gets a template
// of its own and is inserted into the element at runtime, which is where vdom
// puts it too. `dom::is_valid_html_nesting` does not know about void parents.
#[test]
fn void_parent_holds_no_children() {
  for (source, templates) in [
    (
      "<div><hr><div>x</div></hr><i>s</i></div>",
      vec!["<div>x", "<div><hr><i>s"],
    ),
    (
      "<div><input><b>x</b></input><i>s</i></div>",
      vec!["<b>x</b>", "<div><input><i>s"],
    ),
    // a void root holds its children the same way, in order
    (
      "<hr><main>v</main><i>s</i></hr>",
      vec!["<main>v</main>", "<i>s", "<hr>"],
    ),
    // the placeholder of the child's own template still receives its text
    (
      "<div><hr><div>{msg}</div></hr><i>s</i></div>",
      vec!["<div> ", "<div><hr><i>s"],
    ),
  ] {
    let code = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(
      code.matches("_template(").count(),
      templates.len(),
      "{source}\n{code}"
    );
    for template in templates {
      assert!(
        code.contains(&format!("_template(\"{template}\"")),
        "{source}\n{code}"
      );
    }
  }
}

// upstream: `html breakout element in foreign content`. The parser moves an
// html element out of an `<svg>`/`<math>` subtree, while vdom creates it in the
// foreign namespace instead, so it is created imperatively there.
#[test]
fn html_breakout_element_in_foreign_content() {
  for (source, templates, ns) in [
    (
      "<svg><g><div>x</div></g></svg>",
      vec!["x", "<svg><g>"],
      Some(1),
    ),
    (
      "<math><mrow><div>x</div></mrow></math>",
      vec!["x", "<math><mrow>"],
      Some(2),
    ),
    // an html integration point takes html children as written
    (
      "<svg><foreignObject><div>x</div></foreignObject></svg>",
      vec!["<svg><foreignObject><div>x"],
      None,
    ),
  ] {
    let code = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert_eq!(
      code.matches("_template(").count(),
      templates.len(),
      "{source}\n{code}"
    );
    for template in templates {
      assert!(
        code.contains(&format!("_template(\"{template}\"")),
        "{source}\n{code}"
      );
    }
    match ns {
      Some(ns) => {
        assert!(
          code.contains(r#"_createPlainElement("div""#),
          "{source}\n{code}"
        );
        assert!(
          code.contains(&format!(", null, null, {ns})")),
          "{source}\n{code}"
        );
      }
      None => assert!(!code.contains("_createPlainElement"), "{source}\n{code}"),
    }
  }
}

// upstream: `text in a table part`. The parser foster-parents text other than
// whitespace out of a table part, so it is written as a placeholder and filled
// from the runtime. Upstream creates the text node itself instead, which needs a
// comment placeholder in the template.
#[test]
fn text_in_a_table_part() {
  for (source, template, sets_text) in [
    (
      "<table><tbody>foo<tr><td>x</td></tr></tbody></table>",
      "<table><tbody> <tr><td>x",
      true,
    ),
    (
      "<table><tbody><tr><td>x</td></tr>tail</tbody></table>",
      "<table><tbody><tr><td>x</tr> ",
      true,
    ),
    // whitespace stays where it is
    (
      "<table><tbody><tr><td>x</td></tr> <tr><td>y</td></tr></tbody></table>",
      "<table><tbody><tr><td>x</tr> <tr><td>y",
      false,
    ),
  ] {
    let code = transform(
      source,
      Some(TransformOptions {
        vapor: true,
        ..Default::default()
      }),
    )
    .code;
    assert!(
      code.contains(&format!("_template({template:?}")),
      "`{source}` should compile to a template starting with {template:?}, got:\n{code}"
    );
    assert_eq!(code.contains("_setNodes("), sets_text, "{source}\n{code}");
  }
}
