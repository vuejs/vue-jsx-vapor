use compiler_rs::transform::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn component_import_resolve_component() {
  let code = transform(
    "<Foo/>",
    Some(TransformOptions {
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn component_resolve_namespaced_component() {
  let code = transform("<Foo.Example/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_() {
  let code = transform("", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_generate_single_root_component() {
  let code = transform("<Comp/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_generate_multi_root_component() {
  let code = transform("<><Comp/>123</>", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_fragment_should_not_mark_as_single_root() {
  let code = transform("<><Comp/></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_v_for_should_not_mark_as_single_root() {
  let code = transform("<Comp v-for={item in items} key={item}/>", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_static_props() {
  let code = transform("<Foo id=\"foo\" class=\"bar\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_dynamic_props() {
  let code = transform("{...obj}", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_dynamic_props_after_static_prop() {
  let code = transform("<Foo id=\"foo\" {...obj} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_dynamic_props_before_static_prop() {
  let code = transform("<Foo {...obj} id=\"foo\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_dynamic_props_between_static_prop() {
  let code = transform("<Foo id=\"foo\" {...obj} class=\"bar\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_props_merging_style() {
  let code = transform(
    "<Foo style=\"color: green\" style={{ color: 'red' }} />",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn component_props_merging_class() {
  let code = transform("<Foo class=\"foo\" class={{ bar: isBar }} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_v_on() {
  let code = transform("<Foo v-on={obj} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_event_with_once_modifier() {
  let code = transform("<Foo onFoo_once={bar} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn component_with_fallback() {
  let code = transform("<foo-bar />", None).code;
  assert_snapshot!(code);
}

#[test]
fn static_props() {
  let code = transform("<div id=\"foo\" class=\"bar\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn props_children() {
  let code = transform("<div id=\"foo\"><span/></div>", None).code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_props() {
  let code = transform("<div {...obj} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_props_after_static_prop() {
  let code = transform("<div id=\"foo\" {...obj} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_props_before_static_prop() {
  let code = transform("<div {...obj} id=\"foo\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_props_between_static_prop() {
  let code = transform("<div id=\"foo\" {...obj} class=\"bar\" />", None).code;
  assert_snapshot!(code);
}

#[test]
fn props_merging_event_handlers() {
  let code = transform("<div onClick_foo={a} onClick_bar={b} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn props_merging_style() {
  let code = transform(
    "<div style=\"color: green\" style={{ color: 'red' }} />",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn props_merging_class() {
  let code = transform("<div class=\"foo\" class={{ bar: isBar }} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn v_on() {
  let code = transform("<div v-on={obj} />", None).code;
  assert_snapshot!(code);
}

#[test]
fn invalid_html_nesting() {
  let code = transform(
    "<><p><div>123</div></p>
    <form><form/></form></>",
    None,
  )
  .code;
  assert_snapshot!(code);
}
