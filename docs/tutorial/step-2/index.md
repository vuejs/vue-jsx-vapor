---
aside: false
prev: false
next: false
---

# Introduction JSX
  
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

<jsx-repl :files :apps prev="/tutorial/step-1" next="/tutorial/step-3">

JSX is the HTML-like syntax that allow you embed dynamic expressions using `{ }` to reference variables and functions.
In this example, we include the string `name` in our JSX using `{name}` inside a div, and we render a JSX element that was directly assigned to the `a` variable.

There are 3 main differences between JSX and HTML:

1. JSX requires all elements to be closed, so even HTML void elements like `input` and `br` must be self-closing (e.g. `<input />`, `<br />`).

2. JSX usually requires a single root Element. To represent multiple top-level elements, wrap them in a Fragment element (`<>...</>`), or use a Fragment component with props (`<Fragment key={key}>...</Fragment>`).

```jsx
<>
  <h1>Title</h1>
  <h2>Sub Title</h2>
</>
```

3. JSX doesn't support HTML Comments `<!--...-->` or special tags like `<!DOCTYPE>`. Use JSX comments `{/*...*/}` instead, and they won't render to HTML.

Now, try to the use `name` variable in JSX, and update the variable `a` to `<a href="#">link</a>`.

</jsx-repl>
