# Custom Element

Vue JSX 既可以直接使用已有的 Custom Element，也可以定义新的 Custom Element。Virtual DOM 和 Vapor 模式使用相同的 JSX 语法。

## 使用 Custom Element

名称为小写且包含连字符的标签会被自动识别为 Custom Element，无需配置 `isCustomElement`。

```tsx
export default () => (
  <user-card name="Ada Lovelace">
    <span slot="avatar">AL</span>
  </user-card>
)
```

通过标准 `slot` 属性，可以向原生具名插槽传递内容。

> [!TIP]
> Custom Element 名称必须包含连字符。Vue JSX 也据此区分 `<user-card>`、`<div>` 等原生 HTML 元素，以及 `<UserCard>` 这样的 Vue 组件。

## 定义 Custom Element

Virtual DOM 使用 Vue 的 `defineCustomElement`，Vapor 模式使用 `defineVaporCustomElement`。

::: code-group

```tsx [Virtual DOM]
import { defineCustomElement } from 'vue'

const UserCard = defineCustomElement(
  (props: { name: string }) => () => (
    <article>
      <slot name="avatar" />
      <strong>{props.name}</strong>
    </article>
  ),
  {
    props: {
      name: { type: String, required: true },
    },
  },
)
```

```tsx [Vapor 模式]
import { defineVaporCustomElement } from 'vue'

const UserCard = defineVaporCustomElement(
  (props: { name: string }) => (
    <article>
      <slot name="avatar" />
      <strong>{props.name}</strong>
    </article>
  ),
  {
    props: {
      name: { type: String, required: true },
    },
  },
)
```

:::

使用标签前，需要把构造器注册到浏览器：

```ts
if (!customElements.get('user-card')) {
  customElements.define('user-card', UserCard)
}
```

开发时使用这个判断，可以避免重复注册同名 Custom Element。在 SSR 项目中，应当只从客户端入口执行注册逻辑。

## Props 与 Attributes

TypeScript 标注用于静态类型检查；运行时的 `props` 选项用于告诉 Vue 需要监听哪些 HTML attribute，并支持 `Number`、`Boolean` 等类型转换。

```tsx
const ProgressRing = defineVaporCustomElement(
  (props: { value: number; compact: boolean }) => (
    <output>{props.compact ? props.value : `${props.value}%`}</output>
  ),
  {
    props: {
      value: Number,
      compact: Boolean,
    },
  },
)
```

```html
<progress-ring value="75" compact></progress-ring>
```

## 事件

Vue Custom Element 发出的事件会作为原生 `CustomEvent` 派发，事件参数可以通过 `event.detail` 访问。

```tsx
const UserCard = defineVaporCustomElement(
  (props: { name: string }, { emit }) => (
    <button onClick={() => emit('select', props.name)}>{props.name}</button>
  ),
  {
    props: { name: String },
    emits: ['select'],
  },
)

export default () => (
  <user-card
    name="Ada"
    onSelect={(event: CustomEvent<[string]>) => {
      console.info(event.detail[0])
    }}
  />
)
```

## Shadow DOM 与样式

Vue Custom Element 默认使用 Shadow DOM。可以通过 `styles` 注入样式；如果需要渲染到 Light DOM，则设置 `shadowRoot: false`。

```tsx
const UserCard = defineVaporCustomElement(render, {
  styles: [
    `:host { display: block; }`,
    `article { border: 1px solid #d8dee4; padding: 12px; }`,
  ],
  shadowRoot: true,
})
```

## 为 JSX 添加严格类型

默认情况下，未知的 Custom Element 标签也可以直接使用，因此第三方组件无需额外配置。应用或组件库可以扩展 `JSX.IntrinsicElements`，为特定标签约束 props 和事件：

```ts
declare module 'vue-jsx' {
  namespace JSX {
    interface IntrinsicElements {
      'user-card': {
        name: string
        compact?: boolean
        onSelect?: (event: CustomEvent<[string]>) => void
      }
    }
  }
}
```

扩展后，`<user-card>` 会获得属性补全和类型检查，同时仍可继续使用其他 Custom Element。
