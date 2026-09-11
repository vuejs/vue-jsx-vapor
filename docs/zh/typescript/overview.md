# TypeScript

Vue JSX 的类型完全由原生 TypeScript 提供，无需编辑器插件。本页介绍 `tsconfig` 配置与 JSX 类型的使用方式。

## tsconfig 配置

```json [tsconfig.json]
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

- `jsx: "preserve"` 保留 JSX 语法，交给 Vue JSX 插件编译，TypeScript 只负责类型检查。
- `jsxImportSource` 选择 `vue-jsx/jsx-runtime`，该模块导出的 `JSX` namespace 供 TypeScript 检查 JSX 调用处使用。

`jsxImportSource` 是必需的：这里没有全局 `JSX` namespace，不设置 `jsxImportSource` 的经典 JSX 模式不会被类型检查。

使用 `vue-jsx-vapor` 包时，改为 `"jsxImportSource": "vue-jsx-vapor"`。

### React/Vue 混合代码库

在共享同一个 `tsconfig` 的 React/Vue 混合代码库中，可以改用文件级 pragma：

```tsx
/** @jsxImportSource vue-jsx */
```

## 导入 JSX 类型

这里没有全局 `JSX` namespace。在类型位置使用时需要显式导入：

```ts
import type { JSX } from 'vue-jsx'

type Element = JSX.Element
type DivAttrs = JSX.IntrinsicElements['div']
```

## 全局 JSX 类型

如果你仍想使用全局 `JSX` namespace，可以自己编写一个 `global.d.ts`：

```ts [global.d.ts]
import type { JSX as VueJSX } from 'vue-jsx'

declare global {
  namespace JSX {
    type Element = VueJSX.Element
    type ElementChildrenAttribute = VueJSX.ElementChildrenAttribute
    type IntrinsicElements = VueJSX.IntrinsicElements
    type IntrinsicAttributes = VueJSX.IntrinsicAttributes
    type LibraryManagedAttributes<Component, Props> = VueJSX.LibraryManagedAttributes<
      Component,
      Props
    >
  }
}
```

这样 `JSX.Element`、`JSX.IntrinsicElements` 等类型无需导入即可在类型位置直接使用。

## 相关页面

- [扩展 JSX 类型](./extending-jsx-types) —— 通过模块增强添加自定义元素或通用 props。
- [Volar 插件](./volar) —— 可选的编辑器与命令行类型检查支持，覆盖 `.tsx` 与 `.vue` 中的指令和宏语法。
