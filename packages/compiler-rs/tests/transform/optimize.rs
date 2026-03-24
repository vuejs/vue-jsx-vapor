use compiler_rs::{TransformOptions, transform};
use insta::assert_snapshot;

#[test]
fn should_optimize_in_functional_compoennt() {
  let code = transform(
    "function Comp(){
      return <Comp>{foo}</Comp>
    }",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  function Comp() {
  	return _openBlock(), _createBlock(Comp, null, {
  		default: _withCtx(() => [_normalizeVNode(() => foo)]),
  		_: 1
  	});
  }
  "#);
}

#[test]
fn should_not_optimize_in_functional_compoennt_with_params() {
  let code = transform(
    "function Comp({ foo }){
      {
        <Comp>{foo}</Comp>
      }
      return <Comp>{foo}</Comp>
    }",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { createBlock as _createBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  function Comp({ foo }) {
  	{
  		_openBlock(), _createBlock(Comp, null, {
  			default: _withCtx(() => [_normalizeVNode(() => foo)]),
  			_: 2
  		}, 1024);
  	}
  	return _openBlock(), _createBlock(Comp, null, {
  		default: _withCtx(() => [_normalizeVNode(() => foo)]),
  		_: 2
  	}, 1024);
  }
  "#);
}

#[test]
fn should_optimize_in_define_compoennt() {
  let code = transform(
    "export default defineComponent({
      setup() {
        return () => <div onClick={() => foo} />
      }
    })",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  export default defineComponent({ setup() {
  	return () => (() => {
  		const _cache = _createVNodeCache("631d214bc2c8427c");
  		return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  	})();
  } });
  "#);
}

#[test]
fn should_optimize_in_functional_define_compoennt() {
  let code = transform(
    "export default defineComponent(() => {
      return () => <div onClick={() => foo} />
    })",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  export default defineComponent(() => {
  	return () => (() => {
  		const _cache = _createVNodeCache("631d214bc2c8427c");
  		return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  	})();
  });
  "#);
}

#[test]
fn should_optimize_in_nested_define_compoennt() {
  let code = transform(
    "export default defineComponent(() => {
      const Comp = defineComponent(() => {
        return () => <div onClick={() => foo} />
      })
      const Comp1 = defineComponent({
        setup: () => {
          return () => <div onClick={() => foo} />
        }
      })
      const Comp2 = () => {
        return <div onClick={() => foo} />
      }
      return () => <div onClick={() => foo} />
    })",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  export default defineComponent(() => {
  	const Comp = defineComponent(() => {
  		return () => (() => {
  			const _cache = _createVNodeCache("631d214bc2c8427c");
  			return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  		})();
  	});
  	const Comp1 = defineComponent({ setup: () => {
  		return () => (() => {
  			const _cache = _createVNodeCache("5c89500e299049d2");
  			return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  		})();
  	} });
  	const Comp2 = () => {
  		return (() => {
  			const _cache = _createVNodeCache("d10877e335888a9");
  			return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  		})();
  	};
  	return () => (() => {
  		const _cache = _createVNodeCache("cecabad81427710a");
  		return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  	})();
  });
  "#);
}

#[test]
fn should_optimize_in_custom_define_compoennt() {
  let code = transform(
    "export default genericComponent(() => {
      const Comp = genericComponent(() => {
        return () => <div onClick={() => foo} />
      })
      const Comp1 = defineCustomElement({
        setup: () => {
          return () => <div onClick={() => foo} />
        }
      })
      const Comp2 = () => {
        return <div onClick={() => foo} />
      }
      return () => <div onClick={() => foo} />
    })",
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  export default genericComponent(() => {
  	const Comp = genericComponent(() => {
  		return () => (() => {
  			const _cache = _createVNodeCache("631d214bc2c8427c");
  			return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  		})();
  	});
  	const Comp1 = defineCustomElement({ setup: () => {
  		return () => (() => {
  			const _cache = _createVNodeCache("5c89500e299049d2");
  			return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  		})();
  	} });
  	const Comp2 = () => {
  		return (() => {
  			const _cache = _createVNodeCache("d10877e335888a9");
  			return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  		})();
  	};
  	return () => (() => {
  		const _cache = _createVNodeCache("cecabad81427710a");
  		return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => foo) });
  	})();
  });
  "#);
}

