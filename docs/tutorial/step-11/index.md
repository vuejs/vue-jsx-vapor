---
aside: false
prev: false
next: false
---

# Expose
  
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

<jsx-repl :files :apps prev="/tutorial/step-10/" next="/tutorial/step-12/">

If you want to access a child component's state from the parent component, there are two ways to expose it:

1. Use the expose function provided in the second context parameter of the functional component.
```jsx
import { computed } from 'vue'

const Comp = (props, { expose }) => {
  const double = computed(() => props.count * 2)
  expose({
    double
  })
  return []
}
```

2. Enable the macros option and use the `defineExpose` macro to expose state.

```jsx
import { computed } from 'vue'

const Comp = (props) => {
  const double = computed(() => props.count * 2)
  defineExpose({
    double
  })
  return []
}
```

## Access exposed state
We can use `ref` prop to take the exposed state and use it later:

```jsx
import { shallowRef } from 'vue'

export default () => {
  const compRef = shallowRef()
  return (
    <>
      <Comp ref={compRef} count={1} />
      {compRef.value?.double}
    </>
  )
}
```

::: tip
We can also use the `useRef` api from `vue-jsx-vapor` to receive exposed state. It's a `shallowRef` alias that can automatically infer the component's exposed types.

```tsx twoslash
import { computed } from 'vue'
import { useRef } from 'vue-jsx-vapor'

const Comp = (props: { count: number }) => {
  const double = computed(() => props.count * 2)
  defineExpose({
    double
  })
  return []
}

export default () => {
  const compRef = useRef()
  return (
    <>
      <Comp ref={compRef} count={1} />
      {compRef.value?.double}
    </>
  )
}
```
:::

Now try it yourself - set ref prop for `<Comp>`.

</jsx-repl>
