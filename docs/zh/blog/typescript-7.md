# 原生 TS7 支持：让 TypeScript 原生推断 props、ref 与 children

Vue JSX 3.3 把普通组件的类型推断放回 TypeScript 自己的 JSX 类型系统里。配置
`jsxImportSource: "vue-jsx"` 后，TypeScript
会读取 `vue-jsx/jsx-runtime` 导出的 JSX namespace，这个 namespace 会告诉 TS：
Vue 组件在 JSX 调用点应该如何检查。

核心是 `JSX.LibraryManagedAttributes`。TypeScript 每次检查 `<Comp ... />` 时，
都会调用这个类型。Vue JSX 就借这个 hook，把组件原始 props 改写成 JSX 用户真正能写
的 props：普通 props 保持原样，emit 变成 `onXxx`，`ref` 指向 exposed 类型，
JSX children 则按 Vue slots 检查。

[English](/blog/typescript-7)

## 类型入口

默认推荐的 TypeScript 配置只有 JSX runtime：

```json
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  }
}
```

`vue-jsx/jsx-runtime/index.d.ts` 会导出 runtime JSX namespace，并把它放进全局类型
空间：

```ts
import type { Fragment, VNode } from 'vue'
export type { JSX } from 'vue-jsx'

declare global {
  export type { JSX } from 'vue-jsx'
}

declare function jsx(type: any, props: any, key: any): VNode

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }
```

这样 TypeScript 自己就能向 Vue JSX 问三个问题：

1. 什么东西算 JSX element？
2. 组件 props 应该从哪里读取？
3. 对这个组件来说，原始 props 要怎么改写？

答案定义在 `packages/runtime/src/jsx.ts` 的 namespace 里：

```ts
export namespace JSX {
  export type Element = RenderResult

  export interface ElementAttributesProperty {
    $props: {}
  }

  export interface ElementChildrenAttribute {
    'v-slots': {}
  }

  export interface IntrinsicElements extends NativeElements {
    [name: string]: any
  }

  export interface IntrinsicAttributes extends ReservedProps {
    class?: ClassValue | undefined
    style?: StyleValue | undefined
  }

  export type LibraryManagedAttributes<Component, Props> = // ...
}
```

`ElementAttributesProperty` 让构造器形式的 Vue 组件从 `$props` 暴露 JSX props。
`ElementChildrenAttribute` 明确告诉 TS：JSX children 不是 React 那种 `children`，
而是流入 `v-slots`。真正的类型适配则交给 `LibraryManagedAttributes`。

## LibraryManagedAttributes 做了什么

下面是 runtime 类型里最关键的一段，只省略了无关的 DOM attributes：

```ts
export type LibraryManagedAttributes<Component, Props> = Props &
  (Component extends abstract new (...args: any[]) => infer Instance
    ? {
        ref?: NodeRef<
          ExtractExposed<
            Props,
            'exposed' extends keyof Instance
              ? string extends keyof NonNullable<
                  NonNullable<Instance['exposed']>
                >
                ? Instance
                : UnwrapRef<Instance['exposed']>
              : Instance
          >
        >
      } & ('v-slots' extends keyof Props
        ? {}
        : '$slots' extends keyof Instance
          ? SlotsToProps<Instance['$slots'] & {}>
          : 'slots' extends keyof Instance
            ? SlotsToProps<Instance['slots'] & {}>
            : {})
    : Component extends (
          props: Props,
          ctx: {
            slots: infer Slots
            attrs: any
            emit: infer Emit
            expose: (exposed: infer Exposed extends Record<string, any>) => void
          },
        ) => any
      ? {
          ref?: 'ref' extends keyof Props
            ? Props['ref']
            : NodeRef<
                string extends keyof Exposed
                  ? NativeElement | VaporComponentInstance
                  : UnwrapRef<Exposed>
              >
        } & EmitFnToProps<Emit, keyof Props> &
          ('v-slots' extends keyof Props ? {} : SlotsToProps<Slots & {}>)
      : {
          ref?: VNodeRef
        })
```

这段类型分三条路。

1. 构造器组件，也就是 Vue `defineComponent` 返回的组件，会暴露一个 instance 类型。
   Vue JSX 从这个 instance 上读取 `$props`、`$slots` 或 `slots`，以及可选的
   `exposed` shape。

2. 直接函数组件的信息来自函数签名：第一个参数是 props，第二个参数里的 `slots`、
   `emit`、`expose` 分别生成 children、事件和 ref 类型。

3. 不认识的组件回退到 Vue 常规的 `VNodeRef`。

## Props 和 Emits

普通 props 最简单：删除 `ref` 后，原始 `Props` 仍然保留在 attribute 类型里。所以
literal props、generic props、union、required props 都仍然按正常 TypeScript 规则
工作。

emit 只需要给函数式组件补一层映射：

```ts
export type EmitFnToProps<T, ExcludeKeys extends PropertyKey = ''> = T extends (
  event: infer Event extends string,
  ...args: infer Args
) => any
  ? string extends Event
    ? {}
    : {
        readonly [K in Event as `on${Capitalize<K>}` extends ExcludeKeys
          ? never
          : `on${Capitalize<K>}`]?: (...args: Args) => any
      }
  : {}
```

如果组件里能写 `emit('change', value)`，JSX 调用点就会得到一个 `onChange` prop，
callback 参数就是同一个 payload。假如同名 prop 已经存在，`ExcludeKeys` 会避免重复
生成。

