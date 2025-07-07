import { transformSync } from '@babel/core'
import { describe, expect, test } from 'vitest'
import jsx from '../src/index'

describe('transform', () => {
  test('transform multiple components', () => {
    const { code } = transformSync(
      `const A = defineComponent(() => {
         defineVaporComponent(() => <div />)
         return () => <div />
       })
       const B = defineVaporComponent(() => {
        const C = defineComponent(() => <div />)
        const D = <div />
        return <div />
       })`,
      {
        filename: 'test.tsx',
        plugins: [[jsx, { interop: true }]],
      },
    )!
    expect(code).toMatchInlineSnapshot(`
      "import { template as _template } from 'vue';
      const _t00 = _template("<div></div>", true);
      const A = defineComponent(() => {
        defineVaporComponent(() => (() => {
          const n0 = _t00();
          return n0;
        })());
        return () => <div />;
      });
      const B = defineVaporComponent(() => {
        const C = defineComponent(() => <div />);
        const D = (() => {
          const n0 = _t00();
          return n0;
        })();
        return (() => {
          const n0 = _t00();
          return n0;
        })();
      });"
    `)
  })
})
