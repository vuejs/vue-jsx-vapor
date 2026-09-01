---
layout: home

hero:
  name: 'Vue JSX'
  text: 'Compiler-powered, high-performance JSX'
  tagline: One compiler for Virtual DOM and Vapor Mode
  actions:
    - theme: brand
      text: Get Started
      link: /introduction/getting-started
    - theme: alt
      text: Tutorial
      link: /tutorial/step-1

features:
  - icon: ⚡️
    title: High Performance
    details: Brings Vue compiler optimizations to JSX for efficient runtime code.
  - icon: 💨
    title: Vapor Mode
    details: Compiles JSX directly to Vapor DOM for fine-grained reactive updates.
  - icon: 🦀
    title: Rust Compiler
    details: Powered by Oxc, 30× faster for Virtual DOM and 50× faster for Vapor than Babel.
  - icon: 🦾
    title: Type Safe
    details: Native TypeScript 7.0 support with automatic inference for JSX component props, refs, and children.
---

## Compiler Benchmark

<script setup>
import PerformanceChart from './.vitepress/theme/components/PerformanceChart.vue'
</script>

<ClientOnly>
  <PerformanceChart />
</ClientOnly>
