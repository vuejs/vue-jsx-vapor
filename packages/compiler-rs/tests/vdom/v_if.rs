use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_if_basic() {
  let code = transform(
    r#"<div>
      <template v-if={foo}>
        <div foo={foo}>foo</div>
      </template>

      <span v-if={foo}></span>
      <span v-else-if={bar}></span>
      <a v-else></a>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      with_fallback: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, resolveComponent as _resolveComponent } from "vue";
  (() => {
    const _cache = _useVdomCache();
    const _component__Fragment = _resolveComponent("_Fragment");
    return _openBlock(), _createElementBlock("div", null, [(_openBlock(), _createElementBlock("_Fragment", null, foo ? (_openBlock(), _createBlock(_component__Fragment, { key: 0 }, [_createElementVNode("div", { foo }, "foo", 8, ["foo"])])) : _createCommentVNode("", true))), (_openBlock(), _createElementBlock("_Fragment", null, foo ? (_openBlock(), _createElementBlock("span", { key: 1 })) : bar ? (_openBlock(), _createElementBlock("span", { key: 2 })) : (_openBlock(), _createElementBlock("a", { key: 3 }))))]);
  })();
  "#);
}
