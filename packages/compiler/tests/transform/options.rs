use compiler::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
pub fn runtime_module_name() {
  let code = transform(
    "<div>{foo}</div>",
    Some(TransformOptions {
      vapor: true,
      runtime_module_name: Some(String::from("vue-jsx-vapor")),
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "vue-jsx-vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo);
  	return _n0;
  })();
  "#);
}

#[test]
pub fn filename() {
  let code = transform(
    "<div>{foo}</div>",
    Some(TransformOptions {
      vapor: true,
      filename: "routes/index.tsx?tsr-split=component",
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _t0();
  	const _x0 = _txt(_n0);
  	_setNodes(_x0, () => foo);
  	return _n0;
  })();
  "#);
}

#[test]
pub fn optimize_slots() {
  let code = transform(
    "<Comp>{foo}</Comp>",
    Some(TransformOptions {
      vapor: false,
      optimize: false,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeSlots as _normalizeSlots } from "/vue-jsx-vapor/vdom";
  import { createVNode as _createVNode } from "vue";
  _createVNode(Comp, null, _normalizeSlots(foo));
  "#);
}

#[test]
pub fn merge_props() {
  let code = transform(
    "<Comp {...foo} onClick={() => {}} />",
    Some(TransformOptions {
      vapor: false,
      optimize: false,
      merge_props: false,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNode as _createVNode } from "vue";
  _createVNode(Comp, Object.assign({}, foo, { onClick: () => {} }));
  "#);
}

#[test]
pub fn merge_props_object() {
  let code = transform(
    "<Comp {...{ onClick: () => {} }} onClick={() => {}} />",
    Some(TransformOptions {
      vapor: false,
      optimize: false,
      merge_props: false,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNode as _createVNode } from "vue";
  _createVNode(Comp, Object.assign({ onClick: () => {} }, { onClick: () => {} }));
  "#);
}

#[test]
fn merge_props_with_v_if() {
  let code = transform(
    r#"<button {...foo} v-if={true}></button>"#,
    Some(TransformOptions {
      vapor: false,
      merge_props: false,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, guardReactiveProps as _guardReactiveProps, normalizeProps as _normalizeProps, openBlock as _openBlock } from "vue";
  true ? (_openBlock(), _createElementBlock("button", Object.assign({ key: 0 }, foo), null, 16)) : _createCommentVNode("", true);
  "#)
}
