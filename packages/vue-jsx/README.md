# vue-jsx

[![NPM version](https://img.shields.io/npm/v/vue-jsx?color=a1b858&label=)](https://www.npmjs.com/package/vue-jsx)

High-performance Vue JSX Compiler powered by Oxc.

## Features

- ⚡️ High Performance: Brings Vue compiler optimizations to JSX for efficient runtime code.
- 💨 Vapor Mode: Compiles JSX for Vapor Mode with fine-grained reactive updates.
- 🦀 Rust Compiler: Powered by Oxc, 30× faster for Virtual DOM and 50× faster for Vapor than Babel.
- 🦾 Type Safe: Native TypeScript 7.0 support with automatic inference for JSX component props, refs, and children.
- ✨ Unplugin: Integrates with Vite, Rollup, Rolldown, webpack, Rspack, Rsbuild, esbuild, Bun, and more.
- 📦 Custom Element: Supports using and defining Custom Elements out of the box.

## Installation

```bash
pnpm add vue-jsx
```

## Vite

```ts
import { defineConfig } from 'vite'
import vueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [vueJsx()],
})
```

Add the JSX runtime to TypeScript:

```json
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

`jsxImportSource` is required: there is no global `JSX` namespace, so classic
JSX mode without `jsxImportSource` won't be type-checked. In a mixed
React/Vue codebase that shares one `tsconfig`, use a per-file pragma instead:

```tsx
/** @jsxImportSource vue-jsx */
```

In type positions, import the `JSX` namespace explicitly instead of relying on
a global:

```ts
import type { JSX } from 'vue-jsx'
```

If you prefer to keep a global `JSX` namespace, write your own `global.d.ts`:

```ts
import type { JSX as VueJSX } from 'vue-jsx'

declare global {
  namespace JSX {
    type Element = VueJSX.Element
    type ElementChildrenAttribute = VueJSX.ElementChildrenAttribute
    type IntrinsicElements = VueJSX.IntrinsicElements
    type IntrinsicAttributes = VueJSX.IntrinsicAttributes
    type LibraryManagedAttributes<Component, Props> = VueJSX.LibraryManagedAttributes<
      Component,
      Props
    >
  }
}
```

To extend the JSX types, augment the module named in `jsxImportSource` from a
module file (one with a top-level `import` or `export`):

```ts
export {}

declare module 'vue-jsx' {
  namespace JSX {
    interface IntrinsicElements {
      'user-card': { name: string }
    }
  }
}
```

## Vapor Mode

```ts
vueJsx({
  vapor: true,
})
```

When `vapor` is omitted or `false`, regular `.tsx` and `.jsx` files compile to
Vue Virtual DOM. You can still opt individual components or files into Vapor by
using `defineVaporComponent`, `defineVaporCustomElement`, `.vapor.tsx`, or
`.vapor.jsx`.

## Integrations

The package also exports plugins for Rollup, Rolldown, webpack, Rspack,
Rsbuild, esbuild, Bun, Nuxt, and Astro.

- [Documentation](https://vuejsx.dev/)
- [Playground](https://repl.vuejsx.dev/)
