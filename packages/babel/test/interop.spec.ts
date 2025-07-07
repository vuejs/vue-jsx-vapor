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
    expect(code).matchSnapshot()
  })
})
