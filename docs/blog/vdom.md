# Virtual DOM: Compiler-Powered JSX

Vue JSX 3.3 treats JSX as a compile target for Vue, not just as syntax sugar for
`h()` calls. That is the biggest difference from ordinary Babel-based Vue JSX:
the compiler understands Vue's update model and emits the same kinds of runtime
hints that make Vue templates fast.

The result is still Virtual DOM. A component still returns VNodes, and Vue still
patches them. But the generated VNodes carry much more information: block
boundaries, patch flags, stable fragment flags, dynamic prop names, cached event
handlers, cached static nodes, and slot stability metadata.

[简体中文](/zh/blog/vdom)

<script setup>
import slotLocalUpdateCode from './examples/vdom-slot-local-update.tsx?raw'
import basicsCode from './examples/vdom-basics.tsx?raw'
</script>

## What Ordinary JSX Compiles To

The REPL below contains ordinary JSX, plus a button that toggles the state so
you can watch updates happen. The compiled output is already open for you (the
`js` tab): the source on top, the Vue JSX compilation result below.

<BlogRepl :app="basicsCode" auto-select-output />

A plain Babel transform turns this into correct VNode creation calls, but it
hands the runtime very little structural information. On update, Vue has to
assume that all children and props might have changed and re-diff the whole
tree.

The output you see in the `js` tab is what you get after parsing through Oxc,
building semantic scope information, lowering JSX into the `VNodeCall` IR, and
generating optimized calls. Every line tells Vue something structural:

- The compiler proves that the only prop that can change on the `<p>` is
  `class`, so the patch flag is `2` (`CLASS`) and the props diff only looks at
  class:

  ```js
  _createElementVNode('p', { class: ... }, [_normalizeVNode(() => text)], 2)
  ```

- Dynamic text is wrapped in a getter that is evaluated on demand:

  ```js
  _normalizeVNode(() => text)
  ```

- The `<button>`'s inline `onClick` only references `isDone`, a ref declared
  in setup scope, so the compiler proves the handler is stable and caches it.
  Every render reuses the same closure — the runtime never receives a fresh
  function prop, and the button needs no patch flag:

  ```js
  _createElementVNode(
    'button',
    { onClick: _cache[0] || (_cache[0] = () => (isDone.value = !isDone.value)) },
    'toggle',
  )
  ```

- The static `<footer>` is created once and skipped forever after (`-1` means
  "never patch"):

  ```js
  _cache[1] || (_cache[1] = _createElementVNode('footer', null, 'static', -1))
  ```

- The `<section>` becomes a block boundary (`_createElementBlock`): on update,
  Vue walks only the dynamic children recorded by the block, not the whole
  tree.

The important part is not the helper names. The important part is that the
compiler has told Vue exactly where the dynamic surface is.

Go back to the REPL above and edit the source — for example, change `class` to
a static string, or make `<footer>` read a ref — and watch how the compiled
output changes. That is the most direct way to feel how the compiler separates
static from dynamic.

## Where The Runtime Work Disappears

The compiler classifies every JSX node and expression before code generation.
Static text, static elements, and cacheable props are lifted out of the hot
path. Stable VNodes are stored in the per-component-instance cache created by
`_createVNodeCache`, and their patch flag
is set to `-1`, so Vue can skip the subtree.

Dynamic props are not diffed as arbitrary objects when the compiler can prove
their names. A dynamic class on a native element becomes the `CLASS` patch flag.
A known dynamic prop list becomes `PROPS` plus a `dynamicProps` array. Dynamic
keys and spreads fall back to `FULL_PROPS`, which is slower but correct.

Event handlers get the same treatment — the toggle button's `onClick` above is
the example. The compiler checks whether an inline handler references
render-local scope or `this`. If it does not, the handler is cached and the
runtime receives the same function identity on every render. If it does, the
closure has to be recreated each render, and the compiler says so explicitly:
`onClick` lands in `dynamicProps` and is diffed on update.

Text gets the same treatment as the static `<footer>` above: literal text is
cached, and dynamic text goes through a getter. That lets Vue normalize values
lazily in a block-aware way, instead of normalizing everything eagerly on every
render.

## Local Updates In Virtual DOM Mode

Virtual DOM mode is not "true fine-grained DOM updates"; Vapor is for that. But
it is still local in the Vue block-tree sense.

When Vue enters optimized mode for a compiled block, it does not blindly walk the
entire child tree. It follows the block's dynamic children and applies the patch
flags attached by the compiler. A static sibling can sit next to a dynamic
sibling without being reconsidered on every update.

Slots are another place where this matters. The compiler tracks slot scopes and
marks slots as stable, dynamic, or forwarded. A stable slot can capture its own
dependencies, so the parent does not have to force the child component to update
just because a slot object exists.

### Try It: Slot-Level Locality

The two slots in the REPL below look almost identical. The only difference is
that the `dynamic` slot reads `offset`, a variable declared inside the parent's
render function, while the `stable` slot only touches setup-local state. The
counters are a probe — mutating state inside a slot is not a style
recommendation; it just makes "was this slot invoked again?" visible.

Click the button to rerender the parent. The `dynamic` counter grows on every
click, because a dynamic slot is re-invoked on each parent render. The `stable`
counter never moves, because a stable slot is not re-invoked at all: the
compiler proves it does not depend on render-local scope, so a parent rerender
does not force the child to update.

<BlogRepl :app="slotLocalUpdateCode" />

The implementation behind this is scope analysis, not a runtime trick. During
the JSX transform, the compiler records slot scopes for each component call. If
slot children touch identifiers from the render-local scope, the slot is marked
dynamic and the generated component VNode receives dynamic slot metadata. If
they do not, the generated slots object carries the stable slot flag, allowing
Vue's optimized path to skip the slot diff and child update pressure.

> [!WARNING]
> Render-local identifiers are not the only trigger. A slot nested inside
> another slot's scope or inside a `map` callback is dynamic too, because
> parameters like `scope` and `item` are fresh on every invocation:
>
> ```jsx
> // both inner slots are dynamic
> <Comp>{(scope) => <Output>{scope.foo}</Output>}</Comp>
> <>{list.map((item) => <Output>{item}</Output>)}</>
> ```

That is the practical runtime win over a plain Babel transform: less allocation,
less normalization, less prop diffing, fewer child visits, and fewer unnecessary
component updates.

## The Compiler Principles

Vue JSX 3.3's Virtual DOM compiler follows four principles.

1. Parse with a real compiler front end. Oxc gives the transform a fast parser,
   typed AST, allocator-backed mutations, and semantic scope analysis.

2. Lower JSX into Vue-aware IR. The compiler does not immediately print helper
   calls. It first records `tag`, `props`, `children`, `patch_flag`,
   `dynamic_props`, `directives`, block requirements, and directive metadata.

3. Prove constants before emitting code. `ConstantTypes` separates values that
   are not constant, can skip patching, can be cached, or can be stringified.
   That proof drives hoisting and VNode caching.

4. Emit runtime hints instead of runtime guesses. Block helpers, patch flags,
   dynamic prop arrays, stable fragment flags, slot flags, and cached handlers
   move work from update time to compile time.

This is why Vue JSX 3.3 can keep the expressiveness of JSX while getting much
closer to the performance profile people expect from Vue's template compiler.
