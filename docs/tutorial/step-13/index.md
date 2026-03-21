---
aside: false
prev: false
next: false
---

# Dynamic Component

<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-12/" next="/tutorial/done/">

Sometimes the component you want to render is not known ahead of time. In that case, you can store the component in a variable and switch it dynamically.

## Basic Usage

A dynamic component can be rendered by evaluating the component reference inside an expression container:

```jsx
import { shallowRef } from 'vue'

const Foo = () => <div>Foo</div>
const Bar = () => <div>Bar</div>

export default () => {
  const DynamicComponent = shallowRef(Foo)
  return <>{<DynamicComponent.value />}</>
}
```

## Why use an expression container?

In Vapor components, a direct `<DynamicComponent.value />` expression is treated as a normal JSX component tag. The key part here is wrapping it in `{}` so it becomes a JSX expression container, which the compiler can treat as a dynamic component expression.

> In Virtual DOM components, it can be used directly without `{}`.

```jsx
<>{<DynamicComponent.value />}</>
```

## Switching between components

A common pattern is to keep the current component in a `shallowRef` and switch it in response to user interaction:

```jsx
const DynamicComponent = shallowRef(Foo)

const toggle = () => {
  DynamicComponent.value = DynamicComponent.value === Foo ? Bar : Foo
}
```

Now try it yourself - toggle between `Foo` and `Bar` by rendering the current component dynamically.

</jsx-repl>
