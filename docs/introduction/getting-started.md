# Getting Started

Vue JSX is a high-performance JSX compiler for Vue, written in Rust and powered
by Oxc. It generates Vue Virtual DOM code by default and can optionally generate
code for Vapor Mode.

This guide assumes familiarity with Vue and Vite.

## Requirements

- Virtual DOM mode supports Vue 3.
- Vapor mode requires Vue 3.6 or later.

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

See [Macros](../features/macros) and [Directives](../features/directives) for
optional syntax transforms and their type support.
