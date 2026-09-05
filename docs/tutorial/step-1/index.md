---
aside: false
prev: false
next: false
---

# Getting Started
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appVaporCode from './app-vapor.tsx?raw'
import appVaporSolvedCode from './app-vapor-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  vapor: { 'src/App.tsx': appVaporCode },
  vaporSolved: { 'src/App.tsx': appVaporSolvedCode }
}
</script>

<jsx-repl :files :apps next="/tutorial/step-2/">

Welcome to the Vue JSX tutorial!

The goal of this tutorial is to quickly give you an experience of what it feels like to work with Vue JSX, right in the browser.

## What is Vue JSX?
Vue JSX is a Vue JSX compiler written in Rust and powered by Oxc. It generates Virtual DOM code by default; use the Vapor switch above to try the optional Vapor output.

## How to Use This Tutorial
You can edit the code below and see the result update instantly. Each step will introduce a core feature of Vue JSX, and you will be expected to complete the code to get the demo working. If you get stuck, you will have a "Solve" button that reveals the working code for you.

</jsx-repl>
