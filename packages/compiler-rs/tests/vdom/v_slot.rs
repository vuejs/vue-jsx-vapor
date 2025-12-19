use common::options::TransformOptions;
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn v_slot_basic() {
  let code = transform(
    r#"<Comp>
      <template v-for={(Slot, slotName) in slots} v-slot:$slotName$={scope}>
        <Slot {...scope} />
      </template>
      <template v-if={foo} v-slot:foo>
        <div />
      </template>
      <template v-else-if={bar} v-slot:bar>
        bar
      </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, createSlots as _createSlots, createTextVNode as _createTextVNode, createVNode as _createVNode, guardReactiveProps as _guardReactiveProps, normalizeProps as _normalizeProps, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [_renderList(slots, (Slot, slotName) => ({
      name: slotName,
      fn: (scope) => [_createVNode(Slot, _normalizeProps(_guardReactiveProps(scope)), null, 16)]
    })), foo ? {
      name: foo,
      fn: () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))],
      key: 0
    } : bar ? {
      name: bar,
      fn: () => [_cache[1] || (_cache[1] = _createTextVNode("bar", -1))],
      key: 1
    } : undefined]), 1024);
  })();
  "#);
}
