---
aside: false
prev: false
next: false
---

# 作用域插槽
  
<script setup>
import appCode from '~/tutorial/step-10/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-10/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-10/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-10/app-interop-solved.tsx?raw'
import appMacrosCode from '~/tutorial/step-10/app-macros.tsx?raw'
import appMacrosSolvedCode from '~/tutorial/step-10/app-macros-solved.tsx?raw'
import appInteropMacrosCode from '~/tutorial/step-10/app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from '~/tutorial/step-10/app-interop-macros-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-9/" next="/zh/tutorial/step-11/">

在某些情况下，让插槽内容能够访问父组件作用域和子组件作用域的数据会非常有用。我们有两种方式来实现：

1. 我们可以像传递 props 给组件一样，向 `<slots.default />` 传递属性
```jsx
const Comp = (props, { slots }) => {
  return <slots.default foo="来自子组件" />
}
```

2. 我们可以像传递 props 给组件一样，向插槽出口传递属性：

```jsx
const Comp = () => {
  return <slot foo="来自子组件"></slot>
}
```

## 使用作用域插槽
我们将展示如何使用插槽接收 props，有四种方式：

1. 在 `<Comp>` 中使用函数表达式：
```jsx
export default () => (
  <Comp>{(slotProps) => <div>{slotProps.foo}</div>}</Comp>
)
```

2. 在 `<Comp>` 中使用对象表达式来处理多个插槽：
```jsx
export default () => (
  <Comp>{{ 
    default: (slotProps) => <div>{slotProps.foo}</div>,
    title: (slotProps) => <div>{slotProps.bar}</div>
  }}</Comp>
)
```

::: warning
注意插槽表达式会被视为动态插槽。如果你想要更好的性能，请使用 `v-slot` 代替。
:::

3. 在 `<Comp>` 中使用 `v-slot` 指令处理多个插槽，就像 Vue 模板一样：
```jsx
export default () => (
  <Comp v-slot={{ foo }}>
    <div>{foo}</div>
  </Comp>
)
```

4. 我们也可以在 `<Comp>` 中使用 `v-slot` 指令，就像 Vue 模板一样：
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

现在自己试试 - 在 `<Comp>` 中渲染 `foo` 插槽 prop。

</jsx-repl>