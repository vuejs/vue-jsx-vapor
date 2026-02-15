# Macros

A collection of compile-time macros for JSX. These macros must be explicitly enabled by setting the `macros` option to `true`.

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

::: details Install as a standalone plugin

A standalone plugin is also available for use in Virtual DOM projects.

```bash
pnpm add @vue-jsx-vapor/macros -D
```

Configuration:

```ts
// vite.config.ts
import jsxMacros from '@vue-jsx-vapor/macros/vite'

export default {
  plugins: [
    jsxMacros()
  ]
}
```

:::

## defineComponent | defineVaporComponent

`defineComponent` is used to define Virtual DOM components, while `defineVaporComponent` is used to define Vapor components.

### Options

```ts
VueJsxVapor({
  defineComponent: {
    /**
     * @default ['defineComponent','defineVaporComponent']
     *
     * Set alias to an empty array to disable the defineComponent macro.
     */
    alias: []
  }
})
```

### Features

- Supports the `await` keyword in async setup functions.
- Automatically collects referenced props and adds them to the component's `props` option.

```tsx twoslash
import { defineComponent, nextTick, Suspense, useAttrs } from 'vue'

const Comp = defineComponent(
  async (props: {
    foo?: string
    bar?: string
    // ^ Unreferenced props are treated as fallthrough attributes.
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

### Props Handling

- Destructured props are automatically restructured to preserve reactivity.
- Append `!` to a prop's default value to mark it as required.
- Rest parameters in props are converted to `useAttrs()`, and `inheritAttrs` defaults to `false`.

```tsx twoslash
// @errors: 2322
import { defineVaporComponent } from 'vue'

const Comp = defineVaporComponent(
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

## defineModel

### Limitations

- Hyphenated model names are not supported.

### Features

- Append `!` to mark the model as required.
- Model values can be read synchronously after modification, without awaiting `nextTick()`. [Related issue](https://github.com/vuejs/core/issues/11080)

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

### Generic Slots

When using generics to define slots, all slots are treated as optional.

```tsx twoslash
const slots = defineSlots<{
  default: () => any
}>()

slots.default?.()
//           ^ optional
```

### Default Slot Values (Recommended)

Providing default implementations for slots is the recommended approach.

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

Functions identically to `defineExpose` in Vue SFCs.

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

### Features

- Supports CSS variable and JavaScript variable binding.
- Multiple `defineStyle` calls can be used within a single file.
- Supports CSS preprocessors: `css`, `scss`, `sass`, `less`, `stylus`, `postcss`.

```ts
defineStyle.scss(`...`)
defineStyle.stylus(`...`)
// ...
```

### Scoped Styles

- Top-level definitions default to `scoped: false`.
- Definitions within functions default to `scoped: true`.

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

### CSS Modules

Assigning `defineStyle` to a variable enables CSS Modules support.

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
