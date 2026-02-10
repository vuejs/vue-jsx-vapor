---
aside: false
prev: false
next: false
---

# Props Bindings
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import cssCode from './main.css?raw'
import files from '../template'

const src = {
  ...files,
  'src/App.tsx': appCode,
  'src/main.css': cssCode,
}
const solvedSrc = {
  ...files,
  'src/App.tsx': appSolvedCode,
  'src/main.css': cssCode,
}
</script>

<jsx-repl :src :solved-src next="/tutorial/step-4">

We use `{ }` to binding props:
```jsx
<div id={id} />
````

## Class Binding
Now, try to add a dynamic `class` binding to the `<h1>`, using the `titleStyle` variable as its value. if it's bound correctly, the text should turn red.

## Event Binding
In JSX, event handlers are usually written as `on` followed by a capitalized letter. 
`vue-jsx-vapor` also supports [event modifiers](https://vuejs.org/guide/essentials/event-handling.html#event-modifiers) that start with `_` :
```tsx
<form onSubmit_prevent={submit}>
  <input onKeyup_enter={submit} />
</form>
```

Now, try to add a event handler `onClick` binding to the `<h1>`, using the `onClick` variable as its value. and then click the `h1`.

</jsx-repl>
