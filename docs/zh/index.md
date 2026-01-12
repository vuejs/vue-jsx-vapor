---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Vue JSX Vapor"
  text: "类型安全、高性能、更舒适的开发体验"
  tagline: Vue JSX 的 Vapor 模式
  image:
    src: /logo.svg
    alt: Vue JSX Vapor
  actions:
    - theme: brand
      text: 快速上手
      link: /zh/introduction/getting-started
    - theme: alt
      text: 功能特性
      link: /zh/features/directives

features:
  - icon: ⚒️ ️
    title: 指令
    details: 支持 Vue 的所有内置指令。
  - icon: ✨
    title: 宏
    details: 支持 Vue 的大部分宏，对 JSX 友好。
  - icon: 🦾
    title: 类型安全
    details: 通过安装 TS Macro (VSCode 插件) 提供 Volar 插件支持。
  - icon: ⚡️
    title: 高性能
    details: 拥有与 Vue Vapor 同等的性能！
  - icon: 🦀
    title: Rust 编译器
    details: 基于 Oxc，相比于 Babel 插件，虚拟DOM 编译速度提升了~30倍，Vapor 编译速度提升了~50倍。
  - icon: ⚙️
    title: ESLint
    details: 提供 ESLint 插件为 vue-jsx-vapor 自动格式化代码。
---

## Compiler 基准测试

<script setup>
import PerformanceChart from '../.vitepress/theme/components/PerformanceChart.vue'
</script>

<ClientOnly>
  <PerformanceChart title="每秒运行次数" />
</ClientOnly>
