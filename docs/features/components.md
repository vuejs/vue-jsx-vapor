# Components

## Component Naming

Vue JSX follows the standard JSX naming convention:

```tsx
import UserCard from './UserCard'

export function App() {
  return (
    <div>
      <UserCard />
      <UserCard.Header />
    </div>
  )
}
```

Tags that start with a lowercase letter are always treated as intrinsic
elements, including unknown or kebab-case tags. They are not resolved as Vue
components:

```tsx
<button />       // Native HTML element
<my-widget />    // Element tag, not a component
```

Use an uppercase identifier or a member expression for Vue components. This
keeps component resolution deterministic and avoids changing the meaning of an
existing component when HTML adds a new native element in the future.

## Defining a Component

`defineComponent` defines a Virtual DOM component and `defineVaporComponent`
defines a Vapor one. Both take a setup function, or an options object with
`setup`:

```tsx
import { defineComponent, defineVaporComponent } from 'vue-jsx'

// Virtual DOM: the setup returns a render function.
const Counter = defineComponent((props: { count: number }) => {
  return () => <button>{props.count}</button>
})

// Vapor: the setup returns the block itself.
const VaporCounter = defineVaporComponent((props: { count: number }) => (
  <button>{props.count}</button>
))
```

```tsx
// The options object form works too, for either entry point.
const Counter = defineComponent({
  props: { count: Number },
  setup(props) {
    return () => <button>{props.count}</button>
  },
})
```

### Import source

`vue-jsx` exports both functions. Unlike `vue`, they default `inheritAttrs` to
`false` and use attrs as props, so you do not have to declare props up front:

```ts
import { defineComponent, defineVaporComponent } from 'vue-jsx'
```

### `inheritAttrs` defaults to `false`

Attrs are not applied to the root element automatically. They stay in `attrs`, and
the component decides which element to spread them on:

```tsx
import { defineVaporComponent } from 'vue-jsx'

const Button = defineVaporComponent(() => <button>Click</button>)

// Renders `<button>Click</button>`; `data-test` is not applied to the root.
export default () => <Button data-test="submit" />
```

Fallthrough becomes explicit. A component that renders several elements no longer
depends on Vue guessing which one is the root, and a wrapper component cannot
leak attributes onto an element it did not intend.

### Attributes stand in for props

When a component declares no `props` option, the attributes are passed to `setup`
as its first argument. It is the same object `useAttrs()` returns, so reading
`props.title` works even though no `props` option declares `title`:

```tsx
import { defineVaporComponent } from 'vue-jsx'

const Card = defineVaporComponent((props: { title: string }) => (
  <section>
    <h2>{props.title}</h2>
  </section>
))

export default () => <Card title="Hello" />
```

## `For`

Vue JSX provides `For` for Virtual DOM and `VaporFor` for Vapor Mode. Both components preserve the item and index types inferred from `in`, without requiring directive-specific language tooling.

### Virtual DOM

Import `For` from `vue-jsx` and return a keyed node from its default slot:

```tsx
import { defineComponent, ref } from 'vue'
import { For } from 'vue-jsx'

export default defineComponent(() => {
  const users = ref([
    { id: 1, name: 'Ada' },
    { id: 2, name: 'Grace' },
  ])

  return () => (
    <ul>
      <For in={users.value}>
        {(user, index) => (
          <li key={user.id}>
            {index}: {user.name}
          </li>
        )}
      </For>
    </ul>
  )
})
```

`For` uses Vue's keyed Fragment list rendering. Place a stable `key` on the root node returned for each item so Vue can reuse and move existing nodes correctly.

### Vapor Mode

Use `VaporFor` when the surrounding component is compiled in Vapor Mode:

```tsx
import { ref } from 'vue'
import { VaporFor } from 'vue-jsx'

export default () => {
  const users = ref([
    { id: 1, name: 'Ada' },
    { id: 2, name: 'Grace' },
  ])

  return (
    <ul>
      <VaporFor in={users.value}>
        {(user, index) => (
          <li>
            {user.name} at {index.value}
          </li>
        )}
      </VaporFor>
    </ul>
  )
}
```

The Vapor slot receives the current index as a `ShallowRef<number>`. Read `index.value` when it is used in JavaScript. JSX expressions remain reactive when the index changes after inserting, removing, or moving an item.

You can also import the shorter Vapor-only alias:

```ts
import { For } from 'vue-jsx/vapor'
```

This `For` is the same component as `VaporFor`.

### Stable Keys in Vapor Mode

By default, `VaporFor` uses the item itself as its key. This works well when objects retain their identity. Use `getKey` when items can be replaced with new objects that represent the same record:

```tsx
<VaporFor in={users.value} getKey={(user) => user.id}>
  {(user, index) => (
    <li>
      {user.value.name} at {index.value}
    </li>
  )}
</VaporFor>
```

When `getKey` is present, the slot receives each item as a `ShallowRef`. This lets Vapor reuse the existing block for a stable key while updating `user.value` to the latest item object.

### Supported Sources

Both components accept arrays, strings, numbers, plain objects, `Set`, and `Map` values.

For arrays and other iterables, the slot receives `(item, index)`. For plain objects, it receives `(value, key, index)`. In `VaporFor`, object keys and indexes are shallow refs.

Prefer these components when you want list rendering with native TypeScript inference. `Array.prototype.map()` remains suitable for small or static lists where keyed update behavior is not important.
