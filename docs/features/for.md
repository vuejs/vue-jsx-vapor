# List Components

Vue JSX provides `For` for Virtual DOM and `VaporFor` for Vapor Mode. Both components preserve the item and index types inferred from `in`, without requiring directive-specific language tooling.

## Virtual DOM

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
        {(user, index) => <li key={user.id}>{index}: {user.name}</li>}
      </For>
    </ul>
  )
})
```

`For` uses Vue's keyed Fragment list rendering. Place a stable `key` on the root node returned for each item so Vue can reuse and move existing nodes correctly.

## Vapor Mode

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
          <li>{user.name} at {index.value}</li>
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

## Stable Keys in Vapor Mode

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

## Supported Sources

Both components accept arrays, strings, numbers, plain objects, `Set`, and `Map` values.

For arrays and other iterables, the slot receives `(item, index)`. For plain objects, it receives `(value, key, index)`. In `VaporFor`, object keys and indexes are shallow refs.

Prefer these components when you want list rendering with native TypeScript inference. `Array.prototype.map()` remains suitable for small or static lists where keyed update behavior is not important.
