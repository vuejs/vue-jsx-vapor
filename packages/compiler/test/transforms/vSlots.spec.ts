import { describe, expect, test } from 'vitest'
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
      `<Comp v-slots={{ default: ({ foo })=> <>{ foo + bar }</> }}> </Comp>`,
    )
    expect(code).toMatchSnapshot()
  })
})
