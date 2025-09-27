import { describe, expect, test, vi } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformVHtml,
} from '../../src'
import { ErrorCodes } from '../../src/utils'
import { makeCompile } from './_utils'

const compileWithVHtml = makeCompile({
  nodeTransforms: [transformElement, transformChildren],
  directiveTransforms: {
    html: transformVHtml,
  },
})

describe('v-html', () => {
  test('should convert v-html to innerHTML', () => {
    const { code, ir, helpers } = compileWithVHtml(
      `<div v-html={code.value}></div>`,
    )

    expect(helpers).contains('setHtml')

    expect(ir.block.operation).toMatchObject([])
    expect(ir.block.effect).toMatchObject([
      {
        expressions: [
          {
            content: 'code.value',
            isStatic: false,
          },
        ],
        operations: [
          {
            type: IRNodeTypes.SET_HTML,
            element: 0,
            value: {
              content: 'code.value',
              isStatic: false,
            },
          },
        ],
      },
    ])

    expect(code).matchSnapshot()
  })

  test('should raise error and ignore children when v-html is present', () => {
    const onError = vi.fn()
    const { ir, helpers, templates } = compileWithVHtml(
      `<div v-html={test.value}>hello</div>`,
      {
        onError,
      },
    )

    expect(helpers).contains('setHtml')

    // children should have been removed
    expect(ir.templates).toEqual(['<div></div>'])

    expect(ir.block.operation).toMatchObject([])
    expect(ir.block.effect).toMatchObject([
      {
        expressions: [
          {
            content: 'test.value',
            isStatic: false,
          },
        ],
        operations: [
          {
            type: IRNodeTypes.SET_HTML,
            element: 0,
            value: {
              content: 'test.value',
              isStatic: false,
            },
          },
        ],
      },
    ])

    expect(onError.mock.calls).toMatchObject([
      [{ code: ErrorCodes.X_V_HTML_WITH_CHILDREN }],
    ])

    // children should have been removed
    expect(templates).includes('_template("<div></div>", true)')
  })

  test('should raise error if has no expression', () => {
    const onError = vi.fn()
    const { code } = compileWithVHtml(`<div v-html></div>`, {
      onError,
    })
    expect(code).matchSnapshot()
    expect(onError.mock.calls).toMatchObject([
      [{ code: ErrorCodes.X_V_HTML_NO_EXPRESSION }],
    ])
  })
})
