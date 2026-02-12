use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn static_template() {
  let code = transform(
    "<div>
      <div>hello</div>
      <input />
      <span />
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div><div>hello</div><input><span>", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn interpolation() {
  let code = transform("<>{ 1 }{ 2 }{a +b +       c }</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes(1, 2, () => a + b + c);
  	return _n0;
  })();
  "#);
}

#[test]
fn on_consecutive_text() {
  let code = transform("<>{ \"hello world\" }</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes("hello world");
  	return _n0;
  })();
  "#);
}

#[test]
fn consecutive_text() {
  let code = transform("<>{ msg }  <div/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const _t0 = _template("<div>");
  (() => {
  	const _n2 = _t0();
  	const _n0 = _createNodes(() => msg, " ");
  	return [_n0, _n2];
  })();
  "#);
}

#[test]
fn escapes_raw_static_text_when_generating_the_template_string() {
  let code = transform(
    "<code>
      &nbsp;&lt;script&gt;&nbsp;
    </code>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<code>\xA0&lt;script&gt;\xA0", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_quotes_in_root_level_text_nodes() {
  let code = transform(r#"<>Howdy y'all</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("Howdy y'all");
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_double_quotes_in_root_level_text_nodes() {
  let code = transform(r#"<>Say "hello"</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("Say \"hello\"");
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_quotes_in_template_v_if_text() {
  // Text inside <template> tag also goes through createNode()
  let code = transform(r#"<template v-if="ok">Howdy y'all</template>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template("Howdy y'all");
  (() => {
  	const _n0 = _createIf(() => "ok", () => {
  		const _n2 = _t0();
  		return _n2;
  	});
  	return _n0;
  })();
  "#);
}

#[test]
fn should_not_escape_quotes_in_component_slot_text() {
  // Text inside component (slot content) also goes through createNode()
  let code = transform("<Comp>Howdy y'all</Comp>", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { template as _template, withVaporCtx as _withVaporCtx } from "vue";
  const _t0 = _template("Howdy y'all");
  (() => {
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _t0();
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#);
}

#[test]
fn text_like() {
  let code = transform("<div>{ (2) }{`foo${1}`}{1}{1n}</div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const _t0 = _template("<div>2foo111", true);
  (() => {
  	const _n0 = _t0();
  	return _n0;
  })();
  "#);
}

#[test]
fn conditional_expression() {
  let code = transform(
    "<>{ok? (<span>{msg}</span>) : fail ? (<div>fail</div>)  : null }</>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<span> ");
  const _t1 = _template("<div>fail");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => msg);
  		return _n2;
  	}, () => _createIf(() => fail, () => {
  		const _n4 = _t1();
  		return _n4;
  	}, () => {
  		const _n6 = _createNodes(null);
  		return _n6;
  	}, false, 1), false, 0);
  	return _n0;
  })();
  "#);
}

#[test]
fn multiple_conditional() {
  let code = transform("<>{ok? ok : fail} {foo ? foo : <span />}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template } from "vue";
  const _t0 = _template(" ");
  const _t1 = _template("<span>");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _createNodes(() => ok);
  		return _n2;
  	}, () => {
  		const _n4 = _createNodes(() => fail);
  		return _n4;
  	}, false, 0);
  	const _n5 = _t0();
  	const _n6 = _createIf(() => foo, () => {
  		const _n8 = _createNodes(() => foo);
  		return _n8;
  	}, () => {
  		const _n10 = _t1();
  		return _n10;
  	}, false, 1);
  	return [
  		_n0,
  		_n5,
  		_n6
  	];
  })();
  "#);
}

#[test]
fn logical_expression() {
  let code = transform("<>{ok && (<div>{msg}</div>)}</>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  (() => {
  	const _n0 = _createIf(() => ok, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => msg);
  		return _n2;
  	}, () => {
  		const _n4 = _createNodes(() => ok);
  		return _n4;
  	}, false, 0);
  	return _n0;
  })();
  "#);
}

#[test]
fn logical_expression_or() {
  let code = transform(r#"<div>{foo || <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  const _t1 = _template("<div>", true);
  (() => {
  	const _n5 = _t1();
  	_setInsertionState(_n5, null, 0, true);
  	const _n0 = _createIf(() => foo, () => {
  		const _n2 = _createNodes(() => foo);
  		return _n2;
  	}, () => {
  		const _n4 = _t0();
  		const _x4 = _txt(_n4);
  		_setNodes(_x4, () => foo);
  		return _n4;
  	}, false, 0);
  	return _n5;
  })();
  "#)
}

#[test]
fn logical_expression_coalesce() {
  let code = transform(r#"<div>{foo ?? <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, setInsertionState as _setInsertionState, template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div> ");
  const _t1 = _template("<div>", true);
  (() => {
  	const _n5 = _t1();
  	_setInsertionState(_n5, null, 0, true);
  	const _n0 = _createIf(() => foo == null, () => {
  		const _n2 = _t0();
  		const _x2 = _txt(_n2);
  		_setNodes(_x2, () => foo);
  		return _n2;
  	}, () => {
  		const _n4 = _createNodes(() => foo);
  		return _n4;
  	}, false, 0);
  	return _n5;
  })();
  "#)
}

#[test]
fn expression_map() {
  let code = transform(
    "<>{Array.from({ length: count.value }).map((_, index) => {
      if (index > 1) {
        return <div>1</div>
      } else {
        return [<span>({index}) lt 1</span>, <br />]
      }
    })}</>",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template, txt as _txt } from "vue";
  const _t0 = _template("<div>1", true);
  const _t1 = _template("<span> ", true);
  const _t2 = _template("<br>", true);
  (() => {
  	const _n0 = _createNodes(() => Array.from({ length: count.value }).map((_, index) => {
  		if (index > 1) {
  			return (() => {
  				const _n0 = _t0();
  				return _n0;
  			})();
  		} else {
  			return [(() => {
  				const _n0 = _t1();
  				const _x0 = _txt(_n0);
  				_setNodes(_x0, "(", () => index, ") lt 1");
  				return _n0;
  			})(), (() => {
  				const _n0 = _t2();
  				return _n0;
  			})()];
  		}
  	}));
  	return _n0;
  })();
  "#);
}

#[test]
fn expression_with_comment() {
  let code = transform(
    r#"<div>
      {foo}
      {/**/}
      <a></a>
    </div>"#,
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, template as _template } from "vue";
  const _t0 = _template("<div> <a>", true);
  (() => {
  	const _n1 = _t0();
  	const _n0 = _child(_n1);
  	_setNodes(_n0, () => foo);
  	return _n1;
  })();
  "#)
}

#[test]
fn slot_interpolation() {
  let code = transform(r#"<Comp>{Hello}</Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _createNodes(() => Hello);
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#)
}

#[test]
fn slot_literal_interpolation() {
  let code = transform(r#"<Comp>{ "Hello" }</Comp>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes, createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { withVaporCtx as _withVaporCtx } from "vue";
  (() => {
  	const _n1 = _createComponent(Comp, null, { default: _withVaporCtx(() => {
  		const _n0 = _createNodes("Hello");
  		return _n0;
  	}) }, true);
  	return _n1;
  })();
  "#)
}

#[test]
fn fragment_with_interpolation() {
  let code = transform(r#"<>Message: { "Hello" }!</>"#, None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createNodes("Message: ", "Hello", "?");
  	return _n0;
  })();
  "#)
}
