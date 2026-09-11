# 扩展 JSX 类型

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

## 规则

要让增强生效，有两条规则：

- 文件必须是一个模块。至少保留一个顶层 `import` 或 `export`（即上面的 `export {}`）。
  在 script 文件里，`declare module 'vue-jsx'` 会变成环境模块声明，遮蔽真实的包，
  导致 `vue-jsx` 导出的所有类型消失。
- 增强目标必须是 `jsxImportSource` 指定的模块名。增强内部的 `@vue-jsx/runtime`
  包不会影响 JSX 检查；使用 `vue-jsx-vapor` 包时，应改为
  `declare module 'vue-jsx-vapor'`。

## 扩展点

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
