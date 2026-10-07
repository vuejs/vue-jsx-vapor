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

async function compile(code: string, resolveType = false): Promise<Record<string, any>> {
  const result = await transformJsxMacros(code, 'component.tsx', new Map(), {
    ...options,
    defineComponent: { ...options.defineComponent, resolveType },
  } as any)
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

/** `{ name: required }` for each declared prop. */
function declaredProps(Comp: any) {
  const props = Comp.props ?? {}
  return Object.fromEntries(
    (Array.isArray(props) ? props : Object.keys(props)).map((key: string) => [
      key,
      !!props[key]?.required,
    ]),
  )
}

function declaredEmits(Comp: any) {
  const emits = Comp.emits ?? []
  return (Array.isArray(emits) ? emits : Object.keys(emits)).toSorted((a: string, b: string) =>
    a.localeCompare(b),
  )
}

const PROPS = `
import type { VNode } from 'vue'
import type { Company } from './company'

interface Props {
  label: string
  count: number
  enabled: boolean
  note?: string
  nullable?: string | null
  flag?: boolean
  tags?: Set<string>
  company?: Company
  renderExtra?: () => VNode[]
}
`

// keys the body never reads are declared too, so they stay out of attrs
const EXPECTED_PROPS = {
  label: true,
  count: true,
  enabled: true,
  note: false,
  nullable: false,
  flag: false,
  tags: false,
  company: false,
  renderExtra: false,
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

// as @vitejs/plugin-vue-jsx derives them with `resolveType: true` (@vue/babel-plugin-resolve-type)
describe('resolveType: props from the setup parameter type', () => {
  test('interface, single parameter', async () => {
    const { Comp } = await compile(
      `${PROPS}
import { defineComponent } from 'vue'
export const Comp = defineComponent((props: Props) => () => props.label)
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual(EXPECTED_PROPS)
  })

  test('type alias', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
type Props = { label: string; note?: string }
export const Comp = defineComponent((props: Props) => () => props.label)
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual({ label: true, note: false })
  })

  test('inline type literal', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
export const Comp = defineComponent(
  (props: { label: string; note?: string }) => () => props.label,
)
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual({ label: true, note: false })
  })

  test('alongside a typed setup context and an options argument', async () => {
    const { Comp } = await compile(
      `${PROPS}
import { defineComponent, type SetupContext } from 'vue'
type Emits = { change(value: string): void }
export const Comp = defineComponent(
  (props: Props, { emit }: SetupContext<Emits>) => () => props.label,
  { name: 'Comp', inheritAttrs: false },
)
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual(EXPECTED_PROPS)
    expect(Comp.name).toBe('Comp')
  })

  test('async setup', async () => {
    const { Comp } = await compile(
      `${PROPS}
import { defineComponent } from 'vue'
export const Comp = defineComponent(async (props: Props) => {
  await Promise.resolve()
  return () => props.label
})
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual(EXPECTED_PROPS)
  })

  test('destructured parameter declares the keys it leaves out', async () => {
    const { Comp } = await compile(
      `${PROPS}
import { defineComponent } from 'vue'
export const Comp = defineComponent(({ label, note = 'none' }: Props) => () => label + note)
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual(EXPECTED_PROPS)
  })

  test('a rest element holds the props not destructured', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
type Props = { label: string; note?: string; extra?: string }
export const Comp = defineComponent(({ label, ...others }: Props) => () => JSON.stringify(others))
`,
      true,
    )
    const vm = mount(Comp, { label: 'a', extra: 'b', 'data-x': 'c' })
    expect(declaredProps(Comp)).toEqual({ label: true, note: false, extra: false })
    expect(JSON.parse(vm.$el.textContent)).toEqual({ extra: 'b' })
    expect(vm.$attrs).toEqual({ 'data-x': 'c' })
  })

  test('@vue-ignore leaves a type out', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type InputHTMLAttributes } from 'vue'
type Props = { label: string } & /* @vue-ignore */ InputHTMLAttributes
export const Comp = defineComponent((props: Props) => () => props.label)
`,
      true,
    )
    expect(declaredProps(Comp)).toEqual({ label: true })
  })

  test('an absent optional boolean is false', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
export const Comp = defineComponent((props: { flag?: boolean }) => () => String(props.flag))
`,
      true,
    )
    expect(mount(Comp).$props.flag).toBe(false)
  })

  test('an absent destructured optional boolean is false', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
export const Comp = defineComponent(({ flag }: { flag?: boolean }) => () => String(flag))
`,
      true,
    )
    expect(mount(Comp).$props.flag).toBe(false)
  })
})

