use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn implicit_default_slot() {
  let code = transform(
    r#"<Comp><div/></Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx(() => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn on_component_default_slot() {
  let code = transform(
    r#"<Comp v-slot={{ foo }}>{ foo }{ bar }</Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx(({ foo }) => [_normalizeVNode(() => foo, 1), _normalizeVNode(() => bar, 1)]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn on_component_named_slot() {
  let code = transform(
    r#"<Comp v-slot:named={{ foo }}>{ foo }{ bar }</Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      named: _withCtx(({ foo }) => [_normalizeVNode(() => foo, 1), _normalizeVNode(() => bar, 1)]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn template_named_slots() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one={{ foo }}>
        { foo }{ bar }
      </template>
      <template v-slot:two={{ bar }}>
        { foo }{ bar }
      </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      one: _withCtx(({ foo }) => [_normalizeVNode(() => foo, 1), _normalizeVNode(() => bar, 1)]),
      two: _withCtx(({ bar }) => [_normalizeVNode(() => foo, 1), _normalizeVNode(() => bar, 1)]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn on_component_dynamically_named_slot() {
  let code = transform(
    r#"<Comp>
      <template v-slot:$named$={{ foo }}>
        { foo }{ bar }
        <Comp v-slot={baz}>{bar}</Comp>
      </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      [named]: _withCtx(({ foo }) => [
        _normalizeVNode(() => foo, 1),
        _normalizeVNode(() => bar, 1),
        _createVNode(Comp, null, {
          default: _withCtx((baz) => [_normalizeVNode(() => bar, 1)]),
          _: 1
        })
      ]),
      _: 2
    }, 1024);
  })();
  "#);
}

#[test]
fn named_slots_with_implicit_default_slot() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one>foo</template>bar<span/>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, {
      one: _withCtx(() => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))]),
      default: _withCtx(() => [..._cache[1] || (_cache[1] = [_normalizeVNode("bar", -1), _createElementVNode("span", null, null, -1)])]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn dynamically_named_slots() {
  let code = transform(
    r#"<Comp>
      <template v-slot:$one$={{ foo }}>
        { foo }{ bar }
      </template>
      <template v-slot:$two$={{ bar }}>
        { foo }{ bar }
      </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      [one]: _withCtx(({ foo }) => [_normalizeVNode(() => foo, 1), _normalizeVNode(() => bar, 1)]),
      [two]: _withCtx(({ bar }) => [_normalizeVNode(() => foo, 1), _normalizeVNode(() => bar, 1)]),
      _: 2
    }, 1024);
  })();
  "#);
}

#[test]
fn nested_slots_scoping() {
  let code = transform(
    r#"<Comp>
      <template v-slot:default={{ foo }}>
        <Inner v-slot={{ bar }}>
          { foo }{ bar }{ baz }
        </Inner>
        { foo }{ bar }{ baz }
      </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx(({ foo }) => [
        _createVNode(Inner, null, {
          default: _withCtx(({ bar }) => [
            _normalizeVNode(() => foo, 1),
            _normalizeVNode(() => bar, 1),
            _normalizeVNode(() => baz, 1)
          ]),
          _: 2
        }, 1024),
        _normalizeVNode(() => foo, 1),
        _normalizeVNode(() => bar, 1),
        _normalizeVNode(() => baz, 1)
      ]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn should_force_dynamic_when_inside_v_for() {
  let code = transform(
    r#"<div v-for={i in list}>
      <Comp v-slot={bar}>foo</Comp>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      default: _withCtx((bar) => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))]),
      _: 1
    })]))), 256);
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars1() {
  let code = transform(
    r#"<div v-for={i in list}>
      <Comp v-slot={bar}>{i}</Comp>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      default: _withCtx((bar) => [_normalizeVNode(() => i, 1)]),
      _: 2
    }, 1024)]))), 256);
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars2() {
  // reference the component's own slot variable should not force dynamic slots
  let code = transform(
    r#"<Comp v-slot={foo}>
      <Comp v-slot={bar}>{bar}</Comp>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx((foo) => [_createVNode(Comp, null, {
        default: _withCtx((bar) => [_normalizeVNode(() => bar, 1)]),
        _: 1
      })]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars3() {
  let code = transform(
    r#"<Comp v-slot={foo}>
      <Comp v-slot={bar}>{foo}</Comp>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: _withCtx((foo) => [_createVNode(Comp, null, {
        default: _withCtx((bar) => [_normalizeVNode(() => foo, 1)]),
        _: 2
      }, 1024)]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars4() {
  let code = transform(
    r#"<div v-for={i in list}>
      <Comp v-slot={bar}><button onClick={fn(i)} /></Comp>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList, withCtx as _withCtx } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      default: _withCtx((bar) => [_createElementVNode("button", { onClick: fn(i) }, null, 8, _hoisted_1)]),
      _: 2
    }, 1024)]))), 256);
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars5() {
  let code = transform(
    r#"<div v-for={i in list}>
      <Comp i={i}>foo</Comp>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, { i }, {
      default: _withCtx(() => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))]),
      _: 1
    }, 8, ["i"])]))), 256);
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars6() {
  let code = transform(
    r#"<div v-for={i in list}>
      <Comp v-slot:$i_value$><button onClick={fn()} /></Comp>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vnode";
  import { Fragment as _Fragment, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(true), _createElementBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      [i.value]: _withCtx(() => [_createElementVNode("button", { onClick: _cache[0] || (_cache[0] = fn()) })]),
      _: 2
    }, 1024)]))), 256);
  })();
  "#);
}

