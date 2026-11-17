# 配置

Vue JSX 的类型完全由原生 TypeScript 提供，无需编辑器插件。本页介绍 `tsconfig` 配置、JSX 类型的使用方式，以及如何扩展这些类型。

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

## 扩展 JSX 类型

`JSX` namespace 是一个普通的导出 namespace，库和应用都可以通过模块增强（module augmentation）来扩展它：

```ts [jsx.d.ts]
export {}

declare module 'vue-jsx' {
  namespace JSX {
    interface IntrinsicElements {
      'user-card': {
        name: string
        compact?: boolean
        onSelect?: (event: CustomEvent<[string]>) => void
      }
    }
  }
}
```

增强之后，`<user-card>` 会接受严格的类型检查，而其他未知元素仍回退到宽松的
`[name: string]: any` 索引签名。完整示例请参考[自定义元素](../features/custom-elements)。

### 规则

要让增强生效，有两条规则：

- 文件必须是一个模块。至少保留一个顶层 `import` 或 `export`（即上面的 `export {}`）。
  在 script 文件里，`declare module 'vue-jsx'` 会变成环境模块声明，遮蔽真实的包，
  导致 `vue-jsx` 导出的所有类型消失。
- 增强目标必须是 `jsxImportSource` 指定的模块名。增强内部的 `@vue-jsx/runtime`
  包不会影响 JSX 检查。

### 扩展点

其他扩展点用于给大量元素添加通用 props：

```ts
export {}

declare module 'vue-jsx' {
  // 每个组件都可用的 props
  namespace JSX {
    interface IntrinsicAttributes {
      'v-focus'?: boolean
    }
  }
  // 每个原生 HTML 元素都可用的 props
  interface HTMLAttributes {
    'v-focus'?: boolean
  }
}
```

只有 `interface` 成员可以合并。`JSX.Element` 和 `JSX.LibraryManagedAttributes`
是 type alias，在增强中重新定义会被静默忽略。

## 相关页面

- [组件类型](./component-types) —— props、slots、emits 与 `ref` 如何到达调用点，以及泛型组件的 helper 类型。
- [Volar 插件](./volar) —— 可选的编辑器与命令行类型检查支持，覆盖 `.tsx` 与 `.vue` 中的指令和宏语法。
