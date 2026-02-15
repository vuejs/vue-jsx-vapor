---
aside: false
prev: false
next: false
---

# Props
  
<script setup>
import appCode from '~/tutorial/step-8/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-8/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-8/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-8/app-interop-solved.tsx?raw'
import appMacrosCode from '~/tutorial/step-8/app-macros.tsx?raw'
import appMacrosSolvedCode from '~/tutorial/step-8/app-macros-solved.tsx?raw'
import appInteropMacrosCode from '~/tutorial/step-8/app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from '~/tutorial/step-8/app-interop-macros-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-7" next="/zh/tutorial/step-9">

Props 在函数式组件的第一个参数中提供。

```jsx
const Comp = (props) => (
  <div>{props.foo}</div>
)
```

## 解构 Props

::: warning
与其他 JSX 框架不同的是，当你解构 props 时会失去响应性：
:::

```jsx
const Comp = ({ foo }) => (
  <div>
    {foo} 这将不再更新
  </div>
)
````

我们有两种解决方案：

1. 直接传递一个响应式 ref 对象作为 prop：
```jsx
function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo} />
}
```

但这个组件不能在 Vue 模板中使用，因为 Vue 模板会自动解包 refs。

2. 启用 macros 选项并将组件包装在 `defineVaporComponent` 或 `defineComponent`（用于 Virtual DOM）中。它会自动将解构的 props 转换为 `__props`，并为每个使用的 prop 添加 `__props.` 前缀。

```jsx
const Comp = defineVaporComponent(({ foo }) => {
  return <div>{foo}</div>
})
```
将被转换为：
```jsx
const Comp = defineVaporComponent((__props) => {
  return <div>{__props.foo}</div>
})
```
这样 `foo` prop 就会重新获得响应性。\
[更多详情](/zh/features/macros.html#definecomponent-definevaporcomponent)

现在自己试试 - 让 `foo` prop 保持响应性。

</jsx-repl>