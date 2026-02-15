---
aside: false
prev: false
next: false
---

# 插槽
  
<script setup>
import appCode from '~/tutorial/step-9/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-9/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-9/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-9/app-interop-solved.tsx?raw'
import appMacrosCode from '~/tutorial/step-9/app-macros.tsx?raw'
import appMacrosSolvedCode from '~/tutorial/step-9/app-macros-solved.tsx?raw'
import appInteropMacrosCode from '~/tutorial/step-9/app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from '~/tutorial/step-9/app-interop-macros-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-8/" next="/zh/tutorial/step-10/">

除了通过 props 传递数据，父组件还可以通过插槽向子组件传递 JSX：

```jsx
<Comp>
  这是默认插槽内容！
</Comp>
```

我们有三种方式使用插槽：

1. 使用函数式组件第二个 context 参数中提供的 slots。

```jsx
const Comp = (props, { slots }) => {
  return <>{slots.default ? <slots.default /> : '后备内容'}</>
}
```

2. 使用 `<slot>` 元素作为插槽出口。`<slot>` 出口内的内容将作为"后备内容"：当父组件没有传递任何插槽内容时，它将被显示。

```jsx
const Comp = (props, { slots }) => {
  return <slot>后备内容</slot>
}
```

> 具名插槽
```jsx
const Comp = (props, { slots }) => {
  return [
    <slot>后备内容</slot>,
    <slot name="title">标题后备内容</slot>
  ]
}
```

3. 启用 macros 选项并使用 `defineSlots` 宏来定义带有后备内容的插槽。

```jsx
const Comp = (props) => {
  const slots = defineSlots({
    default: () => <>后备内容</>
  })
  return <slots.default>后备内容</slots.default>
}
```

目前我们没有向 `<Comp>` 组件传递任何插槽内容，所以你应该看到后备内容。让我们为 Comp 提供一些插槽内容，同时利用父组件的 msg 状态。

</jsx-repl>