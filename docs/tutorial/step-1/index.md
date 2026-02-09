---
aside: false
prev: false
next: false
---

# Getting Started
  
<script setup>
import { serialize } from 'jsx-repl'
import appCode from './app.tsx?raw'
import appResolvedCode from './app-resolved.tsx?raw'
import files from '../template'

const src = serialize({
  ...files,
  'src/App.tsx': appCode,
})
const resolvedSrc = serialize({
  ...files,
  'src/App.tsx': appResolvedCode,
})
</script>

<jsx-repl :src :resolved-src next="/tutorial/step-2">

Welcome to the Vue JSX Vapor tutorial!

The goal of this tutorial is to quickly give you an experience of what it feels like to work with Vue JSX Vapor, right in the browser.

## What is Vue JSX Vapor?
Vue JSX Vapor is a `Vue JSX Compiler` inspired by `Vue Compiler`, written in Rust 🦀, and powered by Oxc. It supports generating Virtual DOM and Vapor Mode.

## How to Use This Tutorial
You can edit the code below and see the result update instantly. Each step will introduce a core feature of Vue, and you will be expected to complete the code to get the demo working. If you get stuck, you will have a "Resolve" button that reveals the working code for you.

</jsx-repl>
