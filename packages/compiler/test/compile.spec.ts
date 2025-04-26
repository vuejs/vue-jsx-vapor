import { describe, expect, test } from 'vitest'
import { compile } from '../src'

describe('compile', () => {
  test('static template', () => {
    const { code } = compile(
      `<div>
        <div>hello</div>
        <input />
        <span />
      </div>`,
    )
    expect(code).toMatchSnapshot()
  })

  test('dynamic root', () => {
    const { code } = compile(`<>{ 1 }{ 2 }</>`)
    expect(code).toMatchSnapshot()
  })

  test('dynamic root', () => {
    const { code } = compile(`<div>{a +b +       c }</div>`)
    expect(code).toMatchSnapshot()
  })

  describe('expression parsing', () => {
    test('interpolation', () => {
      const { code } = compile(`<>{ a + b }</>`, {
        inline: true,
      })
      expect(code).toMatchSnapshot()
      expect(code).contains('a + b')
    })
  })

  describe('setInsertionState', () => {
    test('next, child and nthChild should be above the setInsertionState', () => {
      const { code } = compile(`
      <div>
        <div />
        <Comp />
        <div />
        <div v-if={true} />
        <div>
          <button disabled={foo} />
        </div>
      </div>
      `)
      expect(code).toMatchSnapshot()
    })
  })
})
