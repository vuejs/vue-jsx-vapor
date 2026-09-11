# Extending the JSX Types

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

## Rules

Two rules make the augmentation work:

- The file must be a module. Keep at least one top-level `import` or `export`
  (the `export {}` above). In a script file, `declare module 'vue-jsx'` becomes
  an ambient module declaration that shadows the real package, and every type
  exported from `vue-jsx` disappears.
- Target the module named in `jsxImportSource`. Augmenting the internal
  `@vue-jsx/runtime` package has no effect on JSX checking. With the
  `vue-jsx-vapor` package, use `declare module 'vue-jsx-vapor'` instead.

## Extension Points

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
