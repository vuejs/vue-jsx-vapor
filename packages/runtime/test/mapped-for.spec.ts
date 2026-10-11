// @vitest-environment jsdom
import { expect, test } from 'vitest'
import * as Vue from 'vue'
import { createMappedFor } from '../src/vapor'

function mount(source: () => any, flags = 8) {
  const container = document.createElement('div')
  const component = Vue.defineVaporComponent(() =>
    createMappedFor(
      source,
      (item, index) => {
        const node = document.createElement('span')
        Vue.renderEffect(() => {
          node.textContent = `${index.value}:${item.value.name}`
        })
        return node
      },
      (item) => item.id,
      flags,
    ),
  )
  const app = Vue.createVaporApp(component)
  app.mount(container)
  return { container, app }
}

test('keyed map rows are reused and receive updated items and indices', async () => {
  const items = Vue.ref([
    { id: 1, name: 'a' },
    { id: 2, name: 'b' },
  ])
  const { container, app } = mount(() => items.value)
  const [first, second] = container.querySelectorAll('span')
  items.value = [
    { id: 2, name: 'B' },
    { id: 3, name: 'c' },
    { id: 1, name: 'A' },
  ]
  await Vue.nextTick()
  const nodes = container.querySelectorAll('span')
  expect(nodes[0]).toBe(second)
  expect(nodes[2]).toBe(first)
  expect(container.textContent).toBe('0:B1:c2:A')
  items.value = []
  await Vue.nextTick()
  expect(container.querySelectorAll('span')).toHaveLength(0)
  items.value = [{ id: 4, name: 'd' }]
  await Vue.nextTick()
  expect(container.textContent).toBe('0:d')
  app.unmount()
})

test('sparse arrays skip holes and keep the original callback index', async () => {
  const source = [
    { id: 1, name: 'a' },
    { id: 2, name: 'b' },
    { id: 3, name: 'c' },
  ]
  delete source[1]
  const items = Vue.ref(source)
  const { container, app } = mount(() => items.value)
  expect(container.textContent).toBe('0:a2:c')
  items.value[1] = { id: 2, name: 'b' }
  await Vue.nextTick()
  expect(container.textContent).toBe('0:a1:b2:c')
  delete items.value[0]
  await Vue.nextTick()
  expect(container.textContent).toBe('1:b2:c')
  app.unmount()
})

test('custom map methods keep their callback and return value semantics', () => {
  const source = {
    map(callback: (item: any, index: number) => Vue.Block) {
      return [callback({ id: 1, name: 'custom' }, 7), 'tail']
    },
  }
  const { container, app } = mount(() => source)
  expect(container.textContent).toBe('7:customtail')
  app.unmount()
})

test('an array with an overridden map is evaluated by that method', () => {
  const items = [{ id: 1, name: 'a' }]
  items.map = () => ['override'] as any
  const { container, app } = mount(() => items)
  expect(container.textContent).toBe('override')
  app.unmount()
})

test('an own map getter is evaluated only by the original call', () => {
  const items = [{ id: 1, name: 'a' }]
  let calls = 0
  Object.defineProperty(items, 'map', {
    get() {
      calls++
      return Array.prototype.map
    },
  })
  const { container, app } = mount(() => items)
  expect(container.textContent).toBe('0:a')
  expect(calls).toBe(1)
  app.unmount()
})

test('replacing a reactive array map method switches to ordinary evaluation', async () => {
  const items = Vue.ref([{ id: 1, name: 'a' }])
  const { container, app } = mount(() => items.value)
  items.value.map = () => ['override'] as any
  await Vue.nextTick()
  expect(container.textContent).toBe('override')
  Reflect.deleteProperty(items.value, 'map')
  await Vue.nextTick()
  expect(container.textContent).toBe('0:a')
  app.unmount()
})

test('component rows keep local state when items move or are replaced', async () => {
  const items = Vue.ref([
    { id: 1, name: 'a' },
    { id: 2, name: 'b' },
  ])
  let mounts = 0
  const child = Vue.defineVaporComponent(
    (props: any) => {
      mounts++
      const count = Vue.ref(0)
      const node = document.createElement('button')
      node.addEventListener('click', () => count.value++)
      Vue.renderEffect(() => {
        node.textContent = `${props.name}:${count.value}`
      })
      return node
    },
    { props: ['name'] },
  )
  const container = document.createElement('div')
  const app = Vue.createVaporApp(
    Vue.defineVaporComponent(() =>
      createMappedFor(
        () => items.value,
        (item) => Vue.createComponent(child, { name: () => item.value.name }),
        (item) => item.id,
        2,
      ),
    ),
  )
  app.mount(container)
  const first = container.querySelector('button')!
  first.click()
  await Vue.nextTick()
  items.value = [
    { id: 2, name: 'B' },
    { id: 1, name: 'A' },
  ]
  await Vue.nextTick()
  expect(container.querySelectorAll('button')[1]).toBe(first)
  expect(first.textContent).toBe('A:1')
  expect(mounts).toBe(2)
  app.unmount()
})