describe('without resolveType', () => {
  test('declares only the props the body reads, and no emits', async () => {
    const { Comp } = await compile(`${PROPS}
import { defineComponent, type SetupContext } from 'vue'
type Emits = { change(value: string): void }
export const Comp = defineComponent((props: Props, { emit }: SetupContext<Emits>) => () => props.label)
`)
    expect(declaredProps(Comp)).toEqual({ label: false })
    expect(declaredEmits(Comp)).toEqual([])
  })

  test('a rest element holds the attrs', async () => {
    const { Comp } = await compile(`
import { defineComponent } from 'vue'
type Props = { label: string; extra?: string }
export const Comp = defineComponent(({ label, ...others }: Props) => () => JSON.stringify(others))
`)
    const vm = mount(Comp, { label: 'a', extra: 'b' })
    expect(JSON.parse(vm.$el.textContent)).toEqual({ extra: 'b' })
    expect(Comp.inheritAttrs).toBe(false)
  })
})

describe('resolveType: emits from the setup context type', () => {
  test('SetupContext with a type alias of method signatures', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
type Emits = {
  change(value: string): void
  close(): void
  'update:modelValue'(value: number): void
}
export const Comp = defineComponent((props: { label: string }, { emit }: SetupContext<Emits>) => () => props.label)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change', 'close', 'update:modelValue'])
  })

  test('SetupContext with an interface', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
interface Emits {
  change(value: string): void
  close(): void
}
export const Comp = defineComponent((props: { label: string }, { emit }: SetupContext<Emits>) => () => props.label)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change', 'close'])
  })

  test('SetupContext with tuple and function-property syntax', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
type Emits = {
  change: [value: string]
  close: () => void
}
export const Comp = defineComponent((props: { label: string }, { emit }: SetupContext<Emits>) => () => props.label)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change', 'close'])
  })

  test('EmitFn on the destructured context (#66)', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type EmitFn } from 'vue'
type Events = {
  change(value: string): void
  close(): void
}
export const Comp = defineComponent(
  (props: { label: string }, { emit }: { emit: EmitFn<Events> }) => () => props.label,
)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change', 'close'])
  })

  test('context destructured with attrs, slots and expose', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
type Emits = { change(value: string): void }
export const Comp = defineComponent(
  (props: { label: string }, { attrs, slots, emit, expose }: SetupContext<Emits>) => {
    expose({})
    return () => [props.label, attrs, slots].length
  },
)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change'])
  })

  test('context not destructured', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
type Emits = { change(value: string): void }
export const Comp = defineComponent((props: { label: string }, ctx: SetupContext<Emits>) => () => props.label)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change'])
  })

  test('one events type shared by several components', async () => {
    const { First, Second } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
type Events = { change(id: number): void }
export const First = defineComponent((props: { a: string }, { emit }: SetupContext<Events>) => () => props.a)
export const Second = defineComponent((props: { b: string }, { emit }: SetupContext<Events>) => () => props.b)
`,
      true,
    )
    expect(declaredEmits(First)).toEqual(['change'])
    expect(declaredEmits(Second)).toEqual(['change'])
  })

  test('declared emit listeners stay out of attrs', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, type SetupContext } from 'vue'
type Emits = { change(value: string): void }
export const Comp = defineComponent((props: { label: string }, { emit }: SetupContext<Emits>) => () => props.label)
`,
      true,
    )
    expect(mount(Comp, { label: 'x', onChange: () => {} }).$attrs).toEqual({})
  })
})

