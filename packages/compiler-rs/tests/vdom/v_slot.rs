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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, {
      default: () => [_cache[0] || (_cache[0] = _createElementVNode("div", null, null, -1))],
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
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: ({ foo }) => [foo, bar],
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
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      named: ({ foo }) => [foo, bar],
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
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      one: ({ foo }) => [foo, bar],
      two: ({ bar }) => [foo, bar],
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
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      [named]: ({ foo }) => [
        foo,
        bar,
        _createVNode(Comp, null, {
          default: (baz) => [bar],
          _: 1
        })
      ],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, {
      one: () => [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))],
      default: () => [..._cache[1] || (_cache[1] = [_createTextVNode("bar", -1), _createElementVNode("span", null, null, -1)])],
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
  import { createBlock as _createBlock, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      [one]: ({ foo }) => [foo, bar],
      [two]: ({ bar }) => [foo, bar],
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
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: ({ foo }) => [
        _createVNode(Inner, null, {
          default: ({ bar }) => [
            foo,
            bar,
            baz
          ],
          _: 2
        }, 1024),
        foo,
        bar,
        baz
      ],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createTextVNode as _createTextVNode, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      default: (bar) => [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))],
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
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      default: (bar) => [i],
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
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: (foo) => [_createVNode(Comp, null, {
        default: (bar) => [bar],
        _: 1
      })],
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
  import { createBlock as _createBlock, createVNode as _createVNode, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, {
      default: (foo) => [_createVNode(Comp, null, {
        default: (bar) => [foo],
        _: 2
      }, 1024)],
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
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  const _hoisted_1 = ["onClick"];
  (() => {
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      default: (bar) => [_createElementVNode("button", { onClick: fn(i) }, null, 8, _hoisted_1)],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createTextVNode as _createTextVNode, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, { i }, {
      default: () => [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))],
      _: 1
    }, 8, ["i"])]))), 256);
  })();
  "#);
}

#[test]
fn should_only_force_dynamic_slots_when_actually_using_scope_vars6() {
  let code = transform(
    r#"<div v-for={i in list}>
      <Comp v-slot:$i$><button onClick={fn()} /></Comp>
    </div>"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(true), _createBlock(_Fragment, null, _renderList(list, (i) => (_openBlock(), _createElementBlock("div", null, [_createVNode(Comp, null, {
      $i$: () => [_createElementVNode("button", { onClick: _cache[0] || (_cache[0] = fn()) })],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createSlots as _createSlots, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _createTextVNode("hello", -1))],
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
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [props],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createSlots as _createSlots, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))],
      key: 0
    } : orNot ? {
      name: two,
      fn: (props) => [_cache[1] || (_cache[1] = _createTextVNode("bar", -1))],
      key: 1
    } : {
      name: one,
      fn: () => [_cache[2] || (_cache[2] = _createTextVNode("baz", -1))],
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
  import { createBlock as _createBlock, createSlots as _createSlots, openBlock as _openBlock, renderList as _renderList } from "vue";
  (() => {
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [_renderList(list, (name) => ({
      name,
      fn: () => [name]
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, {
      header: () => [_cache[0] || (_cache[0] = _createTextVNode(" Header ", -1))],
      default: () => [_cache[1] || (_cache[1] = _createTextVNode(" Default ", -1))],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, {
      header: () => [_cache[0] || (_cache[0] = _createTextVNode(" Header ", -1))],
      default: () => [_cache[1] || (_cache[1] = _createElementVNode("p", null, null, -1))],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, {
      header: () => [_cache[0] || (_cache[0] = _createTextVNode(" Header ", -1))],
      default: () => [_cache[1] || (_cache[1] = _createTextVNode("&nbsp;", -1))],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createSlots as _createSlots, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))],
      key: 0
    } : {
      name: two,
      fn: () => [_cache[1] || (_cache[1] = _createTextVNode("baz", -1))],
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
  import { useVdomCache as _useVdomCache } from "vue-jsx-vapor";
  import { createBlock as _createBlock, createSlots as _createSlots, createTextVNode as _createTextVNode, openBlock as _openBlock } from "vue";
  (() => {
    const _cache = _useVdomCache();
    return _openBlock(), _createBlock(Comp, null, _createSlots({ _: 2 }, [ok ? {
      name: one,
      fn: () => [_cache[0] || (_cache[0] = _createTextVNode("foo", -1))],
      key: 0
    } : {
      name: two,
      fn: () => [_cache[1] || (_cache[1] = _createTextVNode("baz", -1))],
      key: 1
    }]), 1024);
  })();
  "#);
}
