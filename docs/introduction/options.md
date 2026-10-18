# Options

The `vueJsx` plugin accepts both plugin options and compiler options:

```ts [vite.config.ts]
import vueJsx from 'vue-jsx/vite'

export default {
  plugins: [
    vueJsx({
      vapor: false,
      optimize: true,
      mergeProps: true,
      sourceMap: true,
      hmr: true,
    }),
  ],
}
```

## Plugin Options

| Option    | Default                                     | Description                                                                                                                   |
| --------- | ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `include` | <code>/\.[cm]?[jt]sx(?=$&#124;[?#])/</code> | Files to transform. Accepts an `unplugin` filter pattern.                                                                     |
| `exclude` | `/node_modules/`                            | Files to skip.                                                                                                                |
| `vapor`   | `false`                                     | Compile all JSX as Vapor. When `false`, Vapor is still enabled by `.vapor.jsx`, `.vapor.tsx`, and Vapor component boundaries. |
| `macros`  | `false`                                     | Enable JSX macros. Pass `true` or a macros options object. See [Macros](../features/macros).                                  |

`include` and `exclude` control which files the plugin sees; they do not
change the compiler's JSX semantics. `vapor` selects the rendering mode for
the matched files, while the filename and component boundaries can opt
individual JSX subtrees into Vapor.

## Compiler Options

These options are passed to `@vue-jsx/compiler`.

### `mergeProps`

Controls how JSX spread attributes are combined. It defaults to `true`, which
uses Vue's `mergeProps` behavior:

- Event listeners such as `onClick` are merged.
- `class` and `style` values are normalized and merged.
- Other properties are overridden by later values.

Set it to `false` when migrating code from manually-written `h()` calls that
relies on ordinary object spread semantics. In that mode, later properties
simply override earlier properties, so the generated code is closer to:

```ts
h('div', { ...firstProps, ...secondProps })
```

### `optimize`

Defaults to `true` and is used only in Virtual DOM mode. It enables compiler
optimizations such as stable slot detection, event handler caching, static
hoisting, and block tree generation.

Set `optimize: false` when compatibility with less optimized JSX output is more
important than runtime performance. The result is structurally closer to the
output of Babel's Vue JSX transform, with a simpler and faster compilation pass.
This can help when migrating existing code or debugging generated output, but
the resulting application gives up the Virtual DOM performance optimizations.
The option has no effect on Vapor output.

### Rendering and Build Options

| Option              | Default                | Description                                                                                                                                                        |
| ------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `vapor`             | `false`                | Enable Vapor output for the transformed JSX.                                                                                                                       |
| `hmr`               | `false`                | Enable HMR component registration. Pass an `Hmr` object to customize `defineComponentName`. Vite development mode enables the plugin's HMR handling automatically. |
| `ssr`               | `false`                | Generate SSR-oriented output. Usually controlled by the bundler's SSR transform.                                                                                   |
| `sourceMap`         | `false`                | Generate a source map for the transformed file. Bundler development mode and build source-map settings may enable this automatically.                              |
| `filename`          | `'index.jsx'`          | Source filename used for source maps and self-recursive component references.                                                                                      |
| `runtimeModuleName` | virtual runtime module | Override the module path used for generated runtime helper imports. Useful for custom runtimes or integrations.                                                    |

### `onError` and `onWarn`

Use these callbacks to collect or customize compiler diagnostics:

```ts
vueJsx({
  onError(error) {
    console.error(error)
  },
  onWarn(warning) {
    console.warn(warning)
  },
})
```
