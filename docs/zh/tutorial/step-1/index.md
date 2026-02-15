---
aside: false
prev: false
next: false
---

# 快速开始
  
<script setup>
import appCode from '~/tutorial/step-1/app.tsx?raw'
import appSolvedCode from '~/tutorial/step-1/app-solved.tsx?raw'
import appInteropCode from '~/tutorial/step-1/app-interop.tsx?raw'
import appInteropSolvedCode from '~/tutorial/step-1/app-interop-solved.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps next="/zh/tutorial/step-2/">

欢迎来到 Vue JSX Vapor 教程！

本教程的目标是让你在浏览器中快速体验使用 Vue JSX Vapor 的感觉。

## 什么是 Vue JSX Vapor？
Vue JSX Vapor 是一个受 `Vue Compiler` 启发的 `Vue JSX 编译器`，使用 Rust 🦀 编写，基于 Oxc 构建。它支持生成 Virtual DOM 和 Vapor Mode 两种模式。

## 如何使用本教程
你可以在下方编辑代码并立即看到结果更新。每个步骤都会介绍 Vue JSX 的一个核心特性，你需要完成代码以使示例正常工作。如果遇到困难，可以点击"解答"按钮查看正确的代码。

</jsx-repl>