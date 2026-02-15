# Directives

Vue JSX Vapor provides full support for Vue's built-in directives within JSX syntax.

| Directive                     | Vue                | Volar              |
| :---------------------------: | :----------------: | :----------------: |
| `v-if`, `v-else-if`, `v-else` | :white_check_mark: | :white_check_mark: |
| `v-slot`, `v-slots`           | :white_check_mark: | :white_check_mark: |
| `v-for`                       | :white_check_mark: | :white_check_mark: |
| `v-model`                     | :white_check_mark: | :white_check_mark: |
| `v-html`, `v-text`            | :white_check_mark: |         /          |
| `v-once`                      | :white_check_mark: |         /          |

## Dynamic Arguments

Variables can be used as directive arguments. Since JSX does not support the `[]` syntax used in Vue templates, use `$` as a substitute.

## Modifiers

Modifiers are special postfixes denoted by `_` that indicate a directive should be bound in a particular way. Since JSX does not support the `.` character in attribute names, use `_` as a substitute.

```tsx
<form onSubmit_prevent>
  <input v-model_number={value} />
</form>
```

## `v-if`, `v-else-if`, `v-else`

Conditional rendering directives work seamlessly with proper type narrowing support.

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

List rendering directive for iterating over arrays or ranges.

```tsx twoslash
export default () => (
  <div v-for={(item, index) in 4} key={index}>
    {item}
  </div>
)
```

## `v-slot`, `v-slots`

> [!WARNING]
> Default parameter values in slot scope destructuring (e.g., `v-slot={({ foo = '' })}`) are not supported due to AST generation limitations.

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

Two-way binding directive with support for dynamic model names and modifiers.

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
