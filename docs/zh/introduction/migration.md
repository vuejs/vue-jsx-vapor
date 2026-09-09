# 迁移指南

## 从 `vue-jsx-vapor` 迁移

3.3 版本把主包迁移到了 `vue-jsx`，并把编译器默认输出改成了 Virtual DOM。

### 修改包名

```diff
- pnpm add vue-jsx-vapor
+ pnpm add vue-jsx
```

更新插件和类型 runtime 的导入：

```diff
- import vueJsxVapor from 'vue-jsx-vapor/vite'
+ import vueJsx from 'vue-jsx/vite'
```

```diff
{
  "compilerOptions": {
-   "jsxImportSource": "vue-jsx-vapor"
+   "jsxImportSource": "vue-jsx"
  }
}
```

相关作用域包现在分别是 `@vue-jsx/compiler`、`@vue-jsx/runtime`、`@vue-jsx/macros` 和 `@vue-jsx/eslint`。

### 选择渲染模式

旧包默认生成 Vapor，并通过 `interop: true` 适配混合模式或 Virtual DOM 项目。新包默认生成 Virtual DOM：

```ts
import vueJsx from 'vue-jsx/vite'

export default {
  plugins: [vueJsx()],
}
```

如果原应用依赖旧版默认的 Vapor 输出，请添加 `vapor: true`：

```ts
vueJsx({
  vapor: true,
})
```

删除旧的 `interop` 选项。需要渐进式使用 Vapor 时，保持 `vapor: false`，并通过 `.vapor.tsx`、`.vapor.jsx`、`defineVaporComponent` 或 `defineVaporCustomElement` 开启。

### 更新 Vapor runtime 导入

`h`、`For` 和 `Transition` 等 Vapor 别名由 `vue-jsx/vapor` 提供：

```ts
import { For, Transition, h } from 'vue-jsx/vapor'
```

### 检查 Vue 版本

Virtual DOM 输出支持 Vue 3；Vapor 输出需要 Vue 3.6 或更高版本。

## 从 Babel Vue JSX 迁移

1. 把 `@vitejs/plugin-vue-jsx`（或对应的 Babel 插件）替换为 `vue-jsx` 的构建工具集成。
2. 把 `jsxImportSource` 设置为 `vue-jsx`。
3. 先使用默认 Virtual DOM 模式，保持原有 Vue JSX 组件语义。
4. 当项目已经升级到 Vue 3.6 并准备采用 Vapor 组件语义后，再单独开启 [Vapor 模式](./interop)。

Vue JSX 可以在 TSX 中直接使用 Vue 指令，具体语法请参考[指令](../features/directives)页面。
