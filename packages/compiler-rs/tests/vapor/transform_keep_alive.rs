use compiler_rs::transform;

// vuejs/core#15583: a `<KeepAlive>` root is transparent like a `<Transition>`
// one, so its child is compiled as the root of the component.
#[test]
fn propagates_component_root_to_keep_alive_child_with_dynamic_class() {
  let code = transform(
    r#"<VaporKeepAlive>
      <div class={["box", data.value.internalClass]}>child</div>
    </VaporKeepAlive>"#,
    None,
  )
  .code;

  assert!(code.contains(r#"const _t0 = _template("<div>child", 1);"#));
  assert!(code.contains(r#"_setClass(_n0, ["box", data.value.internalClass])"#));
}

// the non `Vapor` prefixed tag is the same host
#[test]
fn propagates_component_root_to_keep_alive_component_child() {
  let code = transform(r#"<KeepAlive><Child class={cls} /></KeepAlive>"#, None).code;

  assert!(code.contains(r#"_createComponent(Child, { class: () => cls }, null, true)"#));
}

#[test]
fn propagates_component_root_into_keep_alive_if_branches() {
  let code = transform(
    r#"<KeepAlive><A v-if={ok} /><B v-else /></KeepAlive>"#,
    None,
  )
  .code;

  assert!(code.contains(r#"_createComponent(A, null, null, true)"#));
  assert!(code.contains(r#"_createComponent(B, null, null, true)"#));
}

#[test]
fn does_not_propagate_component_root_through_nested_keep_alive() {
  let code = transform(
    r#"<section>
      <VaporKeepAlive>
        <div>child</div>
      </VaporKeepAlive>
    </section>"#,
    None,
  )
  .code;

  assert!(code.contains(r#"const _t0 = _template("<div>child", 2);"#));
}
