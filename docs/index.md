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
      text: Features
      link: /features/directives

features:
  - icon: ⚒️ ️
    title: Directives
    details: Support all build-in directives of Vue.
  - icon: ✨
    title: Macros
    details: Support most macros of Vue, Friendly to JSX.
  - icon: 🦾
    title: Type Safe
    details: Provide Volar plugin support by install TS Macro (VSCode plugin).
  - icon: ⚡️
    title: High Performance
    details: It has the same performance as Vue Vapor!
  - icon: 🦀
    title: Compiler rewritten in Rust
    details: Support generating Virtual DOM and Vapor. Powered by Oxc, 20x performance improvement over Babel plugin.
  - icon: ⚙️
    title: ESLint
    details: Provide an ESLint plugin for vue-jsx-vapor to automatically format directives and macros.
---
