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

## Component Naming

Vue JSX follows the standard JSX naming convention:

```tsx
import UserCard from './UserCard'

export function App() {
  return (
    <div>
      <UserCard />
      <UserCard.Header />
    </div>
  )
}
```

Tags that start with a lowercase letter are always treated as intrinsic
elements, including unknown or kebab-case tags. They are not resolved as Vue
components:

```tsx
<button />       // Native HTML element
<my-widget />    // Element tag, not a component
```

Use an uppercase identifier or a member expression for Vue components. This
keeps component resolution deterministic and avoids changing the meaning of an
existing component when HTML adds a new native element in the future.

## Optional Macros

Macros are disabled by default:

```ts [vite.config.ts]
vueJsx({
  macros: true,
})
```

The TS Macro extension can discover the Vue JSX integration from
`vite.config.ts` automatically. Alternatively, create `ts-macro.config.ts` in
the project root to configure the `vue-jsx/volar` plugin explicitly:

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx({ macros: true })],
}
```

Keep its `macros` option consistent with `vite.config.ts` so the editor,
command-line type checker, and compiler use the same syntax.

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
