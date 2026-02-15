---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Vue JSX Vapor"
  text: "Type-Safe, Enhanced DX, High Performance"
  tagline: Vue JSX with Vapor Mode Support
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
    details: Full support for all Vue built-in directives in JSX syntax.
  - icon: ✨
    title: Macros
    details: Support most macros of Vue, optimized for JSX.
  - icon: 🦾
    title: Type Safe
    details: Provide Volar plugin support by installing TS Macro (VSCode plugin).
  - icon: ⚡️
    title: High Performance
    details: It has the same performance as Vue Vapor!
  - icon: 🦀
    title: Rust Compiler
    details: Powered by Oxc, ~30× faster (Virtual DOM) and ~50× faster (Vapor) than Babel.
  - icon: ⚙️
    title: ESLint Integration
    details: Includes an ESLint plugin for automatic formatting of directives and macros.
---

## Compiler Benchmark
<script setup>
import PerformanceChart from './.vitepress/theme/components/PerformanceChart.vue'
</script>

<ClientOnly>
  <PerformanceChart />
</ClientOnly>
