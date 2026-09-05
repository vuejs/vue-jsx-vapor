# Vapor 模式

Vue JSX 默认把普通 JSX 编译为 Vue Virtual DOM。Vapor 是一项可选的输出模式，需要 Vue 3.6 或更高版本。

## 全局开启 Vapor

如果整个应用都基于 Vapor，可以设置 `vapor: true`：

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [
    vueJsx({
      vapor: true,
    }),
  ],
})
```

根组件需要使用 `createVaporApp` 挂载：

```ts [main.ts]
import { createVaporApp } from 'vue'
import App from './App.tsx'

createVaporApp(App).mount('#app')
```

## 渐进式启用

不必为整个项目开启 Vapor。在默认的 `vapor: false` 下，以下代码仍会按 Vapor 编译：

- 以 `.vapor.jsx` 或 `.vapor.tsx` 结尾的文件。
- `defineVaporComponent` 或 `defineVaporCustomElement` 管理的 JSX。

```tsx
import { defineVaporComponent } from 'vue'

export const Counter = defineVaporComponent((props: { count: number }) => {
  return <button>{props.count}</button>
})
```

其他普通组件和文件仍然生成 Virtual DOM。

## 混合两种渲染模式

当组件树需要跨越 Virtual DOM 和 Vapor 边界时，请安装 Vue 提供的 `vaporInteropPlugin`。

### 在 Virtual DOM 应用中使用 Vapor

```ts [main.ts]
import { createApp, vaporInteropPlugin } from 'vue'
import App from './App.tsx'

createApp(App).use(vaporInteropPlugin).mount('#app')
```

保持默认的 `vapor: false`，再用 `defineVaporComponent` 或 `.vapor.tsx` 文件定义 Vapor 子树。

### 在 Vapor 应用中使用 Virtual DOM

```ts [main.ts]
import { createVaporApp, vaporInteropPlugin } from 'vue'
import App from './App.vapor.tsx'

createVaporApp(App).use(vaporInteropPlugin).mount('#app')
```

把 Virtual DOM 组件放在普通文件中，并使用 `defineComponent` 定义。

## Vapor Runtime 辅助方法

`vue-jsx/vapor` 入口用常见名称导出了 Vapor 专用 runtime：

```ts
import {
  For,
  KeepAlive,
  Teleport,
  Transition,
  TransitionGroup,
  h,
} from 'vue-jsx/vapor'
```

这个入口只影响 runtime 导入。编译模式仍由 `vapor` 选项、文件名或 Vapor 组件边界决定。
