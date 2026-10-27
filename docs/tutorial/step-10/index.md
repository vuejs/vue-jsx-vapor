---
aside: false
prev: false
next: false
---

# Scoped Slots

<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appVaporCode from './app-vapor.tsx?raw'
import appVaporSolvedCode from './app-vapor-solved.tsx?raw'
import appMacrosCode from './app-macros.tsx?raw'
import appMacrosSolvedCode from './app-macros-solved.tsx?raw'
import appVaporMacrosCode from './app-vapor-macros.tsx?raw'
import appVaporMacrosSolvedCode from './app-vapor-macros-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  vapor: { 'src/App.tsx': appVaporCode },
  vaporSolved: { 'src/App.tsx': appVaporSolvedCode },
  macros: { 'src/App.tsx': appMacrosCode },
  macrosSolved: { 'src/App.tsx': appMacrosSolvedCode },
  vaporMacros: { 'src/App.tsx': appVaporMacrosCode },
  vaporMacrosSolved: { 'src/App.tsx': appVaporMacrosSolvedCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-9/" next="/tutorial/step-11/">

There are cases where it could be useful if a slot's content can make use of data from both the parent scope and the child scope. We have two ways to achieve that:

1. We can pass attributes to the `<slots.default />` just like passing props to a component

```jsx
const Comp = (props, { slots }) => {
  return <slots.default foo="from child" />
}
```

2. We can pass attributes to a slot outlet just like passing props to a component:

```jsx
const Comp = () => {
  return <slot foo="from child"></slot>
}
```

## Using Scoped Slots

We are going to show how to receive props using slots, we have four ways:

1. Using a function expression in `<Comp>`:

```jsx
export default () => <Comp>{(slotProps) => <div>{slotProps.foo}</div>}</Comp>
```

2. Using an object expression in `<Comp>` for multiple slots:

```jsx
export default () => (
  <Comp>
    {{
      default: (slotProps) => <div>{slotProps.foo}</div>,
      title: (slotProps) => <div>{slotProps.bar}</div>,
    }}
  </Comp>
)
```

::: warning
Note that slot expressions are treated as dynamic slots. If you want better performance, use `v-slot` instead.
:::

3. Using the `v-slot` directive in `<Comp>` for multiple slots just like Vue template:

```jsx
export default () => (
  <Comp v-slot={{ foo }}>
    <div>{foo}</div>
  </Comp>
)
```

4. We can also use the `v-slot` directive in `<Comp>` just like Vue template:

```jsx
export default () => (
  <Comp>
    <template v-slot={{ foo }}>
      <div>{foo}</div>
    </template>
    <template v-slot:title={{ foo }}>
      <div>{foo}</div>
    </template>
  </Comp>
)
```

Now try it yourself - render the `foo` slot prop in the `<Comp>`.

</jsx-repl>