```tsx
import { type EmitFn } from 'vue'

const Counter = (
  props: { value: number },
  { emit }: { emit: EmitFn<{ change: [value: number] }> },
) => <button onClick={() => emit('change', props.value + 1)} />

;<Counter
  value={1}
  onChange={(value) => {
    value.toFixed()
  }}
/>
```

这里不需要任何编辑器插件去虚拟生成 `onChange`。它是在 TypeScript 原生 JSX 检查过程中
由 `LibraryManagedAttributes` 推出来的。

## Ref 指向 Exposed

`ref` 是 Vue JSX 3.3 不只是透传 `VNodeRef` 的地方。组件公开类型里已经知道自己暴露
了什么，JSX 层只需要把它提取出来。

```ts
export type NodeRef<T> =
  | ((ref: T | null, refs: Record<string, any>) => void)
  | Ref
  | string

declare const exposedType: unique symbol

export type ExtractExposed<
  Props,
  Default = never,
> = typeof exposedType extends keyof Props
  ? Exclude<Props[typeof exposedType], undefined>
  : Default

export type ExposedToProps<T extends Record<string, any>> =
  string extends keyof T
    ? {}
    : [keyof T] extends [never]
      ? {}
      : {
          readonly [exposedType]?: T
          readonly ref?: NodeRef<T>
        }
```

构造器组件会优先看 instance 上有没有 `exposed`。如果 exposed 是明确对象，`ref`
拿到的是 `UnwrapRef<Instance['exposed']>`；如果 exposed 太宽，则回退到完整组件实例。

直接函数组件则更直接：`ctx.expose()` 的参数就是 ref 目标。

```tsx
import { computed, type Ref } from 'vue'

const Doubler = (
  props: { count: number },
  { expose }: { expose: (exposed: { double: Ref<number> }) => void },
) => {
  const double = computed(() => props.count * 2)
  expose({ double })
  return <span>{props.count}</span>
}

;<Doubler
  count={2}
  ref={(exposed) => {
    exposed?.double.toFixed()
  }}
/>
```

callback 里看到的是 `{ double: number } | null`，因为这条类型链路经过了 `UnwrapRef`。

## Children 就是 Slots

slot 映射是 Vue 用户最关心的部分。JSX children 是语法，但 Vue children 是 slots。
Vue JSX 用 `ElementChildrenAttribute` 和 `SlotsToProps` 把两者接起来。

```ts
type ResolveSlots<Slots> = {
  readonly [Key in keyof Slots]?: Slots[Key] extends (
    ...args: infer Args
  ) => VNode | VNode[]
    ? (...args: Args) => NodeChild
    : Slots[Key]
}

export type SlotsToProps<
  RawSlots extends SlotsType | Record<string, any> = Record<string, any>,
  Slots = ResolveSlots<
    RawSlots extends SlotsType
      ? SetupContext<EmitsOptions, RawSlots>['slots']
      : RawSlots
  >,
> = string extends keyof Slots
  ? {}
  : [keyof Slots] extends [never]
    ? {}
    : {
        readonly 'v-slots'?:
          | ('default' extends keyof Slots ? Slots['default'] | Slots : Slots)
          | NoInfer<NodeChild>
      }
```

这段辅助类型做了几件细活：

1. 同时接受 Vue `SlotsType` 和普通 slots record。
2. 保留 slot 参数类型，但把返回值放宽到 Vue JSX 的 `NodeChild`。
3. 对开放索引或空 slots 返回 `{}`，避免未声明 slots 的组件变得很吵。
4. 建模默认 slot 的写法：有 `default` 时，调用者既可以直接把 default slot 函数写成
   children，也可以通过 `v-slots` 传完整 slots object。

```tsx
import { defineComponent } from 'vue-jsx'

const Panel = defineComponent(
  (
    props: { title: string },
    {
      slots,
    }: {
      slots: {
        default?: (scope: { active: boolean }) => JSX.Element
        footer?: (scope: { close: () => void }) => JSX.Element
      }
    },
  ) => {
    return () => (
      <section>
        <h2>{props.title}</h2>
        {slots.default?.({ active: true })}
      </section>
    )
  },
  { props: ['title'] },
)

;<Panel title="Settings">
  {({ active }) => <div>{active ? 'open' : 'closed'}</div>}
</Panel>
;<Panel
  title="Settings"
  v-slots={{
    default: ({ active }) => <div>{active}</div>,
    footer: ({ close }) => <button onClick={close}>Close</button>,
  }}
/>
```

`active` 和 `close` 都来自组件自己的 slots 类型。调用者不需要安装任何编辑器插件，
也没有写生成文件。

## 为什么不再需要编辑器插件

以前 JSX 类型经常依赖编辑器插件，是因为标准 TypeScript 在 JSX 调用点看不到足够多
Vue 语义。插件需要创建虚拟文件或额外类型上下文，让 props、emits、slots、refs 看起来
像 Vue。

Vue JSX 3.3 把这件事收进包声明里：

1. `jsxImportSource` 选择 `vue-jsx/jsx-runtime`。
2. runtime 暴露 `JSX` namespace。
3. TypeScript 向 `LibraryManagedAttributes<Component, Props>` 请求最终 attribute 类型。
4. Vue JSX 返回普通 props，加上生成的 `onXxx`、typed `ref`、以及通过 `v-slots`
   建模的 children slots。

宏和编辑器语法增强仍然可以作为可选能力存在，但它们不是默认类型方案。对普通组件编写
来说，类型真相来自组件自己的 TypeScript 类型，推断发生在原生 TypeScript checker 里。
