import { createRuleTester } from 'eslint-vitest-rule-tester'
import { describe, expect, it } from 'vitest'
import defineStyle from '../src/rules/define-style'

describe('define-style', () => {
  const { invalid } = createRuleTester({
    name: 'define-style',
    rule: defineStyle,
  })

  it('basic', async () => {
    const { result } = await invalid({
      code: `
        defineStyle(\` .foo { color: red; } \`)
      `,
      errors: ['define-style'],
    })
    expect(result.output).toMatchSnapshot()
  })

  it('syntax error', async () => {
    const { result } = await invalid({
      code: `
        defineStyle(\`
          .foo {
            color: red
            background: blue;
          }
        \`)
        `,
      errors: ['define-style-syntax-error'],
    })
    expect(result.output).toMatchSnapshot()
  })
})
