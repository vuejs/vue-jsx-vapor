---
aside: false
prev: false
next: false
---

# Components
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import childCode from './Child.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref({
  ...getDefaultFiles(),
  'src/Child.tsx': childCode
})
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode,  },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-5" next="/tutorial/step-7">

In this example, let's add out `Child` component to our app. We've defined it in another file, though you can put multiple components in the same file. First we must import it:

```jsx
import Child from './Child'
```

Then, we can use the component in the JSX as:
```jsx
import Child from './Child'

export default () => {
  return (
    <>
      Parent
      <Child />
    </>
  )
}
```

Now try it yourself - import the `Child` component and render it in the JSX.

</jsx-repl>
