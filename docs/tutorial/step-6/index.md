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

<jsx-repl :files :apps prev="/tutorial/step-4" next="/tutorial/step-6">

We can use `map(...)` to render a list if performance is not a concern.

```jsx
<ul>
  {todos.map((todo) => {
    return <li key={todo.id}>{todo.text}</li>
  })}
</ul>
```

## `v-for` directive

We can use also the `v-for` directive to render a list that has the same performance as Vue template:

```jsx
<ul>
  <li v-for={todo in todos} key={todo.id}>
    {todo.text}
  </li>
</ul>
```

Here todo is a local variable representing the array element currently being iterated on. It's only accessible on or inside the v-for element, similar to a function scope.

Notice how we are also giving each todo object a unique id, and binding it as the special key attribute for each `<li>`. The key allows Vue to accurately move each `<li>` to match the position of its corresponding object in the array.

There are two ways to update the list:
1. Call mutating methods on the source array:
```ts
todos.value.push(newTodo)
```
2. Replace the array with a new one:
```ts
todos.value = todos.value.filter(/* ... */)
```

Here we have a simple todo list - try to implement the logic for `addTodo()` and `removeTodo()` methods to make it work!

</jsx-repl>
