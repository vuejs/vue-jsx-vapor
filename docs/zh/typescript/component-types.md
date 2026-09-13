# 组件类型

props、slots、emits 和 `expose` 抵达 JSX 调用点靠的是普通 TypeScript：
`JSX.LibraryManagedAttributes` 把组件声明类型改写成 TypeScript 接下来检查的
attributes。本页说明这套改写能推断什么、在泛型组件哪里会停下来，以及导出的几个
helper 类型如何补上缺失的部分。

## 改写什么

| 组件上       | 在 JSX 中                         |
| ------------ | --------------------------------- |
| `props` 参数 | attribute 本身                    |
| `emit`       | `onXxx` 回调 props                |
| `slots`      | `v-slots` prop，以及 JSX children |
| `exposed`    | `ref` 的目标类型                  |

```tsx
const Panel = (props: { step: number }, { slots }: { slots: { default?: (n: number) => any } }) => (
  <div>{slots.default?.(props.step)}</div>
)

export default () => <Panel step={1}>{(n) => <span>{n.toFixed()}</span>}</Panel>
```

不需要任何语言工具，`n` 就是 `number`：slot 的签名本来就是组件类型的一部分。
[类型推断：Props、Ref 与 Children](/zh/blog/type-inference) 逐项讲过这套改写的
三个分支——构造函数组件、函数组件与未知组件。

## 泛型 props 停在 setup context

类型实参是从 **attributes 对象**推断的。组件通过第二个参数、或通过实例类型暴露
出去的东西，都在这一步推断之前就被解析，所以只用在那里的类型参数会保持自己的
约束——没有约束时就是 `unknown`：

```tsx
const List = <T,>(props: { items: T[] }, { slots }: { slots: { row?: (item: T) => any } }) => (
  <ul>{slots.row?.(props.items[0])}</ul>
)

// 下面 slot 里的 `item` 是 `unknown`
export default () => <List items={[{ id: 1 }]}>{{ row: (item) => <li>{item.id}</li> }}</List>
```

- `items` 仍然按 `T[]` 检查：props 就是 props。
- slot 里的 `item` 是 `unknown`；写成 `T extends string | number` 时它是
  `string | number`——那是约束，不是类型实参。
- 显式类型实参 `<List<{ id: number }>>` 也只影响 props。生成的 `v-slots` 仍来自
  未解析的那份签名。

出路是别再让改写去负责这三样：把它们声明成 props，放进 TypeScript 推断所在的位置。

## Props helper

导出的三个类型，能把 setup context 的一部分变成 prop：

- `SlotsToProps<Slots>` —— 把 slot 签名合成一个可选的 `v-slots` prop。
- `ExposedToProps<Exposed>` —— `ref` 的目标类型，外加一个内部标记 key。
- `SetupContextToProps<Emits, Slots, Exposed>` —— emits、slots、exposed 一起。

它们从 `vue-jsx` 导出：

```ts
import type { ExposedToProps, SetupContextToProps, SlotsToProps } from 'vue-jsx'
```

改写不会重复生成它们：只有 `'v-slots' extends keyof Props` 为假时才合成
`v-slots`，只有 `'ref' extends keyof Props` 为假时才从组件推断 `ref`。已声明的
prop 永远优先。

### SlotsToProps

```tsx
import type { SlotsToProps } from 'vue-jsx'

type ListProps<T> = { items: T[] } & SlotsToProps<{ row?: (item: T) => any }>

const List = <T,>(props: ListProps<T>) => <ul>{props.items.length}</ul>

export default () => <List items={[{ id: 1 }]} v-slots={{ row: (item) => <li>{item.id}</li> }} />
```

现在 `item` 是 `{ id: number }`，因为 `v-slots` 成了 TypeScript 据以推断类型实参
的 attribute 之一。children 同理——`ElementChildrenAttribute` 让 JSX children 按
`v-slots` 检查。

`SlotsToProps` 接受普通的 slot 记录或 Vue 的 `SlotsType`，并把每个 slot 的返回
类型放宽到 `NodeChild`，Virtual DOM 与 Vapor 的渲染结果都放得下。

### ExposedToProps

```tsx
import type { ExposedToProps } from 'vue-jsx'

type CardProps<T> = { value: T } & ExposedToProps<{ reset: () => void }>

const Card = <T,>(props: CardProps<T>) => <div>{props.value}</div>

export default () => <Card value={1} ref={(exposed) => exposed?.reset()} />
```

`ExposedToProps<T>` 加上内部标记 key，并把 `ref` 标成 `NodeRef<T>`，于是
`exposed` 是 `{ reset: () => void } | null`——和非泛型组件从 `ctx.expose()` 拿到
的形状一样。

### SetupContextToProps

`SetupContextToProps<Emits, Slots, Exposed>` 一次给全整个 context：
`EmitsToProps`、`SlotsToProps` 与 `ExposedToProps` 的交叉。

```tsx
import type { SetupContextToProps } from 'vue-jsx'

type ListProps<T> = { items: T[] } & SetupContextToProps<
  { change: [value: T] },
  { row?: (item: T) => any },
  { first: T }
>

const List = <T,>(props: ListProps<T>) => <ul>{props.items.length}</ul>

export default () => (
  <List
    items={[1, 2, 3]}
    onChange={(value) => value.toFixed()}
    ref={(exposed) => exposed?.first.toFixed()}
  >
    {{ row: (item) => <li>{item.toFixed()}</li> }}
  </List>
)
```

## 该用哪个

| 场景                                 | 用法                        |
| ------------------------------------ | --------------------------- |
| 普通组件，context 有标注             | 什么都不用加，推断已经完整  |
| 泛型组件，children 需要看到 `T`      | props 里加 `SlotsToProps`   |
| 泛型组件，`ref` 需要看到 `T`         | props 里加 `ExposedToProps` |
| 泛型组件，emits、slots、exposed 都要 | `SetupContextToProps`       |
