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
  const t0 = _template("<div> </div>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		const x2 = _txt(n2);
  		_setNodes(x2, () => msg);
  		return n2;
  	});
  	return n0;
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
  const t0 = _template("<div></div>");
  const t1 = _template("hello");
  const t2 = _template("<p> </p>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		const n3 = t1();
  		const n4 = t2();
  		const x4 = _txt(n4);
  		_renderEffect(() => _setText(x4, _toDisplayString(msg)));
  		return [
  			n2,
  			n3,
  			n4
  		];
  	});
  	return n0;
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
  const t0 = _template("<div>hello</div>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		return n2;
  	});
  	const n3 = _createIf(() => ok, () => {
  		const n5 = t0();
  		return n5;
  	});
  	return [n0, n3];
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
  	const n0 = _createIf(() => foo, () => {
  		const n2 = _createComponent(Comp);
  		return n2;
  	});
  	return n0;
  })();
  "#);
}

#[test]
fn v_if_v_else() {
  let code = transform("<><div v-if={ok}/><p v-else/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const t0 = _template("<div></div>");
  const t1 = _template("<p></p>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		return n2;
  	}, () => {
  		const n4 = t1();
  		return n4;
  	});
  	return n0;
  })();
  "#);
}

#[test]
fn v_if_v_if_else() {
  let code = transform("<><div v-if={ok}/><p v-else-if={orNot}/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const t0 = _template("<div></div>");
  const t1 = _template("<p></p>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		return n2;
  	}, () => _createIf(() => orNot, () => {
  		const n4 = t1();
  		return n4;
  	}));
  	return n0;
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
  const t0 = _template("<div></div>");
  const t1 = _template("<p></p>");
  const t2 = _template("fine");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		return n2;
  	}, () => _createIf(() => orNot, () => {
  		const n4 = t1();
  		return n4;
  	}, () => {
  		const n7 = t2();
  		return n7;
  	}));
  	return n0;
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
  const t0 = _template("<span>foo</span>");
  const t1 = _template("<span>bar</span>");
  const t2 = _template("<span>baz</span>");
  const t3 = _template("<div></div>", true);
  (() => {
  	const n8 = t3();
  	_setInsertionState(n8, null);
  	const n0 = _createIf(() => "foo", () => {
  		const n2 = t0();
  		return n2;
  	});
  	_setInsertionState(n8, null, true);
  	const n3 = _createIf(() => "bar", () => {
  		const n5 = t1();
  		return n5;
  	}, () => {
  		const n7 = t2();
  		return n7;
  	});
  	return n8;
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
  const t0 = _template("<div></div>");
  const t1 = _template("<p></p>");
  const t2 = _template("fine");
  const t3 = _template("<div>text</div>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		return n2;
  	}, () => _createIf(() => orNot, () => {
  		const n4 = t1();
  		return n4;
  	}, () => {
  		const n7 = t2();
  		return n7;
  	}));
  	const n9 = t3();
  	return [n0, n9];
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
  const t0 = _template("<button>w/ v-if</button>");
  (() => {
  	const n0 = _createIf(() => true, () => {
  		const n2 = t0();
  		_renderEffect(() => _setDynamicEvents(n2, { click: clickEvent }));
  		return n2;
  	}, null, true);
  	return n0;
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
