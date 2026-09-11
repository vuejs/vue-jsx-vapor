# Vapor Mode: Real Local Updates And Real Functional Programming

Vapor Mode is where Vue JSX stops building a Virtual DOM tree on the hot path.
The compiler turns JSX into a DOM template plus a set of small reactive
operations. The component still reads like a function — but it runs exactly
once. After that, every state change lands in one targeted DOM operation.

The result is not "a faster VDOM". There is no VNode, no diff, no re-render of
the component. Static DOM is created once from a template string, and each
dynamic binding becomes its own effect that updates one position in the DOM.

[简体中文](/zh/blog/vapor)

<script setup>
import basicsCode from './examples/vapor-basics.tsx?raw'
</script>

## The Component Runs Once

The REPL below is the same kind of ordinary JSX as in the
[Virtual DOM post](/blog/vdom): a dynamic class, dynamic text, a conditional
branch, two event handlers. The compiled output is already open for you (the `js` tab): the
source on top, the Vue JSX Vapor compilation result below.

<BlogRepl :app="basicsCode" auto-select-output vapor />

The output is small enough to read line by line. Note the two import sources:
`template`, `txt`, `on`, `renderEffect`, `setClassName`, `createIf`, and
`setInsertionState` come from Vue's Vapor runtime; `setNodes` comes from the
vue-jsx helper module.

- The entire static structure is one HTML string:

  ```js
  const _t2 = _template(
    '<section class=demo><p> </p><!><button>increment</button><button>toggle',
    1,
  )
  ```

  Static props like `class=demo` are baked into the string. The space inside
  `<p>` is the anchor reserved for dynamic text, and `<!>` is the anchor for
  the conditional branch. The trailing `1` is a flag marking a component root.

- The two branches of the ternary are standalone templates of their own:

  ```js
  const _t0 = _template('<p>on', 2)
  const _t1 = _template('<p>off', 2)
  ```

- Cloning the template creates all the DOM at once, then the code walks to the
  dynamic nodes with direct paths:

  ```js
  const _n9 = _t2()
  const _n0 = _child(_n9)
  const _n8 = _next(_n0)
  const _n6 = _next(_n8)
  const _n7 = _next(_n6)
  ```

  `_child` takes the first child; `_next` steps to the next sibling. Adjacent
  nodes reuse the cursor instead of querying the parent by index again.

- Dynamic text is a static prefix plus a getter, attached to the anchor:

  ```js
  const _x0 = _txt(_n0)
  _setNodes(_x0, 'count: ', () => count.value)
  ```

  The prefix is resolved once. The getter runs inside a render effect, so when
  `count` changes, only this text position updates.

- Events are bound once:

  ```js
  _on(_n6, 'click', () => count.value++)
  _on(_n7, 'click', () => (ok.value = !ok.value))
  ```

  There is no second render, so the handler-caching question from the Virtual
  DOM post simply does not exist here.

- The dynamic class is one effect around one setter:

  ```js
  _renderEffect(() => _setClassName(_n0, ok.value ? 1 : 0, 'active'))
  ```

  When `ok` changes, the effect re-runs and flips the `active` class bit on the
  `<p>`. No props diff, no class normalization at update time.

- The conditional branch is a single `_createIf` call:

  ```js
  _setInsertionState(_n9, _n8)
  const _n1 = _createIf(
    () => ok.value,
    () => {
      const _n3 = _t0()
      return _n3
    },
    () => {
      const _n5 = _t1()
      return _n5
    },
  )
  ```

  The `on` branch renders first; when `ok` flips, the runtime swaps the whole
  branch DOM at the `<!>` anchor for the other one.

The important observation: the component function never appears a second time.
Updates are not "render again and compare" — they are individual effects
re-running their setters.

Edit the source in the REPL — make the class a static string, or make a
button label read a ref — and watch how the template string and the effect
list change.

## Where Updates Happen

Every dynamic binding compiles to an effect around a targeted operation. But
the direction of that operation differs between elements and components.

On elements, the compiler emits setters. A dynamic class becomes
`_renderEffect(() => _setClassName(...))`, and a dynamic prop becomes
`_renderEffect(() => _setProp(_n0, 'id', id.value))`. Your component's
effect runs the setter — it pushes the value into the DOM.

Component props are the opposite: they become getters.

```js
const _n0 = _createComponent(Comp, {
  prop: () => ok.value,
  static: 'x',
})
```

Dynamic prop values are passed as getter functions; static values pass through
as-is. Evaluation is deferred into the child component: the child's own
effects call the getters and track the dependencies themselves, so the child
updates itself. The parent does not push updates — the child pulls.

## Why Call It Functional Programming

Because a component is just a plain function: props go in, UI comes out. No
`this`, no instance, and no need to care when the framework might call you
again — it runs exactly once, at mount.

So who handles updates? The effects you already met in the compiled output:
`_setNodes` owns the text, `_renderEffect` owns the dynamic class, `_createIf`
owns the branch — each one guards a tiny piece of the DOM. When state changes,
the matching effect re-runs on its own; your function never knows.

The comparison makes it obvious: in the Virtual DOM Options API, every
state change re-runs the whole render function, rebuilding the virtual tree
and diffing it. In Vapor you write it once and it runs once — not a single
line of your code says "mutate the DOM"; the compiler wrote all of those.

## The Practical Payoff

Vapor mode is ideal for interfaces with frequent localized updates: counters,
forms, dashboards, editable rows, live data, and animation-adjacent UI. The
static DOM is created once, and each reactive read updates only the place
where it is used.

Virtual DOM mode remains the compatibility default. Vapor is opt-in: set
`vapor: true` in the plugin options, name a file `*.vapor.tsx` / `*.vapor.jsx`,
or wrap a component in `defineVaporComponent` / `defineVaporCustomElement`.
That makes it possible to adopt real local updates per file or per component,
incrementally.
