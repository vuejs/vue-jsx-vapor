import { DOMErrorCodes } from '@vue/compiler-dom'
import { describe, expect, test, vi } from 'vitest'
import { transformChildren, transformElement, transformVShow } from '../../src'
import { makeCompile } from './_utils'

const compileWithVShow = makeCompile({
  nodeTransforms: [transformElement, transformChildren],
  directiveTransforms: {
    show: transformVShow,
  },
})

describe('compiler: v-show transform', () => {
  test('simple expression', () => {
    const { code } = compileWithVShow(`<div v-show="foo"/>`)
    expect(code).toMatchSnapshot()
  })

  test('should raise error if has no expression', () => {
    const onError = vi.fn()
    compileWithVShow(`<div v-show/>`, { onError })

    expect(onError).toHaveBeenCalledTimes(1)
    expect(onError).toHaveBeenCalledWith(
      expect.objectContaining({
        code: DOMErrorCodes.X_V_SHOW_NO_EXPRESSION,
      }),
    )
  })
})
