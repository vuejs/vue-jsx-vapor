# 指令

用于 JSX 的 Vue 内置指令。

## 类型支持

指令转换在编译阶段完成，不需要额外的编辑器插件。如果希望在编辑器或命令行类型检查中获得指令类型支持，请安装 [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro)，并启用 `vue-jsx/volar` 插件：

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx()],
}
```

命令行类型检查请安装 `@ts-macro/tsc`，并使用 `tsmc` 替代 `tsc`：

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

|             指令              |        Vue         |       Volar        |
| :---------------------------: | :----------------: | :----------------: |
| `v-if`, `v-else-if`, `v-else` | :white_check_mark: | :white_check_mark: |
|      `v-slot`, `v-slots`      | :white_check_mark: | :white_check_mark: |
|            `v-for`            | :white_check_mark: | :white_check_mark: |
|           `v-model`           | :white_check_mark: | :white_check_mark: |
|      `v-html`, `v-text`       | :white_check_mark: |         /          |
|           `v-once`            | :white_check_mark: |         /          |

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
> 由于无法为带有默认值的指令表达式（例如 `v-slot={({ foo = '' })}`）生成正确的 AST，因此不支持默认值。

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

## 修饰符

修饰符是以 `_` 表示的特殊后缀，表示指令应以某种特殊方式绑定。
因为 JSX 不支持 `.` 关键字，所以用 `_` 代替。

```tsx
<form onSubmit_prevent>
  <input v-model_number={value} />
</form>
```

## 动态参数

动态参数可以通过变量的形式传递给数组值的第二个参数，第三个参数为指令的修饰符。

### `v-model`

```tsx twoslash
import { ref } from 'vue'

const Comp = () => {
  const model = defineModel<string, 'm1' | 'm2'>('model')
  const models = defineModel<string[]>('models')
  return <div />
}

export default () => {
  const foo = ref('')
  const name = ref('model')
  return (
    <Comp
      v-model={[foo.value, name.value, ['m1', 'm2']]}
      v-model:model={foo.value}
      //       ^|
    />
  )
}
```

### `v-slot`

```tsx twoslash
const Comp = () => {
  const slots = defineSlots<{
    default: () => any
  }>()
  return <div />
}

export default (_, { slots }: { slots: { default: () => any } }) => (
  <Comp>
    <template v-for={(Slot, name) in slots} v-slot={[scope, name]}>
      <Slot {...scope} />
    </template>
  </Comp>
)
```
