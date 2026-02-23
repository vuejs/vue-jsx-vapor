---
aside: false
prev: false
next: false
---

# List Rendering
  
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

<jsx-repl :files :apps prev="/tutorial/step-5/" next="/tutorial/step-7">

We can use `map(...)` to render a list if performance is not a concern.

```jsx
<ul>
  {todos.map((todo) => {
    return <li key={todo.id}>{todo.text}</li>
  })}
</ul>
```

## `v-for` directive

We can also use the `v-for` directive to render a list that has the same performance as Vue template:

```jsx
<ul>
  <li v-for={todo in todos} key={todo.id}>
    {todo.text}
  </li>
</ul>
```

Here, `todo` is a local variable representing the array element currently being iterated on. It's only accessible on or inside the v-for element, similar to a function scope.

Notice how we are also giving each todo object a unique id, and binding it as the special key attribute for each `<li>`. The key allows Vue to accurately move each `<li>` to match the position of its corresponding object in the array.

Currently, we have a simple to-do list that currently renders only one item. Try rendering all the to-do items so it works correctly!

</jsx-repl>
