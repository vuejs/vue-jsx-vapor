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
      const { code } = compile(`<>{ a + b }</>`)
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

  describe('execution order', () => {
    test('basic', () => {
      const { code } = compile(`<div foo={true}>{foo}</div>`)
      expect(code).matchSnapshot()
      expect(code).contains(
        `_setProp(n0, "foo", true)
  const x0 = _child(n0)
  _setNodes(x0, () => (foo))`,
      )
    })
    test('with v-once', () => {
      const { code, templates } = compile(
        `<div>
        1{/**/}2
        </div>`,
      )
      expect(templates).toMatchInlineSnapshot(`
        [
          "_template("<div>12</div>", true)",
        ]
      `)
      expect(code).toMatchInlineSnapshot(`
        "
          const n0 = t0()
          return n0
        "
      `)
      //     expect(code).contains(
      //       `_setNodes(n1, () => (bar))
      // _setNodes(n2, () => (baz))`,
      //     )
    })
  })
})
