# 宏

一系列的宏。需要通过将 `macros` 设置为 `true` 来手动启用。

|     宏     |        Vue         |       Volar        |
| :---------------: | :----------------: | :----------------: |
| `defineComponent` | :white_check_mark: | :white_check_mark: |
|   `defineModel`   | :white_check_mark: | :white_check_mark: |
|   `defineSlots`   | :white_check_mark: | :white_check_mark: |
|  `defineExpose`   | :white_check_mark: | :white_check_mark: |
|   `defineStyle`   | :white_check_mark: | :white_check_mark: |

## 配置

::: code-group

```ts {7} [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ],
})
```

```ts {6} [ts-macro.config.ts]
import vueJsxVapor from 'vue-jsx-vapor/volar'

export default {
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ],
}
```

:::

## defineComponent

- 支持 `await` 关键字。
- 自动收集使用到的 props 到 `defineComponent` 的 `props` 选项中。

```tsx twoslash
import { defineComponent, nextTick, Suspense, useAttrs } from 'vue'

const Comp = defineComponent(
  async (props: {
    foo?: string
    bar?: string
    // ^ 未使用的 prop 将作为 fallthrough attribute。
  }) => {
    await nextTick()
    const attrs = useAttrs()
    return (
      <div>
        <span {...attrs}>{props.foo}</span>
      </div>
    )
  },
)

export default () => (
  <Suspense>
    <Comp foo="foo" bar="bar" />
  </Suspense>
)
```

::: details 编译后代码

```tsx
import { defineComponent, useAttrs, withAsyncContext } from 'vue'
defineComponent(
  async (props) => {
    let __temp, __restore
    ;([__temp, __restore] = withAsyncContext(() => nextTick())),
      await __temp,
      __restore()
    const attrs = useAttrs()
    return () => (
      <div>
        <span {...attrs}>{props.foo}</span>
      </div>
    )
  },
  { props: { foo: null } },
)
```

:::

- 解构的 props 将被自动重构。
- 如果 prop 的默认值以 `!` 结尾，则该 prop 将被推断为必需的。
- 如果定义了 rest prop，它将被转换为 `useAttrs()`，并且 `inheritAttrs` 选项将默认为 `false`。

```tsx twoslash
// @errors: 2322
import { defineComponent } from 'vue'

const Comp = defineComponent(
  <T,>({ foo = undefined as T, bar = ''!, ...attrs }) => {
    return (
      <div>
        <span {...attrs}>{foo}</span>
      </div>
    )
  },
)

export default () => <Comp<string> foo={1} bar="bar" />
```

::: details 编译后代码

```tsx
import { defineComponent } from 'vue'
import { createPropsDefaultProxy } from '/vue-macros/jsx-macros/with-defaults'
defineComponent(
  (_props) => {
    const props = createPropsDefaultProxy(_props, { bar: '' })
    const attrs = useAttrs()
    return () => (
      <div>
        <span {...attrs}>{props.foo}</span>
      </div>
    )
  },
  { props: { foo: null, bar: { required: true } }, inheritAttrs: false },
)
```

:::

## defineModel

- 不支持带连字符的 model 名称。
- 当表达式以 `!` 结尾时，将被推断为必需的 prop。
- 修改后的 model 值可以同步读取，无需 `await nextTick()`。[相关 issue](https://github.com/vuejs/core/issues/11080)

```tsx twoslash
import { ref } from 'vue'

function Comp() {
  const modelValue = defineModel<string>()!
  modelValue.value = 'foo'
  return <div>{modelValue.value}</div>
  //                      ^?
}

export default () => {
  const foo = ref('')
  return <input value={foo.value} />
}
```

::: details 编译后代码

```tsx
import { ref } from 'vue'
import { useModel } from '/vue-macros/jsx-macros/use-model'

function Comp(_props: {
  modelValue: string
  'onUpdate:modelValue': (value: string) => any
}) {
  const modelValue = useModel<string>(_props, 'modelValue', { required: true })
  modelValue.value = 'foo'
  return <div>{modelValue.value}</div>
}
```

:::

## defineSlots

- 如果使用泛型定义插槽，所有插槽都将是可选的。

```tsx twoslash
const slots = defineSlots<{
  default: () => any
}>()

slots.default?.()
//           ^ optional
```

- 支持默认插槽（推荐）。

```tsx twoslash
function Comp<const T>() {
  const slots = defineSlots({
    title: (props: { bar?: T }) => <div>title slot: {props.bar}</div>,
    default: (props: { foo: number }) => <div>default slot: {props.foo}</div>,
  })

  return (
    <>
      <slots.title />
      <slots.default foo={1} />
    </>
  )
}

// ---cut-start---
// prettier-ignore
// ---cut-end---
export default () => (
  <Comp<1>>
    <template v-slot={{ foo }}>{foo}</template>
    <template v-slot:title={{ bar }}>{bar}</template>
    //                        ^?
  </Comp>
)
```

## defineExpose

与在 Vue SFC 中一样。

```tsx twoslash
import { useRef } from 'vue-jsx-vapor'

const Comp = <T,>({ foo = undefined as T }) => {
  defineExpose({
    foo,
  })
  return <div />
}

export default () => {
  const compRef = useRef()
  compRef.value?.foo
  //             ^?


  return <Comp ref={compRef} foo={1 as const} />
}
```

::: details 编译后代码

```tsx
import { currentInstance } from 'vue'
import { useRef } from 'vue-jsx-vapor'
import { useExpose } from '/vue-macros/jsx-macros/use-expose'

const Comp = ({ foo }) => {
  currentInstance.exposed = {
    foo,
  }
  return <div />
}
```

:::

## defineStyle

```ts
declare function defineStyle(
  style: string,
  options?: { scoped?: boolean },
): void
```

- 支持 CSS 变量和 JS 变量绑定。
- 支持在文件中定义多个样式宏。
- 支持 CSS 预处理器：`css`、`scss`、`sass`、`less`、`stylus`、`postcss`。

```ts
defineStyle.scss(`...`)
defineStyle.stylus(`...`)
// ...
```

- 支持作用域模式。
  - 如果在文件顶层定义，`scoped` 选项默认为 `false`。
  - 如果在函数内部定义，`scoped` 选项默认为 `true`。

```tsx twoslash
function Comp({ color = 'red' }) {
  defineStyle.scss(`
    .foo {
      color: ${color};

      :deep(.bar) {
        color: blue;
      }
    }
  `)
  return <div color="red" class="foo bar">foo</div>
}

defineStyle(`
  .bar {
    background: black;
  }
`)
```

- 支持 `css modules`，如果宏是赋值表达式。

```tsx twoslash
export default () => {
  const styles = defineStyle.scss(`
    .foo {
      color: blue;
      .bar {
        background: red;
      }
    }
  `)

  return <div class={styles.bar} />
  //                         ^?
}
```
