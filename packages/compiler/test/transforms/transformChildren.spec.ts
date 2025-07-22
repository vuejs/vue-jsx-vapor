import { NodeTypes } from '@vue/compiler-dom'

import { describe, expect, test } from 'vitest'
import {
  IRDynamicPropsKind,
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithElementTransform = makeCompile({
  nodeTransforms: [transformText, transformElement, transformChildren],
  directiveTransforms: { bind: transformVBind },
})

describe('compiler: children transform', () => {
  test('basic', () => {
    const { code, helpers } = compileWithElementTransform(
      `<div>
        {foo} {bar}
       </div>`,
    )
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = t0()
        const x0 = _child(n0)
        _setNodes(x0, () => (foo), " ", () => (bar))
        return n0
      "
    `)
    expect(helpers).contains.all.keys('setNodes')
  })

  test('comments', () => {
    const { code } = compileWithElementTransform(
      '<>{/*foo*/}<div>{/*bar*/}</div></>',
    )
    expect(code).toMatchInlineSnapshot(`
      "
        const n1 = t0()
        return n1
      "
    `)
  })

  test('fragment', () => {
    const { code } = compileWithElementTransform('<>{foo}</>')
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = _createNodes(() => (foo))
        return n0
      "
    `)
  })

  test('children & sibling references', () => {
    const { code, helpers } = compileWithElementTransform(
      `<div id={id}>
        <p>{ first }</p> 
        123 { second } 456 {foo}
        <p>{ forth }</p>
      </div>`,
    )
    expect(code).toMatchInlineSnapshot(`
      "
        const n3 = t0()
        const n0 = _child(n3)
        const n1 = _next(n0)
        const n2 = _next(n1)
        const x0 = _child(n0)
        _setNodes(x0, () => (first))
        _setNodes(n1, "123 ", () => (second), " 456 ", () => (foo))
        const x2 = _child(n2)
        _setNodes(x2, () => (forth))
        _renderEffect(() => _setProp(n3, "id", id))
        return n3
      "
    `)
    expect(Array.from(helpers)).containSubset([
      'child',
      'renderEffect',
      'next',
      'setNodes',
      'template',
    ])
  })

  test('{...obj}', () => {
    const { code, ir } = compileWithElementTransform(`<div {...obj} />`)
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = t0()
        _renderEffect(() => _setDynamicProps(n0, [obj], true))
        return n0
      "
    `)
    expect(ir.block.effect).toMatchObject([
      {
        expressions: [
          {
            type: NodeTypes.SIMPLE_EXPRESSION,
            content: 'obj',
            isStatic: false,
          },
        ],
        operations: [
          {
            type: IRNodeTypes.SET_DYNAMIC_PROPS,
            element: 0,
            props: [
              {
                kind: IRDynamicPropsKind.EXPRESSION,
                value: {
                  type: NodeTypes.SIMPLE_EXPRESSION,
                  content: 'obj',
                  isStatic: false,
                },
              },
            ],
          },
        ],
      },
    ])
    expect(code).contains('_setDynamicProps(n0, [obj], true)')
  })

  test('efficient traversal', () => {
    const { code } = compileWithElementTransform(
      `<div>
    <div>x</div>
    <div><span>{{ msg }}</span></div>
    <div><span>{{ msg }}</span></div>
    <div><span>{{ msg }}</span></div>
  </div>`,
    )
    expect(code).toMatchSnapshot()
  })

  test('efficient find', () => {
    const { code } = compileWithElementTransform(
      `<div>
        <div>x</div>
        <div>x</div>
        <div>{ msg }</div>
      </div>`,
    )
    expect(code).contains(`const n0 = _nthChild(n1, 2)`)
    expect(code).toMatchSnapshot()
  })

  test('JSXComponent in JSXExpressionContainer', () => {
    const { code } = compileWithElementTransform(
      `<div>
        {<Comp />}
      </div>`,
    )
    expect(code).contains(`_setNodes(x0, () => (<Comp />))`)
    expect(code).toMatchSnapshot()
  })
})
