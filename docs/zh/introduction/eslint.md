# ESLint

用于给 `vue-jsx-vapor` 自动格式化代码的 ESLint 插件。

## 安装

```sh
pnpm add @vue-jsx-vapor/eslint
```

## 配置

```ts
// eslint.config.ts
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor()
]
```

## define-style

使用 `prettier` 来格式化 `defineStyle` 宏中的样式。

```ts twoslash
import vueJsxVapor from '@vue-jsx-vapor/eslint'

export default [
  vueJsxVapor({
    rules: {
      'vue-jsx-vapor/define-style': ['error', { tabWidth: 2 }]
    }
  })
]
```

## jsx-sort-props

这是 [@stylistic/jsx/jsx-sort-props](https://eslint.style/rules/jsx/jsx-sort-props) 的修改版，支持自定义 `reservedFirst` 和 `reservedLast` 选项。

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

默认为 `['v-if', 'v-else-if', 'v-else', 'v-for', 'key', 'ref', 'v-model']`

如果提供一个数组，数组中的值将覆盖默认的保留 props 列表。
这些 props 将遵循数组中指定的顺序：

```jsx
// 转换前
const Before = <App a v-for={i in list} v-if={list} b />

// 转换后
const After = <App v-if={list} v-for={i in list} a b />
```

### `reservedLast`

默认为 `['v-slot', 'v-slots', 'v-text', 'v-html']`

这是一个数组选项。这些 props 必须在所有其他 props 之后列出。
它们将遵循数组中指定的顺序：

```jsx
// 转换前
const Before = <App v-slot={{ foo }} onClick={onClick} />

// 转换后
const After = <App onClick={onClick} v-slot={{ foo }} />
```
