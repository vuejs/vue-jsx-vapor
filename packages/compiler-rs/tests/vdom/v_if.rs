use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_if_basic() {
  let code = transform(
    r#"<>
      <template v-if={foo}>
        <div foo={foo}>foo</div>
      </template>

      <span v-if={foo}></span>
      <span v-else-if={bar}></span>
      <a v-else></a>
    </>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
    return _createVNode(_Fragment, null, [(_openBlock(), _createBlock(_Fragment, null, foo ? (_openBlock(), _createBlock(_Fragment, { key: 0 }, [_createElementVNode("div", { foo }, "foo", 8, ["foo"])])) : _createCommentVNode("", true))), foo ? (_openBlock(), _createElementBlock("span", { key: 1 })) : bar ? (_openBlock(), _createElementBlock("span", { key: 2 })) : (_openBlock(), _createElementBlock("a", { key: 3 }))]);
  })();
  "#);
}
