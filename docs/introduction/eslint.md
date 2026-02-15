# ESLint

An ESLint plugin for `vue-jsx-vapor` that provides automatic code formatting for directives and macros.

## Installation

```sh
pnpm add @vue-jsx-vapor/eslint
```

## Configuration

```ts 
// eslint.config.ts
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor()
]
```

## Rules

### define-style

Formats styles within the `defineStyle` macro using Prettier.

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

### jsx-sort-props

A modified version of [@stylistic/jsx/jsx-sort-props](https://eslint.style/rules/jsx/jsx-sort-props) with support for custom `reservedFirst` and `reservedLast` options.

```ts twoslash
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor({
    rules: {
      'vue-jsx-vapor/jsx-sort-props': [2, { 
        reservedFirst: ['v-if', 'v-for'], 
        reservedLast: ['v-slot'],
      }]
    }
  })
]
```

#### `reservedFirst`

**Default:** `['v-if', 'v-else-if', 'v-else', 'v-for', 'key', 'ref', 'v-model']`

When provided as an array, these values override the default list of reserved props. Props listed here will appear first, respecting the specified order:

```jsx
// Before
const Before = <App a v-for={i in list} v-if={list} b />

// After
const After = <App v-if={list} v-for={i in list} a b />
```

#### `reservedLast`

**Default:** `['v-slot', 'v-slots', 'v-text', 'v-html']`

When provided as an array, these props will be placed after all other props, respecting the specified order:

```jsx
// Before
const Before = <App v-slot={{ foo }} onClick={onClick} />

// After
const After = <App onClick={onClick} v-slot={{ foo }} />
```
