// @vitest-environment happy-dom
import { createApp, createVaporApp } from 'vue'
import { expect, test } from 'vite-plus/test'
import { defineVaporComponent } from '../src/vapor'
import { defineComponent } from '../src/vdom'

// A vapor setup returns a DOM block; a vdom setup returns a render function.
const vaporSetup = () => document.createElement('div')
const vdomSetup = () => () => null

test('defineVaporComponent defaults inheritAttrs to false', () => {
  expect((defineVaporComponent(vaporSetup) as any).inheritAttrs).toBe(false)
})

test('defineVaporComponent keeps an explicit inheritAttrs', () => {
  expect((defineVaporComponent(vaporSetup, { inheritAttrs: true }) as any).inheritAttrs).toBe(true)
})

test('defineVaporComponent defaults inheritAttrs on the options object form', () => {
  expect((defineVaporComponent({ setup: vaporSetup }) as any).inheritAttrs).toBe(false)
})

test('defineComponent defaults inheritAttrs to false', () => {
  expect((defineComponent(vdomSetup) as any).inheritAttrs).toBe(false)
  expect((defineComponent(vdomSetup, { inheritAttrs: true }) as any).inheritAttrs).toBe(true)
  expect((defineComponent({ setup: vdomSetup }) as any).inheritAttrs).toBe(false)
})

test('arity is never inspected, so rest-args setups still get attrs', () => {
  // `(...args)` and `(props = {})` report `length === 0` yet do use the argument, so a
  // zero-arg setup is wrapped like any other. Only a declared `props` option skips it.
  const seen: any[] = []
  const Comp = defineComponent((...args: any[]) => {
    seen.push(args[0])
    return () => null
  }) as any

  Comp.setup({}, { attrs: { foo: 1 } })

  expect(seen[0]).toMatchObject({ foo: 1 })
  expect((defineVaporComponent(vaporSetup) as any).setup).not.toBe(vaporSetup)
})

test('a component without props receives attrs as its first argument', () => {
  const seen: any[] = []
  // Declared with two parameters so the attrs can be read off the context.
  const Comp = defineVaporComponent((props: any, _ctx: any) => {
    seen.push(props)
    return document.createElement('div')
  }) as any

  const attrs = { foo: 1, bar: 2 }
  Comp.setup({}, { attrs })

  expect(seen[0]).toBe(attrs)
})

test('any props option is respected, even an empty one', () => {
  // Only the *presence* of `props` is checked — its contents are never inspected, so an
  // explicit `[]` / `{}` counts as a decision just like a populated declaration does.
  expect((defineVaporComponent(vaporSetup, { props: { foo: {} } }) as any).setup).toBe(vaporSetup)
  expect((defineVaporComponent(vaporSetup, { props: ['foo'] }) as any).setup).toBe(vaporSetup)
  expect((defineVaporComponent(vaporSetup, { props: [] }) as any).setup).toBe(vaporSetup)
  expect((defineVaporComponent(vaporSetup, { props: {} }) as any).setup).toBe(vaporSetup)
})

test('defineComponent respects a props option too', () => {
  expect((defineComponent(vdomSetup, { props: { foo: {} } }) as any).setup).toBe(vdomSetup)
  expect((defineComponent(vdomSetup, { props: [] }) as any).setup).toBe(vdomSetup)
})

// --- integration: what the real runtime actually hands over ---

const mount = (Comp: any, props: any = {}) => {
  const root = document.createElement('div')
  // props go to `createVaporApp(comp, props)` — `mount(root, isHydrate, namespace)` takes
  // hydration flags, so passing props there silently switches on hydration.
  createVaporApp(Comp, props).mount(root)
  return root
}

test('integration: a props-only setup receives attrs (widened, reads ctx.attrs)', () => {
  let received: any
  const Comp = defineVaporComponent((props: any) => {
    received = props
    return document.createElement('div')
  })

  mount(Comp, { foo: 1, bar: 'x' })

  expect(received).toMatchObject({ foo: 1, bar: 'x' })
})

test('integration: a two-arg setup receives attrs (goes through ctx.attrs)', () => {
  let received: any
  const Comp = defineVaporComponent((props: any, _ctx: any) => {
    received = props
    return document.createElement('div')
  })

  mount(Comp, { foo: 1, bar: 'x' })

  expect(received).toMatchObject({ foo: 1, bar: 'x' })
})

test('integration: inheritAttrs false keeps attrs off the root element', () => {
  const Comp = defineVaporComponent(() => document.createElement('div'))
  expect(mount(Comp, { 'data-foo': '1' }).innerHTML).toBe('<div></div>')
})

test('integration: inheritAttrs true still falls through', () => {
  const Comp = defineVaporComponent(() => document.createElement('div'), {
    inheritAttrs: true,
  })
  expect(mount(Comp, { 'data-foo': '1' }).innerHTML).toBe('<div data-foo="1"></div>')
})

test('integration (vdom): a props-only setup receives attrs', () => {
  let received: any
  const Comp = defineComponent((props: any) => {
    received = props
    return () => null
  })

  createApp(Comp, { foo: 1 }).mount(document.createElement('div'))

  expect(received).toMatchObject({ foo: 1 })
})
