use compiler_rs::{TransformOptions, transform};

fn assert_lowered(mapped: &str, directive: &str) {
  let code = transform(mapped, None).code;
  assert!(code.contains("_createMappedFor("), "{code}");
  let helper_import = code
    .lines()
    .find(|line| line.starts_with("import ") && line.contains("createMappedFor"))
    .unwrap();
  assert!(
    helper_import.ends_with("from \"/vue-jsx-vapor/vapor\";"),
    "{code}"
  );
  assert!(!code.contains(".map("), "{code}");
  assert!(!code.contains("_createKeyedFragment("), "{code}");
  let body = |code: String| {
    code
      .lines()
      .filter(|line| !line.starts_with("import "))
      .collect::<Vec<_>>()
      .join("\n")
      .replace("_createMappedFor", "_createFor")
  };
  assert_eq!(body(code), body(transform(directive, None).code));
}

#[test]
fn keyed_map_uses_v_for_reconciliation_and_fast_paths() {
  assert_lowered(
    "<ul>{items.map(item => <li key={item.id}>{item.name}</li>)}</ul>",
    "<ul><li v-for={item in items} key={item.id}>{item.name}</li></ul>",
  );
}

#[test]
fn index_alias_and_single_return_block() {
  assert_lowered(
    "<ul>{items.map((item, index) => { return (<li key={item.id}>{index}: {item.name}</li>); })}</ul>",
    "<ul><li v-for={(item, index) in items} key={item.id}>{index}: {item.name}</li></ul>",
  );
}

#[test]
fn adjacent_text_and_element_siblings_keep_insertion_order() {
  assert_lowered(
    "<ul>before{items.map(item => <li key={item.id}>{item.name}</li>)}after<b>end</b>{tail}</ul>",
    "<ul>before<li v-for={item in items} key={item.id}>{item.name}</li>after<b>end</b>{tail}</ul>",
  );
}

#[test]
fn multiple_maps_and_fragment_roots() {
  assert_lowered(
    "<>{a.map(item => <i key={item.id}>{item.name}</i>)}{b.map(item => <b key={item.id}>{item.name}</b>)}</>",
    "<><i v-for={item in a} key={item.id}>{item.name}</i><b v-for={item in b} key={item.id}>{item.name}</b></>",
  );
}

#[test]
fn nested_maps_and_shadowed_callback_parameters() {
  assert_lowered(
    "<ul>{rows.map(row => <li key={row.id}>{row.items.map(row => <b key={row.id}>{row.name}</b>)}</li>)}</ul>",
    "<ul><li v-for={row in rows} key={row.id}><b v-for={row in row.items} key={row.id}>{row.name}</b></li></ul>",
  );
}

#[test]
fn component_rows_keep_getters_and_event_handler_scope() {
  assert_lowered(
    "<div>{items.map(item => <Comp key={item.id} value={item.name} onClick={() => remove(item)} />)}</div>",
    "<div><Comp v-for={item in items} key={item.id} value={item.name} onClick={() => remove(item)} /></div>",
  );
}

#[test]
fn selection_bindings_use_create_selector() {
  let mapped = "<ul>{items.map(item => <li key={item.id} class={{ selected: item.id === selected.value }}>{item.name}</li>)}</ul>";
  assert_lowered(
    mapped,
    "<ul><li v-for={item in items} key={item.id} class={{ selected: item.id === selected.value }}>{item.name}</li></ul>",
  );
  assert!(transform(mapped, None).code.contains("_createSelector("));
}

#[test]
fn filtered_sources_and_nested_handler_locals() {
  assert_lowered(
    "<ul>{items.filter(visible).map(item => <li key={item.id} onClick={(item) => handle(item)}>{item.name}</li>)}</ul>",
    "<ul><li v-for={item in items.filter(visible)} key={item.id} onClick={(item) => handle(item)}>{item.name}</li></ul>",
  );
}

#[test]
fn v_once_uses_the_existing_list_handling() {
  assert_lowered(
    "<ul v-once>{items.map(item => <li key={item.id}>{item.name}</li>)}</ul>",
    "<ul v-once><li v-for={item in items} key={item.id}>{item.name}</li></ul>",
  );
}

#[test]
fn template_refs_keep_the_existing_map_semantics() {
  for row in [
    "<li key={item.id} ref={elements} />",
    "<li key={item.id}><span ref={elements} /></li>",
    "<Comp key={item.id} content={<span ref={elements} />} />",
  ] {
    let code = transform(&format!("<ul>{{items.map(item => {row})}}</ul>"), None).code;
    assert!(!code.contains("_createMappedFor("), "{code}");
    assert!(code.contains(".map("), "{code}");
    assert!(!code.contains("elements, true"), "{code}");
  }
}

#[test]
fn typed_parameters_do_not_become_part_of_the_alias() {
  let code = transform(
    "<ul>{items.map((item: Item, index: number) => <li key={item.id}>{index}:{item.name}</li>)}</ul>",
    Some(TransformOptions { filename: "index.tsx", ..Default::default() }),
  ).code;
  assert!(code.contains("(item, index) => item.id"), "{code}");
  assert!(code.contains("_for_key0.value"), "{code}");
  assert!(code.contains("_for_item0.value.name"), "{code}");
  assert!(!code.contains(".map("), "{code}");
}

#[test]
fn maps_that_need_javascript_evaluation_are_not_lowered() {
  for expression in [
    "items.map(item => <li>{item.name}</li>)",
    "items.map(({ id, name }) => <li key={id}>{name}</li>)",
    "items.map((item = fallback) => <li key={item.id} />)",
    "items.map((item, index, array) => <li key={item.id}>{array.length}</li>)",
    "items.map((...args) => <li key={args[0].id} />)",
    "items.map(item => { log(item); return <li key={item.id} />; })",
    "items.map(item => <li key={(item = item.next).id}>{item.name}</li>)",
    "items.map((item, index) => <li key={item.id}>{index++}</li>)",
    "items.map(item => <li key={item.id} onClick={() => item = item.next} />)",
    "items.map(item => item.ok ? <li key={item.id} /> : null)",
    "items.map(item => <li key={item.id} v-if={item.ok} />)",
    "items.map(item => <li key={item.id} />, context)",
    "items.map(async item => <li key={item.id} />)",
    "items?.map(item => <li key={item.id} />)",
    "items.map?.(item => <li key={item.id} />)",
    "items.map(item => <li key={item.id} />).filter(visible)",
  ] {
    let code = transform(&format!("<ul>{{{expression}}}</ul>"), None).code;
    assert!(!code.contains("_createMappedFor("), "{expression}: {code}");
  }
}

#[test]
fn virtual_dom_maps_are_not_lowered() {
  let code = transform(
    "<ul>{items.map(item => <li key={item.id}>{item.name}</li>)}</ul>",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert!(code.contains(".map("), "{code}");
  assert!(!code.contains("_createFor("), "{code}");
  assert!(!code.contains("_createMappedFor("), "{code}");
}
