---
aside: false
prev: false
next: false
---

# Expose
  
<script setup>
import appCode from '~/tutorial/step-11/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-11/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-11/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-11/app-interop-solved.tsx?raw'
import appMacrosCode from '~/tutorial/step-11/app-macros.tsx?raw'
import appMacrosSolvedCode from '~/tutorial/step-11/app-macros-solved.tsx?raw'
import appInteropMacrosCode from '~/tutorial/step-11/app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from '~/tutorial/step-11/app-interop-macros-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-10/" next="/zh/tutorial/step-12/">

如果你想从父组件访问子组件的状态，有两种方式可以暴露它：

1. 使用函数式组件第二个上下文参数中提供的 expose 函数。
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

2. 启用 macros 选项并使用 `defineExpose` 宏来暴露状态。

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

## 访问暴露的状态
我们可以使用 `ref` prop 来获取暴露的状态并在之后使用它：

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
我们也可以使用 `vue-jsx-vapor` 提供的 `useRef` API 来接收暴露的状态。它是 `shallowRef` 的别名，可以自动推断组件暴露的类型。

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

现在试试看 - 为 `<Comp>` 设置 ref prop。

</jsx-repl>