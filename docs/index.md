---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Vue JSX Vapor"
  text: "Type-safe, Improve DX, High Performance"
  tagline: Vapor Mode of Vue JSX
  image:
    src: /logo.svg
    alt: Vue JSX Vapor
  actions:
    - theme: brand
      text: Get Started
      link: /introduction/getting-started
    - theme: alt
      text: Tutorial
      link: /tutorial/step-1

features:
  - icon: ⚒️ ️
    title: Directives
    details: Support all built-in directives of Vue.
  - icon: ✨
    title: Macros
    details: Support most macros of Vue, Friendly to JSX.
  - icon: 🦾
    title: Type Safe
    details: Provide Volar plugin support by installing TS Macro (VSCode plugin).
  - icon: ⚡️
    title: High Performance
    details: It has the same performance as Vue Vapor!
  - icon: 🦀
    title: Compiler rewritten in Rust
    details: Powered by Oxc, ~30x (Virtual DOM) and ~50x (Vapor) performance improvement over Babel.
  - icon: ⚙️
    title: ESLint
    details: Provide an ESLint plugin for vue-jsx-vapor to automatically format directives and macros.
---

## Compiler Benchmark
<script setup>
import PerformanceChart from './.vitepress/theme/components/PerformanceChart.vue'
</script>

<ClientOnly>
  <PerformanceChart />
</ClientOnly>
