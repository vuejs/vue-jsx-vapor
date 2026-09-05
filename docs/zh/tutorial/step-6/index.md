---
aside: false
prev: false
next: false
---

# 列表渲染
  
<script setup>
import appCode from '~/tutorial/step-6/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-6/app-solved.tsx?raw'
import appVaporCode from '~/tutorial/step-6/app-vapor.tsx?raw'
import appVaporSolvedCode from '~/tutorial/step-6/app-vapor-solved.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  vapor: { 'src/App.tsx': appVaporCode },
  vaporSolved: { 'src/App.tsx': appVaporSolvedCode }
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-5/" next="/zh/tutorial/step-7">

Vue JSX 支持使用 `map()` 渲染列表：

```tsx
<ul>
  {todos.map((todo) => {
    return <li key={todo.id}>{todo.text}</li>
  })}
</ul>
```

这种熟悉的写法适合大多数小型或简单列表。对于更新频繁或性能要求较高的场景，推荐使用对应渲染模式的列表组件。

## Virtual DOM

Virtual DOM 使用 `For`：

```tsx
import { For } from 'vue-jsx'

<ul>
  <For in={todos}>
    {(todo) => <li key={todo.id}>{todo.text}</li>}
  </For>
</ul>
```

插槽会接收到当前 item 和 index。与使用 `map()` 时一样，需要为返回的根节点提供稳定的 `key`。

## Vapor 模式

Vapor 模式使用 `VaporFor`：

```tsx
import { VaporFor } from 'vue-jsx'

<ul>
  <VaporFor in={todos}>
    {(todo, index) => (
      <li>
        {todo.text}，位置：{index.value}
      </li>
    )}
  </VaporFor>
</ul>
```

`VaporFor` 会把每一项作为独立 block 管理。它的 index 是 shallow ref，因此在 JavaScript 中需要读取 `index.value`。组件默认使用 item 本身作为 key；如果列表可能用新对象替换原有 item，可以传入 `getKey={(todo) => todo.id}`。

当前的待办事项列表只渲染了第一项。请用对应渲染模式的列表组件替换临时代码，渲染全部待办事项。

</jsx-repl>
