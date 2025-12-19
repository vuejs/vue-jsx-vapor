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
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["foo"];
  const _hoisted_2 = { key: 1 };
  const _hoisted_3 = { key: 2 };
  const _hoisted_4 = { key: 3 };
  (() => {
    return _openBlock(), _createBlock(_Fragment, null, [(_openBlock(), _createBlock(_Fragment, null, foo ? (_openBlock(), _createBlock(_Fragment, { key: 0 }, [_createElementVNode("div", { foo }, "foo", 8, _hoisted_1)])) : _createCommentVNode("", true))), foo ? (_openBlock(), _createElementBlock("span", _hoisted_2)) : bar ? (_openBlock(), _createElementBlock("span", _hoisted_3)) : (_openBlock(), _createElementBlock("a", _hoisted_4))], 64);
  })();
  "#);
}
