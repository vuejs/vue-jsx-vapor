# Getting Started

`vue-jsx-vapor` is a Vapor Mode of `vue-jsx`. It supports all directives and most macros of Vue.

We assume you are already familiar with the basic usages of Vue before you continue.

## Requirements

- Vue `>= v3.6`.
- VSCode extension [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) and install `@ts-macro/tsc` instead of `tsc` to typecheck.
  ```json
  // package.json
  {
    "scripts": {
      "typecheck": "tsmc --noEmit"
      // ...
    }
  }
  ```

## Install

```bash [pnpm]
# plugin
pnpm add vue-jsx-vapor

# runtime
pnpm add https://pkg.pr.new/vue@1c02a3e
```

The Vue Vapor runtime is not release, so we use [pkg.pr.new](https://github.com/stackblitz-labs/pkg.pr.new) to install.

## Setup

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

Due to `vue-jsx-vapor`'s support for all Vue directives and most Vue macros, the [ts-macro](https://github.com/ts-macro/ts-macro) VSCode plugin is required to enable TypeScript support when using the `vue-jsx-vapor/volar` plugin.

After installing the `ts-macro` VSCode plugin, it automatically loads `vue-jsx-vapor/volar` by analyzing vite.config.ts and shared vueJsxVapor options, eliminating the need to configure tsm.config.ts.

::: code-group

```ts [tsm.config.ts]
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

## Templates

- [vitesse-jsx-vapor](https://github.com/zhiyuanzmj/vitesse-jsx-vapor)
