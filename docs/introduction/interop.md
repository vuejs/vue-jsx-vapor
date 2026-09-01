# Vapor Mode

Vue JSX compiles regular JSX to Vue Virtual DOM by default. Vapor is an optional
output mode and requires Vue 3.6 or later.

## Enable Vapor Globally

Set `vapor: true` when an application is fully based on Vapor:

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [
    vueJsx({
      vapor: true,
    }),
  ],
})
```

Mount the root component with `createVaporApp`:

```ts [main.ts]
import { createVaporApp } from 'vue'
import App from './App.tsx'

createVaporApp(App).mount('#app')
```

## Opt In Incrementally

You do not need to enable Vapor for the entire project. With the default
`vapor: false`, the compiler enables Vapor for:

- Files ending in `.vapor.jsx` or `.vapor.tsx`.
- JSX owned by `defineVaporComponent` or `defineVaporCustomElement`.

```tsx
import { defineVaporComponent } from 'vue'

export const Counter = defineVaporComponent((props: { count: number }) => {
  return <button>{props.count}</button>
})
```

Regular components and files continue to compile to Virtual DOM.

## Mixing Rendering Modes

Install Vue's `vaporInteropPlugin` when a component tree crosses between Virtual
DOM and Vapor components.

### Vapor inside a Virtual DOM app

```ts [main.ts]
import { createApp, vaporInteropPlugin } from 'vue'
import App from './App.tsx'

createApp(App).use(vaporInteropPlugin).mount('#app')
```

Keep `vapor` at its default `false`, then use `defineVaporComponent` or a
`.vapor.tsx` file for the Vapor subtree.

### Virtual DOM inside a Vapor app

```ts [main.ts]
import { createVaporApp, vaporInteropPlugin } from 'vue'
import App from './App.vapor.tsx'

createVaporApp(App).use(vaporInteropPlugin).mount('#app')
```

Place Virtual DOM components in regular files and define them with
`defineComponent`.

## Vapor Runtime Helpers

The `vue-jsx/vapor` entry exposes Vapor-specific helpers under familiar names:

```ts
import {
  For,
  KeepAlive,
  Teleport,
  Transition,
  TransitionGroup,
  h,
} from 'vue-jsx/vapor'
```

This entry changes runtime imports only. Compilation is still controlled by the
`vapor` option, filename, or Vapor component boundary.
