use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_slot_basic() {
  let code = transform(
    r#"<Comp v-slots={{ default: () => <>{<span/>}</> }}></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, { default: () => (() => {
      return _createVNode(_Fragment, null, [(() => {
        return _openBlock(), _createElementBlock("span");
      })()]);
    })() }, 1024);
  })();
  "#);
}