#[test]
fn should_cache_in_root_arrow_function_without_params() {
  let code = transform(
    r#"() => <div onClick={() => item} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache } from "/vue-jsx-vapor/vdom";
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  () => (() => {
  	const _cache = _createVNodeCache("631d214bc2c8427c");
  	return _openBlock(), _createElementBlock("div", { onClick: _cache[0] || (_cache[0] = () => item) });
  })();
  "#);
}

#[test]
fn should_not_cache_in_root_arrow_function_with_params() {
  let code = transform(
    r#"(item) => <div onClick={() => item} />"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  (item) => (_openBlock(), _createElementBlock("div", { onClick: () => item }, null, 8, _hoisted_1));
  "#);
}

#[test]
fn should_not_cache_in_root_function_with_params() {
  let code = transform(
    r#"function comp(item) {
      return <div onClick={() => item} />
    }"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  function comp(item) {
  	return _openBlock(), _createElementBlock("div", { onClick: () => item }, null, 8, _hoisted_1);
  }
  "#);
}

#[test]
fn should_not_cache_in_for_statement() {
  let code = transform(
    r#"for (let i = 0; i < 3; i++) {
      stmts.push(<div onClick={() => i} />)
    }"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  for (let i = 0; i < 3; i++) {
  	stmts.push((_openBlock(), _createElementBlock("div", { onClick: () => i }, null, 8, _hoisted_1)));
  }
  "#);
}

#[test]
fn should_not_cache_in_for_in_statement() {
  let code = transform(
    r#"for (let i in [1, 2, 3]) {
      stmts.push(<div onClick={() => i} />)
    }"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  for (let i in [
  	1,
  	2,
  	3
  ]) {
  	stmts.push((_openBlock(), _createElementBlock("div", { onClick: () => i }, null, 8, _hoisted_1)));
  }
  "#);
}

#[test]
fn should_not_cache_in_for_of_statement() {
  let code = transform(
    r#"for (let i of [1, 2, 3]) {
      stmts.push(<div onClick={() => i} />)
    }"#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  for (let i of [
  	1,
  	2,
  	3
  ]) {
  	stmts.push((_openBlock(), _createElementBlock("div", { onClick: () => i }, null, 8, _hoisted_1)));
  }
  "#);
}

#[test]
fn should_not_optimize_multiple_statments() {
  let code = transform(
    r#"const Comp = defineComponent((props) => {
      return () => {
        const Foo = <>{props.foo}</>
        return <Comp onClick={() => props.bar}>{Foo}</Comp>
      }
    })
    export default () => (
      <Comp>{Foo}</Comp>
    )
    "#,
    Some(TransformOptions {
      interop: true,
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createVNodeCache as _createVNodeCache, normalizeVNode as _normalizeVNode } from "/vue-jsx-vapor/vdom";
  import { Fragment as _Fragment, createBlock as _createBlock, createElementBlock as _createElementBlock, openBlock as _openBlock, withCtx as _withCtx } from "vue";
  const Comp = defineComponent((props) => {
  	return () => {
  		const Foo = (_openBlock(), _createElementBlock(_Fragment, null, [_normalizeVNode(() => props.foo)], 64));
  		return (() => {
  			const _cache = _createVNodeCache("631d214bc2c8427c");
  			return _openBlock(), _createBlock(Comp, { onClick: _cache[0] || (_cache[0] = () => props.bar) }, {
  				default: _withCtx(() => [_normalizeVNode(() => Foo)]),
  				_: 2
  			}, 1024);
  		})();
  	};
  });
  export default () => (_openBlock(), _createBlock(Comp, null, {
  	default: _withCtx(() => [_normalizeVNode(() => Foo)]),
  	_: 1
  }));
  "#);
}

#[test]
fn should_not_optimize_in_nested_scopes() {
  let code = transform(
    r#"const renderRow = ({
      rowInfo,
    }) => {
      const cells = iteratedCols.map((col) => {
        return (
          <div onClick={() => {
            handleUpdateExpanded(rowInfo.tmNode)
          }} />
        )
      })
    }"#,
    Some(TransformOptions {
      interop: true,
      filename: "index.tsx",
      ..Default::default()
    }),
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createElementBlock as _createElementBlock, openBlock as _openBlock } from "vue";
  const _hoisted_1 = ["onClick"];
  const renderRow = ({ rowInfo }) => {
  	const cells = iteratedCols.map((col) => {
  		return _openBlock(), _createElementBlock("div", { onClick: () => {
  			handleUpdateExpanded(rowInfo.tmNode);
  		} }, null, 8, _hoisted_1);
  	});
  };
  "#);
}
