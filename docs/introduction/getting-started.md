# Getting Started

Vue JSX is a high-performance JSX compiler for Vue, written in Rust and powered
by Oxc. It generates Vue Virtual DOM code by default and can optionally generate
code for Vapor Mode.

This guide assumes familiarity with Vue and Vite.

## Requirements

- Virtual DOM mode supports Vue 3.
- Vapor mode requires Vue 3.6 or later.
- The optional directives, ref, and macros type transforms use the
  [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro)
  VS Code extension. Use `@ts-macro/tsc` when these transforms must also run in
  command-line type checking.

## Installation

```bash
pnpm add vue-jsx
```

## Vite Configuration

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [vueJsx()],
})
```

This configuration compiles regular `.jsx` and `.tsx` files to Vue Virtual DOM.
See [Vapor Mode](./interop) when you want Vapor output.

## TypeScript Configuration

```json [tsconfig.json]
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

`jsxImportSource` selects the JSX types and automatic JSX runtime declarations.
It does not enable Vapor mode; the `vapor` compiler option controls the emitted
rendering mode.

## Optional Macros

Macros are disabled by default:

```ts [vite.config.ts]
vueJsx({
  macros: true,
})
```

The TS Macro extension discovers `vue-jsx/volar` from `vite.config.ts`. For an
explicit configuration:

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx({ macros: true })],
}
```

For command-line type checking:

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
