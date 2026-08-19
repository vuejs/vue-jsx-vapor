# vue-jsx

[![NPM version](https://img.shields.io/npm/v/vue-jsx?color=a1b858&label=)](https://www.npmjs.com/package/vue-jsx)

High-performance Vue JSX Compiler powered by Oxc.

## Features

- ⚡️ High Performance: The same compiler principles as Vue.
- 💨 Vapor Mode: The same compiler principles as Vue Vapor.
- 🦀 Rust Compiler: Powered by Oxc, 35× faster (Virtual DOM) and 50× faster (Vapor) than Babel.
- 🦾 Type Safe: Native type support for Typescript 7.0.
- ✨ Unplugin: Provide `vite`, `rollup`, `rolldown` `webpack`, `rspack`, `rsbuild` , `esbuild`, `bun` and more plugins.
- 📦 Custom Element: Support custom-element by default.

## Installation

```bash
npm i vue-jsx
```

## Usage

- [📜 Documentation](https://vuejsx.dev/)
- [🛰️ Playground](https://repl.vuejsx.dev)

<details>
<summary>Vite</summary><br>

```ts
// vite.config.ts
import VueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [VueJsx()],
})
```

Example: [`playground/`](./playground/)

<br></details>

<details>
<summary>Rollup</summary><br>

```ts
// rollup.config.js
import VueJsx from 'vue-jsx/rollup'

export default {
  plugins: [VueJsx()],
}
```

<br></details>

<details>
<summary>Webpack</summary><br>

```ts
// webpack.config.js
module.exports = {
  /* ... */
  plugins: [require('vue-jsx/webpack')()],
}
```

<br></details>

<details>
<summary>Nuxt</summary><br>

```ts
// nuxt.config.js
export default defineNuxtConfig({
  modules: ['vue-jsx/nuxt'],
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
    plugins: [require('vue-jsx/webpack')()],
  },
}
```

<br></details>

<details>
<summary>esbuild</summary><br>

```ts
// esbuild.config.js
import { build } from 'esbuild'
import VueJsx from 'vue-jsx/esbuild'

build({
  plugins: [VueJsx()],
})
```

<br></details>
