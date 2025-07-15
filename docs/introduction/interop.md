# Interop

`vue-jsx-vapor` support virtual-dom and vapor-dom co-usage. After set interop to `true`, vue-jsx-vapor will be only convert in `defineVaporComponent`'s JSX.

## Vapor in Virtual DOM

[Playground](https://repl.zmjs.dev/vuejs/vapor-in-virtual-dom)

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

## Virtual DOM in Vapor

[Playground](https://repl.zmjs.dev/vuejs/virtual-dom-in-vapor)

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
