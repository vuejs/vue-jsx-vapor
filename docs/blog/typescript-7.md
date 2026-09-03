# Native TypeScript 7 Support: Props, Refs, And Children Without Volar

Vue JSX 3.3 makes ordinary component inference a TypeScript feature, not an
editor-plugin trick. With `jsxImportSource: "vue-jsx"`, TypeScript reads the
JSX namespace exported by `vue-jsx/jsx-runtime`, and that namespace teaches the
compiler how Vue components should look at JSX call sites.

The key is `JSX.LibraryManagedAttributes`. TypeScript calls this type whenever
it checks `<Comp ... />`. Vue JSX uses that hook to rewrite the raw component
props into the actual JSX-facing props: normal props stay normal, emitted events
become `onXxx`, `ref` points at the exposed type, and JSX children are checked
as Vue slots.

[简体中文](/zh/blog/typescript-7)

## The Entry Point

The only default TypeScript setup is the JSX runtime:

```json
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

`vue-jsx/jsx-runtime/index.d.ts` exports the runtime JSX namespace and also
places it in the global type space:

```ts
import type { Fragment, VNode } from 'vue'
export type { JSX } from 'vue-jsx'

declare global {
  export type { JSX } from 'vue-jsx'
}

declare function jsx(type: any, props: any, key: any): VNode

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }
```

That is enough for TypeScript itself to ask Vue JSX three questions:

1. What counts as a JSX element?
2. Where should component props be read from?
3. How should the raw props be rewritten for this component?

The namespace answers those questions in `packages/runtime/src/jsx.ts`:

```ts
export namespace JSX {
  export type Element = RenderResult

  export interface ElementAttributesProperty {
    $props: {}
  }

  export interface ElementChildrenAttribute {
    'v-slots': {}
  }

  export interface IntrinsicElements extends NativeElements {
    [name: string]: any
  }

  export interface IntrinsicAttributes extends Omit<ReservedProps, 'ref'> {
    class?: ClassValue | undefined
    style?: StyleValue | undefined
  }

  export type LibraryManagedAttributes<Component, Props> = // ...
}
```

`ElementAttributesProperty` makes constructor-style Vue components expose their
JSX props through `$props`. `ElementChildrenAttribute` says JSX children are not
React-style `children`; they flow through `v-slots`. `LibraryManagedAttributes`
then performs the real type-level adaptation.

## What LibraryManagedAttributes Does

Here is the important part of the type, copied from the runtime declarations and
trimmed only around unrelated DOM attributes:

```ts
type DistributiveOmit<T, K extends PropertyKey> = T extends unknown
  ? Omit<T, K>
  : never

export type LibraryManagedAttributes<Component, Props> =
  DistributiveOmit<Props, 'ref'> &
    (Component extends abstract new (...args: any[]) => infer Instance
      ? {
          ref?: NodeRef<
            ExtractExposed<
              Props,
              'exposed' extends keyof Instance
                ? string extends keyof NonNullable<
                    NonNullable<Instance['exposed']>
                  >
                  ? Instance
                  : UnwrapRef<Instance['exposed']>
                : Instance
            >
          >
        } & ('v-slots' extends keyof Props
          ? {}
          : '$slots' extends keyof Instance
            ? SlotsToProps<Instance['$slots'] & {}>
            : 'slots' extends keyof Instance
              ? SlotsToProps<Instance['slots'] & {}>
              : {})
      : Component extends (
            props: Props,
            ctx: {
              slots: infer Slots
              attrs: any
              emit: infer Emit
              expose: (
                exposed: infer Exposed extends Record<string, any>,
              ) => void
            },
          ) => any
        ? {
            ref?: 'ref' extends keyof Props
              ? Props['ref']
              : NodeRef<
                  string extends keyof Exposed
                    ? NativeElement | VaporComponentInstance
                    : UnwrapRef<Exposed>
                >
          } & EmitFnToProps<Emit, keyof Props> &
            ('v-slots' extends keyof Props ? {} : SlotsToProps<Slots & {}>)
        : {
            ref?: VNodeRef
          })
```

The type has three branches.

1. Constructor components, including components returned by Vue's
   `defineComponent`, expose an instance type. Vue JSX reads `$props`, `$slots`
   or `slots`, and the optional `exposed` shape from that instance.

2. Direct function components expose their information through the function
   signature: first parameter for props, second parameter for `slots`, `emit`,
   and `expose`.

3. Unknown components fall back to Vue's regular `VNodeRef`.

Before any branch runs, `DistributiveOmit<Props, 'ref'>` removes the raw `ref`
field from the component props. `ref` is special in Vue JSX, so the type deletes
it first and then rebuilds it from the actual component shape.

## Props And Emits

Plain props are the easy part: after `ref` is removed, the original `Props` type
remains in the attribute type. That is why literal props, generic props, unions,
and required props still behave like normal TypeScript.

Emits are added only for function-style components. The helper is small:

```ts
export type EmitFnToProps<T, ExcludeKeys extends PropertyKey = ''> =
  T extends (event: infer Event extends string, ...args: infer Args) => any
    ? string extends Event
      ? {}
      : {
          readonly [K in Event as `on${Capitalize<K>}` extends ExcludeKeys
            ? never
            : `on${Capitalize<K>}`]?: (...args: Args) => any
        }
    : {}
