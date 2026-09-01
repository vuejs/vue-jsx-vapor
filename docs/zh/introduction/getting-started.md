# 快速上手

Vue JSX 是一个高性能的 Vue JSX 编译器，使用 Rust 编写并基于 Oxc 构建。它默认生成 Vue Virtual DOM 代码，也可以按需生成 Vapor DOM 代码。

本指南假设你已经熟悉 Vue。

## 环境要求

- Virtual DOM 模式支持 Vue 3。
- Vapor 模式需要 Vue 3.6 或更高版本。
- 如果使用了 directives 或者 macros 语法, 建议安装 VSCode 扩展 [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) 用于类型提示，然后再安装 `@ts-macro/tsc` 来替代 `tsc` 进行类型检查。
  ```json
  // package.json
  {
    "scripts": {
      "typecheck": "tsmc --noEmit"
      // ...
    }
  }
  ```

## 安装

```bash [pnpm]
# 插件
pnpm add vue-jsx

# 运行时
pnpm add vue@3.6.0-rc.5
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

## 可选宏

宏默认关闭，可以显式开启：

```ts [vite.config.ts]
vueJsx({
  macros: true,
})
```

TS Macro 扩展可以从 `vite.config.ts` 自动发现 `vue-jsx/volar`。也可以手动配置：

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx({ macros: true })],
}
```

命令行类型检查可以使用：

```bash
pnpm add -D @ts-macro/tsc
```

```json [package.json]
{
  "scripts": {
    "typecheck": "tsmc --noEmit"
  }
}
```
