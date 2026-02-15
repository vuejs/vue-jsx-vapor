---
aside: false
prev: false
next: false
---

# 列表渲染
  
<script setup>
import appCode from '~/tutorial/step-6/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-6/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-6/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-6/app-interop-solved.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-5/" next="/zh/tutorial/step-7">

如果对性能没有特别要求，我们可以使用 `map(...)` 来渲染列表。

```jsx
<ul>
  {todos.map((todo) => {
    return <li key={todo.id}>{todo.text}</li>
  })}
</ul>
```

## `v-for` 指令

我们也可以使用 `v-for` 指令来渲染列表，它具有与 Vue 模板相同的性能：

```jsx
<ul>
  <li v-for={todo in todos} key={todo.id}>
    {todo.text}
  </li>
</ul>
```

这里的 todo 是一个局部变量，代表当前正在迭代的数组元素。它只能在 v-for 元素上或内部访问，类似于函数作用域。

注意我们还给每个 todo 对象一个唯一的 id，并将其绑定为每个 `<li>` 的特殊 key 属性。key 使 Vue 能够准确地移动每个 `<li>` 以匹配其对应对象在数组中的位置。

有两种方式可以更新列表：
1. 在源数组上调用变更方法：
```ts
todos.value.push(newTodo)
```
2. 用一个新数组替换原数组：
```ts
todos.value = todos.value.filter(/* ... */)
```

这里我们有一个简单的待办事项列表 - 尝试实现 `addTodo()` 和 `removeTodo()` 方法的逻辑，使其正常工作！

</jsx-repl>