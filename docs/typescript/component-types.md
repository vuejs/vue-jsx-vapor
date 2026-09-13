# Component Types

Props, slots, emits, and `expose` reach the JSX call site through ordinary
TypeScript: `JSX.LibraryManagedAttributes` rewrites the declared component type
into the attributes TypeScript then checks. This page covers what that rewrite
infers, where it stops for generic components, and the exported types that put
the missing part back.

## What Gets Rewritten

| On the component      | In JSX                               |
| --------------------- | ------------------------------------ |
| the `props` parameter | the attributes themselves            |
| `emit`                | `onXxx` callback props               |
| `slots`               | the `v-slots` prop, and JSX children |
| `exposed`             | the target type of `ref`             |

```tsx
const Panel = (props: { step: number }, { slots }: { slots: { default?: (n: number) => any } }) => (
  <div>{slots.default?.(props.step)}</div>
)

export default () => <Panel step={1}>{(n) => <span>{n.toFixed()}</span>}</Panel>
```

`n` is `number` without any language tooling: the slot signature is part of the
component type. [Type Inference: Props, Refs, And Children](/blog/type-inference)
walks through the three branches of the rewrite — constructor components,
function components, and unknown components.

## Generic Props Stop At The Setup Context

Type arguments are inferred from the **attributes object**. Everything the
component exposes through its second parameter, or through the instance type, is
resolved before that inference happens, so a type parameter used only there keeps
its constraint — `unknown` when there is none:

```tsx
const List = <T,>(props: { items: T[] }, { slots }: { slots: { row?: (item: T) => any } }) => (
  <ul>{slots.row?.(props.items[0])}</ul>
)

// `item` is `unknown` in the slot below
export default () => <List items={[{ id: 1 }]}>{{ row: (item) => <li>{item.id}</li> }}</List>
```

- `items` is still checked as `T[]`: props are props.
- `item` inside the slot is `unknown`, and with `T extends string | number` it is
  `string | number` — the constraint, not the type argument.
- An explicit type argument, `<List<{ id: number }>>`, changes the props only.
  The generated `v-slots` still comes from the unresolved signature.

The way out is to stop asking the rewrite for those three: declare them as
props, in the position TypeScript infers from.

## The Props Helpers

Three exported types turn a piece of the setup context into a prop:

- `SlotsToProps<Slots>` — the slot signatures as one optional `v-slots` prop.
- `ExposedToProps<Exposed>` — the target type of `ref`, plus an internal marker
  key.
- `SetupContextToProps<Emits, Slots, Exposed>` — emits, slots, and exposed
  together.

They are exported from `vue-jsx`:

```ts
import type { ExposedToProps, SetupContextToProps, SlotsToProps } from 'vue-jsx'
```

The rewrite never duplicates them: it synthesizes `v-slots` only when
`'v-slots' extends keyof Props` is false, and infers `ref` from the component
only when `'ref' extends keyof Props` is false. A declared prop always wins.

### SlotsToProps

```tsx
import type { SlotsToProps } from 'vue-jsx'

type ListProps<T> = { items: T[] } & SlotsToProps<{ row?: (item: T) => any }>

const List = <T,>(props: ListProps<T>) => <ul>{props.items.length}</ul>

export default () => <List items={[{ id: 1 }]} v-slots={{ row: (item) => <li>{item.id}</li> }} />
```

`item` is `{ id: number }` now, because `v-slots` is one of the attributes
TypeScript infers the type argument from. Children work the same way — JSX
children are checked against `v-slots` by `ElementChildrenAttribute`.

`SlotsToProps` accepts a plain slot record or Vue's `SlotsType`, and widens each
slot's return type to `NodeChild` so Virtual DOM and Vapor render results both
fit.

### ExposedToProps

```tsx
import type { ExposedToProps } from 'vue-jsx'

type CardProps<T> = { value: T } & ExposedToProps<{ reset: () => void }>

const Card = <T,>(props: CardProps<T>) => <div>{props.value}</div>

export default () => <Card value={1} ref={(exposed) => exposed?.reset()} />
```

`ExposedToProps<T>` adds the internal marker key and types `ref` as
`NodeRef<T>`, so `exposed` is `{ reset: () => void } | null` — the same shape a
non-generic component gets from `ctx.expose()`.

### SetupContextToProps

`SetupContextToProps<Emits, Slots, Exposed>` is the whole context at once:
`EmitsToProps`, `SlotsToProps`, and `ExposedToProps` intersected.

```tsx
import type { SetupContextToProps } from 'vue-jsx'

type ListProps<T> = { items: T[] } & SetupContextToProps<
  { change: [value: T] },
  { row?: (item: T) => any },
  { first: T }
>

const List = <T,>(props: ListProps<T>) => <ul>{props.items.length}</ul>

export default () => (
  <List
    items={[1, 2, 3]}
    onChange={(value) => value.toFixed()}
    ref={(exposed) => exposed?.first.toFixed()}
  >
    {{ row: (item) => <li>{item.toFixed()}</li> }}
  </List>
)
```

## Which To Use

| Situation                                        | Use                                          |
| ------------------------------------------------ | -------------------------------------------- |
| ordinary component with an annotated context     | nothing extra, inference is already complete |
| generic component, children must see `T`         | `SlotsToProps` in the props                  |
| generic component, `ref` must see `T`            | `ExposedToProps` in the props                |
| generic component with emits, slots, and exposed | `SetupContextToProps`                        |
