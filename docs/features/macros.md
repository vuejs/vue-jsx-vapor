# Macros

A collection of macros. They are need to muanually enabled by set `macros` to `true`.

|     Directive     |        Vue         |       Volar        |
| :---------------: | :----------------: | :----------------: |
| `defineComponent` | :white_check_mark: | :white_check_mark: |
|   `defineModel`   | :white_check_mark: | :white_check_mark: |
|   `defineSlots`   | :white_check_mark: | :white_check_mark: |
|  `defineExpose`   | :white_check_mark: | :white_check_mark: |
|   `defineStyle`   | :white_check_mark: | :white_check_mark: |

## Setup

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

- Support `await` keyword.
- Automatically collects used props to the defineComponent's props option.

```tsx twoslash
import { defineComponent, nextTick, Suspense, useAttrs } from 'vue'

const Comp = defineComponent(
  async (props: {
    foo?: string
    bar?: string
    // ^ unused prop will be as a fallthrough attribute.
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

::: details Compiled Code

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

- The destructured props will be automatically restructured.
- If the prop's default value ends with `!`, the prop will be inferred as required.
- If a rest prop is defined, it will be converted to `useAttrs()`, and the `inheritAttrs` option will default to `false`.

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

::: details Compiled Code

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

- Doesn't support hyphenated model names.
- Will be inferred as a required prop when the expression ends with `!`.
- The modified model's value can be read synchronously, without needing to `await nextTick()`. [Related issue](https://github.com/vuejs/core/issues/11080)

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

::: details Compiled Code

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

- If using generics to define slots, all slots will be optional.

```tsx twoslash
const slots = defineSlots<{
  default: () => any
}>()

slots.default?.()
//           ^ optional
```

- Support default slots (Recommended).

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

Just like in Vue SFC.

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

::: details Compiled Code

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

- Support CSS-variable and JS-variable binding.
- Support defining multiple style macros in a file.
- Support CSS pre-processors: `css`, `scss`, `sass`, `less`, `stylus`, `postcss`.

```ts
defineStyle.scss(`...`)
defineStyle.stylus(`...`)
// ...
```

- Support scoped mode.
  - If defined at the top level of the file, the scoped option defaults to `false`.
  - If defined within a function, the scoped option defaults to `true`.

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

- Support `css modules`, if the macro is an assignment expression.

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
