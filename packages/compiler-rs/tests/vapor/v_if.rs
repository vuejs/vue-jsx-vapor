use std::cell::RefCell;

use common::{
  error::ErrorCodes,
  options::TransformOptions,
  patch_flag::{VaporBlockShape, VaporIfFlags},
};
use compiler_rs::transform;
use insta::assert_snapshot;

const SINGLE_ROOT_NO_SCOPE: i32 =
  VaporBlockShape::SingleRoot as i32 | VaporIfFlags::TrueNoScope as i32;

#[test]
fn basic() {
  let code = transform("<div v-if={ok}>{msg}</div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => msg);
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn omits_default_single_root_flags_when_branch_needs_scope() {
  let code = transform("<div v-if={ok}>{msg}</div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ", 1);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => msg);
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn marks_pure_static_single_root_branch_as_no_scope() {
  let code = transform("<div v-if={ok}>static</div>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>static", 3);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	return _n0;
  })();
  "#);
  assert!(code.contains(&format!("}}, null, {SINGLE_ROOT_NO_SCOPE})")));
}

#[test]
fn marks_pure_static_multi_root_branch_as_no_scope() {
  let code = transform(
    "<template v-if={ok}><div>one</div><p>two</p></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>one</div>", 2);
  const _t1 = _template("<p>two", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _n3 = _t1();
  		return [_n2, _n3];
  	}, null, 34);
  	return _n0;
  })();
  "#);

  assert!(code.contains(&format!(
    "}}, null, {})",
    VaporBlockShape::MultiRoot as i32 | VaporIfFlags::TrueNoScope as i32
  )));
}

#[test]
fn does_not_mark_scoped_branches_as_no_scope() {
  let cases = [
    "<div v-if={ok}>{msg}</div>",
    "<div v-if={ok} class={foo}></div>",
    "<div v-if={ok} onClick={foo}></div>",
    "<Comp v-if={ok} />",
    "<div v-if={ok} ref=\"el\"></div>",
    "<template v-if={ok}><div/>{msg}</template>",
    "<div v-if={ok}><span v-if={bar} /></div>",
  ];

  for source in cases {
    let code = transform(source, None).code;
    assert!(
      !code.contains(&format!(", null, {SINGLE_ROOT_NO_SCOPE})")),
      "{source}"
    );
    assert!(
      !code.contains(&format!(
        ", null, {}",
        VaporBlockShape::MultiRoot as i32 | VaporIfFlags::TrueNoScope as i32
      )),
      "{source}"
    );
  }
}

#[test]
fn packs_once_flag() {
  let code = transform("<div v-if={ok} v-once />", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>", 3);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 49);
  	return _n0;
  })();
  "#);
}

#[test]
fn packs_branch_index() {
  let code = transform("<><div v-if={foo}>foo</div><div v-else>bar</div></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>foo", 2);
  const _t1 = _template("<div>bar", 2);
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, 229);
  	return _n0;
  })();
  "#);
}

