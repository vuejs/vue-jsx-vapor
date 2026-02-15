---
aside: false
prev: false
next: false
---

# 组件
  
<script setup>
import appCode from '~/tutorial/step-7/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-7/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-7/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-7/app-interop-solved.tsx?raw'
import childCode from '~/tutorial/step-7/Child.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref({
  ...getDefaultFiles(),
  'src/Child.tsx': childCode
})
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode,  },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-6/" next="/zh/tutorial/step-8/">

在这个例子中，让我们将 `Child` 组件添加到我们的应用中。我们已经在另一个文件中定义了它，当然你也可以把多个组件放在同一个文件中。首先我们需要导入它：

```jsx
import Child from './Child'
```

然后，我们可以在 JSX 中使用该组件：
```jsx
import Child from './Child'

export default () => {
  return (
    <div>
      Parent
      <Child />
    </div>
  )
}
```

现在试试看 - 导入 `Child` 组件并在 JSX 中渲染它。

</jsx-repl>