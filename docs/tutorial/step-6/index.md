---
aside: false
prev: false
next: false
---

# List Rendering
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appVaporCode from './app-vapor.tsx?raw'
import appVaporSolvedCode from './app-vapor-solved.tsx?raw'
import appMacrosSolvedCode from './app-macros-solved.tsx?raw'
import appVaporMacrosSolvedCode from './app-vapor-macros-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  vapor: { 'src/App.tsx': appVaporCode },
  vaporSolved: { 'src/App.tsx': appVaporSolvedCode },
  macros: { 'src/App.tsx': appCode },
  macrosSolved: { 'src/App.tsx': appMacrosSolvedCode },
  vaporMacros: { 'src/App.tsx': appVaporCode },
  vaporMacrosSolved: { 'src/App.tsx': appVaporMacrosSolvedCode}
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-5/" next="/tutorial/step-7">

Vue JSX supports rendering lists with `map()`:

```tsx
<ul>
  {todos.map((todo) => {
    return <li key={todo.id}>{todo.text}</li>
  })}
</ul>
```

This familiar approach is suitable for most small or simple lists. For frequently updated lists or performance-sensitive scenarios, use the list component provided for each rendering mode.

## Virtual DOM

Use `For` with Virtual DOM:

```tsx
import { For } from 'vue-jsx'

<ul>
  <For in={todos}>
    {(todo) => <li key={todo.id}>{todo.text}</li>}
  </For>
</ul>
```

The slot receives the current item and index. Give the returned root node a stable `key`, just as you would when using `map()`.

## Vapor Mode

Use `VaporFor` in Vapor Mode:

```tsx
import { VaporFor } from 'vue-jsx'

<ul>
  <VaporFor in={todos}>
    {(todo, index) => (
      <li>
        {todo.text} at {index.value}
      </li>
    )}
  </VaporFor>
</ul>
```

`VaporFor` manages each item as an independent block. Its index is a shallow ref, so read it with `index.value` in JavaScript. By default, the item itself is used as its key; use `getKey={(todo) => todo.id}` when the list may replace items with new objects.

The current to-do list renders only its first item. Replace that temporary code with the appropriate list component and render every item.

</jsx-repl>
