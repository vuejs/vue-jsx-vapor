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
pnpm add https://pkg.pr.new/vue@73bceb2
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

Since `vue-jsx-vapor` supports Vue directives and Vue macros, so need to install the [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) VSCode plugin to load the `vue-jsx-vapor/volar` plugin for type support.

The `TS Macro` VSCode plugin will automatically loads the `vue-jsx-vapor/volar` by analyzing `vite.config.ts` and shares the user configuration of the `vue-jsx-vapor/vite` plugin, without the need to manually configure `ts-macro.config.ts`.

::: details manually configuration
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

## Templates

- [vitesse-jsx-vapor](https://github.com/zhiyuanzmj/vitesse-jsx-vapor)
