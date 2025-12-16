use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_text() {
  let code = transform(
    r#"<div v-text={foo}></div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, toDisplayString as _toDisplayString } from "vue";
  const _hoisted_1 = ["textContent"];
  (() => {
    return _openBlock(), _createElementBlock("div", { textContent: _toDisplayString(foo) }, null, 8, _hoisted_1);
  })();
  "#)
}