#[test]
fn named_slot_with_v_if() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one v-if={ok}>hello</template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _normalizeVNode("hello", -1))],
      key: 0
    } : undefined]), 1024);
  })();
  "#);
}

#[test]
fn named_slot_with_v_if2() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one v-if={ok}>{props}</template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_normalizeVNode(() => props, 1)],
      key: 0
    } : undefined]), 1024);
  })();
  "#);
}

#[test]
fn named_slot_with_v_if_v_else_if_v_else() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one v-if={ok}>foo</template>
      <template v-slot:two={props} v-else-if={orNot}>bar</template>
      <template v-slot:one v-else>baz</template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))],
      key: 0
    } : orNot ? {
      name: two,
      fn: (props) => [_cache[1] || (_cache[1] = _normalizeVNode("bar", -1))],
      key: 1
    } : {
      name: one,
      fn: () => [_cache[2] || (_cache[2] = _normalizeVNode("baz", -1))],
      key: 2
    }]), 1024);
  })();
  "#);
}

#[test]
fn named_slot_with_v_for() {
  let code = transform(
    r#"<Comp>
      <template v-for={name in list} v-slot:$name$>{ name }</template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [_renderList(list, (name) => ({
      name,
      fn: () => [_normalizeVNode(() => name, 1)]
    }))]), 1024);
  })();
  "#);
}

#[test]
fn error_on_extraneous_children_with_named_default_slot() {
  let error = RefCell::new(None);
  transform(
    "<Comp><template v-slot:default>foo</template>bar</Comp>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(
    *error.borrow(),
    Some(ErrorCodes::VSlotExtraneousDefaultSlotChildren)
  );
}

#[test]
fn error_on_duplicated_slot_names() {
  let error = RefCell::new(None);
  transform(
    "<Comp><template v-slot:foo></template><template v-slot:foo></template></Comp>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotDuplicateSlotNames));
}

#[test]
fn error_on_invalid_mixed_slot_usage() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slot={foo}><template v-slot:foo></template></Comp>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMixedSlotUsage));
}

#[test]
fn error_on_v_slot_usage_on_plain_elements() {
  let error = RefCell::new(None);
  transform(
    "<div v-slot/>",
    Some(TransformOptions {
      interop: true,
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMisplaced));
}

#[test]
fn named_default_slot_with_implicit_whitespace_content() {
  let code = transform(
    r#"<Comp>
      <template v-slot:header> Header </template>
      <template v-slot:default> Default </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, {
      header: _withCtx(() => [_cache[0] || (_cache[0] = _normalizeVNode(" Header ", -1))]),
      default: _withCtx(() => [_cache[1] || (_cache[1] = _normalizeVNode(" Default ", -1))]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn implicit_default_slot_with_whitespace() {
  let code = transform(
    r#"<Comp>
      <template v-slot:header> Header </template>
      <p/>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, {
      header: _withCtx(() => [_cache[0] || (_cache[0] = _normalizeVNode(" Header ", -1))]),
      default: _withCtx(() => [_cache[1] || (_cache[1] = _createElementVNode("p", null, null, -1))]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn implicit_default_slot_with_non_breaking_space() {
  let code = transform(
    r#"<Comp>
      &nbsp;
      <template v-slot:header> Header </template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, {
      header: _withCtx(() => [_cache[0] || (_cache[0] = _normalizeVNode(" Header ", -1))]),
      default: _withCtx(() => [_cache[1] || (_cache[1] = _normalizeVNode("&nbsp;", -1))]),
      _: 1
    });
  })();
  "#);
}

#[test]
fn named_slot_with_v_if_v_else() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one v-if={ok}>foo</template>
      <template v-slot:two v-else>baz</template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))],
      key: 0
    } : {
      name: two,
      fn: () => [_cache[1] || (_cache[1] = _normalizeVNode("baz", -1))],
      key: 1
    }]), 1024);
  })();
  "#);
}

#[test]
fn named_slot_with_v_if_v_else_and_comments() {
  let code = transform(
    r#"<Comp>
      <template v-slot:one v-if={ok}>foo</template>
      {/* start */}

      {/* end */}
      <template v-slot:two v-else>baz</template>
    </Comp>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vnode";
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _createVNodeCache(0);
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _normalizeVNode("foo", -1))],
      key: 0
    } : {
      name: two,
      fn: () => [_cache[1] || (_cache[1] = _normalizeVNode("baz", -1))],
      key: 1
    }]), 1024);
  })();
  "#);
}
