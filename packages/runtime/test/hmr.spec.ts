// @vitest-environment happy-dom
import { expect, test, vi } from 'vite-plus/test'
import { createApp, defineComponent, h, nextTick } from 'vue'
import { defineVaporHmrComponent } from '../src/vapor'
import { defineHmrComponent } from '../src/vdom'

// Two call paths have to end up in the current implementation: a mounted
// instance re-renders through `render`, and a fresh mount calls the component
// itself. Only `render` is patched by an update, so the component forwards
// through it.
test('defineVaporHmrComponent re-renders a mounted instance through render', () => {
  const first: any = defineVaporHmrComponent(() => 'v1')
  const second: any = defineVaporHmrComponent(() => 'v2')

  // what `rerender` does to a mounted instance, which calls
  // `render(setupState, props, ...)`
  first.render = second.render

  expect(first.render({}, { n: 1 })).toBe('v2')
})

test('defineVaporHmrComponent mounts a captured binding with the current body', () => {
  const first: any = defineVaporHmrComponent(() => 'v1')
  const second: any = defineVaporHmrComponent(() => 'v2')

  first.render = second.render

  // a fresh mount runs the component itself, as `(props, instance)`
  expect(first({ n: 1 }, null)).toBe('v2')
})

test('defineVaporHmrComponent survives a reload of its module', () => {
  const first: any = defineVaporHmrComponent(() => 'v1')
  const second: any = defineVaporHmrComponent(() => 'v2')

  // what `reload` does: `extend(record.initialDef, newComp)`
  Object.assign(first, second)

  expect(first.render({}, { n: 1 })).toBe('v2')
  expect(first({ n: 1 }, null)).toBe('v2')
})

test('defineVaporHmrComponent keeps both call signatures', () => {
  const impl = vi.fn((_props: any, _instance: any) => 'v1')
  const component: any = defineVaporHmrComponent(impl)

  // the component path forwards both arguments — and must keep declaring two
  // parameters: Vue passes `null` instead of the context when a functional
  // component declares fewer than two (`render.length > 1`)
  expect(component.length).toBe(2)
  expect(component({ n: 1 }, { slots: {} })).toBe('v1')
  expect(impl.mock.lastCall![0]).toEqual({ n: 1 })
  expect(impl.mock.lastCall![1]).toEqual({ slots: {} })

  // the render path rebuilds the mount signature and reads the instance from
  // the current render context
  expect(component.render({}, { n: 2 })).toBe('v1')
  expect(impl.mock.lastCall![0]).toEqual({ n: 2 })
})

// The compiler wraps by name convention, so a wrapped binding can be an
// ordinary function — e.g. an exported hook — whose extra arguments must
// survive the wrapper (useClosable(config, context, fallback) rendered no
// close icon when the third argument was dropped).
test('defineVaporHmrComponent forwards every argument to the implementation', () => {
  const impl = vi.fn(() => 'v1')
  const component: any = defineVaporHmrComponent(impl)

  expect(component('a', 'b', 'c')).toBe('v1')
  expect(impl.mock.lastCall).toEqual(['a', 'b', 'c'])
})

test('defineHmrComponent forwards every argument to the implementation', () => {
  const impl = vi.fn(() => 'v1')
  const component: any = defineHmrComponent(impl)

  expect(component('a', 'b', 'c')).toBe('v1')
  expect(impl.mock.lastCall).toEqual(['a', 'b', 'c'])
})

// The tests below mount the component for real. What they pin is the primitive
// both update paths rely on: an interop component is a plain function, so the
// vdom runtime mounts it as a FUNCTIONAL_COMPONENT (`setupStatefulComponent` is
// skipped, `instance.render` is never read), but it is still a full instance
// that `mountComponent` registers through `__hmrId`. Nothing is refreshed
// without a registered instance, so these tests fail if that ever stops
// happening.
//
// Measured caveat, not reproducible in jsdom: during a real HMR flush a vdom
// (interop) function component updated with `rerender` keeps rendering the old
// body until it remounts, which is why the compiler emits `reload` for interop
// function components. Calling the APIs directly, as below, does refresh.
const hmr = () => (globalThis as any).__VUE_HMR_RUNTIME__

function mountInterop(id: string, impl: any, wrap: any = defineVaporHmrComponent) {
  const component: any = wrap(impl)
  component.__hmrId = id
  hmr().createRecord(id, component)

  const root = document.createElement('div')
  createApp(defineComponent({ render: () => h(component, { n: 1 }) })).mount(root)
  return root
}

test('rerender refreshes a mounted vdom function component', async () => {
  const root = mountInterop('interop-rerender', (props: any) => `v1:${props.n}`)
  expect(root.textContent).toBe('v1:1')

  // what the emitted ternary passes for a wrapped component: `mod.Comp.render`
  const next: any = defineVaporHmrComponent((props: any) => `v2:${props.n}`)
  hmr().rerender('interop-rerender', next.render)
  await nextTick()
  expect(root.textContent).toBe('v2:1')
})

test('reload refreshes a mounted vdom function component', async () => {
  const root = mountInterop('interop-reload', (props: any) => `v1:${props.n}`)
  expect(root.textContent).toBe('v1:1')

  // what the interop branch emits: `reload(mod.Comp.__hmrId, mod.Comp)`
  const next: any = defineVaporHmrComponent((props: any) => `v2:${props.n}`)
  hmr().reload('interop-reload', next)
  await nextTick()
  expect(root.textContent).toBe('v2:1')
})

test('reload refreshes a mounted vdom wrapper component', async () => {
  const root = mountInterop(
    'interop-vdom-wrapper',
    (props: any) => `v1:${props.n}`,
    defineHmrComponent,
  )
  expect(root.textContent).toBe('v1:1')

  // what the interop branch emits: `reload(mod.Comp.__hmrId, mod.Comp)` — the
  // new wrapper's `__hmrImpl` is copied onto the registered one
  const next: any = defineHmrComponent((props: any) => `v2:${props.n}`)
  hmr().reload('interop-vdom-wrapper', next)
  await nextTick()
  expect(root.textContent).toBe('v2:1')
})
