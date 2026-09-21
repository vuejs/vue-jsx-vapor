# 选项

`vueJsx` 插件同时接受插件选项和编译器选项：

```ts [vite.config.ts]
import vueJsx from 'vue-jsx/vite'

export default {
  plugins: [
    vueJsx({
      vapor: false,
      optimize: true,
      mergeProps: true,
      sourceMap: true,
      hmr: true,
    }),
  ],
}
```

## 插件选项

| 选项      | 默认值                                      | 说明                                                                                                       |
| --------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `include` | <code>/\.[cm]?[jt]sx(?=$&#124;[?#])/</code> | 要转换的文件，接受 `unplugin` 的过滤规则。                                                                 |
| `exclude` | `/node_modules/`                            | 要跳过的文件。                                                                                             |
| `vapor`   | `false`                                     | 将所有 JSX 编译为 Vapor。关闭时，`.vapor.jsx`、`.vapor.tsx` 文件和 Vapor 组件边界中的 JSX 仍会启用 Vapor。 |
| `macros`  | `false`                                     | 启用 JSX 宏。可以设置为 `true` 或传入宏选项对象，详见[宏](../features/macros)。                            |

`include` 和 `exclude` 只控制插件处理哪些文件，不会改变 JSX 的语义。`vapor` 控制匹配文件的渲染模式，而文件名和组件边界可以让单个 JSX 子树启用 Vapor。

## 编译器选项

以下选项会传递给 `@vue-jsx/compiler`。

### `mergeProps`

控制 JSX spread 属性的合并方式。默认值为 `true`，使用 Vue 的 `mergeProps` 行为：

- `onClick` 等事件监听器会被合并。
- `class` 和 `style` 会被规范化并合并。
- 其他属性由后面的值覆盖前面的值。

从手写 `h()` 调用迁移，并且依赖普通对象展开语义时，可以设置为 `false`。此时后面的属性会直接覆盖前面的属性，生成结果更接近：

```ts
h('div', { ...firstProps, ...secondProps })
```

### `optimize`

默认值为 `true`，只在 Virtual DOM 模式下生效。它会启用稳定插槽检测、事件处理函数缓存、静态提升和 block tree 等编译器优化。

当兼容性比运行时性能更重要时，可以设置 `optimize: false`。这样生成的代码结构会更接近 Babel 的 Vue JSX 转换结果，编译过程也更简单、更快，适合迁移已有代码或调试生成结果。但应用会失去 Virtual DOM 的编译期性能优化。该选项对 Vapor 输出没有影响。

### 渲染和构建选项

| 选项                | 默认值            | 说明                                                                                                                            |
| ------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `vapor`             | `false`           | 为转换后的 JSX 启用 Vapor 输出。                                                                                                |
| `hmr`               | `false`           | 启用 HMR 组件注册。传入 `Hmr` 对象可以通过 `defineComponentName` 自定义组件定义函数名。Vite 开发模式会自动启用插件的 HMR 处理。 |
| `ssr`               | `false`           | 生成面向 SSR 的输出，通常由构建工具的 SSR 转换流程控制。                                                                        |
| `sourceMap`         | `false`           | 为转换后的文件生成 source map。构建工具的开发模式和 source map 配置可能会自动启用它。                                           |
| `filename`          | `'index.jsx'`     | 用于 source map 和组件自身递归引用的源文件名。                                                                                  |
| `runtimeModuleName` | 虚拟 runtime 模块 | 覆盖生成的 runtime 辅助函数导入路径，用于自定义 runtime 或集成场景。                                                            |

### `onError` 和 `onWarn`

可以使用这两个回调收集或处理编译器诊断信息：

```ts
vueJsx({
  onError(error) {
    console.error(error)
  },
  onWarn(warning) {
    console.warn(warning)
  },
})
```
