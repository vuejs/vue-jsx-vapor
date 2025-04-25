// import {
//   type BindingMetadata,
//   NodeTypes,
// } from '@vue/compiler-dom'
import { describe, expect, test } from 'vitest'
import {
  // IRDynamicPropsKind,
  // IRNodeTypes,
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVOn,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithElementTransform = makeCompile({
  nodeTransforms: [transformElement, transformChildren, transformText],
  directiveTransforms: {
    bind: transformVBind,
    on: transformVOn,
  },
})

describe('compiler: element transform', () => {
  describe('component', () => {
    test('import + resolve component', () => {
      const { code, helpers } = compileWithElementTransform(`<Foo/>`)
      expect(code).toMatchInlineSnapshot(`
        "
          const n0 = _createComponent(Foo)
          return n0
        "
      `)
      expect(helpers).contains.all.keys('createComponent')
    })
  })

  test('resolve namespaced component from setup bindings (inline const)', () => {
    const { code, helpers } = compileWithElementTransform(`<Foo.Example/>`, {
      inline: true,
    })
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = _createComponent(Foo.Example)
        return n0
      "
    `)
    expect(code).contains(`Foo.Example`)
    expect(helpers).not.toContain('resolveComponent')
  })

  test('props merging: style', () => {
    const { code } = compileWithElementTransform(
      `<Foo style="color: green" style={{ color: 'red' }} />`,
    )
    expect(code).toMatchSnapshot()
  })

  test('props merging: class', () => {
    const { code } = compileWithElementTransform(
      `<Foo class="foo" class={{ bar: isBar }} />`,
    )
    expect(code).toMatchSnapshot()
  })
})
