# 互操作性

`vue-jsx-vapor` 支持 Virtual DOM 和 Vapor DOM 混合使用。将 `interop` 设置为 `true` 后，
在 `defineVaporComponent` 中定义的 JSX 会被编译为 Vapor DOM, \
在 `defineVaporComponent` 外定义的 JSX 会被编译为 Virtual DOM。

## 在 Virtual DOM 中使用 Vapor

[演练场](https://repl.zmjs.dev/vuejs/vapor-in-virtual-dom)

::: code-group

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    vueJsxVapor({
      macros: true,
      interop: true,
    }),
  ],
})
```

```ts [main.ts]
import { createApp, vaporInteropPlugin } from 'vue'
import App from './App.tsx'
createApp(App).use(vaporInteropPlugin).mount('#app')
```

```tsx [App.tsx] twoslash
import {
  computed,
  defineComponent,
  defineVaporComponent,
  ref,
} from 'vue'
import { useRef } from 'vue-jsx-vapor'

const Comp = defineVaporComponent(({ count = 0 }) => {
  defineExpose({
    double: computed(() => count * 2),
  })
  return <span> x 2 = </span>
})

export default defineComponent(() => {
  const count = ref(1)
  const compRef = useRef()
  return () => (
    <>
      <input v-model={count.value} />
      <Comp count={count.value} ref={compRef}></Comp>
      {compRef.value?.double}
    </>
  )
})
```

:::

## 在 Vapor 中使用 Virtual DOM

[演练场](https://repl.zmjs.dev/vuejs/virtual-dom-in-vapor)

::: code-group

```ts [vite.config.ts]
import { defineConfig } from 'vite'
import vueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    vueJsxVapor({
      macros: true,
      interop: true,
    }),
  ],
})
```

```ts [main.ts]
import { createVaporApp, vaporInteropPlugin } from 'vue'
import App from './App.tsx'
createVaporApp(App).use(vaporInteropPlugin).mount('#app')
```

```tsx [App.tsx] twoslash
import {
  computed,
  defineComponent,
  defineVaporComponent,
  ref,
} from 'vue'
import { useRef } from 'vue-jsx-vapor'

const Comp = defineVaporComponent(({ count = 0 }) => {
  defineExpose({
    double: computed(() => count * 2),
  })
  return <span> x 2 = </span>
})

export default defineComponent(() => {
  const count = ref(1)
  const compRef = useRef()
  return () => (
    <>
      <input v-model={count.value}/>
      <Comp count={count.value} ref={compRef}></Comp>
      {compRef.value?.double}
    </>
  )
})
```

:::
