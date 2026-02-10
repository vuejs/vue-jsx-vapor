---
aside: false
prev: false
next: false
---

# Conditional Rendering
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import files from '../template'

const src = {
  ...files,
  'src/App.tsx': appCode,
}
const solvedSrc = {
  ...files,
  'src/App.tsx': appSolvedCode,
}
</script>

<jsx-repl :src :solved-src next="/tutorial/step-5">

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
Currently, the demo is showing both `<h1>`s at the same time, and the button does nothing. Try to add `v-if` and `v-else` directives to them, and implement the `toggle()` method so that we can use the button to toggle between them.

</jsx-repl>
