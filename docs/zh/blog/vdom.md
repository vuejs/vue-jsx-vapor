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
import basicsCode from '../../blog/examples/vdom-basics.tsx?raw'
</script>

## 普通 JSX 会编译成什么

下面 REPL 里是一段普通的 JSX，外加一个切换状态的按钮，方便观察更新。
编译输出已经为你打开（`js` 标签）：上面是源码，
下面就是 Vue JSX 的编译产物。

<BlogRepl :app="basicsCode" auto-select-output />

普通 Babel transform 会把它转成正确的 VNode 创建调用，但交给 runtime 的
结构信息太少。更新时，Vue 只能假设所有 children 和 props 都可能变化，
整棵树重新对比。

而你在 `js` 标签里看到的输出，是经过 Oxc 解析、语义作用域分析、降级到
`VNodeCall` IR 后再生成的调用。每一行都在向 Vue 传递结构信息：

- 编译器证明了 `<p>` 上唯一会变的 prop 是 `class`，于是 patch flag 是 `2`
  （`CLASS`），props diff 只看 class，其他属性不参与：

  ```js
  _createElementVNode('p', { class: ... }, [_normalizeVNode(() => text)], 2)
  ```

- 动态文本包装成 getter，按需取值：

  ```js
  _normalizeVNode(() => text)
  ```

- `<button>` 的内联 `onClick` 只引用了 setup 作用域里的 `isDone`，编译器因此
  证明这个 handler 是稳定的，并把它缓存起来。每次 render 复用同一个闭包，runtime
  不会再拿到新的函数 prop，按钮本身也不需要 patch flag：

  ```js
  _createElementVNode(
    'button',
    { onClick: _cache[0] || (_cache[0] = () => (isDone.value = !isDone.value)) },
    'toggle',
  )
  ```

- 静态的 `<footer>` 只创建一次，之后永远跳过（`-1` 表示「无需 patch」）：

  ```js
  _cache[1] || (_cache[1] = _createElementVNode('footer', null, 'static', -1))
  ```

- `<section>` 成为 block 边界（`_createElementBlock`），更新时 Vue 不再遍历
  整棵 children 树，只走 block 记录下来的动态节点。

重点不是 helper 名称，而是编译器已经把「哪里会变」精确告诉了 Vue。

回到上面的 REPL 动手改一改——比如把 `class` 换成静态字符串，或者让
`<footer>` 读一个 ref——然后观察编译输出的变化，就能直观感受编译器是
怎么区分静态和动态的。

## 运行时少做了什么

编译器会在代码生成前分类每个 JSX 节点和表达式。静态文本、静态元素、可缓存
props 会从热路径中移走。稳定 VNode 保存在 `_createVNodeCache` 为每个组件实例创建的缓存里，并带上
`-1` patch flag，让 Vue 跳过整棵静态子树。

当编译器能证明动态 prop 的名字时，runtime 不再需要把 props 当任意对象 diff。
原生元素上的动态 class 会变成 `CLASS` patch flag；已知动态 props 会变成
`PROPS` 加 `dynamicProps` 数组；动态 key 和 spread 则回退到 `FULL_PROPS`，
慢一点但正确。

事件处理器也是同样的待遇——上面 toggle 按钮的 `onClick` 就是例子。编译器会判断
内联 handler 是否引用了 render-local 作用域或 `this`。没有引用就缓存，runtime
每次 render 拿到的是同一个函数引用；引用了就只能在每次 render 重建闭包，
编译器也会明确说明这一点：`onClick` 进入 `dynamicProps`，更新时参与 diff。

文本的待遇和上面静态 `<footer>` 那条一致：静态文本缓存，动态文本走 getter。这样
Vue 可以在 block 语义下按需归一化，而不是每次 render 都急着处理所有值。

## VDOM 模式里的局部更新

Virtual DOM 模式不是「真正的细粒度 DOM 更新」，那是 Vapor 的工作。但它已经是
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
为了让「这个 slot 有没有被重新调用」一眼可见。

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

Vue JSX 3.3 的 Virtual DOM 编译器大致遵循四条原则。

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
