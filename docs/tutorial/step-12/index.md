---
aside: false
prev: false
next: false
---

# Two-way Binding

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

<jsx-repl :files :apps prev="/tutorial/step-11/" next="/tutorial/done/">

Two-way binding allows a parent component to both pass data down to a child and receive updates back from it. In Vue JSX, this is done via `v-model`.

## Without `v-model`

Without `v-model`, you need to manually pass `modelValue` as a prop and listen to the `onUpdate:modelValue` event:

```jsx
<Comp modelValue={msg.value} onUpdate:modelValue={(v) => (msg.value = v)} />
```

The child component receives `modelValue` as a prop and emits `onUpdate:modelValue` to notify the parent of changes:

```jsx
const Comp = (props) => {
  return (
    <input
      value={props.modelValue}
      onInput={(e) => props['onUpdate:modelValue'](e.target.value)}
    />
  )
}
```

## Using `v-model`

`v-model` is syntactic sugar for the above pattern. It automatically binds `modelValue` and `onUpdate:modelValue`:

```jsx
<Comp v-model={msg.value} />
```

## Named `v-model`

You can also use named `v-model` to bind multiple values:

```jsx
<Comp v-model:title={title.value} v-model:content={content.value} />
```

The child receives them as separate props:

```jsx
const Comp = (props) => {
  return (
    <>
      <input
        value={props.title}
        onInput={(e) => props['onUpdate:title'](e.target.value)}
      />
      <input
        value={props.content}
        onInput={(e) => props['onUpdate:content'](e.target.value)}
      />
    </>
  )
}
```

## Using `defineModel` Macro

When the macros option is enabled, you can use the `defineModel` macro inside the child component to simplify two-way binding. It returns a writable `ref` that automatically syncs with the parent:

```jsx
const Comp = () => {
  const model = defineModel()
  return (
    <input
      value={model.value}
      onInput={(e) => (model.value = e.target.value)}
    />
  )
}
```

For named models, pass the model name as the first argument:

```jsx
const Comp = () => {
  const title = defineModel('title')
  const content = defineModel('content')
  return (
    <>
      <input
        value={title.value}
        onInput={(e) => (title.value = e.target.value)}
      />
      <input
        value={content.value}
        onInput={(e) => (content.value = e.target.value)}
      />
    </>
  )
}
```

::: tip
You can append `!` to mark the model as required:

```jsx
const model = defineModel<string>()!
```

:::

Now try it yourself - replace the manual `modelValue` + `onUpdate:modelValue` binding with `v-model`.

</jsx-repl>
