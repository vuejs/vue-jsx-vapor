import { transformSync } from '@babel/core'
import { describe, expect, test } from 'vitest'
import jsx from '../src/index'

describe('transform', () => {
  test('transform multiple components', () => {
    const { code } = transformSync(
      `const a = <div onClick={onClick}>{Hello}</div>
       const b = <>{foo? <div onClick={onClick}>Hello</div> : <div onDblclick={onDblclick}>World</div>}</>`,
      {
        filename: 'test.tsx',
        plugins: [[jsx]],
      },
    )!
    expect(code).toMatchInlineSnapshot(`
      "import { child as _child, delegateEvents as _delegateEvents, template as _template, createIf as _createIf } from 'vue';
      import { setNodes as _setNodes } from 'vue-jsx-vapor/runtime';
      const _t00 = _template("<div> </div>", true);
      const _t10 = _template("<div>Hello</div>");
      const _t11 = _template("<div>World</div>");
      _delegateEvents("click", "dblclick");
      const a = (() => {
        const n0 = _t00();
        const x0 = _child(n0);
        _setNodes(x0, () => Hello);
        n0.$evtclick = e => onClick(e);
        return n0;
      })();
      const b = (() => {
        const n0 = _createIf(() => foo, () => {
          const n2 = _t10();
          n2.$evtclick = e => onClick(e);
          return n2;
        }, () => {
          const n4 = _t11();
          n4.$evtdblclick = e => onDblclick(e);
          return n4;
        });
        return n0;
      })();"
    `)
  })
})
