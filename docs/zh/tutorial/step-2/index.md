---
aside: false
prev: false
next: false
---

# JSX 介绍
  
<script setup>
import appCode from '~/tutorial/step-2/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-2/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-2/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-2/app-interop-solved.tsx?raw'
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

<jsx-repl :files :apps prev="/zh/tutorial/step-1/" next="/zh/tutorial/step-3/">

JSX 是一种类似 HTML 的语法，允许你使用 `{ }` 来嵌入动态表达式以引用变量和函数。
在这个例子中，我们使用 `{name}` 在 div 中包含字符串 `name`，并渲染一个直接赋值给变量 `a` 的 JSX 元素。

JSX 和 HTML 之间有 4 个主要区别：

1. JSX 要求所有元素都必须闭合，因此即使是 HTML 空元素如 `input` 和 `br` 也必须自闭合（例如 `<input />`、`<br />`）。

2. JSX 组件名应使用 PascalCase 命名（例如 `<MyComp />`），而不是像 `<my-comp />` 这样的 kebab-case 命名。属性名应使用 camelCase（例如 `fooBar`），而不是 kebab-case `foo-bar`。

3. JSX 通常需要一个单一的根元素。要表示多个顶层元素，可以将它们包装在 Fragment 元素中（`<>...</>`），或者使用带有 props 的 Fragment 组件（`<Fragment key={key}>...</Fragment>`）。

```jsx
<>
  <h1>标题</h1>
  <h2>副标题</h2>
</>
```

4. JSX 不支持 HTML 注释 `<!--...-->` 或特殊标签如 `<!DOCTYPE>`。请改用 JSX 注释 `{/*...*/}`，它们不会渲染到 HTML 中。

现在，试着在 JSX 中使用 `name` 变量，并将变量 `a` 更新为 `<a href="#">链接</a>`。

</jsx-repl>