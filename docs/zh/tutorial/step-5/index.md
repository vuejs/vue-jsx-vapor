---
aside: false
prev: false
next: false
---

# 条件渲染
  
<script setup>
import appCode from '~/tutorial/step-5/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-5/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-5/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-5/app-interop-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-4/" next="/zh/tutorial/step-6/">

我们可以使用三元表达式 `{ a ? b : c }` 或布尔表达式 `{ a && b }` 来控制渲染：

```jsx
<>
  { toggle ? <h1>标题</h1> : null }
  { toggle && <h1>标题</h1> }
</>
```

## `v-if` / `v-else-if` / `v-else` 指令

我们也可以使用 `v-if` 指令来条件性地渲染元素：

```jsx
<>
  <h1 v-if={level === 1}>标题</h1>
  <h2 v-else-if={level === 2}>副标题</h2>
  <div v-else>内容</div>
</>
```
目前，示例同时显示了两个 `<h1>`，而且按钮没有任何效果。尝试给它们添加 `v-if` 和 `v-else` 指令，并实现 `toggle()` 方法，使我们可以使用按钮在它们之间切换。

</jsx-repl>