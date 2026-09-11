# TypeScript

Vue JSX is typed in pure TypeScript — no editor plugin required. This page
covers the `tsconfig` setup and how to use the JSX types.

## tsconfig

```json [tsconfig.json]
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

- `jsx: "preserve"` keeps JSX syntax intact so the Vue JSX plugin compiles it;
  TypeScript only type-checks the JSX expressions.
- `jsxImportSource` selects `vue-jsx/jsx-runtime`, which exports the `JSX`
  namespace TypeScript uses to check JSX call sites.

`jsxImportSource` is required: there is no global `JSX` namespace, so classic
JSX mode without `jsxImportSource` won't be type-checked.

For the `vue-jsx-vapor` package, set `"jsxImportSource": "vue-jsx-vapor"`
instead.

### Mixed React/Vue Codebase

In a mixed React/Vue codebase that shares one `tsconfig`, use a per-file
pragma instead:

```tsx
/** @jsxImportSource vue-jsx */
```

## Importing JSX Types

There is no global `JSX` namespace. In type positions, import the namespace
explicitly:

```ts
import type { JSX } from 'vue-jsx'

type Element = JSX.Element
type DivAttrs = JSX.IntrinsicElements['div']
```

## Global JSX Types

If you still want a global `JSX` namespace, write your own `global.d.ts`:

```ts [global.d.ts]
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

This restores `JSX.Element`, `JSX.IntrinsicElements`, and friends in type
positions without imports.

## Related Pages

- [Extending the JSX Types](./extending-jsx-types) — add Custom Elements or
  shared props with module augmentation.
- [Volar Plugin](./volar) — optional editor and command-line type support for
  directive and macro syntax, in both `.tsx` and `.vue` files.
