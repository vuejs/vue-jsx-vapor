---
aside: false
prev: false
next: false
---

# 双向绑定

<script setup>
import appCode from '~/tutorial/step-12/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-12/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-12/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-12/app-interop-solved.tsx?raw'
import appMacrosCode from '~/tutorial/step-12/app-macros.tsx?raw'
import appMacrosSolvedCode from '~/tutorial/step-12/app-macros-solved.tsx?raw'
import appInteropMacrosCode from '~/tutorial/step-12/app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from '~/tutorial/step-12/app-interop-macros-solved.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-11/" next="/zh/tutorial/done/">

双向绑定允许父组件向子组件传递数据，同时也能接收子组件的更新。在 Vue JSX 中，这是通过 `v-model` 实现的。

## 不使用 `v-model`

不使用 `v-model` 时，你需要手动传递 `modelValue` prop 并监听 `onUpdate:modelValue` 事件：

```jsx
<Comp modelValue={msg.value} onUpdate:modelValue={(v) => (msg.value = v)} />
```

子组件接收 `modelValue` 作为 prop，并通过 `onUpdate:modelValue` 通知父组件数据变化：

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

## 使用 `v-model`

`v-model` 是上述写法的语法糖，它自动绑定 `modelValue` 和 `onUpdate:modelValue`：

```jsx
<Comp v-model={msg.value} />
```

## 具名 `v-model`

你也可以使用具名 `v-model` 来绑定多个值：

```jsx
<Comp v-model:title={title.value} v-model:content={content.value} />
```

子组件将它们作为独立的 prop 接收：

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

## 使用 `defineModel` 宏

启用 macros 选项后，你可以在子组件中使用 `defineModel` 宏来简化双向绑定。它返回一个可写的 `ref`，会自动与父组件保持同步：

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

对于具名 model，将 model 名称作为第一个参数传入：

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

现在试试看 - 将手动的 `modelValue` + `onUpdate:modelValue` 绑定替换为 `v-model`。

</jsx-repl>
