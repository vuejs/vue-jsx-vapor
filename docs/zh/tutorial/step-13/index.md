---
aside: false
prev: false
next: false
---

# 动态组件

<script setup>
import appCode from '~/tutorial/step-13/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-13/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-13/app-interop.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropCode },
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-12/" next="/zh/tutorial/14/">

有时候，你想要渲染的组件在编写代码时并不能提前确定。这种情况下，你可以把组件保存到一个变量中，并在运行时动态切换它。

## 基本用法

动态组件可以通过在表达式容器中使用组件引用来渲染：

```jsx
import { shallowRef } from 'vue'

const Foo = () => <div>Foo</div>
const Bar = () => <div>Bar</div>

export default () => {
  const DynamicComponent = shallowRef(Foo)
  return <>{<DynamicComponent.value />}</>
}
```

## 为什么要使用表达式容器？

在 Vapor 组件中，直接写 `<DynamicComponent.value />` 会被当作普通的 JSX 组件标签处理。这里的关键在于用 `{}` 把它包起来，这样编译器才能把它当作动态组件表达式来处理。

> 而在 Virtual DOM 组件中，则可以直接使用，不需要 `{}`。

```jsx
<>{<DynamicComponent.value />}</>
```

## 在组件之间切换

一种常见的写法是把当前组件保存在 `shallowRef` 中，并在用户交互时切换它：

```jsx
const DynamicComponent = shallowRef(Foo)

const toggle = () => {
  DynamicComponent.value = DynamicComponent.value === Foo ? Bar : Foo
}
```

现在试试看 - 通过动态渲染当前组件，在 `Foo` 和 `Bar` 之间切换。

</jsx-repl>
