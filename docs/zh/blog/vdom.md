# Virtual DOM 篇：编译器驱动的 JSX

Vue JSX 3.3 对 JSX 的定位不是 `h()` 的语法糖，而是 Vue 编译器的输入格式。
这是它和普通基于 Babel 的 Vue JSX 最大的区别：编译器理解 Vue 的更新模型，
因此能生成和 Vue 模板编译器同一类运行时提示。

最终产物仍然是 Virtual DOM。组件仍然返回 VNode，Vue 仍然负责 patch。
但这些 VNode 会携带更多信息：block 边界、patch flag、稳定 Fragment 标记、
动态 props 名称、缓存过的事件处理器、缓存过的静态节点，以及 slot 稳定性。

[English](/blog/vdom)

<script setup>
import slotLocalUpdateCode from '../../blog/examples/vdom-slot-local-update.tsx?raw'
</script>

## Babel 基线

普通 JSX transform 通常只做一件事：把 JSX 转成 VNode 创建调用。它可以保持
Vue 语义正确，但输出往往接近这样：

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

这段代码是对的，但 runtime 得到的结构信息太少。更新时，Vue 只能假设很多
children 和 props 都可能变化。

Vue JSX 3.3 会经过 Oxc 解析，建立语义作用域信息，把 JSX 降级到自己的
`VNodeCall` IR，然后再生成优化后的 Vue runtime 调用。同一段源码，优化后输出
的形态明显不同：

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

重点不是 helper 名称，而是编译器已经把动态边界精确告诉了 Vue。

## 运行时少做了什么

编译器会在代码生成前分类每个 JSX 节点和表达式。静态文本、静态元素、可缓存
props 会从热路径中移走。稳定 VNode 会通过 `_createVNodeCache` 保存，并带上
`-1` patch flag，让 Vue 跳过整棵静态子树。

当编译器能证明动态 prop 的名字时，runtime 不再需要把 props 当任意对象 diff。
原生元素上的动态 class 会变成 `CLASS` patch flag；已知动态 props 会变成
`PROPS` 加 `dynamicProps` 数组；动态 key 和 spread 则回退到 `FULL_PROPS`，
慢一点但正确。

事件处理器也会被分析。编译器会判断内联 handler 是否引用了局部作用域或 `this`。
稳定 handler 会被缓存，避免每次 render 都传给 runtime 一个新闭包：

```js
onClick: _cache[0] || (_cache[0] = () => count.value++)
```

文本也不是简单地每次归一化。静态文本会缓存，动态文本可以被包装成 getter：

```js
_normalizeVNode(() => count.value)
```

这样 Vue 可以在 block 语义下按需归一化，而不是每次 render 都急着处理所有值。

## VDOM 模式里的局部更新

Virtual DOM 模式不是“真正的细粒度 DOM 更新”，那是 Vapor 的工作。但它已经是
Vue block tree 意义上的局部更新。

当 Vue 进入编译产物的 optimized mode 后，它不会盲目遍历整棵 children 树。
它会沿着 block 记录的 dynamic children 走，并根据 compiler 生成的 patch flag
更新。一个静态兄弟节点可以和动态兄弟节点并排存在，而不必每次更新都重新参与判断。

slot 也是关键。编译器会追踪 slot 作用域，把 slot 标记为 stable、dynamic 或
forwarded。稳定 slot 能自己捕获依赖，父组件不需要因为存在 slot 对象就强制子组件
更新。

### 试试看：slot 级别的局部性

下面 REPL 里的两个 slot 几乎一模一样，唯一的区别是：`dynamic` slot 读到了
`offset`——一个声明在父组件 render 函数里的变量，而 `stable` slot 只碰
setup 作用域里的状态。计数器只是探针——在 slot 里改状态不是编码建议，只是
为了让“这个 slot 有没有被重新调用”一眼可见。

点击按钮让父组件 rerender。`dynamic` 的数字每次点击都会增长，因为 dynamic
slot 在父组件每次 render 时都会被重新调用；`stable` 的数字永远不动，因为
stable slot 根本不会重新调用——编译器证明了它不依赖 render-local 作用域，
父组件 rerender 不会连累子组件更新。

<BlogRepl :app="slotLocalUpdateCode" />

背后的实现靠的是作用域分析，不是 runtime 临场猜测。JSX transform 过程中，编译器
会为每个组件调用记录 slot scope。如果 slot children 触碰了 render-local scope
里的标识符，就标记为 dynamic，并在生成的组件 VNode 上带上动态 slot 元信息。如果
没有触碰，则生成的 slots object 会携带 stable slot flag，让 Vue 的 optimized path
跳过 slot diff 和不必要的子组件更新压力。

> [!WARNING]
> 不止 render-local 标识符。嵌在其他 slot 的作用域或 `map` 回调里的 slot 同样会被
> 标记为 dynamic，因为 `scope`、`item` 这类参数每次调用都是新的：
>
> ```jsx
> // 两个内层 slot 都是 dynamic
> <Comp>{(scope) => <Output>{scope.foo}</Output>}</Comp>
> <>{list.map((item) => <Output>{item}</Output>)}</>
> ```

这就是相对普通 Babel transform 的 runtime 收益：更少分配、更少归一化、更少
props diff、更少 children 遍历，以及更少不必要的组件更新。

## 编译器原理

Vue JSX 3.3 的 Virtual DOM 编译器大致遵循四步。

1. 使用真正的 compiler 前端。Oxc 提供快速 parser、类型化 AST、allocator-backed
   AST 修改，以及语义作用域分析。

2. 先降级到 Vue-aware IR。编译器不会立刻打印 helper 调用，而是先记录 `tag`、
   `props`、`children`、`patch_flag`、`dynamic_props`、`directives`、block
   需求和指令元信息。

3. 先证明常量，再生成代码。`ConstantTypes` 会区分非静态、可跳过 patch、可缓存、
   可字符串化。这个证明结果决定 hoist 和 VNode cache。

4. 生成 runtime hints，而不是让 runtime 猜。block helper、patch flag、
   dynamic prop 数组、stable fragment flag、slot flag、缓存 handler，都是把更新时
   的工作提前到编译时完成。

所以 Vue JSX 3.3 能保留 JSX 的表达力，同时把运行时行为拉近 Vue 模板编译器的
性能模型。
