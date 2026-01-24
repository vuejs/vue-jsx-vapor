# ESLint

An ESLint plugin for `vue-jsx-vapor` to automatically format code.

## Install

```sh
pnpm add @vue-jsx-vapor/eslint
```

## Setup

```ts 
// eslint.config.ts
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor()
]
```

## define-style

Use `prettier` to format styles in the defineStyle macro.

```ts twoslash
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor({
    rules: {
      'vue-jsx-vapor/define-style': [1, { tabWidth: 2 }]
    }
  })
]
```

## jsx-sort-props

This is a modified version of [@stylistic/jsx/jsx-sort-props](https://eslint.style/rules/jsx/jsx-sort-props), supporting custom reservedFirst and reservedLast options.

```ts twoslash
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor({
    rules: {
      'vue-jsx-vapor/jsx-sort-props': ['error', { 
        reservedFirst: ['v-if', 'v-for'], 
        reservedLast: ['v-slot'],
      }]
    }
  })
]
```

### `reservedFirst`

Defaults to `['v-if', 'v-else-if', 'v-else', 'v-for', 'key', 'ref', 'v-model']`

If given as an array, the array's values will override the default list of reserved props.
These props will respect the order specified in the array:

```jsx
// before
const Before = <App a v-for={i in list} v-if={list} b />

// after
const After = <App v-if={list} v-for={i in list} a b />
```

### `reservedLast`

Defaults to `['v-slot', 'v-slots', 'v-text', 'v-html']`

This can be an array option. These props must be listed after all other props.
These will respect the order specified in the array:

```jsx
// before
const Before = <App v-slot={{ foo }} onClick={onClick} />

// after
const After = <App onClick={onClick} v-slot={{ foo }} />
```
