use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    r#"<div v-for={{ x, y } in list} key={x}>
      <span>foobar</span>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, ({ x, y }) => (_openBlock(), _createElementBlock("div", { key: x }, [_createElementVNode("span", null, "foobar")]))), 128);
  })();
  "#);
}
