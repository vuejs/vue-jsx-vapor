import { describe, expect, test } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformText,
  transformVOnce,
  type IfIRNode,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithVIf = makeCompile({
  nodeTransforms: [
    transformVOnce,
    transformElement,
    transformText,
    transformChildren,
  ],
})

describe('compiler: expression', () => {
  test('conditional expression', () => {
    const { code, helpers, ir } = compileWithVIf(
      `<>{ok? <span>{msg}</span> : fail ? <div>fail</div>  : null }</>`,
    )

    expect(code).toMatchSnapshot()

    expect(helpers).contains('createIf')

    expect(ir.templates).toEqual(['<span> </span>', '<div>fail</div>'])
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.IF,
      id: 0,
      condition: {
        content: 'ok',
        isStatic: false,
      },
      positive: {
        type: IRNodeTypes.BLOCK,
        dynamic: {
          children: [{ template: 0 }],
        },
      },
    })
    expect(ir.block.returns).toEqual([0])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 0 }],
    })

    expect(ir.block.effect).toEqual([])
    expect((op as IfIRNode).positive.effect).lengthOf(0)

    expect(code).matchSnapshot()
  })
  test('logical expression', () => {
    const { code, helpers, ir } = compileWithVIf(
      `<>{ok && <div>{msg}</div>}</>`,
    )

    expect(helpers).contains('createIf')

    expect(ir.templates).toEqual(['<div> </div>'])
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.IF,
      id: 0,
      condition: {
        content: 'ok',
        isStatic: false,
      },
      positive: {
        type: IRNodeTypes.BLOCK,
        dynamic: {
          children: [{ template: 0 }],
        },
      },
    })
    expect(ir.block.returns).toEqual([0])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 0 }],
    })

    expect(ir.block.effect).toEqual([])
    expect((op as IfIRNode).positive.effect).lengthOf(0)
    expect(code).toMatchSnapshot()
  })
  test('conditional expression with v-once', () => {
    const { code, helpers, ir } = compileWithVIf(
      `<div>{ok? <span>{msg}</span> : <div>fail</div> }</div>`,
    )
    expect(code).toMatchSnapshot()

    expect(helpers).contains('createIf')
    expect(ir.templates).toEqual([
      '<span> </span>',
      '<div>fail</div>',
      '<div></div>',
    ])
    expect(ir.block.returns).toEqual([5])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 5 }],
    })
  })
})
