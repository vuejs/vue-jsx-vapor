import { describe, expect, test } from 'vitest'
import {
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVFor,
  transformVOn,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithElementTransform = makeCompile({
  nodeTransforms: [
    transformVFor,
    transformElement,
    transformText,
    transformChildren,
  ],
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
          const n0 = _createComponent(Foo, null, null, true)
          return n0
        "
      `)
      expect(helpers).contains.all.keys('createComponent')
    })
  })

  test('resolve namespaced component from setup bindings (inline const)', () => {
    const { code, helpers } = compileWithElementTransform(`<Foo.Example/>`)
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = _createComponent(Foo.Example, null, null, true)
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

  test('generate single root component', () => {
    const { code } = compileWithElementTransform(`<Comp/>`)
    expect(code).toMatchSnapshot()
    expect(code).contains('_createComponent(Comp, null, null, true)')
  })

  test('Fragment should not mark as single root', () => {
    const { code } = compileWithElementTransform(`<><Comp/></>`)
    expect(code).toMatchSnapshot()
    expect(code).contains('_createComponent(Comp)')
  })

  test('v-for on component should not mark as single root', () => {
    const { code } = compileWithElementTransform(
      `<Comp v-for={item in items} key={item}/>`,
    )
    expect(code).toMatchSnapshot()
    expect(code).contains('_createComponent(Comp)')
  })

  test('component with fallback', () => {
    const { code } = compileWithElementTransform(`<foo-bar />`)
    expect(code).toMatchSnapshot()
    expect(code).contains(
      '_createComponentWithFallback(_component_foo_bar, null, null, true)',
    )
  })

  test('number value', () => {
    const { code } = compileWithElementTransform(`<div foo={1} />`)
    expect(code).toMatchSnapshot()
    expect(code).not.contains('_setProp(n0, "foo", 1)')
  })
})
