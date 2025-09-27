import { describe, expect, test } from 'vitest'
import { IRNodeTypes } from '../../src'
import { getBaseTransformPreset } from '../../src/compile'
import { makeCompile } from './_utils'

const [nodeTransforms, directiveTransforms] = getBaseTransformPreset()
const compileWithOnce = makeCompile({
  nodeTransforms,
  directiveTransforms,
})

describe('compiler: v-once', () => {
  test('basic', () => {
    const { ir, code } = compileWithOnce(
      `<div v-once>
        { msg }
        <span class={clz} />
      </div>`,
    )

    expect(code).toMatchSnapshot()
    expect(ir.block.effect).lengthOf(0)
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_NODES,
        element: 0,
        values: [
          {
            content: 'msg',
            isStatic: false,
          },
        ],
      },
      {
        element: 1,
        type: IRNodeTypes.SET_PROP,
        prop: {
          key: {
            content: 'class',
            isStatic: true,
          },
          values: [
            {
              content: 'clz',
              isStatic: false,
            },
          ],
        },
      },
    ])
  })

  test('as root node', () => {
    const { ir, code } = compileWithOnce(`<div id={foo} v-once />`)

    expect(code).toMatchSnapshot()

    expect(ir.block.effect).lengthOf(0)
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_PROP,
        element: 0,
        prop: {
          key: {
            content: 'id',
            isStatic: true,
          },
          values: [
            {
              content: 'foo',
              isStatic: false,
            },
          ],
        },
      },
    ])
    expect(code).not.contains('effect')
  })

  test('on nested plain element', () => {
    const { ir, code } = compileWithOnce(`<div><div id={foo} v-once /></div>`)

    expect(code).toMatchSnapshot()

    expect(ir.block.effect).lengthOf(0)
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_PROP,
        element: 0,
        prop: {
          runtimeCamelize: false,
          key: {
            content: 'id',
            isStatic: true,
          },
          values: [
            {
              content: 'foo',
              isStatic: false,
            },
          ],
        },
      },
    ])
  })

  test('on component', () => {
    const { ir, code } = compileWithOnce(`<div><Comp id={foo} v-once /></div>`)
    expect(code).toMatchSnapshot()
    expect(ir.block.effect).lengthOf(0)
    const op = ir.block.dynamic.children[0].children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      id: 0,
      tag: 'Comp',
      once: true,
    })
  })

  test.todo('on slot outlet')

  test('inside v-once', () => {
    const { ir, code } = compileWithOnce(`<div v-once><div v-once/></div>`)

    expect(code).toMatchSnapshot()

    expect(ir.block.effect).lengthOf(0)
    expect(ir.block.operation).lengthOf(0)
  })

  test('with v-if', () => {
    const { ir, code } = compileWithOnce(`<div v-if={expr} v-once />`)
    expect(code).toMatchSnapshot()

    expect(ir.block.effect).lengthOf(0)
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.IF,
      id: 0,
      once: true,
      condition: {
        content: 'expr',
        isStatic: false,
      },
      positive: {
        type: IRNodeTypes.BLOCK,
        dynamic: {
          children: [{ template: 0 }],
        },
      },
    })
  })

  test('with v-if/else', () => {
    const { ir, code } = compileWithOnce(
      `<><div v-if={expr} v-once /><p v-else/></>`,
    )
    expect(code).toMatchSnapshot()

    expect(ir.block.effect).lengthOf(0)
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.IF,
      id: 0,
      once: true,
      condition: {
        content: 'expr',
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
    })
  })

  test('with v-for', () => {
    const { ir, code } = compileWithOnce(`<div v-for={i in list} v-once />`)
    expect(code).toMatchSnapshot()
    expect(ir.block.effect).lengthOf(0)
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      id: 0,
      once: true,
    })
  })
})
