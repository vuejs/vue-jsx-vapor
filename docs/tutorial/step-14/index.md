---
aside: false
prev: false
next: false
---

# HyperScript

<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-13/" next="/tutorial/step-done/">

Besides JSX, `vue-jsx-vapor` also provides an `h` function for manually creating Vapor nodes. This style is commonly called HyperScript, and it is useful when you want finer-grained control over node creation.

## Basic Usage

Vapor's `h()` is similar to Virtual DOM's `h()`, but if you provide a `children` argument, you must explicitly pass `props`.

```tsx
h(type, props?, children?)
```

For example, the following code creates a simple `div`:

```tsx
import { h } from 'vue-jsx-vapor'
export default () => h('div', null, 'hello')
```

## Reactive values need to be wrapped in functions

In Vapor mode, if you pass reactive values, you usually need to wrap them in functions so the runtime can track their changes.

### Dynamic component

For dynamic components, use a function to return the current component:

```tsx
h(Fragment, null, [() => h('El' + props.type)])
```

### Dynamic props

For dynamic props, you also need to pass a function:

```tsx
h('div', { id: () => props.id })
```

### Merging props

If you need to merge multiple props objects, you can pass an array to `$:` and wrap each object in a function.

```tsx
h('div', { $: [() => ({ id: props.id }), () => attrs] })
```

### `ref` prop

`ref` supports both reactive variables and callback functions.

```tsx
const divRef = shallowRef()
h('div', { ref: divRef })
h('div', { ref: (el) => (divRef.value = el) })
```

## children

`children` can be a single child node or an array of child nodes:

```tsx
h('div', null, 'hello')
h('div', null, ['hello', () => world])
```

### Default slot

The default slot can be passed directly as a function:

```tsx
h(Comp, null, (slotProps) => ['hello', () => slotProps.foo])
```

### Multiple slots

Multiple slots must be passed as an object:

```tsx
h(Comp, null, {
  default: (slotProps) => ['hello', () => slotProps.foo],
  other: () => 'other slot',
})
```

## Why wrap values in functions?

In Vapor mode, the arguments passed to `h()` are not automatically transformed into reactive expressions like they are in templates or JSX. Once a value is wrapped in a function, the runtime can re-evaluate it when needed and track dependencies correctly.

You can think of it like this:

- Static values: pass them directly
- Dynamic values: pass them as functions

## When should you use `h()`?

`h()` is useful in the following cases:

- You want to manually control how nodes are created
- You are writing low-level runtime logic
- You need to generate components, props, or slots dynamically

## Difference from JSX

In JSX, many expressions are automatically transformed by the compiler. With `h()`, you need to be more explicit about which values are dynamic.

For example, the following two forms express a similar idea:

```tsx
<>{<DynamicComponent.value />}</>
```

```tsx
h(Fragment, null, [() => h(DynamicComponent.value)])
```

Now try it yourself - wrap the dynamic values in functions so the `h()` calls in the example stay reactive.

</jsx-repl>
