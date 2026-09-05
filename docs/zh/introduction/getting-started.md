# 快速上手

Vue JSX 是一个高性能的 Vue JSX 编译器，使用 Rust 编写并基于 Oxc 构建。它默认生成 Vue Virtual DOM 代码，也可以按需生成 Vapor Mode 代码。

本指南假设你已经熟悉 Vue。

## 环境要求

- Virtual DOM 模式支持 Vue 3。
- Vapor 模式需要 Vue 3.6 或更高版本。

## 安装

```bash [pnpm]
# 插件
pnpm add vue-jsx

# 运行时
pnpm add vue@3.6.0-rc.7
```

## Vite 配置

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [vueJsx()],
})
```

默认情况下，普通 `.jsx` 和 `.tsx` 文件会被编译为 Vue Virtual DOM。需要生成 Vapor 代码时，请阅读 [Vapor 模式](./interop)。

## TypeScript 配置

```json [tsconfig.json]
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

`jsxImportSource` 用来选择 JSX 类型和自动 JSX runtime 声明，它不会开启 Vapor 模式。最终生成哪种渲染代码由编译器的 `vapor` 选项决定。

## 组件命名

Vue JSX 遵循标准 JSX 的标签命名约定：

```tsx
import UserCard from './UserCard'

export function App() {
  return (
    <div>
      <UserCard />
      <UserCard.Header />
    </div>
  )
}
```

小写字母开头的标签始终按原生元素处理，包括未知标签和 kebab-case 标签，不会被解析为 Vue 组件：

```tsx
<button />       // 原生 HTML 元素
<my-widget />    // 元素标签，不是组件
```

使用大写标识符或成员表达式表示 Vue 组件。这样可以让组件解析行为保持确定，避免未来 HTML 新增原生标签时改变已有组件的含义。

可选语法转换及其类型支持请阅读[宏](../features/macros)和[指令](../features/directives)章节。
