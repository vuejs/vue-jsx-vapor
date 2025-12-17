use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_once() {
  let code = transform(
    r#"<div v-once>{foo}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementVNode as _createElementVNode, setBlockTracking as _setBlockTracking } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _cache[0] || (_setBlockTracking(-1, true), (_cache[0] = _createElementVNode("div", null, foo)).cacheIndex = 0, _setBlockTracking(1), _cache[0]);
  })();
  "#)
}
