---
aside: false
prev: false
next: false
---

# Conditional Rendering
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import appMacrosSolvedCode from './app-macros-solved.tsx?raw'
import appInteropMacrosSolvedCode from './app-interop-macros-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode },
  macros: { 'src/App.tsx': appCode },
  macrosSolved: { 'src/App.tsx': appMacrosSolvedCode },
  interopMacros: { 'src/App.tsx': appInteropCode },
  interopMacrosSolved: { 'src/App.tsx': appInteropMacrosSolvedCode}
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-4/" next="/tutorial/step-6/">

We can use ternaries `{ a ? b : c }` or boolean expressions `{ a && b }` to control rendering:

```jsx
<>
  { toggle ? <h1>Title</h1> : null }
  { toggle && <h1>Title</h1> }
</>
```

## `v-if` / `v-else-if` / `v-else` directives

We can also use the `v-if` directive to conditionally render an element:

```jsx
<>
  <h1 v-if={level === 1}>Title</h1>
  <h2 v-else-if={level === 2}>Sub Title</h2>
  <div v-else>Content</div>
</>
```

Currently, the demo is showing both `<h1>` elements at the same time, please ensure that only one `<h1>` is shown on the page.

</jsx-repl>
