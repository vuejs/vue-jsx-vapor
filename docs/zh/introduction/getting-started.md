# 快速上手

Vue JSX Vapor 是一个受 `Vue Compiler` 启发的 `Vue JSX 编译器`，用 Rust 🦀 编写，并由 Oxc 驱动。它支持生成 Virtual DOM 和 Vapor Mode。

在继续之前，我们假设您已经熟悉 Vue 的基本用法。

## 环境要求

- Vapor 模式需要 Vue `>= v3.6`。如果是使用 interop 模式的虚拟 DOM 项目，`Vue >= 3.0` 即可。
- 如果使用了 directives 或者 macros, 我们建议安装 VSCode 扩展 [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) 来让 Volar 插件支持 TSX，然后再安装 `@ts-macro/tsc` 来替代 `tsc` 进行类型检查。
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
pnpm add vue@3.6.0-beta.5
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

### 配置 `tsconfig.json`
```json
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx-vapor",
    // ...
  }
}
```

### Volar 插件

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
- [vue-jsx-vapor-ssr](https://github.com/zhiyuanzmj/vue-jsx-vapor-ssr)
