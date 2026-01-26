use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<div v-if={ok}>{msg}</div>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> </div>");
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
fn template() {
  let code = transform(
    "<template v-if={ok}><div/>hello<p v-text={msg}></p></template>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, renderEffect as _renderEffect, setText as _setText, template as _template, toDisplayString as _toDisplayString, txt as _txt } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("hello");
  const _t2 = _template("<p> </p>");
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
  const _t0 = _template("<div>hello</div>");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	const _n3 = _createIf(() => ok, () => {
  		const _n5 = _t0();
  		return _n5;
  	});
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
  		const _n2 = _createComponent(Comp);
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
  		const _n2 = _createSlot("default");
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
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_else() {
  let code = transform("<><div v-if={ok}/><p v-else/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("<p></p>");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => {
  		const _n4 = _t1();
  		return _n4;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_if_else() {
  let code = transform("<><div v-if={ok}/><p v-else-if={orNot}/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("<p></p>");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		return _n2;
  	}, () => _createIf(() => orNot, () => {
  		const _n4 = _t1();
  		return _n4;
  	}));
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_else_if_v_else() {
  let code = transform(
    "<><div v-if={ok}/><p v-else-if={orNot}/><template v-else>fine</template></>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("<div></div>");
  const _t1 = _template("<p></p>");
  const _t2 = _template("fine");
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
  	}));
  	return _n0;
  })();
  "#);
}

#[test]
fn v_if_v_if_or_v_elses() {
  let code = transform(
    "<div>
      <span v-if=\"foo\">foo</span>
      <span v-if=\"bar\">bar</span>
      <span v-else>baz</span>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const _t0 = _template("<span>foo</span>");
  const _t1 = _template("<span>bar</span>");
  const _t2 = _template("<span>baz</span>");
  const _t3 = _template("<div></div>", true);
  (() => {
  	const _n8 = _t3();
  	_setInsertionState(_n8, null);
  	const _n0 = _createIf(() => "foo", () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	_setInsertionState(_n8, null, true);
  	const _n3 = _createIf(() => "bar", () => {
  		const _n5 = _t1();
  		return _n5;
  	}, () => {
  		const _n7 = _t2();
  		return _n7;
  	});
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
  const _t0 = _template("<div></div>");
  const _t1 = _template("<p></p>");
  const _t2 = _template("fine");
  const _t3 = _template("<div>text</div>");
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
  	}));
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
  const _t0 = _template("<button>w/ v-if</button>");
  (() => {
  	const _n0 = _createIf(() => true, () => {
  		const _n2 = _t0();
  		_renderEffect(() => _setDynamicEvents(_n2, { click: clickEvent }));
  		return _n2;
  	}, null, true);
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
