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
</script>

## The Babel Baseline

A syntax-level JSX transform usually has one job: turn JSX into VNode creation
calls. It can preserve Vue semantics, but the output tends to look like this:

```js
_createVNode("section", null, [
  _createVNode("h2", null, "Todo"),
  _createVNode("ul", null, [
    _createVNode(_Fragment, null, _renderList(items, (item, i) =>
      _createVNode("li", {
        key: item.id,
        class: _normalizeClass({ active: item.id === selected })
      }, [
        _normalizeVNode(i),
        _normalizeVNode(": "),
        _normalizeVNode(item.text)
      ])
    ))
  ]),
  _createVNode("footer", null, "static")
])
```

That code is correct, but it leaves the runtime with little structural
information. On update, Vue has to treat many children and props as if they
might have changed.

Vue JSX 3.3 runs through Oxc, builds semantic scope information, lowers JSX
into its own `VNodeCall` IR, and then generates optimized Vue runtime calls.
For the same source, optimized output has a very different shape:

```js
const _cache = _createVNodeCache("9a69eae3d9f3c58f")
return _openBlock(), _createElementBlock("section", null, [
  _cache[1] || (_cache[1] = _createElementVNode("h2", null, "Todo", -1)),
  _createElementVNode("ul", null, [
    (_openBlock(true), _createElementBlock(_Fragment, null,
      _renderList(items, (item, i) =>
        (_openBlock(), _createElementBlock("li", {
          key: item.id,
          class: _normalizeClass({ active: item.id === selected })
        }, [
          _normalizeVNode(() => i),
          _cache[0] || (_cache[0] = _normalizeVNode(": ", -1)),
          _normalizeVNode(() => item.text)
        ], 2))
      ),
      128
    ))
  ]),
  _cache[2] || (_cache[2] = _createElementVNode("footer", null, "static", -1))
])
```

The important part is not the helper names. The important part is that the
compiler has told Vue exactly where the dynamic surface is.

## Where The Runtime Work Disappears

The compiler classifies every JSX node and expression before code generation.
Static text, static elements, and cacheable props are lifted out of the hot
path. Stable VNodes are stored through `_createVNodeCache`, and their patch flag
is set to `-1`, so Vue can skip the subtree.

Dynamic props are not diffed as arbitrary objects when the compiler can prove
their names. A dynamic class on a native element becomes the `CLASS` patch flag.
A known dynamic prop list becomes `PROPS` plus a `dynamicProps` array. Dynamic
keys and spreads fall back to `FULL_PROPS`, which is slower but correct.

Event handlers get the same treatment. The compiler checks whether an inline
handler references local scope or `this`. Stable handlers are cached so the
runtime does not receive a fresh closure on every render:

```js
onClick: _cache[0] || (_cache[0] = () => count.value++)
```

Text is also compiled with intent. Literal text is cached. Dynamic text can be
wrapped in a getter:

```js
_normalizeVNode(() => count.value)
```

That getter lets Vue normalize the value lazily in a block-aware way, instead of
normalizing everything eagerly on every render.

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

The REPL below intentionally uses `foo++` and `bar += offset` inside slot
functions. That is not application code style advice; it is a small probe that
makes slot invocation visible.

Click `+ 2`. The parent `count` changes every time. The first `<Output>` slot
also changes because its body closes over `offset`, a value created inside the
parent render. The second `<Output>` slot stays put because it only closes over
setup-local `foo`, so the compiler can treat that slot as stable and avoid
forcing the child update from the parent render.

<BlogRepl :app="slotLocalUpdateCode" />

The implementation behind this is scope analysis, not a runtime trick. During
the JSX transform, the compiler records slot scopes for each component call. If
slot children touch identifiers from the render-local scope, the slot is marked
dynamic and the generated component VNode receives dynamic slot metadata. If
they do not, the generated slots object carries the stable slot flag, allowing
Vue's optimized path to skip the slot diff and child update pressure.

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
