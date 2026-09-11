# Volar Plugin

Directives and macros are compile-time transforms: your code runs without any
editor tooling. The `vue-jsx/volar` plugin is optional — install it only when you
want that syntax to be understood by your editor and by command-line type
checking.

The plugin adds type support for:

- **Directives** and **`ref`** — see [Directives](../features/directives) for
  which directives the plugin covers.
- **Macros** — `defineModel`, `defineSlots`,
  `defineExpose`, and `defineStyle`. Off by default; opt in with the `macros`
  option, and see [Macros](../features/macros).

The same module serves two setups. Pick the one that matches where your TSX
lives.

## TSX Files: TS Macro

For `.tsx` files, install
[TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro)
and create `ts-macro.config.ts`:

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx()],
}
```

When macros are enabled in the Vite plugin, enable the macro transform here too
and keep the options consistent with the Vite configuration. See
[Macros](../features/macros):

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx({ macros: true })],
}
```

### Command-Line Type Checking

Install `@ts-macro/tsc` and use `tsmc` in place of `tsc`:

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

`tsmc` reads the same `ts-macro.config.ts`, so no extra configuration is needed.

## SFC Files: Vue Language Tools

`.vue` files are served by Vue Language Tools — the official Vue extension and
`vue-tsc` — not by TS Macro. Register the same module as a Vue Language Tools
plugin in `tsconfig.json`:

```json [tsconfig.json]
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  },
  "vueCompilerOptions": {
    "plugins": ["vue-jsx/volar"]
  }
}
```

The plugin runs on every `<script>` block that uses TSX, so both
`<script setup lang="tsx">` and `<script lang="tsx">` are covered. Command-line
type checking uses the `vue-tsc` you already have — the plugin runs in the editor
and under `vue-tsc` alike:

```json [package.json]
{
  "scripts": {
    "typecheck": "vue-tsc --noEmit"
  }
}
```

::: tip
Type checking an SFC project with `tsmc` doesn't work: TS Macro doesn't parse
`.vue` files. Keep `tsmc` for `.tsx` files and use `vue-tsc` for SFCs.
:::

## Options

Directive support is always on. `ref` and `macros` are the knobs, and both setups
take the same shape — as the factory argument in `ts-macro.config.ts`, or under
`vueCompilerOptions['vue-jsx']` (`vueCompilerOptions['vue-jsx-vapor']` for that
package) in `tsconfig.json`:

```json [tsconfig.json]
{
  "vueCompilerOptions": {
    "plugins": ["vue-jsx/volar"],
    "vue-jsx": {
      "macros": true
    }
  }
}
```

|    Option    |    Default     | Description                                                  |
| :----------: | :------------: | :----------------------------------------------------------- |
| `directives` | always enabled | Type support for `v-if`, `v-for`, `v-slot`, and `v-model`.   |
|    `ref`     |     `true`     | Set `false` to skip it, or an object to configure the alias. |
|   `macros`   |    `false`     | Set `true` or an options object to enable macro syntax.      |

::: warning
Enabling macros here only affects type checking. The transform that makes them
work at runtime is configured in the Vite plugin — keep the two in sync, as
described in [Macros](../features/macros).
:::

## Related Pages

- [Directives](../features/directives) — what each directive compiles to, and
  which ones the plugin types.
- [Macros](../features/macros) — enabling the macro transform in the Vite plugin.
- [Extending the JSX Types](./extending-jsx-types) — augment the JSX namespace
  that the plugin's generated code resolves against.
