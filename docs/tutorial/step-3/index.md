---
aside: false
prev: false
next: false
---

# Attribute Bindings
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import cssCode from './main.css?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref({ ...getDefaultFiles(), 'src/main.css': cssCode })
const apps  = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode }
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-2" next="/tutorial/step-4">

We use `{ }` to dynamically bind a prop, or use the spread operator `{...}` to bind multiple attributes:
```jsx
export default (props) => (
  <>
    <div id={props.id} />
    {/* multiple bindings */}
    <div {...props} />
  </>
)
````

## Style Bindings
We can use string, object or array expression to conditionally bind styles:
```tsx
export default (props: { hidden: boolean }) => (
  <>
    <h1 style={`display: ${ props.hidden ? 'none' : 'block' }`}>h1</h1>
    <h2 style={{ display: props.hidden ? 'none': undefined }}>h2</h2>
    <h3 style={[ props.hidden && 'display: none;' ]}>h3</h3>
  </>
)
```

## Class Bindings
We can use string, object or array expression to conditionally bind classes:

```tsx
export default (props: { hidden: boolean }) => (
  <>
    <h1 style={props.hidden && 'hidden'}>h1</h1>
    <h2 class={{ 'hidden': props.hidden }}>h2</h2>
    <h3 class={[ props.hidden && 'hidden' ]}>h3</h3>
  </>
)
```

Now, try to add a dynamic `class` binding to the `<h1>`, using the `titleClass` variable as its value. if it's bound correctly, the text should turn red.

</jsx-repl>
