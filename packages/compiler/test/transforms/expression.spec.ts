import { NodeTypes } from '@vue/compiler-dom'
import { describe, expect, test } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  // transformComment,
  transformElement,
  transformText,
  type IfIRNode,
  // transformVIf,
  // transformVOnce,
  // transformVText,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithVIf = makeCompile({
  nodeTransforms: [
    // transformVOnce,
    // transformVIf,
    transformText,
    transformElement,
    // transformComment,
    transformChildren,
  ],
  directiveTransforms: {
    // text: transformVText,
  },
})

describe('compiler: v-if', () => {
  test('conditional expression', () => {
    const { code, helpers, ir } = compileWithVIf(
      `<>{ok? <div>{msg}</div> : fail ? <div>fail</div>  : null }</>`,
    )

    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = _createIf(() => (ok), () => {
          const n2 = t0()
          _setText(n2, () => (msg))
          return n2
        }, () => {
          const n4 = _createIf(() => (fail), () => {
            const n6 = t1()
            return n6
          }, () => {
            const n8 = _createTextNode(null)
            return n8
          })
          return n4
        })
        return n0
      "
    `)

    expect(helpers).contains('createIf')

    expect(ir.template).toEqual(['<div></div>', '<div>fail</div>'])
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.IF,
        id: 0,
        condition: {
          type: NodeTypes.SIMPLE_EXPRESSION,
          content: 'ok',
          isStatic: false,
        },
        positive: {
          type: IRNodeTypes.BLOCK,
          dynamic: {
            children: [{ template: 0 }],
          },
        },
      },
    ])
    expect(ir.block.returns).toEqual([0])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 0 }],
    })

    expect(ir.block.effect).toEqual([])
    expect((ir.block.operation[0] as IfIRNode).positive.effect).lengthOf(0)

    // expect(code).matchSnapshot()
  })
  test('logical expression', () => {
    const { code, helpers, ir } = compileWithVIf(
      `<>{ok && <div>{msg}</div>}</>`,
    )
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = _createIf(() => (ok), () => {
          const n2 = t0()
          _setText(n2, () => (msg))
          return n2
        }, () => {
          const n4 = _createTextNode(() => (ok))
          return n4
        })
        return n0
      "
    `)

    expect(helpers).contains('createIf')

    expect(ir.template).toEqual(['<div></div>'])
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.IF,
        id: 0,
        condition: {
          type: NodeTypes.SIMPLE_EXPRESSION,
          content: 'ok',
          isStatic: false,
        },
        positive: {
          type: IRNodeTypes.BLOCK,
          dynamic: {
            children: [{ template: 0 }],
          },
        },
      },
    ])
    expect(ir.block.returns).toEqual([0])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 0 }],
    })

    expect(ir.block.effect).toEqual([])
    expect((ir.block.operation[0] as IfIRNode).positive.effect).lengthOf(0)
  })

  /*
  test('template v-if', () => {
    const { code, ir } = compileWithVIf(
      `<template v-if="ok"><div/>hello<p v-text="msg"/></template>`,
    )
    expect(code).matchSnapshot()

    expect(ir.template).toEqual(['<div></div>', 'hello', '<p></p>'])
    expect(ir.block.effect).toEqual([])
    expect((ir.block.operation[0] as IfIRNode).positive.effect).toMatchObject([
      {
        operations: [
          {
            type: IRNodeTypes.SET_TEXT,
            element: 4,
            values: [
              {
                content: 'msg',
                type: NodeTypes.SIMPLE_EXPRESSION,
                isStatic: false,
              },
            ],
          },
        ],
      },
    ])
    expect((ir.block.operation[0] as IfIRNode).positive.dynamic).toMatchObject({
      id: 1,
      children: {
        2: {
          id: 4,
        },
      },
    })
  })

  test('dedupe same template', () => {
    const { code, ir } = compileWithVIf(
      `<div v-if="ok">hello</div><div v-if="ok">hello</div>`,
    )
    expect(code).matchSnapshot()
    expect(ir.template).toEqual(['<div>hello</div>'])
    expect(ir.block.returns).toEqual([0, 3])
  })

  test.todo('v-if with v-once')
  test.todo('component v-if')

  test('v-if + v-else', () => {
    const { code, ir, vaporHelpers, helpers } = compileWithVIf(
      `<div v-if="ok"/><p v-else/>`,
    )
    expect(code).matchSnapshot()
    expect(ir.template).toEqual(['<div></div>', '<p></p>'])

    expect(vaporHelpers).contains('createIf')
    expect(ir.block.effect).lengthOf(0)
    expect(helpers).lengthOf(0)
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.IF,
        id: 0,
        condition: {
          type: NodeTypes.SIMPLE_EXPRESSION,
          content: 'ok',
          isStatic: false,
        },
        positive: {
          type: IRNodeTypes.BLOCK,
          dynamic: {
            children: [{ template: 0 }],
          },
        },
        negative: {
          type: IRNodeTypes.BLOCK,
          dynamic: {
            children: [{ template: 1 }],
          },
        },
      },
    ])
    expect(ir.block.returns).toEqual([0])
  })

  test('v-if + v-else-if', () => {
    const { code, ir } = compileWithVIf(
      `<div v-if="ok"/><p v-else-if="orNot"/>`,
    )
    expect(code).matchSnapshot()
    expect(ir.template).toEqual(['<div></div>', '<p></p>'])

    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.IF,
        id: 0,
        condition: {
          type: NodeTypes.SIMPLE_EXPRESSION,
          content: 'ok',
          isStatic: false,
        },
        positive: {
          type: IRNodeTypes.BLOCK,
          dynamic: {
            children: [{ template: 0 }],
          },
        },
        negative: {
          type: IRNodeTypes.IF,
          condition: {
            type: NodeTypes.SIMPLE_EXPRESSION,
            content: 'orNot',
            isStatic: false,
          },
          positive: {
            type: IRNodeTypes.BLOCK,
            dynamic: {
              children: [{ template: 1 }],
            },
          },
        },
      },
    ])
    expect(ir.block.returns).toEqual([0])
  })

  test('v-if + v-else-if + v-else', () => {
    const { code, ir } = compileWithVIf(
      `<div v-if="ok"/><p v-else-if="orNot"/><template v-else>fine</template>`,
    )
    expect(code).matchSnapshot()
    expect(ir.template).toEqual(['<div></div>', '<p></p>', 'fine'])

    expect(ir.block.returns).toEqual([0])
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.IF,
        id: 0,
        positive: {
          type: IRNodeTypes.BLOCK,
          dynamic: {
            children: [{ template: 0 }],
          },
        },
        negative: {
          type: IRNodeTypes.IF,
          positive: {
            type: IRNodeTypes.BLOCK,
            dynamic: {
              children: [{ template: 1 }],
            },
          },
          negative: {
            type: IRNodeTypes.BLOCK,
            dynamic: {
              children: [{ template: 2 }],
            },
          },
        },
      },
    ])
  })

  test('comment between branches', () => {
    const { code, ir } = compileWithVIf(`
      <div v-if="ok"/>
      <!--foo-->
      <p v-else-if="orNot"/>
      <!--bar-->
      <template v-else>fine</template>
      <input v-text="text" />
    `)
    expect(code).matchSnapshot()
    expect(ir.template).toEqual([
      '<div></div>',
      '<!--foo-->',
      '<p></p>',
      '<!--bar-->',
      'fine',

      '<input>',
    ])
  })

  describe.todo('errors')
  describe.todo('codegen')
  test.todo('v-on with v-if')
  */
})
