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
    expect(code).matchSnapshot()
  })
})
