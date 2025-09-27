// TODO: add tests for this transform
import { describe, expect, it } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVOn,
} from '../../src'

import { makeCompile } from './_utils'

const compileWithTextTransform = makeCompile({
  nodeTransforms: [transformElement, transformChildren, transformText],
  directiveTransforms: {
    bind: transformVBind,
    on: transformVOn,
  },
})

describe('compiler: text transform', () => {
  it('no consecutive text', () => {
    const { code, ir, helpers } = compileWithTextTransform(
      '<>{ "hello world" }</>',
    )
    expect(code).toMatchSnapshot()
    expect(helpers).contains.all.keys('createNodes')
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.CREATE_NODES,
        values: [
          {
            content: 'hello world',
            isStatic: true,
          },
        ],
      },
    ])
  })

  it('consecutive text', () => {
    const { code, ir, helpers } = compileWithTextTransform('<>{ msg }</>')
    expect(code).toMatchSnapshot()
    expect(helpers).contains.all.keys('createNodes')
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.CREATE_NODES,
        values: [
          {
            content: 'msg',
            isStatic: false,
          },
        ],
      },
    ])
    expect(ir.block.effect.length).toBe(0)
  })

  it('escapes raw static text when generating the template string', () => {
    const { ir } = compileWithTextTransform('<code>&lt;script&gt;</code>')
    expect(ir.templates[0]).toContain('<code>&lt;script&gt;</code>')
    expect(ir.templates[0]).not.toContain('<code><script></code>')
  })
})
