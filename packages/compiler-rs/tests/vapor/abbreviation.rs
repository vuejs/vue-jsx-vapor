use compiler_rs::transform;
use insta::assert_snapshot;

// basic - last child can omit closing tag
#[test]
fn template_abbreviation() {
  let code = transform("<div>hello</div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>hello", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn template_abbreviation1() {
  let code = transform("<div><div>hello</div></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>hello", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// non-last child needs closing tag
#[test]
fn template_abbreviation2() {
  let code = transform("<div><span>foo</span><span></span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span>foo</span><span>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn template_abbreviation3() {
  let code = transform("<div><hr/><div></div></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><hr><div>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn template_abbreviation4() {
  let code = transform("<div><div></div><hr/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div></div><hr>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// multi-root: each root generates its own template
#[test]
fn template_abbreviation5() {
  let code = transform("<><span></span>hello</>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<span>");
  const _t1 = _template("hello");
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
  let code = transform("<div><b>bold</b></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><b>bold", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn formatting_tags1() {
  let code = transform("<div><i><b>text</b></i></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><i><b>text", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// formatting tags NOT on rightmost path need closing tag
#[test]
fn formatting_tags2() {
  let code = transform("<div><b>bold</b><span></span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><b>bold</b><span>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn formatting_tags3() {
  let code = transform("<div><b>1</b><b>2</b></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><b>1</b><b>2", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// same-name on rightmost path can omit
#[test]
fn same_name_nested_tags() {
  let code = transform("<div><div>inner</div></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>inner", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// same-name NOT on rightmost path needs closing tag
#[test]
fn same_name_nested_tags1() {
  let code = transform("<div><div>a</div><div>b</div></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>a</div><div>b", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn same_name_nested_tags2() {
  let code = transform("<span><span>1</span><span>2</span></span>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<span><span>1</span><span>2", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// void tags never need closing tags
#[test]
fn void_tags() {
  let code = transform("<div><br/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><br>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn void_tags1() {
  let code = transform("<div><hr/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><hr>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn void_tags2() {
  let code = transform("<div><input/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><input>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn void_tags3() {
  let code = transform("<div><img/></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><img>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn deeply_nested() {
  let code = transform("<div><div><div><span>deep</span></div></div></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><div><span>deep", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn deeply_nested1() {
  let code = transform("<div><div><span>a</span><span>b</span></div></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><span>a</span><span>b", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// button always needs closing tag unless on rightmost path
#[test]
fn always_close_tags() {
  let code = transform("<div><button>click</button></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><button>click", true);
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><button>click</button><span>sibling", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// select always needs closing tag unless rightmost
#[test]
fn always_close_tags2() {
  let code = transform("<div><select></select></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><select>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags3() {
  let code = transform("<div><select></select><span>sibling</span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><select></select><span>sibling", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// table always needs closing tag unless rightmost
#[test]
fn always_close_tags4() {
  let code = transform("<div><table></table></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><table>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags5() {
  let code = transform("<div><table></table><span>sibling</span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><table></table><span>sibling", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// textarea always needs closing tag unless rightmost
#[test]
fn always_close_tags6() {
  let code = transform("<div><textarea></textarea></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><textarea>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags7() {
  let code = transform("<div><textarea></textarea><span>sibling</span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><textarea></textarea><span>sibling", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// template always needs closing tag unless rightmost
#[test]
fn always_close_tags8() {
  let code = transform("<div><template></template></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><template>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags9() {
  let code = transform("<div><template></template><span>sibling</span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><template></template><span>sibling", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// script always needs closing tag unless rightmost
#[test]
fn always_close_tags10() {
  let code = transform("<div><script></script></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><script>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn always_close_tags11() {
  let code = transform("<div><script></script><span>sibling</span></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><script><\/script><span>sibling", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// without always-close elements, normal abbreviation should work
#[test]
fn always_close_tags12() {
  let code = transform("<div><form><input/></form></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><form><input>", true);
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
  let code = transform("<div><span><div>text</div></span><p>after</p></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><div>text</div></span><p>after", true);
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><p>text</p></span><span>after", true);
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><div>text", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

// Normal case - no inline/block issue
#[test]
fn inline_block_ancestor_relationships3() {
  let code = transform("<div><p>text</p></div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><p>text", true);
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div><p>text</div><span>after", true);
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><b><div>text</div></b></span><p>after", true);
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
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><span><div><span><div>text</div></div></span><p>after", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}
