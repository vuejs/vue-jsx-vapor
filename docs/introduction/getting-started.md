# Getting Started

Vue JSX Vapor is a `Vue JSX Compiler` inspired by `Vue Compiler`, written in Rust 🦀, and powered by Oxc. It supports generating Virtual DOM and Vapor Mode.

We assume you are already familiar with the basic usages of Vue before you continue.

## Setup

- Vapor Mode requires Vue >= 3.6. For interop in Virtual DOM projects, Vue >= 3.0 is enough.
- If you use directives or macros, we recommend installing the VSCode extension [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) to enable the Volar plugin for TSX and install `@ts-macro/tsc` instead of `tsc` for type checking.
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
pnpm add vue@3.6.0-beta.6
```

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

### Configure `tsconfig.json`
```json
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx-vapor",
    // ...
  }
}
```

### Volar plugin

The `TS Macro` VSCode plugin automatically loads the `vue-jsx-vapor/volar` by analyzing `vite.config.ts` and shares the user configuration of the `vue-jsx-vapor/vite` plugin, so you don't need to configure `ts-macro.config.ts` manually.

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
- [vue-jsx-vapor-ssr](https://github.com/zhiyuanzmj/vue-jsx-vapor-ssr)
