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
        {(user, index) => <li key={user.id}>{index}: {user.name}</li>}
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
          <li>{user.name}，位置：{index.value}</li>
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