// The cases in vuejs/babel-plugin-jsx packages/babel-plugin-resolve-type/test/resolve-type.test.tsx
describe('resolveType: @vue/babel-plugin-resolve-type parity', () => {
  test('intersection of interfaces', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, h } from 'vue'
interface Props { msg: string; optional?: boolean }
interface Props2 { set: Set<string> }
export const Comp = defineComponent((props: Props & Props2) => () => h('div', props.msg))
`,
      true,
    )
    expect(Comp.props).toMatchObject({
      msg: { type: String, required: true },
      optional: { type: Boolean, required: false },
      set: { type: Set, required: true },
    })
  })

  test('props from a type argument', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, h } from 'vue'
interface Props { msg: string; optional?: boolean }
export const Comp = defineComponent<Props>((props) => () => h('div', props.msg))
`,
      true,
    )
    expect(Comp.props).toMatchObject({
      msg: { type: String, required: true },
      optional: { type: Boolean, required: false },
    })
  })

  test('static parameter default with a type argument', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, h } from 'vue'
type Props = { msg: string; optional?: boolean }
export const Comp = defineComponent<Props>((props = { msg: 'hello' }) => () => h('div', props.msg))
`,
      true,
    )
    expect(Comp.props.msg).toMatchObject({ type: String, default: 'hello' })
    expect(mount(Comp).$props.msg).toBe('hello')
  })

  test('static parameter default with an annotation', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, h } from 'vue'
export const Comp = defineComponent((props: { msg?: string } = { msg: 'hello' }) => () => h('div', props.msg))
`,
      true,
    )
    expect(mount(Comp).$props.msg).toBe('hello')
  })

  test('dynamic parameter default', async () => {
    const { Comp } = await compile(
      `
import { defineComponent, h } from 'vue'
const defaults = { msg: 'hello' }
export const Comp = defineComponent((props: { msg?: string } = defaults) => () => h('div', props.msg))
`,
      true,
    )
    expect(mount(Comp).$props.msg).toBe('hello')
  })

  test('array of object literals', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
interface MyProps { list?: Array<{ id: number; title: string }> }
export const Comp = defineComponent((props: MyProps) => () => props.list?.length)
`,
      true,
    )
    expect(Comp.props).toMatchObject({ list: { type: Array, required: false } })
  })

  test('emits from an inline SetupContext type', async () => {
    const { Comp } = await compile(
      `
import { type SetupContext, defineComponent } from 'vue'
export const Comp = defineComponent(
  (props, { emit }: SetupContext<{ change(val: string): void; click(): void }>) => () => null,
)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change', 'click'])
  })

  test('emits from a type argument', async () => {
    const { Comp } = await compile(
      `
import { defineComponent } from 'vue'
type EmitEvents = { change(val: string): void; click(): void }
export const Comp = defineComponent<{}, EmitEvents>((props, { emit }) => () => null)
`,
      true,
    )
    expect(declaredEmits(Comp)).toEqual(['change', 'click'])
  })

  test('infers the name from the variable', async () => {
    const { Foo, Bar, Baz } = await compile(
      `
import { defineComponent } from 'vue'
const opts = { inheritAttrs: true }
export const Foo = defineComponent(() => () => null)
export const Bar = defineComponent(() => () => null, { foo: 'bar' } as any)
export const Baz = defineComponent(() => () => null, opts)
`,
      true,
    )
    expect(Foo.name).toBe('Foo')
    expect(Bar).toMatchObject({ name: 'Bar', foo: 'bar' })
    expect(Baz).toMatchObject({ name: 'Baz', inheritAttrs: true })
  })

  test('an explicit name wins', async () => {
    const { Foo } = await compile(
      `
import { defineComponent } from 'vue'
export const Foo = defineComponent(() => () => null, { name: 'Explicit' })
`,
      true,
    )
    expect(Foo.name).toBe('Explicit')
  })

  test('options passed as a spread are left alone', async () => {
    const { Foo } = await compile(
      `
import { defineComponent } from 'vue'
const args = [{ inheritAttrs: true }] as const
export const Foo = defineComponent(() => () => null, ...args)
`,
      true,
    )
    expect(Foo).toMatchObject({ inheritAttrs: true })
    expect(Foo.name).not.toBe('Foo')
  })
})
