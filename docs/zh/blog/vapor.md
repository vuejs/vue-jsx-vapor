# Vapor 模式：真正的局部更新，真正的函数式编程

Vapor Mode 是 Vue JSX 不再为热路径创建 Virtual DOM 树的地方。编译器会把 JSX
转换成 DOM block factory 加上一组很小的响应式操作。组件写起来仍然像函数，但更新
不再是组件级 render pass。

[English](/blog/vapor)

## 从 JSX 到 Block Factory

看一个小组件：

```tsx
import { ref } from 'vue'

export default () => {
  const count = ref(0)
  const ok = ref(true)

  return () => (
    <div class={{ active: ok.value }} onClick={() => count.value++}>
      count: {count.value}
    </div>
  )
}
```

在 Virtual DOM 模式下，它会生成 block VNode。在 Vapor 模式下，输出更接近这样：

```js
const _t0 = _template("<div> ", 1)

return () => (() => {
  const _n0 = _t0()
  _on(_n0, "click", () => count.value++)
  const _x0 = _txt(_n0)
  _setNodes(_x0, "count: ", () => count.value)
  _renderEffect(() => _setClassName(_n0, ok.value ? 1 : 0, "active"))
  return _n0
})()
```

这里没有为了 text 或 class 做 VNode diff。template 只负责创建 DOM。编译器找到
文本节点，`count.value` 只更新这个文本位置；`ok.value` 只更新这个 class bit。

## 什么是真正的局部更新

Vapor 的局部更新意味着响应式单元是 operation，而不是组件 render 函数。

编译器会构建 Vapor IR，里面有 `SetText`、`SetProp`、`SetNodes`、
`CreateComponent`、`If`、`For`、`SlotOutlet`、`Key` 等操作节点。随后编译器判断
每个操作是可以执行一次，还是必须被包进 `renderEffect`。

如果 prop 是静态值，它会进入 HTML template 字符串；如果是动态值，它会变成一个
定向 setter。如果一段文本包含动态值，编译器会留下 anchor 或 text child，然后生成
`setNodes` 或 `setText`。

runtime 的 `setNodes` 只解析传给这个位置的值。动态函数由 `renderEffect` 追踪；
变化时只围绕同一个 anchor 更新或替换之前的节点/Fragment。组件其余部分不会为了
重新发现同一棵 DOM 结构而再次执行。

## 静态 HTML 与动态岛

Vapor 编译器遍历 JSX 时会维护一个 `template` buffer。带静态属性的原生元素会被
字符串化：

```html
<button type=button> </button>
```

生成代码再调用 Vue 的 `template` helper，并用直接路径访问动态 children：

```js
const _n0 = _t0()
const _x0 = _txt(_n0)
```

更深的节点会通过 `_child`、`_next`、`_nthChild` 访问。相邻动态节点还能复用游标，
避免反复从父节点按下标查询。这是一个很小的细节，但很好地说明了 Vapor 的设计：
只要编译器已经知道 DOM 形状，runtime 就不应该再重新发现一次。

## 控制流也会被编译

`v-if` 不会变成“重新调用 render 函数再 diff 结果”。它会变成
`createIf(condition, positiveBlock, negativeBlock, flags)`。flags 会编码分支形状、
`v-once`、slot-root 行为，以及某个分支是否可以跳过自己的 `EffectScope`。

`v-for` 会变成 `createFor(source, block, getKey, flags)`。编译器会传入形状信息，
例如“这个列表是父元素唯一 child”、“列表项是组件”、“block 是单节点”或“block 是
Fragment”。有 key 时，编译器还可以为 `item.id === id` 这类模式创建 selector，
让列表项内部 effect 只在相关 key 改变时 reset。

换句话说，Vapor 不是去掉编译器。Vapor 更依赖编译器，因为正是编译器把声明式 JSX
变成精确的 DOM 操作。

## 为什么它仍然是函数式的

作者面对的模型仍然是函数。组件接收 props 和 context，闭包里读取响应式状态，然后
返回 block result。心智模型里不需要实例代理，也不需要基于 `this` 的 render 契约。

真正会修改 DOM 的非函数式部分，由编译器生成，并隔离成一个个小操作。你写的代码在
描述一个值；编译器输出的代码知道如何把这个值维护到 DOM 上。

这就是 Vapor JSX 的“真正的函数式编程”：组件保持为可组合函数，而 effect 变得显式、
局部，并且由编译器生成。

## 实际收益

Vapor 模式适合频繁局部更新的界面：计数器、表单、dashboard、可编辑行、实时数据、
接近动画的 UI。静态 DOM 创建一次，每个响应式读取只更新它被使用的位置。

Virtual DOM 模式仍然是兼容性默认值。Vapor 可以通过 `vapor: true`、`.vapor.tsx`、
`.vapor.jsx`、`defineVaporComponent` 或 `defineVaporCustomElement` 渐进启用。
这样你可以逐步把真正的局部更新引入项目。
