---
aside: false
prev: false
next: false
---

# 事件绑定
  
<script setup>
import appCode from '~/tutorial/step-4/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-4/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-4/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-4/app-interop-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-3/" next="/zh/tutorial/step-5/">

在 JSX 中，事件处理器通常写成 `on` 后面跟一个大写字母。你也可以使用 `v-on` 指令来绑定多个事件处理器，而不需要 `on` 前缀。

```tsx
<>
  <div onClick={onClick} />
  {/* 多个绑定 */}
  <form v-on={{ click: onClick, submit: onSubmit }}></form>
</>
```

我们还支持以 `_` 开头的[事件修饰符](https://cn.vuejs.org/guide/essentials/event-handling.html#event-modifiers)：
```tsx
<form onSubmit_prevent>
  <input onKeyup_enter={submit} />
</form>
```

现在，尝试给 `<h1>` 添加一个事件处理器 `onClick` 绑定，使用 `onClick` 变量作为它的值，然后点击 `h1`。

</jsx-repl>