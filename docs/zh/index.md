---
layout: home

hero:
  name: 'Vue JSX'
  text: '编译器驱动的高性能 JSX'
  tagline: 同时支持 Virtual DOM 与 Vapor Mode
  actions:
    - theme: brand
      text: 快速上手
      link: /zh/introduction/getting-started
    - theme: alt
      text: 互动教程
      link: /zh/tutorial/step-1

features:
  - icon: ⚡️
    title: 高性能
    details: 将 Vue 编译器的优化能力带到 JSX，生成高效的运行时代码。
  - icon: 💨
    title: Vapor 模式
    details: 将 JSX 直接编译为 Vapor DOM，实现细粒度响应式更新。
  - icon: 🦀
    title: Rust 编译器
    details: 基于 Oxc，相比 Babel，Virtual DOM 编译速度提升 30 倍，Vapor 模式提升 50 倍。
  - icon: 🦾
    title: 类型安全
    details: 原生支持 TypeScript 7.0，自动推断 JSX 组件的 props、ref 和 children 类型。
---

## 编译器基准测试

<script setup>
import PerformanceChart from '../.vitepress/theme/components/PerformanceChart.vue'
</script>

<ClientOnly>
  <PerformanceChart title="每秒运行次数" />
</ClientOnly>