```

If `emit` can be called as `emit('change', value)`, the JSX call site gets an
`onChange` prop whose callback receives the same payload. If the prop already
exists, `ExcludeKeys` prevents the helper from generating a duplicate key.

```tsx
import { type EmitFn } from 'vue'

const Counter = (
  props: { value: number },
  { emit }: { emit: EmitFn<{ change: [value: number] }> },
) => <button onClick={() => emit('change', props.value + 1)} />

;<Counter
  value={1}
  onChange={(value) => {
    value.toFixed()
  }}
/>
```

No Volar plugin needs to synthesize `onChange`. It is produced by
`LibraryManagedAttributes` during TypeScript's own JSX checking.

## Ref Means Exposed

`ref` is where Vue JSX differs most from a simple `VNodeRef` pass-through. The
public component type already knows what the component exposes; the JSX layer
only has to extract it.

```ts
export type NodeRef<T> =
  | ((ref: T | null, refs: Record<string, any>) => void)
  | Ref
  | string

declare const exposedType: unique symbol

export type ExtractExposed<Props, Default = never> =
  typeof exposedType extends keyof Props
    ? Exclude<Props[typeof exposedType], undefined>
    : Default

export type ExposedToProps<T extends Record<string, any>> =
  string extends keyof T
    ? {}
    : [keyof T] extends [never]
      ? {}
      : {
          readonly [exposedType]?: T
          readonly ref?: NodeRef<T>
        }
```

For constructor components, `LibraryManagedAttributes` looks for an `exposed`
field on the component instance. If it is specific, `ref` receives
`UnwrapRef<Instance['exposed']>`. If the exposed object is too wide, it falls
back to the full instance type.

For direct function components, the `ctx.expose()` parameter becomes the ref
target:

```tsx
import { computed, type Ref } from 'vue'

const Doubler = (
  props: { count: number },
  { expose }: { expose: (exposed: { double: Ref<number> }) => void },
) => {
  const double = computed(() => props.count * 2)
  expose({ double })
  return <span>{props.count}</span>
}

;<Doubler
  count={2}
  ref={(exposed) => {
    exposed?.double.toFixed()
  }}
/>
```

The callback sees `{ double: number } | null`, because the type path goes through
`UnwrapRef`.

## Children Are Slots

The slot mapping is the most important part for Vue users. JSX children are
syntax, but Vue children are slots. Vue JSX connects the two with
`ElementChildrenAttribute` and `SlotsToProps`.

```ts
type ResolveSlots<Slots> = {
  readonly [Key in keyof Slots]?: Slots[Key] extends (
    ...args: infer Args
  ) => VNode | VNode[]
    ? (...args: Args) => NodeChild
    : Slots[Key]
}

export type SlotsToProps<
  RawSlots extends SlotsType | Record<string, any> = Record<string, any>,
  Slots = ResolveSlots<
    RawSlots extends SlotsType
      ? SetupContext<EmitsOptions, RawSlots>['slots']
      : RawSlots
  >,
> = string extends keyof Slots
  ? {}
  : [keyof Slots] extends [never]
    ? {}
    : {
        readonly 'v-slots'?:
          | ('default' extends keyof Slots ? Slots['default'] | Slots : Slots)
          | NoInfer<NodeChild>
      }
```

This helper does several small but important things:

1. It accepts both Vue `SlotsType` and plain slot records.
2. It keeps slot parameter types while relaxing return values to Vue JSX's
   `NodeChild`.
3. It returns nothing for open-ended or empty slot objects, so untyped
   components do not become noisy.
4. It models the default slot ergonomics: if a component has `default`, the
   caller may pass the default slot function directly as children, or pass a
   slot object through `v-slots`.

```tsx
import { defineComponent } from 'vue-jsx'

const Panel = defineComponent(
  (
    props: { title: string },
    {
      slots,
    }: {
      slots: {
        default?: (scope: { active: boolean }) => JSX.Element
        footer?: (scope: { close: () => void }) => JSX.Element
      }
    },
  ) => {
    return () => (
      <section>
        <h2>{props.title}</h2>
        {slots.default?.({ active: true })}
      </section>
    )
  },
  { props: ['title'] },
)

;<Panel title="Settings">
  {({ active }) => <div>{active ? 'open' : 'closed'}</div>}
</Panel>

;<Panel
  title="Settings"
  v-slots={{
    default: ({ active }) => <div>{active}</div>,
    footer: ({ close }) => <button onClick={close}>Close</button>,
  }}
/>
```

`active` and `close` are inferred from the component's slot type. The caller did
not import a Volar plugin and did not write a generated helper file.

## Why This Removes The Volar Requirement

Older JSX typing setups often depended on editor plugins because standard
TypeScript did not see enough Vue-specific intent at the JSX call site. The
plugin had to create a virtual file or extra type context so that props, emits,
slots, and refs could be checked like Vue.

Vue JSX 3.3 moves that intent into package declarations:

1. `jsxImportSource` selects `vue-jsx/jsx-runtime`.
2. The runtime exposes the `JSX` namespace.
3. TypeScript asks `LibraryManagedAttributes<Component, Props>` for the final
   attribute type.
4. Vue JSX answers with props plus generated `onXxx`, typed `ref`, and slot
   children through `v-slots`.

Macros and editor-only syntax helpers can still exist as optional extensions.
They are not the default type story. For normal component authoring, the source
of truth is the component's TypeScript type, and the inference runs in the
native TypeScript checker.
