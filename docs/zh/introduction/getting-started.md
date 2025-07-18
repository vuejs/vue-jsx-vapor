# 快速上手

`vue-jsx-vapor` 是 `vue-jsx` 的 Vapor 模式。它支持 Vue 的所有指令和大部分宏。

在继续之前，我们假设您已经熟悉 Vue 的基本用法。

## 环境要求

- Vue `>= v3.6`。
- VSCode 扩展 [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) 并且需要安装 `@ts-macro/tsc` 来替代 `tsc` 进行类型检查。
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
pnpm add vue-jsx-vapor

# 运行时
pnpm add @vue@3.6.0-alpha.2
```

## 配置

::: code-group

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ],
})
```

:::

## Typescript

由于 `vue-jsx-vapor` 支持 Vue 指令和 Vue 宏，所以需要安装 [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) 的 VSCode 插件来加载 `vue-jsx-vapor/volar` 插件, 以获得类型支持。

`TS Macro` 的 VSCode 会通过分析 `vite.config.ts` 来自动加载 `vue-jsx-vapor/volar` 插件并共享 `vue-jsx-vapor/vite` 插件的用户配置，无需手动配置 `ts-macro.config.ts`。


::: details 手动配置

::: code-group

```ts [ts-macro.config.ts]
import vueJsxVapor from 'vue-jsx-vapor/volar'

export default {
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ],
}
```

:::

## 模板

- [vitesse-jsx-vapor](https://github.com/zhiyuanzmj/vitesse-jsx-vapor)
