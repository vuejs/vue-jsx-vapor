# Configuration

Vue JSX is typed in pure TypeScript — no editor plugin required. This page
covers the `tsconfig` setup, how to use the JSX types, and how to extend them.

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

## Extending the JSX Types

The `JSX` namespace is a regular exported namespace, so libraries and
applications can extend it with module augmentation:

```ts [jsx.d.ts]
export {}

declare module 'vue-jsx' {
  namespace JSX {
    interface IntrinsicElements {
      'user-card': {
        name: string
        compact?: boolean
        onSelect?: (event: CustomEvent<[string]>) => void
      }
    }
  }
}
```

After augmentation, `<user-card>` is strictly checked, while other unknown
elements keep falling back to the permissive `[name: string]: any` index
signature. See [Custom Elements](../features/custom-elements) for a complete
example.

### Rules

Two rules make the augmentation work:

- The file must be a module. Keep at least one top-level `import` or `export`
  (the `export {}` above). In a script file, `declare module 'vue-jsx'` becomes
  an ambient module declaration that shadows the real package, and every type
  exported from `vue-jsx` disappears.
- Target the module named in `jsxImportSource`. Augmenting the internal
  `@vue-jsx/runtime` package has no effect on JSX checking.

### Extension Points

Other extension points cover props shared by many elements:

```ts
export {}

declare module 'vue-jsx' {
  // available on every component
  namespace JSX {
    interface IntrinsicAttributes {
      'v-focus'?: boolean
    }
  }
  // available on every native HTML element
  interface HTMLAttributes {
    'v-focus'?: boolean
  }
}
```

Only `interface` members merge. `JSX.Element` and `JSX.LibraryManagedAttributes`
are type aliases; redefining them in an augmentation is silently ignored.

## Related Pages

- [Component Types](./component-types) — how props, slots, emits and `ref` reach
  the call site, and the helper types for generic components.
- [Volar Plugin](./volar) — optional editor and command-line type support for
  directive and macro syntax, in both `.tsx` and `.vue` files.
