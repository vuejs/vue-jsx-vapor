# 迁移指南

## 从 `vue-jsx` 迁移

1. 使用 `defineVaporComponent` 替代 `defineComponent` 来定义 Vapor 组件。`defineVaporComponent` 的 setup 函数现在可以直接返回 JSX 表达式，无需再返回一个函数。
2. 连字符的 prop 名称和组件名称不会被转换成驼峰命名。
3. `v-model` 不支持数组表达式，请改用 `v-model:$name$_trim={foo}`。
    ```tsx
      <>
        {/* ❌ Not supported */}
        <input v-model={[foo, ['trim']]} />
        
        {/* ✅ Use this instead */}
        <input v-model:$name$_trim={foo} />
      </>
    ```
4. 不支持 `v-models` 指令。
5. 解构 props：

> [!CAUTION]
> ❌ 在函数式组件中解构 props 会导致响应性丢失。

```tsx
function Comp({ foo }) {
  return <div>{foo}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo.value} />
}
```

#### 两种解决方案

1. ✅ 将 ref 变量作为 prop 传递：

```tsx
function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo} />
}
```

2. ✅ 将 `macros` 选项设置为 `true`，然后使用 `defineVaporComponent` 宏进行包装。

  - 配置

    ```ts {7}
    // vite.config.ts
    import vueJsxVapor from 'vue-jsx-vapor/vite'

    export default defineConfig({
      plugins: [
        vueJsxVapor({
          macros: true,
        }),
      ]
    })

    ```

  - 用法

    ```tsx
    import { defineVaporComponent, ref } from 'vue'

    const Comp = defineVaporComponent(({ foo }) => {
      return <>{foo}</>
    })
    // 将被转换为：
    const Comp = defineVaporComponent((_props) => {
      return <>{_props.foo}</>
    }, { props: ['foo'] })

    export default () => {
      const foo = ref('foo')
      return <Comp foo={foo.value} />
    }
    ```

## 从 `react` 迁移

建议使用 ESLint 插件 [eslint-plugin-react2vue](https://github.com/zhiyuanzmj/eslint-plugin-react2vue) 将 React Hooks API 转换为 Vue 组合式 API 和宏。

### useState

```ts
// 转换前
const [foo, setFoo] = useState(count)
console.log([foo, setFoo(1), setFoo])

// 转换后
const foo = ref(0)
console.log([foo.value, foo.value = 1, val => foo.value = val])
```

### useEffect

使用 `watchEffect` 替代 `useEffect`。

```ts
// 转换前
useEffect(() => {
  console.log(foo)
}, [foo])

// 转换后
watchEffect(() => {
  console.log(foo)
})
```

### useMemo

使用 `computed` 替代 `useMemo`。

```ts
// 转换前
const double = useMemo(() => foo * 2, [foo])
console.log({ double }, [double])

// 转换后
const double = computed(() => foo * 2)
console.log({ double: double.value }, [double.value])
```

### defineVaporComponent

使用 `defineVaporComponent` 宏来支持解构 props。

```tsx
// 转换前
const Comp = ({ count = 1 }) => {
  return <div>{count}</div>
}

// 转换后
const Comp = defineVaporComponent(({ count = 1 }) => {
  return <div>{count}</div>
})
```

### defineSlots

使用 `defineSlots` 替代 `children` prop。

```tsx
// 转换前
const Comp = ({ children }) => {
  return children
}

// 转换后
const Comp = ({ children }) => {
  const slots = defineSlots()
  return <slots.default />
}
```

### useCallback

移除 `useCallback`。

```ts
// 转换前
const callback = useCallback(() => {
  console.log(foo)
}, [foo])

// 转换后
const callback = () => {
  console.log(foo)
}
```

### forwardRef

移除 `forwardRef`。

```tsx
// 转换前
const Comp = forwardRef(({ count }, ref) => {
  return <div>{count}</div>
})

// 转换后
const Comp = ({ count }) => {
  return <div>{count}</div>
}
```

### useImperativeHandle

使用 `defineExpose` 替代 `useImperativeHandle`。

```tsx
// 转换前
const Comp = ({ count, ref }) => {
  useImperativeHandle(ref, () => {
    return {
      count: count * 2
    }
  }, [count])
  return <div>{count}</div>
}

// 转换后
const Comp = ({ count }) => {
  defineExpose(computed(() => {
    return {
      count: count * 2
    }
  }))
  return <div>{count}</div>
}
```
