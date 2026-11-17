---
aside: false
prev: false
next: false
---

# Custom Element

<script setup>
import appCode from '~/tutorial/step-15/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-15/app-solved.tsx?raw'
import appVaporCode from '~/tutorial/step-15/app-vapor.tsx?raw'
import appVaporSolvedCode from '~/tutorial/step-15/app-vapor-solved.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  vapor: { 'src/App.tsx': appVaporCode },
  vaporSolved: { 'src/App.tsx': appVaporSolvedCode },
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-14/" next="/zh/tutorial/done/">

Custom Element 是拥有独立标签名的浏览器原生组件。Vue JSX 会自动识别名称为小写且包含连字符的标签，因此 `<tutorial-user-card>` 无需额外配置编译器。

## 定义元素

Virtual DOM 组件使用 `defineCustomElement`：

```tsx
const UserCardElement = defineCustomElement(
  (props: { name: string }) => () => <strong>{props.name}</strong>,
  { props: { name: String } },
)
```

Vapor 模式使用 `defineVaporCustomElement`，并直接返回 JSX：

```tsx
const UserCardElement = defineVaporCustomElement(
  (props: { name: string }) => <strong>{props.name}</strong>,
  { props: { name: String } },
)
```

运行时 `props` 选项会让 `name` 成为 Custom Element 监听的 attribute；TypeScript 标注则负责 setup 函数内部的静态类型检查。

## 注册标签

浏览器需要知道标签对应哪个构造器。通过 `customElements.define()` 注册一次：

```ts
if (!customElements.get('tutorial-user-card')) {
  customElements.define('tutorial-user-card', UserCardElement)
}
```

开发和热更新期间，这个判断可以避免重复注册同名标签的错误。

## 传递具名插槽

通过标准 `slot` 属性，可以把内容分配给原生具名插槽：

```tsx
<tutorial-user-card name="Ada Lovelace">
  <span slot="avatar">AL</span>
</tutorial-user-card>
```

Custom Element 内部通过 `<slot name="avatar">` 渲染这段内容。Vue Custom Element 默认使用 Shadow DOM，因此示例通过 `styles` 选项传入组件样式。

现在把 `UserCardElement` 注册为 `<tutorial-user-card>`。完成后切换 Virtual DOM 和 Vapor 模式，确认两种模式下的元素具有一致的交互行为。

</jsx-repl>