test('the source getter runs once per invalidation', async () => {
  const items = Vue.ref([{ id: 1, name: 'a' }])
  let calls = 0
  const { app } = mount(() => {
    calls++
    return items.value
  })
  expect(calls).toBe(1)
  items.value = [{ id: 1, name: 'A' }]
  await Vue.nextTick()
  expect(calls).toBe(2)
  app.unmount()
})

test('source invalidation updates a raw array even when its identity is unchanged', async () => {
  const items = [{ id: 1, name: 'a' }]
  const tick = Vue.ref(0)
  const { container, app } = mount(() => {
    Vue.unref(tick)
    return items
  })
  items.push({ id: 2, name: 'b' })
  tick.value++
  await Vue.nextTick()
  expect(container.textContent).toBe('0:a1:b')
  app.unmount()
})

test('ref items retain their value instead of being unwrapped as list aliases', async () => {
  const item = Vue.ref({ id: 1, name: 'a' })
  const items = Vue.shallowRef([item])
  const container = document.createElement('div')
  const app = Vue.createVaporApp(
    Vue.defineVaporComponent(() =>
      createMappedFor(
        () => items.value,
        (item) => {
          const node = document.createElement('span')
          Vue.renderEffect(() => {
            node.textContent = item.value.value.name
          })
          return node
        },
        (item) => item.value.id,
        8,
      ),
    ),
  )
  app.mount(container)
  expect(container.textContent).toBe('a')
  item.value.name = 'A'
  await Vue.nextTick()
  expect(container.textContent).toBe('A')
  items.value = [Vue.ref({ id: 2, name: 'b' })]
  await Vue.nextTick()
  expect(container.textContent).toBe('b')
  app.unmount()
})

test('v-once does not update either list path', async () => {
  for (const sparse of [false, true]) {
    const source = [
      { id: 0, name: 'zero' },
      { id: 1, name: 'a' },
    ]
    if (sparse) delete source[0]
    const items = Vue.ref(source)
    const { container, app } = mount(() => items.value, 4 | 8)
    const content = container.textContent
    items.value = [{ id: 2, name: 'b' }]
    await Vue.nextTick()
    expect(container.textContent).toBe(content)
    app.unmount()
  }
})

test('selector resets follow the reconciled list across fallback switches', async () => {
  const items = Vue.ref([
    { id: 1, name: 'a' },
    { id: 2, name: 'b' },
  ])
  const selected = Vue.ref(1)
  const container = document.createElement('div')
  const app = Vue.createVaporApp(
    Vue.defineVaporComponent(() => {
      const selector = Vue.createSelector(() => selected.value)
      const block = createMappedFor(
        () => items.value,
        (item) => {
          const node = document.createElement('span')
          Vue.renderEffect(() => {
            node.textContent = item.value.name
          })
          selector(item.value.id, () => {
            node.className = item.value.id === selected.value ? 'selected' : ''
          })
          return node
        },
        (item) => item.id,
        8,
      )
      block.onReset(selector.reset)
      return block
    }),
  )
  app.mount(container)
  selected.value = 2
  await Vue.nextTick()
  expect(container.querySelector('.selected')?.textContent).toBe('b')
  items.value = [
    { id: 2, name: 'B' },
    { id: 3, name: 'c' },
  ]
  await Vue.nextTick()
  expect(container.querySelector('.selected')?.textContent).toBe('B')
  delete items.value[0]
  await Vue.nextTick()
  selected.value = 3
  await Vue.nextTick()
  expect(container.querySelector('.selected')?.textContent).toBe('c')
  items.value = [
    { id: 3, name: 'C' },
    { id: 4, name: 'd' },
  ]
  await Vue.nextTick()
  selected.value = 4
  await Vue.nextTick()
  expect(container.querySelector('.selected')?.textContent).toBe('d')
  app.unmount()
})
