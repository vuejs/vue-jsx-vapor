---
aside: false
prev: false
next: false
---

# Event Bindings
  
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

In JSX, event handlers are usually written as `on` followed by a capitalized letter. You can also use `v-on` directive to bind multiple event handlers without the `on` prefix.

```tsx
<>
  <div onClick={onClick} />
  {/* multiple bindings */}
  <form v-on={{ click: onClick, submit: onSubmit }}></form>
</>
```

We also support [event modifiers](https://vuejs.org/guide/essentials/event-handling.html#event-modifiers) that start with `_`:
```tsx
<form onSubmit_prevent>
  <input onKeyup_enter={submit} />
</form>
```

Now, try to add a event handler `onClick` binding to the `<h1>`, using the `onClick` variable as its value. and then click the `h1`.

</jsx-repl>
