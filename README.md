# vue-jsx-vapor

[![NPM version](https://img.shields.io/npm/v/vue-jsx-vapor?color=a1b858&label=)](https://www.npmjs.com/package/vue-jsx-vapor)

Vue JSX Vapor.

[Docs](https://jsx-vapor.netlify.app/) . [Playground](https://repl.zmjs.dev/vuejs/vue-jsx-vapor)

## Install

```bash
npm i vue-jsx-vapor
```

> [!CAUTION]
> ❌ The destructuring of props in a functional component will cause loss of reactivity.

```tsx
function Comp({ foo }) {
  return <div>{foo}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo.value} />
}
```

#### Two Solutions

1. ✅ Pass a ref variable as prop:

```tsx
function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo} />
}
```

2. ✅ Set the macros option to true, then use the `defineComponent` macro for wrapping.

Setup

```ts
// vite.config.ts
import vueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ]
})

```

Usage

```tsx
const Comp = defineComponent(({ foo }) => {
  return <>{foo}</>
})
// Will be convert to:
const Comp = defineComponent((_props) => {
  return <>{_props.foo}</>
}, { props: ['foo'] })

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo.value} />
}
```

## Typescript

Because of vue-jsx-vapor support all directives and most macros of Vue,\
so we need the VSCode plugin [ts-macro](https://github.com/ts-macro/ts-macro) to use the `vue-jsx-vapor/volar` plugin for Typescript support.\
It works similarly to [@vue/language-tools](https://github.com/vuejs/language-tools) but only used for `ts` or `tsx` files.

By default, after installing the `ts-macro` VSCode plugin, `ts-macro` will automatically load `vue-jsx-vapor/volar` by analyzing `vite.config.ts` and shared vueJsxVapor's options. so you don't need to config `tsm.config.ts`. But if you want, you can also configure it manually:

```ts
// tsm.config.ts
import vueJsxVapor from 'vue-jsx-vapor/volar'

export default {
  plugins: [
    vueJsxVapor({
      macros: true,
    }),
  ],
}

```


<details>
<summary>Vite</summary><br>

```ts
// vite.config.ts
import VueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [VueJsxVapor()],
})
```

Example: [`playground/`](./playground/)

<br></details>

<details>
<summary>Rollup</summary><br>

```ts
// rollup.config.js
import VueJsxVapor from 'vue-jsx-vapor/rollup'

export default {
  plugins: [VueJsxVapor()],
}
```

<br></details>

<details>
<summary>Webpack</summary><br>

```ts
// webpack.config.js
module.exports = {
  /* ... */
  plugins: [require('vue-jsx-vapor/webpack')()],
}
```

<br></details>

<details>
<summary>Nuxt</summary><br>

```ts
// nuxt.config.js
export default defineNuxtConfig({
  modules: ['vue-jsx-vapor/nuxt'],
})
```

> This module works for both Nuxt 2 and [Nuxt Vite](https://github.com/nuxt/vite)

<br></details>

<details>
<summary>Vue CLI</summary><br>

```ts
// vue.config.js
module.exports = {
  configureWebpack: {
    plugins: [require('vue-jsx-vapor/webpack')()],
  },
}
```

<br></details>

<details>
<summary>esbuild</summary><br>

```ts
// esbuild.config.js
import { build } from 'esbuild'
import VueJsxVapor from 'vue-jsx-vapor/esbuild'

build({
  plugins: [VueJsxVapor()],
})
```

<br></details>
