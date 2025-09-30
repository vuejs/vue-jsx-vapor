# Directives

Vue built-in directives for JSX.

|           Directive           |        Vue         |       Volar        |
| :---------------------------: | :----------------: | :----------------: |
| `v-if`, `v-else-if`, `v-else` | :white_check_mark: | :white_check_mark: |
|      `v-slot`, `v-slots`      | :white_check_mark: | :white_check_mark: |
|            `v-for`            | :white_check_mark: | :white_check_mark: |
|           `v-model`           | :white_check_mark: | :white_check_mark: |
|      `v-html`, `v-text`       | :white_check_mark: |         /          |
|           `v-once`            | :white_check_mark: |         /          |

## Dynamic Arguments

It is also possible to use a variable in a directive argument.
Because JSX doesn't support `[]` keyword, use `$` instead.

## Modifiers

Modifiers are special postfixes denoted by a `_`, which indicate that a directive should be bound in some special way.
Because JSX doesn't support `.` keyword, use `_` instead.

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

> [!WARNING]
> Due to the inability to generate a correct AST for directive expressions with default values (e.g., `v-slot={({ foo = '' })}`), so default values are not supported.

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
    default slot
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
      default: () => <>default slot</>,
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
