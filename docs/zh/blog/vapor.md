# Vapor 模式篇：真正的局部更新，真正的函数式编程

Vapor 模式是 Vue JSX 不再为热路径构建 Virtual DOM 树的地方。编译器把 JSX
转换成 DOM 模板加一组很小的响应式操作。组件写起来仍然像函数——但它只执行
一次。之后每次状态变化，都只落在一条定向的 DOM 操作上。

产物不是「更快的 VDOM」。这里没有 VNode、没有 diff、没有组件级 re-render。
静态 DOM 从模板字符串创建一次，每个动态绑定变成自己的 effect，只更新 DOM
里的一个位置。

[English](/blog/vapor)

<script setup>
import basicsCode from '../../blog/examples/vapor-basics.tsx?raw'
</script>

## 组件只执行一次

下面 REPL 里的 JSX 和 [Virtual DOM 篇](/zh/blog/vdom)是同一类普通代码：
一个动态 class、一段动态文本、一个条件分支、两个事件处理器。编译输出已经为你打开
（`js` 标签）：上面是源码，下面就是 Vue JSX Vapor 的编译产物。

<BlogRepl :app="basicsCode" auto-select-output vapor />

产物足够短，可以逐行读。注意两个 import 来源：`template`、`txt`、`on`、
`renderEffect`、`setClassName`、`createIf`、`setInsertionState` 来自 Vue 的
Vapor runtime；`setNodes` 来自 vue-jsx 的 helper 模块。

- 整个静态结构是一个 HTML 字符串：

  ```js
  const _t2 = _template(
    '<section class=demo><p> </p><!><button>increment</button><button>toggle',
    1,
  )
  ```

  `class=demo` 这类静态 prop 直接烤进字符串。`<p>` 里的空格是为动态文本
  预留的锚点，`<!>` 是条件分支的锚点。末尾的 `1` 是标记组件根节点的 flag。

- 三元表达式的两个分支各自是一个独立模板：

  ```js
  const _t0 = _template('<p>on', 2)
  const _t1 = _template('<p>off', 2)
  ```

- 克隆模板会一次性创建所有 DOM，然后代码用直接路径走到动态节点：

  ```js
  const _n9 = _t2()
  const _n0 = _child(_n9)
  const _n8 = _next(_n0)
  const _n6 = _next(_n8)
  const _n7 = _next(_n6)
  ```

  `_child` 取第一个子节点，`_next` 走到下一个兄弟节点。相邻节点复用游标，
  不需要再按下标向父节点查询。

- 动态文本是「静态前缀 + getter」，挂到锚点上：

  ```js
  const _x0 = _txt(_n0)
  _setNodes(_x0, 'count: ', () => count.value)
  ```

  前缀只解析一次。getter 在 render effect 里执行，所以 `count` 变化时，
  只有这个文本位置会更新。

- 事件只绑定一次：

  ```js
  _on(_n6, 'click', () => count.value++)
  _on(_n7, 'click', () => (ok.value = !ok.value))
  ```

  不存在第二次 render，Virtual DOM 篇里 handler 缓存的问题在这里根本
  不存在。

- 动态 class 是一个 effect 包一个 setter：

  ```js
  _renderEffect(() => _setClassName(_n0, ok.value ? 1 : 0, 'active'))
  ```

  `ok` 变化时，effect 重新执行，翻转 `<p>` 上的 `active` class 位。没有
  props diff，更新时也没有 class 归一化。

- 条件分支是一个 `_createIf` 调用：

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

  初始渲染 `on` 分支；`ok` 翻转时，runtime 在 `<!>` 锚点处把整棵分支 DOM
  换成另一棵。

最重要的观察是：组件函数不会第二次出现。更新不是「重新 render 再对比」，
而是各个 effect 重新执行各自的 setter。

在 REPL 里改一改源码——把 class 换成静态字符串，或者让按钮文本读一个
ref——然后观察模板字符串和 effect 列表怎么变。

## 更新发生在哪里

每个动态绑定都编译成一个 effect 包一个定向操作。但操作的方向，在元素和
组件上是相反的。

在元素上，编译器生成 setter。动态 class 变成
`_renderEffect(() => _setClassName(...))`，动态 prop 变成
`_renderEffect(() => _setProp(_n0, 'id', id.value))`。你组件里的 effect 执行
setter——把值推进 DOM。

组件 props 恰好相反：它们变成 getter。

```js
const _n0 = _createComponent(Comp, {
  prop: () => ok.value,
  static: 'x',
})
```

动态 prop 值以 getter 函数的形式传入，静态值原样传递。求值被推迟到子组件
内部：子组件自己的 effect 调用这些 getter，自己追踪依赖，自己更新自己。
父组件不推送更新——子组件按需拉取。

## 为什么说是「函数式编程」

因为组件就是一个普通的函数：props 进去，UI 出来。没有 `this`，没有实例，
也不用关心框架什么时候再调用你——它只在挂载时执行一次。

那更新谁来做？就是你在编译产物里已经认识的那些 effect：`_setNodes` 管文本，
`_renderEffect` 管动态 class，`_createIf` 管分支，各自只守着自己那一小块
DOM。状态变了，对应的 effect 自己重新运行，你的函数完全不知情。

对比一下就很清楚：Virtual DOM 的 Options API 里，每次状态变化，整个
render 函数都要重新执行一遍，重建虚拟树再 diff。Vapor 里你写一遍，它只
跑一遍——你的代码里没有一行「去改 DOM」，那些都是编译器写的。

## 实际收益与渐进启用

Vapor 模式适合频繁局部更新的界面：计数器、表单、dashboard、可编辑行、
实时数据、接近动画的 UI。静态 DOM 创建一次，每个响应式读取只更新它被
使用的位置。

Virtual DOM 模式仍然是兼容性默认值。Vapor 按需启用：在插件选项里设置
`vapor: true`、把文件命名为 `*.vapor.tsx` / `*.vapor.jsx`、或用
`defineVaporComponent` / `defineVaporCustomElement` 包裹组件。这样可以按
文件、按组件渐进地引入真正的局部更新。
