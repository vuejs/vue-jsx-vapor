# 宏

为 JSX 设计的一系列宏。宏默认关闭，需要通过将 `macros` 设置为 `true` 来手动启用。

## 配置

```ts {7} [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [
    vueJsx({
      macros: true,
    }),
  ],
})
```

`vue-jsx/volar` 插件是可选的。只有在需要宏语法的编辑器或类型检查支持时才需要启用它，并保持其选项与 Vite 配置一致。配置方式请参考 [Volar 插件](../typescript/volar)。

::: details 作为单独的插件安装

我们也发布了一个单独的插件，可以给 虚拟DOM 的项目使用。

```bash
pnpm add @vue-jsx/macros -D
```

配置:

```ts
// vite.config.ts
import jsxMacros from '@vue-jsx/macros/vite'

export default {
  plugins: [jsxMacros()],
}
```

:::

## defineComponent | defineVaporComponent

`defineComponent` 用于定义虚拟 DOM 组件，`defineVaporComponent` 用于定义 Vapor 组件

### 选项

```ts
vueJsx({
  defineComponent: {
    /**
     * @default ['defineComponent','defineVaporComponent']
     *
     * 可以通过把 alias 设为空数组来禁用 defineComponent 宏.
     */
    alias: [],
    /**
     * @default false
     *
     * 根据 setup 函数参数的类型生成 props 和 emits。
     */
    resolveType: true,
  },
})
```

- 支持 `await` 关键字。
- 自动收集使用到的 props 到 `defineVaporComponent` 的 `props` 选项中。

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
    return () => (
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
    ;(([__temp, __restore] = withAsyncContext(() => nextTick())), await __temp, __restore())
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
- 如果定义了 rest prop，它将被转换为 `useAttrs()`，生成的选项中会带上 `inheritAttrs: false`。

```tsx twoslash
// @errors: 2322
import { defineVaporComponent } from 'vue'

const Comp = defineVaporComponent(<T,>({ foo = undefined as T, bar = ''!, ...attrs }) => {
  return (
    <div>
      <span {...attrs}>{foo}</span>
    </div>
  )
})

export default () => <Comp<string> foo={1} bar="bar" />
```

::: details 编译后代码

```tsx
import { defineVaporComponent } from 'vue'
import { createPropsDefaultProxy } from '/vue-macros/jsx-macros/with-defaults'
defineVaporComponent(
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

### 基于类型的 Props 和 Emits

开启 `resolveType: true` 后，会根据 setup 函数第一个参数的类型生成 `props` 选项，根据第二个参数上的 `SetupContext<E>` 或 `{ emit: EmitFn<E> }` 生成 `emits`，使用与 `<script setup>` 中 `defineProps<T>()` 和 `defineEmits<T>()` 相同的类型解析。类型中的每个键都会成为 prop，包括函数中没有使用到的；导入的类型也会被解析。

与 `@vitejs/plugin-vue-jsx` 的 `resolveType` 一致：类型参数（`defineComponent<Props, Emits>(...)`）优先于参数上的类型注解；props 参数的默认值（`(props: Props = defaults) => ...`）会作为 props 的默认值；`const Foo = defineComponent(...)` 会被命名为 `Foo`，除非选项中已经设置了 name。

```tsx twoslash
import { defineComponent, nextTick, Suspense, type HTMLAttributes, type SetupContext } from 'vue'

const Comp = defineComponent(
  async (
    props: { foo?: string } & /* @vue-ignore */ HTMLAttributes,
    // ^ `@vue-ignore` 会让该类型作为 fallthrough attribute，与 `<script setup>` 一致。
    { attrs, emit }: SetupContext<{ change: [value: string] }>,
  ) => {
    await nextTick()
    return () => (
      <span {...attrs} onClick={() => emit('change', 'foo')}>
        {props.foo}
      </span>
    )
  },
)

export default () => (
  <Suspense>
    <Comp foo="foo" title="title" />
  </Suspense>
)
```

::: details 编译后代码

```tsx
import { defineComponent, withAsyncContext } from 'vue'
defineComponent(
  async (props, { attrs, emit }) => {
    let __temp, __restore
    ;(([__temp, __restore] = withAsyncContext(() => nextTick())), await __temp, __restore())
    return () => (
      <span {...attrs} onClick={() => emit('change', 'foo')}>
        {props.foo}
      </span>
    )
  },
  { props: { foo: { type: String, required: false } }, emits: ['change'] },
)
```

:::

此时 props 参数中的 rest 元素收集未被解构的 props，与 `<script setup>` 中的 props 解构一致，而不是 fallthrough attribute。fallthrough attribute 可以通过 setup 上下文的 `attrs` 获取。

```tsx twoslash
import { defineVaporComponent } from 'vue'

const Comp = defineVaporComponent(
  ({ foo = 'foo', bar = ''!, ...rest }: { foo?: string; bar: string; baz?: string }) => {
    return (
      <div>
        <span {...rest}>{foo}</span>
      </div>
    )
  },
)

export default () => <Comp bar="bar" baz="baz" />
```

::: details 编译后代码

```tsx
import { createPropsRestProxy, defineVaporComponent } from 'vue'
defineVaporComponent(
  (__props) => {
    const rest = createPropsRestProxy(__props, ['foo', 'bar'])
    return (
      <div>
        <span {...rest}>{__props.foo}</span>
      </div>
    )
  },
  {
    props: {
      foo: { type: String, required: false, default: 'foo' },
      bar: { type: String, required: true, default: '' },
      baz: { type: String, required: false },
    },
  },
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

function Comp(_props: { modelValue: string; 'onUpdate:modelValue': (value: string) => any }) {
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
import { useRef } from 'vue-jsx'

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
import { useRef } from 'vue-jsx'
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
declare function defineStyle(style: string, options?: { scoped?: boolean }): void
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
  return (
    <div color="red" class="foo bar">
      foo
    </div>
  )
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
