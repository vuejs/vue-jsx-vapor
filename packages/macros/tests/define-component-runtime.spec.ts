// @vitest-environment happy-dom
import { mkdir, rm, writeFile } from 'node:fs/promises'
import path from 'node:path'
import { createApp } from 'vue'
import { afterAll, describe, expect, test } from 'vite-plus/test'
import { transformJsxMacros } from '../src/core'

// Each source goes through the macros and is then imported, so the assertions are on the
// component an app would actually get.

const options = {
  include: ['*.tsx'],
  version: 3.6,
  defineModel: { alias: ['defineModel'] },
  defineSlots: { alias: ['defineSlots'] },
  defineStyle: { alias: ['defineStyle'] },
  defineExpose: { alias: ['defineExpose'] },
  defineComponent: { alias: ['defineComponent', 'defineVaporComponent'] },
}

// what the plugin's `resolveId`/`load` hooks serve for these
const helpers: Record<string, string> = {
  '/vue-jsx/props': path.resolve(import.meta.dirname, '../../runtime/src/props.ts'),
  '/vue-jsx/macros/': `${path.resolve(import.meta.dirname, '../src/core/helper')}/`,
}

// vitest only imports files under the project root
const tempDir = path.join(import.meta.dirname, '.temp')
let compiled = 0

afterAll(() => rm(tempDir, { recursive: true, force: true }))

async function compile(code: string): Promise<Record<string, any>> {
  const result = await transformJsxMacros(code, 'component.tsx', new Map(), options as any)
  const file = path.join(tempDir, `component-${compiled++}.ts`)
  await mkdir(tempDir, { recursive: true })
  await writeFile(
    file,
    (result?.code ?? code).replaceAll(
      /(["'])\/vue-jsx\/(props|macros\/)/g,
      (_, quote, rest) => quote + helpers[`/vue-jsx/${rest}`],
    ),
  )
  return import(/* @vite-ignore */ file)
}

function mount(Comp: any, props: Record<string, unknown> = {}) {
  return createApp(Comp, props).mount(document.createElement('div')) as any
}

describe('destructured defaults', () => {
  test('literals', async () => {
    const { Comp } = await compile(`
import { defineComponent } from 'vue'
export const Comp = defineComponent(
  ({
    str = 'a',
    num = 1,
    bool = false,
    nil = null as string | null,
    arr = [] as string[],
    obj = { a: 1 },
  }) =>
    () => [str, num, bool, nil, arr, obj].length,
)
`)
    expect(mount(Comp).$props).toEqual({
      str: 'a',
      num: 1,
      bool: false,
      nil: null,
      arr: [],
      obj: { a: 1 },
    })
  })

  test('other expressions', async () => {
    const { Comp } = await compile(`
import { defineComponent } from 'vue'
const LIMIT = 10
const Units = { mm: 'mm' } as const
const initial = () => 'initial'
export const Comp = defineComponent(
  ({
    negative = -1,
    member = Units.mm,
    template = \`\${LIMIT}px\`,
    set = new Set<string>(),
    map = new Map<string, number>(),
    call = initial(),
    identifier = LIMIT,
  }) =>
    () => [negative, member, template, set, map, call, identifier].length,
)
`)
    expect(mount(Comp).$props).toEqual({
      negative: -1,
      member: 'mm',
      template: '10px',
      set: new Set(),
      map: new Map(),
      call: 'initial',
      identifier: 10,
    })
  })

  test('object and array defaults are fresh per instance', async () => {
    const { Comp } = await compile(`
import { defineComponent } from 'vue'
export const Comp = defineComponent(
  ({ set = new Set<string>(), arr = [] as string[] }) => () => [set, arr].length,
)
`)
    const [a, b] = [mount(Comp).$props, mount(Comp).$props]
    expect(a.set).not.toBe(b.set)
    expect(a.arr).not.toBe(b.arr)
  })
})
