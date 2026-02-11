---
aside: false
prev: false
next: false
---

# Getting Started
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps next="/tutorial/step-2">

Welcome to the Vue JSX Vapor tutorial!

The goal of this tutorial is to quickly give you an experience of what it feels like to work with Vue JSX Vapor, right in the browser.

## What is Vue JSX Vapor?
Vue JSX Vapor is a `Vue JSX Compiler` inspired by `Vue Compiler`, written in Rust 🦀, and powered by Oxc. It supports generating Virtual DOM and Vapor Mode.

## How to Use This Tutorial
You can edit the code below and see the result update instantly. Each step will introduce a core feature of Vue, and you will be expected to complete the code to get the demo working. If you get stuck, you will have a "Solve" button that reveals the working code for you.

</jsx-repl>
