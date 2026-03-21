---
aside: false
prev: false
next: false
---

# HyperScript

<script setup>
import appCode from '~/tutorial/step-14/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-14/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-14/app-interop.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-13/" next="/zh/tutorial/done/">

除了 JSX 之外，`vue-jsx-vapor` 还提供了 `h` 函数，可用于手动创建 Vapor 节点。这种写法通常被称为 HyperScript，适合在你想要更细粒度地控制节点创建时使用。

## 基本用法

Vapor 的 `h()` 和 Virtual DOM 的 `h()` 类似，但如果提供了 `children` 参数，则必须显式传入 `props`。

```tsx
h(type, props?, children?)
```

例如，下面的代码会创建一个简单的 `div`：

```tsx
import { h } from 'vue-jsx-vapor'
export default () => h('div', null, 'hello')
```

## 响应式值需要用函数包裹

在 Vapor 模式下，如果你传入的是响应式值，通常需要再包一层函数，这样运行时才能追踪它的变化。

### 动态组件

对于动态组件，需要用函数返回当前组件：

```tsx
h(Fragment, null, [() => h('El' + props.type)])
```

### 动态 props

对于动态 prop，也需要传入函数：

```tsx
h('div', { id: () => props.id })
```

### 合并 props

如果需要合并多个属性对象，可以通过 `$:` 传入一个数组，并使用函数包裹每个对象。

```tsx
h('div', { $: [() => ({ id: props.id }), () => attrs] })
```

### ref prop

`ref` 既支持响应式变量，也支持回调函数形式。

```tsx
const divRef = shallowRef()
h('div', { ref: divRef })
h('div', { ref: (el) => (divRef.value = el) })
```

## children

`children` 可以是单个子节点，也可以是由多个子节点组成的数组：

```tsx
h(div, null, 'hello')
h(div, null, ['hello', () => world])
```

### 默认插槽

默认插槽可以直接传入一个函数:

```tsx
h(Comp, null, (slotProps) => ['hello', () => slotProps.foo])
```

### 多个插槽

多个插槽需要传入一个对象:

```tsx
h(Comp, null, {
  default: (slotProps) => ['hello', () => slotProps.foo],
  other: () => 'other slot',
})
```

## 为什么需要包一层函数？

在 Vapor 模式下，`h()` 的参数不会像模板或 JSX 那样自动转换成响应式表达式。将值写成函数后，运行时就可以在需要时重新求值，并正确追踪依赖。

你可以把它理解成：

- 静态值：直接传
- 动态值：用函数传

## 什么时候使用 `h()`？

`h()` 适合这些场景：

- 你想手动控制节点的创建方式
- 你在写底层运行时逻辑
- 你需要动态生成组件、props 或 slot

## 和 JSX 的区别

在 JSX 中，很多表达式会由编译器自动转换；而在 `h()` 中，你需要更明确地告诉运行时哪些值是动态的。

例如，下面这两种写法表达的是类似的意思：

```tsx
<>{<DynamicComponent.value />}</>
```

```tsx
h(Fragment, null, [() => h(DynamicComponent.value)])
```

现在试试看 - 用函数包裹动态值，使示例中的 `h()` 调用保持响应式。

</jsx-repl>
