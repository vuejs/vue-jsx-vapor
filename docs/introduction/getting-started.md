# Getting Started

`vue-jsx-vapor` is a Vapor Mode of `vue-jsx`. It supports all directives and most macros of Vue.

We assume you are already familiar with the basic usages of Vue before you continue.

## Requirements

- Node.js `>= v18.0.0`.
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
pnpm add https://pkg.pr.new/vue@b43f4f5
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

Because of `vue-jsx-vapor` support all directives and most macros of Vue, so we need the VSCode plugin [ts-macro](https://github.com/ts-macro/ts-macro) to use the `vue-jsx-vapor/volar` plugin for Typescript support.\
It works similarly to [@vue/language-tools](https://github.com/vuejs/language-tools) but only used for `ts` or `tsx` files.

By default, after installing the `ts-macro` VSCode plugin, `ts-macro` will automatically load `vue-jsx-vapor/volar` by analyzing `vite.config.ts` and shared vueJsxVapor's options. so you don't need to config `tsm.config.ts`. But if you want, you can also configure it manually:

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
