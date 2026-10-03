# 组件

## 组件命名

Vue JSX 遵循标准 JSX 的标签命名约定：

```tsx
import UserCard from './UserCard'

export function App() {
  return (
    <div>
      <UserCard />
      <UserCard.Header />
    </div>
  )
}
```

小写字母开头的标签始终按原生元素处理，包括未知标签和 kebab-case 标签，不会被解析为 Vue 组件：

```tsx
<button />       // 原生 HTML 元素
<my-widget />    // 元素标签，不是组件
```

使用大写标识符或成员表达式表示 Vue 组件。这样可以让组件解析行为保持确定，避免未来 HTML 新增原生标签时改变已有组件的含义。

## 定义组件

`defineComponent` 用来定义 Virtual DOM 组件，`defineVaporComponent` 用来定义 Vapor 组件。两者都可以接收一个 setup 函数，或者一个带 `setup` 的选项对象：

```tsx
import { defineComponent, defineVaporComponent } from 'vue-jsx'

// Virtual DOM：setup 返回渲染函数。
const Counter = defineComponent((props: { count: number }) => {
  return () => <button>{props.count}</button>
})

// Vapor：setup 直接返回块。
const VaporCounter = defineVaporComponent((props: { count: number }) => (
  <button>{props.count}</button>
))
```

```tsx
// 选项对象形式也可以，两个入口都支持。
const Counter = defineComponent({
  props: { count: Number },
  setup(props) {
    return () => <button>{props.count}</button>
  },
})
```

### 导入来源

`vue-jsx` 也提供了这两个函数，与 `vue` 不同的是 `inheritAttrs` 默认为 `false` 并使用 attrs 作为 props，意味着不用提前定义 props 了：

```ts
import { defineComponent, defineVaporComponent } from 'vue-jsx'
```

### `inheritAttrs` 默认为 `false`

属性不会自动落到根元素上，而是留在 `attrs` 里，由组件决定展开到哪个元素：

```tsx
import { defineVaporComponent } from 'vue-jsx'

const Button = defineVaporComponent(() => <button>Click</button>)

// 渲染结果是 `<button>Click</button>`，`data-test` 不会应用到根元素上。
export default () => <Button data-test="submit" />
```

这样透传就是显式的：渲染多个元素的组件不再依赖 Vue 去猜测哪个是根元素，包装组件也无法把属性泄漏到它并不希望的元素上。

### 用属性充当 props

当组件没有声明 `props` 选项时，属性会作为第一个参数传给 `setup`。它与 `useAttrs()` 返回的是同一个对象，因此即使没有任何 `props` 选项声明 `title`，也可以读取 `props.title`：

```tsx
import { defineVaporComponent } from 'vue-jsx'

const Card = defineVaporComponent((props: { title: string }) => (
  <section>
    <h2>{props.title}</h2>
  </section>
))

export default () => <Card title="Hello" />
```

## `For`

Vue JSX 为 Virtual DOM 提供了 `For`，为 Vapor 模式提供了 `VaporFor`。两个组件都能直接从 `in` 推断 item 和 index 类型，不依赖指令专用的语言工具。

### Virtual DOM

从 `vue-jsx` 导入 `For`，并在默认插槽中返回带 key 的节点：

```tsx
import { defineComponent, ref } from 'vue'
import { For } from 'vue-jsx'

export default defineComponent(() => {
  const users = ref([
    { id: 1, name: 'Ada' },
    { id: 2, name: 'Grace' },
  ])

  return () => (
    <ul>
      <For in={users.value}>
        {(user, index) => (
          <li key={user.id}>
            {index}: {user.name}
          </li>
        )}
      </For>
    </ul>
  )
})
```

`For` 使用 Vue 的 keyed Fragment 列表渲染。请为每一项返回的根节点设置稳定的 `key`，以便 Vue 正确复用和移动已有节点。

### Vapor 模式

在 Vapor 模式编译的组件中使用 `VaporFor`：

```tsx
import { ref } from 'vue'
import { VaporFor } from 'vue-jsx'

export default () => {
  const users = ref([
    { id: 1, name: 'Ada' },
    { id: 2, name: 'Grace' },
  ])

  return (
    <ul>
      <VaporFor in={users.value}>
        {(user, index) => (
          <li>
            {user.name}，位置：{index.value}
          </li>
        )}
      </VaporFor>
    </ul>
  )
}
```

Vapor 插槽接收到的 index 是 `ShallowRef<number>`。在 JavaScript 中通过 `index.value` 读取；插入、删除或移动列表项后，JSX 中使用的 index 会保持响应式更新。

也可以从 Vapor 专用入口导入更短的别名：

```ts
import { For } from 'vue-jsx/vapor'
```

这里的 `For` 与 `VaporFor` 是同一个组件。

### Vapor 模式的稳定 key

`VaporFor` 默认使用 item 本身作为 key，适合对象引用保持不变的列表。当新对象仍表示同一条数据时，可以通过 `getKey` 提供稳定 key：

```tsx
<VaporFor in={users.value} getKey={(user) => user.id}>
  {(user, index) => (
    <li>
      {user.value.name}，位置：{index.value}
    </li>
  )}
</VaporFor>
```

提供 `getKey` 后，插槽中的 item 也会变成 `ShallowRef`。这样 Vapor 可以根据稳定 key 复用已有 block，同时把 `user.value` 更新为最新的 item 对象。

### 支持的数据源

两个组件都支持数组、字符串、数字、普通对象、`Set` 和 `Map`。

数组和其他 iterable 的插槽参数为 `(item, index)`；普通对象的插槽参数为 `(value, key, index)`。在 `VaporFor` 中，对象的 key 和 index 同样是 shallow ref。

需要原生 TypeScript 推断和高效列表更新时，推荐使用这两个组件。对于不关心 keyed 更新的小型或静态列表，也可以继续使用 `Array.prototype.map()`。
