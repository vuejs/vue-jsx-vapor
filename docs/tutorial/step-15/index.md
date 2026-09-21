---
aside: false
prev: false
next: false
---

# Custom Elements

<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appVaporCode from './app-vapor.tsx?raw'
import appVaporSolvedCode from './app-vapor-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  vapor: { 'src/App.tsx': appVaporCode },
  vaporSolved: { 'src/App.tsx': appVaporSolvedCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-14/" next="/tutorial/step-done/">

A Custom Element is a browser-native component with its own tag name. Vue JSX recognizes lowercase tags containing a hyphen automatically, so an element such as `<tutorial-user-card>` needs no compiler configuration.

## Define the element

Use `defineCustomElement` for a Virtual DOM component:

```tsx
const UserCardElement = defineCustomElement(
  (props: { name: string }) => () => <strong>{props.name}</strong>,
  { props: { name: String } },
)
```

In Vapor Mode, use `defineVaporCustomElement` and return the JSX directly:

```tsx
const UserCardElement = defineVaporCustomElement(
  (props: { name: string }) => <strong>{props.name}</strong>,
  { props: { name: String } },
)
```

The runtime `props` option makes `name` an observed Custom Element attribute. The TypeScript annotation provides static checking inside the setup function.

## Register the tag

The browser must know which constructor belongs to the tag. Register it once with `customElements.define()`:

```ts
if (!customElements.get('tutorial-user-card')) {
  customElements.define('tutorial-user-card', UserCardElement)
}
```

The guard prevents duplicate-registration errors during development and hot updates.

## Pass named slot content

Content is assigned to a native named slot with the standard `slot` attribute:

```tsx
<tutorial-user-card name="Ada Lovelace">
  <span slot="avatar">AL</span>
</tutorial-user-card>
```

Inside the Custom Element, `<slot name="avatar">` renders that content. Vue Custom Elements use Shadow DOM by default, so the example passes its component styles through the `styles` option.

Now register `UserCardElement` as `<tutorial-user-card>`. Then switch between Virtual DOM and Vapor Mode and confirm that the element behaves the same in both modes.

</jsx-repl>
