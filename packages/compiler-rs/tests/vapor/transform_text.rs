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
  const t0 = _template("<div><div>hello</div><input><span></span></div>", true);
  (() => {
  	const n0 = t0();
  	return n0;
  })();
  "#);
}

#[test]
fn interpolation() {
  let code = transform("<>{ 1 }{ 2 }{a +b +       c }</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const n0 = _createNodes(1, 2, () => a + b + c);
  	return n0;
  })();
  "#);
}

#[test]
fn on_consecutive_text() {
  let code = transform("<>{ \"hello world\" }</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  (() => {
  	const n0 = _createNodes("hello world");
  	return n0;
  })();
  "#);
}

#[test]
fn consecutive_text() {
  let code = transform("<>{ msg }  <div/></>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { template as _template } from "vue";
  const t0 = _template("<div></div>");
  (() => {
  	const n2 = t0();
  	const n0 = _createNodes(() => msg, " ");
  	return [n0, n2];
  })();
  "#);
}

#[test]
fn escapes_raw_static_text_when_generating_the_template_string() {
  let code = transform("<code>&nbsp;&lt;script&gt;</code>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<code>\xA0&lt;script&gt;</code>", true);
  (() => {
  	const n0 = t0();
  	return n0;
  })();
  "#);
}

#[test]
fn text_like() {
  let code = transform("<div>{ (2) }{`foo${1}`}{1}{1n}</div>", None).code;
  assert_snapshot!(code, @r#"
  import { template as _template } from "vue";
  const t0 = _template("<div>2foo111</div>", true);
  (() => {
  	const n0 = t0();
  	return n0;
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
  import { child as _child, createIf as _createIf, template as _template } from "vue";
  const t0 = _template("<span> </span>");
  const t1 = _template("<div>fail</div>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		const x2 = _child(n2);
  		_setNodes(x2, () => msg);
  		return n2;
  	}, () => _createIf(() => fail, () => {
  		const n4 = t1();
  		return n4;
  	}, () => {
  		const n6 = _createNodes(null);
  		return n6;
  	}));
  	return n0;
  })();
  "#);
}

#[test]
fn multiple_conditional() {
  let code = transform("<>{ok? ok : fail} {foo ? foo : <span />}</>", None).code;
  assert_snapshot!(code, @r#"
  import { createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { createIf as _createIf, template as _template } from "vue";
  const t0 = _template(" ");
  const t1 = _template("<span></span>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = _createNodes(() => ok);
  		return n2;
  	}, () => {
  		const n4 = _createNodes(() => fail);
  		return n4;
  	});
  	const n5 = t0();
  	const n6 = _createIf(() => foo, () => {
  		const n8 = _createNodes(() => foo);
  		return n8;
  	}, () => {
  		const n10 = t1();
  		return n10;
  	});
  	return [
  		n0,
  		n5,
  		n6
  	];
  })();
  "#);
}

#[test]
fn logical_expression() {
  let code = transform("<>{ok && (<div>{msg}</div>)}</>", None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  (() => {
  	const n0 = _createIf(() => ok, () => {
  		const n2 = t0();
  		const x2 = _child(n2);
  		_setNodes(x2, () => msg);
  		return n2;
  	}, () => {
  		const n4 = _createNodes(() => ok);
  		return n4;
  	});
  	return n0;
  })();
  "#);
}

#[test]
fn logical_expression_or() {
  let code = transform(r#"<div>{foo || <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  const t1 = _template("<div></div>", true);
  (() => {
  	const n5 = t1();
  	_setInsertionState(n5);
  	const n0 = _createIf(() => foo, () => {
  		const n2 = _createNodes(() => foo);
  		return n2;
  	}, () => {
  		const n4 = t0();
  		const x4 = _child(n4);
  		_setNodes(x4, () => foo);
  		return n4;
  	});
  	return n5;
  })();
  "#)
}

#[test]
fn logical_expression_coalesce() {
  let code = transform(r#"<div>{foo ?? <div>{foo}</div>}</div>"#, None).code;
  assert_snapshot!(code, @r#"
  import { setNodes as _setNodes, createNodes as _createNodes } from "/vue-jsx-vapor/vapor";
  import { child as _child, createIf as _createIf, setInsertionState as _setInsertionState, template as _template } from "vue";
  const t0 = _template("<div> </div>");
  const t1 = _template("<div></div>", true);
  (() => {
  	const n5 = t1();
  	_setInsertionState(n5);
  	const n0 = _createIf(() => foo == null, () => {
  		const n2 = t0();
  		const x2 = _child(n2);
  		_setNodes(x2, () => foo);
  		return n2;
  	}, () => {
  		const n4 = _createNodes(() => foo);
  		return n4;
  	});
  	return n5;
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
  import { child as _child, template as _template } from "vue";
  const t0 = _template("<div>1</div>", true);
  const t1 = _template("<span> </span>", true);
  const t2 = _template("<br>", true);
  (() => {
  	const n0 = _createNodes(() => Array.from({ length: count.value }).map((_, index) => {
  		if (index > 1) {
  			return (() => {
  				const n0 = t0();
  				return n0;
  			})();
  		} else {
  			return [(() => {
  				const n0 = t1();
  				const x0 = _child(n0);
  				_setNodes(x0, "(", () => index, ") lt 1");
  				return n0;
  			})(), (() => {
  				const n0 = t2();
  				return n0;
  			})()];
  		}
  	}));
  	return n0;
  })();
  "#);
}
