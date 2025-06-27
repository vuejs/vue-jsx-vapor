import { ErrorCodes } from '@vue/compiler-dom'
import { describe, expect, test, vi } from 'vitest'
import {
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVOn,
  transformVSlot,
  transformVSlots,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithSlots = makeCompile({
  nodeTransforms: [
    transformElement,
    transformText,
    transformVSlot,
    transformChildren,
  ],
  directiveTransforms: {
    bind: transformVBind,
    on: transformVOn,
    slots: transformVSlots,
  },
})

describe('compiler: transform v-slots', () => {
  test('basic', () => {
    const { code } = compileWithSlots(
      `<Comp v-slots={{ default: ({ foo })=> <>{ foo + bar }</> }}></Comp>`,
    )
    expect(code).toMatchSnapshot()
  })

  test('error on invalid mixed slot usage', () => {
    const onError = vi.fn()
    const source = `<Comp v-slots={{ default: ({ foo })=> <>{ foo + bar }</> }}>foo</Comp>`
    compileWithSlots(source, { onError })
    const index = source.lastIndexOf('foo')
    expect(onError.mock.calls[0][0]).toMatchObject({
      code: ErrorCodes.X_V_SLOT_MIXED_SLOT_USAGE,
      loc: {
        start: {
          offset: index,
          line: 1,
          column: index + 1,
        },
        end: {
          offset: index + 3,
          line: 1,
          column: index + 4,
        },
      },
    })
  })
})
