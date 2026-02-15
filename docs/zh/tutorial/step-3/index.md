---
aside: false
prev: false
next: false
---

# 属性绑定
  
<script setup>
import appCode from '~/tutorial/step-3/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-3/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-3/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-3/app-interop-solved.tsx?raw'
import cssCode from '~/tutorial/step-3/main.css?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref({ ...getDefaultFiles(), 'src/main.css': cssCode })
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-2/" next="/zh/tutorial/step-4/">

我们使用 `{ }` 来动态绑定 prop，或者使用展开运算符 `{...}` 来绑定多个属性：
```jsx
export default (props) => (
  <>
    <div id={props.id} />
    {/* 多个绑定 */}
    <div {...props} />
  </>
)
````

## 样式绑定
我们可以使用字符串、对象或数组表达式来条件性地绑定样式：
```tsx
export default (props: { hidden: boolean }) => (
  <>
    <h1 style={`display: ${ props.hidden ? 'none' : 'block' }`}>h1</h1>
    <h2 style={{ display: props.hidden ? 'none': undefined }}>h2</h2>
    <h3 style={[ props.hidden && 'display: none;' ]}>h3</h3>
  </>
)
```

## 类名绑定
我们可以使用字符串、对象或数组表达式来条件性地绑定类名：

```tsx
export default (props: { hidden: boolean }) => (
  <>
    <h1 class={props.hidden && 'hidden'}>h1</h1>
    <h2 class={{ 'hidden': props.hidden }}>h2</h2>
    <h3 class={[ props.hidden && 'hidden' ]}>h3</h3>
  </>
)
```

现在，尝试给 `<h1>` 添加一个动态的 `class` 绑定，使用 `titleClass` 变量作为它的值。如果绑定正确，文本应该会变成红色。

</jsx-repl>
