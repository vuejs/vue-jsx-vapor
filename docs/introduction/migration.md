# Migration Guide

## From `vue-jsx-vapor`

Version 3.3 moves the primary package to `vue-jsx` and changes the default
compiler output to Virtual DOM.

### Rename the package

```diff
- pnpm add vue-jsx-vapor
+ pnpm add vue-jsx
```

Update plugin and type-runtime imports:

```diff
- import vueJsxVapor from 'vue-jsx-vapor/vite'
+ import vueJsx from 'vue-jsx/vite'
```

```diff
{
  "compilerOptions": {
-   "jsxImportSource": "vue-jsx-vapor"
+   "jsxImportSource": "vue-jsx"
  }
}
```

The scoped companion packages are now `@vue-jsx/compiler`,
`@vue-jsx/runtime`, `@vue-jsx/macros`, and `@vue-jsx/eslint`.

### Choose the rendering mode

The old package compiled to Vapor by default and used `interop: true` for
mixed/Virtual DOM projects. The new package compiles to Virtual DOM by default:

```ts
import vueJsx from 'vue-jsx/vite'

export default {
  plugins: [vueJsx()],
}
```

To preserve an application that previously used the default Vapor output, add
`vapor: true`:

```ts
vueJsx({
  vapor: true,
})
```

Remove the old `interop` option. For incremental Vapor adoption, keep
`vapor: false` and use `.vapor.tsx`, `.vapor.jsx`, `defineVaporComponent`, or
`defineVaporCustomElement`.

### Update Vapor runtime imports

Vapor aliases such as `h`, `For`, and `Transition` live under `vue-jsx/vapor`:

```ts
import { For, Transition, h } from 'vue-jsx/vapor'
```

### Check the Vue version

Virtual DOM output works with Vue 3. Vapor output requires Vue 3.6 or later.

## From Babel Vue JSX

1. Replace `@vitejs/plugin-vue-jsx` (or the equivalent Babel plugin) with the
   `vue-jsx` integration for your bundler.
2. Set `jsxImportSource` to `vue-jsx`.
3. Start with the default Virtual DOM mode so existing Vue JSX component
   contracts stay familiar.
4. Enable [Vapor Mode](./interop) separately when the application is ready for
   Vue 3.6 and Vapor component semantics.

Vue JSX supports Vue directives directly in TSX. Review the
[Directives](../features/directives) page for syntax differences.
