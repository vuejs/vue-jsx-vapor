# vue-jsx-vapor

[![NPM version](https://img.shields.io/npm/v/vue-jsx-vapor?color=a1b858&label=)](https://www.npmjs.com/package/vue-jsx-vapor)

Vapor Mode of Vue JSX.

## Features

- ⚡️ High Performance: It has the same performance as Vue Vapor!
- ⚒️ Directives: Support all build-in directives of Vue.
- ✨ Macros: Support most macros of Vue, Friendly to JSX.
- 🌈 Hot Module Replacement: Support functional components or defined by defineComponent.
- 🦾 Type Safe: Provide Volar plugin support by install TS Macro (VSCode plugin).
- ⚙️ ESLint: Provide an ESLint plugin for vue-jsx-vapor to automatically fix code.

## Installation

```bash
npm i vue-jsx-vapor
```

## Usage

- [📜 Documentation](https://jsx-vapor.netlify.app/)
- [🛰️ Playground](https://repl.zmjs.dev/vuejs/vue-jsx-vapor)

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
