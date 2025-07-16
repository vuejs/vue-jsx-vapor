# 指令

用于 JSX 的 Vue 内置指令。

|           指令           |        Vue         |       Volar        |
| :---------------------------: | :----------------: | :----------------: |
| `v-if`, `v-else-if`, `v-else` | :white_check_mark: | :white_check_mark: |
|      `v-slot`, `v-slots`      | :white_check_mark: | :white_check_mark: |
|            `v-for`            | :white_check_mark: | :white_check_mark: |
|           `v-model`           | :white_check_mark: | :white_check_mark: |
|      `v-html`, `v-text`       | :white_check_mark: |         /          |
|           `v-once`            | :white_check_mark: |         /          |

## 动态参数

也可以在指令参数中使用变量。
因为 JSX 不支持 `[]` 关键字，所以使用 `$` 代替。

## 修饰符

修饰符是以 `_` 表示的特殊后缀，表示指令应以某种特殊方式绑定。
因为 JSX 不支持 `.` 关键字，所以用 `_` 代替。

```tsx
<form onSubmit_prevent>
  <input v-model_number={value} />
</form>
```

## `v-if`, `v-else-if`, `v-else`

```tsx twoslash
export default ({ foo = 0 }) => {
  // ---cut-start---
  // prettier-ignore
  // ---cut-end---
  return (
    <>
      <div v-if={foo === 0}>{foo}</div>

      <div v-else-if={foo === 1}>{foo}</div>
      //                          ^?

      <div v-else>{foo}</div>
      //           ^?
    </>
  )
}
```

## `v-for`

```tsx twoslash
export default () => (
  <div v-for={(item, index) in 4} key={index}>
    {item}
  </div>
)
```

## `v-slot`, `v-slots`

::: code-group

```tsx [v-slot] twoslash
const Comp = () => {
  defineSlots<{
    default: () => any
    slot: (scope: { bar: number }) => any
    slots: (scope: { baz: boolean }) => any
  }>()
  return <div />
}

// ---cut-start---
// prettier-ignore
// ---cut-end---
export default () => (
  <Comp>
    默认插槽
    <template v-slot:slot={{ bar }}>
      //              ^|
      {bar}
    </template>
  </Comp>
)
```

```tsx [v-slots] twoslash
const Comp = () => {
  defineSlots<{
    default: () => any
    slot: (scope: { bar: number }) => any
    slots: (scope: { baz: boolean }) => any
  }>()
  return <div />
}

export default () => (
  <Comp
    v-slots={{
      default: () => <>默认插槽</>,
      slot: ({ bar }) => <>{bar}</>,
    }}
  />
)
```

:::

## `v-model`

```tsx twoslash
import { ref } from 'vue'

const Comp = () => {
  const model = defineModel<string>('model')
  const models = defineModel<string[]>('models')
  return <div />
}

export default () => {
  const foo = ref('')
  const name = ref('model')
  return (
    <Comp
      v-model:$name_value$={foo.value}
      v-model:model={foo.value}
      //       ^|
    />
  )
}
```
