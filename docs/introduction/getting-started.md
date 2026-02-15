# Getting Started

Vue JSX Vapor is a Vue JSX compiler inspired by the Vue Compiler, written in Rust 🦀 and powered by Oxc. It supports both Virtual DOM and Vapor Mode code generation.

This guide assumes familiarity with Vue fundamentals.

## Prerequisites

- **Vapor Mode** requires Vue 3.6 or later. For projects using only Virtual DOM (Interop Mode), Vue 3.0+ is sufficient.
- If you plan to use directives or macros, we recommend installing the [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro) VSCode extension to enable the Volar plugin for TSX. Additionally, use `@ts-macro/tsc` instead of `tsc` for type checking:

  ```json
  // package.json
  {
    "scripts": {
      "typecheck": "tsmc --noEmit"
    }
  }
  ```

## Installation

```bash
# Plugin
pnpm add vue-jsx-vapor

# Runtime
pnpm add vue@3.6.0-beta.6
```

## Configuration

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

## TypeScript Configuration

### tsconfig.json

```json
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx-vapor"
  }
}
```

### Volar Plugin

The TS Macro VSCode extension automatically loads `vue-jsx-vapor/volar` by analyzing your `vite.config.ts`. It shares the configuration from the `vue-jsx-vapor/vite` plugin, eliminating the need to manually configure `ts-macro.config.ts`.

::: details Manual Configuration


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

## Starter Templates

- [vitesse-jsx-vapor](https://github.com/zhiyuanzmj/vitesse-jsx-vapor) – Opinionated starter template
- [vue-jsx-vapor-ssr](https://github.com/zhiyuanzmj/vue-jsx-vapor-ssr) – SSR example
