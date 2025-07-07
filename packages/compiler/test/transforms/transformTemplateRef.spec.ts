import { describe, expect, test } from 'vitest'
import {
  DynamicFlag,
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformTemplateRef,
  transformVFor,
  transformVIf,
  type ForIRNode,
  type IfIRNode,
} from '../../src'

import { makeCompile } from './_utils'

const compileWithTransformRef = makeCompile({
  nodeTransforms: [
    transformVIf,
    transformVFor,
    transformTemplateRef,
    transformElement,
    transformChildren,
  ],
})

describe('compiler: template ref transform', () => {
  test('static ref', () => {
    const { ir, code } = compileWithTransformRef(`<div ref="foo" />`)

    expect(ir.block.dynamic.children[0]).toMatchObject({
      id: 0,
      flags: DynamicFlag.REFERENCED,
    })
    expect(ir.template).toEqual(['<div></div>'])
    expect(ir.block.operation).lengthOf(1)
    expect(ir.block.operation[0]).toMatchObject({
      type: IRNodeTypes.SET_TEMPLATE_REF,
      element: 0,
      value: {
        content: 'foo',
        isStatic: true,
        loc: {
          start: { line: 1, column: 10, offset: 9 },
          end: { line: 1, column: 15, offset: 14 },
        },
      },
    })
    expect(code).matchSnapshot()
    expect(code).contains('const _setTemplateRef = _createTemplateRefSetter()')
    expect(code).contains('_setTemplateRef(n0, "foo")')
  })

  test('dynamic ref', () => {
    const { ir, code } = compileWithTransformRef(`<div ref={foo} />`)

    expect(ir.block.dynamic.children[0]).toMatchObject({
      id: 0,
      flags: DynamicFlag.REFERENCED,
    })
    expect(ir.template).toEqual(['<div></div>'])
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.DECLARE_OLD_REF,
        id: 0,
      },
    ])
    expect(ir.block.effect).toMatchObject([
      {
        operations: [
          {
            type: IRNodeTypes.SET_TEMPLATE_REF,
            element: 0,
            value: {
              content: 'foo',
              isStatic: false,
            },
          },
        ],
      },
    ])
    expect(code).matchSnapshot()
    expect(code).contains('_setTemplateRef(n0, foo, r0)')
  })

  test('ref + v-if', () => {
    const { ir, code } = compileWithTransformRef(
      `<div ref={foo} v-if={true} />`,
    )
    expect(code).toMatchSnapshot()

    const op = ir.block.dynamic.children[0].operation as IfIRNode
    expect(op.type).toBe(IRNodeTypes.IF)

    const { positive } = op
    expect(positive.effect[0].operations).toMatchObject([
      {
        type: IRNodeTypes.SET_TEMPLATE_REF,
        element: 2,
        value: {
          content: 'foo',
          isStatic: false,
        },
        effect: true,
      },
    ])
    expect(code).contains('_setTemplateRef(n2, foo, r2)')
  })

  test('ref + v-for', () => {
    const { ir, code } = compileWithTransformRef(
      `<div ref={foo} v-for={item in [1,2,3]} />`,
    )
    expect(code).toMatchSnapshot()

    const { render } = ir.block.dynamic.children[0].operation as ForIRNode
    expect(render.effect[0].operations).toMatchObject([
      {
        type: IRNodeTypes.SET_TEMPLATE_REF,
        element: 2,
        value: {
          content: 'foo',
          isStatic: false,
        },
        refFor: true,
        effect: true,
      },
    ])
    expect(code).contains('_setTemplateRef(n2, foo, r2, true)')
  })
})
