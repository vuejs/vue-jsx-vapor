use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_memo() {
  let code = transform(
    r#"<div v-memo={foo}>{foo}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock, withMemo as _withMemo } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _withMemo(foo, () => (_openBlock(), _createElementBlock("div", null, foo)), _cache, 0);
  })();
  "#)
}

#[test]
fn v_memo_with_v_for() {
  let code = transform(
    r#"<div v-for={i in list} v-memo={i}>{foo}</div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, isMemoSame as _isMemoSame, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i, __, ___, _cached) => {
      const _memo = i;
      if (_cached && _isMemoSame(_cached, _memo)) return _cached;
      const _item = (_openBlock(), _createElementBlock("div", null, foo));
      _item.memo = _memo;
      return _item;
    }, _cache, 0), 256);
  })();
  "#)
}