#[test]
fn template() {
  let code = transform(
    "<template v-if={ok}><div/>hello<p v-text={msg}></p></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, renderEffect as _renderEffect, setText as _setText, template as _template, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  const _t0 = _template("<div></div>", 2);
  const _t1 = _template("hello", 2);
  const _t2 = _template("<p> ");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _n3 = _t1();
  		const _n4 = _t2();
  		const _x4 = _txt(_n4);
  		_renderEffect(() => _setText(_x4, _toDisplayString(msg)));
  		return [
  			_n2,
  			_n3,
  			_n4
  		];
  	}, null, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_v_for() {
  let code = transform(
    r#"<template v-if={arr.length > 0} v-for={(item, index) in arr} key={index}>
      <div>item: { item }</div>
    </template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor, createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createIf(() => arr.length > 0, () => {
  		const _n2 = _createFor(() => arr, (_for_item0, _for_key0) => {
  			const _n4 = _t0();
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, "item: ", () => _for_item0.value);
  			return _n4;
  		}, (item, index) => index, 8);
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_text() {
  let code = transform(r#"<template v-if={foo}>hello</template>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("hello", 2);
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_single_element() {
  let code = transform(r#"<template v-if={foo}><div>hi</div></template>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>hi", 2);
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_multiple_element() {
  let code = transform(
    r#"<template v-if={foo}><div>hi</div><div>ho</div></template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>hi</div>", 2);
  const _t1 = _template("<div>ho", 2);
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _t0();
  		const _n3 = _t1();
  		return [_n2, _n3];
  	}, null, 34);
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_v_for_inside() {
  let code = transform(
    r#"<template v-if={foo}><div v-for={i in list}/></template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createFor as _createFor, createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _createFor(() => list, (_for_item0) => {
  			const _n4 = _t0();
  			return _n4;
  		}, void 0, 8);
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_key() {
  let code = transform(
    r#"<template v-if={arr.length > 0} key={index}>
      <div>item: { item }</div>
    </template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, createKeyedFragment as _createKeyedFragment, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  const _t1 = _template("<template>");
  (() => {
  	const _n0 = _createIf(() => arr.length > 0, () => {
  		const _n2 = _createKeyedFragment(() => index, () => {
  			const _n4 = _t0();
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, "item: ", () => item);
  			return _n4;
  		});
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn dedupe_same_template() {
  let code = transform(
    "<><div v-if={ok}>hello</div><div v-if={ok}>hello</div></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>hello", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	const _n3 = _createIf(() => ok, () => {
  		const _n5 = _t0();
  		return _n5;
  	}, null, 33);
  	return [_n0, _n3];
  })();
  "#);
}

#[test]
fn component() {
  let code = transform("<Comp v-if={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf } from "vue";
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _createComponent(Comp, null, null, true);
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_single_slot_child() {
  let code = transform(r#"<template v-if={ok}><slot/></template>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _createSlot();
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_on_slot() {
  let code = transform(r#"<slot v-if="ok"></slot>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, createSlot as _createSlot } from "vue";
  (() => {
  	const _n0 = _createIf(() => "ok", () => {
  		const _n2 = _createSlot("default");
  		return _n2;
  	}, null, 17);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_else() {
  let code = transform("<><div v-if={ok}/><p v-else/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<p>", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, 229);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_if_else() {
  let code = transform("<><div v-if={ok}/><p v-else-if={orNot}/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<p>", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => _createIf(() => orNot, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, null, 33), 165);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_else_if_v_else() {
  let code = transform(
    "<><div v-if={ok}/><p v-else-if={orNot}/><p v-else-if={false}/><template v-else>fine</template></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<p>", 2);
  const _t2 = _template("fine", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => _createIf(() => orNot, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, () => _createIf(() => false, () => {
  		const _n7 = _t1();
  		return _n7;
  	}, () => {
  		const _n10 = _t2();
  		return _n10;
  	}, 117), 293), 165);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_if_or_v_elses() {
  let code = transform(
    "<div>
      <span v-if={foo}>foo</span>
      <span v-if={bar}>bar</span>
      <span v-else>baz</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>foo", 2);
  const _t1 = _template("<span>bar", 2);
  const _t2 = _template("<span>baz", 2);
  const _t3 = _template("<div>", 1);
  (() => {
  	const _n8 = _t3();
  	_setInsertionState(_n8, null, 0);
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, null, 33);
  	_setInsertionState(_n8, null, 1);
  	const _n3 = _createIf(() => bar, () => {
  		const _n5 = _t1();
  		return _n5;
  	}, () => {
  		const _n7 = _t2();
  		return _n7;
  	}, 357);
  	return _n8;
  })();
  "#);
}

#[test]
fn comment_between_branches() {
  let code = transform(
    "<>
      <div v-if={ok}/>
      {/* foo */}
      <p v-else-if={orNot}/>
      {/* bar */}
      <template v-else>fine{/* fine */}</template>
      <div v-text=\"text\" />
    </>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>", 2);
  const _t1 = _template("<p>", 2);
  const _t2 = _template("fine", 2);
  const _t3 = _template("<div>text", 2);
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => _createIf(() => orNot, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, () => {
  		const _n7 = _t2();
  		return _n7;
  	}, 357), 165);
  	const _n9 = _t3();
  	return [_n0, _n9];
  })();
  "#);
}

#[test]
fn v_on_with_v_if() {
  let code = transform(
    "<button v-on={{ click: clickEvent }} v-if={true}>w/ v-if</button>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, renderEffect as _renderEffect, setDynamicEvents as _setDynamicEvents, template as _template } from "vue";
  const _t0 = _template("<button>w/ v-if", 1);
  (() => {
  	const _n0 = _createIf(() => true, () => {
  		const _n2 = _t0();
  		_renderEffect(() => _setDynamicEvents(_n2, { click: clickEvent }));
  		return _n2;
  	}, null, 17);
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_in_template_v_for_forces_multi_root_shape() {
  let code = transform(
    r#"<template v-for={item in list}>
      <span v-if={item.ok}>
        <span>{item.text}</span>
      </span>
    </template>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createFor as _createFor, createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span><span> ");
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createIf(() => _for_item0.value.ok, () => {
  			const _n5 = _t0();
  			const _n4 = _child(_n5);
  			const _x4 = _txt(_n4);
  			_setNodes(_x4, () => _for_item0.value.text);
  			return _n5;
  		}, null, 10);
  		return _n2;
  	}, void 0, 16);
  	return _n0;
  })();
  "#);
}

#[test]
fn template_v_if_with_normal_v_else() {
  let code = transform(
    r#"<><template v-if={foo}><div>hi</div><div>ho</div></template><div v-else/></>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div>hi</div>", 2);
  const _t1 = _template("<div>ho", 2);
  const _t2 = _template("<div>", 2);
  (() => {
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _t0();
  		const _n3 = _t1();
  		return [_n2, _n3];
  	}, () => {
  		const _n5 = _t2();
  		return _n5;
  	}, 230);
  	return _n0;
  })();
  "#);
}

#[test]
fn error_on_v_else_missing_adjacent_v_if() {
  let error = RefCell::new(None);
  transform(
    "<div v-else/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn error_on_v_else_if_missing_adjacent_v_if_or_v_else_if() {
  let error = RefCell::new(None);
  transform(
    "<div v-else-if={foo}/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VElseNoAdjacentIf));
}

#[test]
fn error_on_v_if_no_expression() {
  let error = RefCell::new(None);
  transform(
    "<div v-if/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VIfNoExpression));
}

// TODO codegen
