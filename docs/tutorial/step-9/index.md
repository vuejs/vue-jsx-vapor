---
aside: false
prev: false
next: false
---

# Slots
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import appMacrosCode from './app-macros.tsx?raw'
import appMacrosSolvedCode from './app-macros-solved.tsx?raw'
import appInteropMacrosCode from './app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from './app-interop-macros-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode },
  macros: { 'src/App.tsx': appMacrosCode },
  macrosSolved: { 'src/App.tsx': appMacrosSolvedCode },
  interopMacros: { 'src/App.tsx': appInteropMacrosCode },
  interopMacrosSolved: { 'src/App.tsx': appInteropMacrosSolvedCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-8/" next="/tutorial/step-10/">

In addition to passing data via props, the parent component can also pass down template fragments to the child via slots:

```jsx
<Comp>
  This is the default slot content!
</Comp>
```

We have three ways to use slots:

1. Use the slots provided in the second context parameter of the functional component.

```jsx
const Comp = (props, { slots }) => {
  return <>{slots.default ? <slots.default /> : 'Fallback content'}</>
}
```

2. Use the `<slot>` element as outlet. Content inside the `<slot>` outlet will be treated as "fallback" content: it will be displayed if the parent did not pass down any slot content.

```jsx
const Comp = (props, { slots }) => {
  return <slot>Fallback content</slot>
}
```

> Named slot
```jsx
const Comp = (props, { slots }) => {
  return [
    <slot>Fallback content</slot>,
    <slot name="title">Title fallback content</slot>
  ]
}
```

3. Enable the macros option and use the `defineSlots` macro to define slots with fallback content.

```jsx
const Comp = (props) => {
  const slots = defineSlots({
    default: () => <>Fallback content</>
  })
  return <slots.default>Fallback content</slots.default>
}
```

Currently we are not passing any slot content to the `<Comp>` component, so you should see the fallback content. Let's provide some slot content to the Comp while making use of the parent's msg state.

</jsx-repl>
