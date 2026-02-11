---
aside: false
prev: false
next: false
---

# Props
  
<script setup>
import appCode from './app.tsx?raw'
import appSolvedCode from './app-solved.tsx?raw'
import appInteropCode from './app-interop.tsx?raw'
import appInteropSolvedCode from './app-interop-solved.tsx?raw'
import appMacrosCode from './app-macros.tsx?raw'
import appMacrosSolvedCode from './app-macros-solved.tsx?raw'
import appInteropMacrosCode from './app-interop-macros.tsx?raw'
import appInteropMacrosSolvedCode from './app-interop-macros-solved.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps = {
  app: { 'src/App.tsx': appCode },
  solved: { 'src/App.tsx': appSolvedCode },
  interop: { 'src/App.tsx': appInteropCode },
  interopSolved: { 'src/App.tsx': appInteropSolvedCode },
  macros: { 'src/App.tsx': appMacrosCode },
  macrosSolved: { 'src/App.tsx': appMacrosSolvedCode },
  interopMacros: { 'src/App.tsx': appInteropMacrosCode },
  interopMacrosSolved: { 'src/App.tsx': appInteropMacrosSolvedCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-5" next="/tutorial/step-7">

The props are defined in the first parameter of the functional component.

```jsx
const Comp = (props) => (
  <div>{props.foo}</div>
)
```

## Deconstruct Props

::: warning
The different from other JSX frameworks is that props lose reactivity when you deconstruct them:
:::

```jsx
const Comp = ({ foo }) => (
  <div>
    {foo} this will no longer update
  </div>
)
````

We have two Solutions:

1. Pass a reactive ref object as a prop directly:
```jsx
function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo} />
}
```

But this component can't be used in Vue templates, since templates automatically unwrap refs.

2. Enable the macros feature and wrap the component in a `defineVaporComponent` or `defineComponent` (for Virtual DOM). It will automatically convert the deconstruct props to a `__props`, and prefix each used prop with `__props.`.

```jsx
const Comp = defineVaporComponent(({ foo }) => {
  return <div>{foo}</div>
})
```
Will be converted to:
```jsx
const Comp = defineVaporComponent((__props) => {
  return <div>{__props.foo}</div>
})
```
Then the `foo` prop will regain reactivity.

Now try it yourself - make the `foo` prop reactivity.

</jsx-repl>
