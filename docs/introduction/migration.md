# Migration

## Migration from `vue-jsx`

1. Use `defineVaporComponent` instead of `defineComponent` to define Vapor components. The setup function of `defineVaporComponent` now returns a JSX expression directly—there is no need to return a function anymore.
2. Hyphenated prop names and hyphenated component names are not converted to camelCase prop names.
3. `v-model` does not support array expressions. Use `v-model:$name$_trim={foo}` instead.
4. The `v-models` directive is not supported.
5. Destructuring props:

> [!CAUTION]
> ❌ The destructuring of props in a functional component will cause loss of reactivity.

```tsx
function Comp({ foo }) {
  return <div>{foo}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo.value} />
}
```

#### Two Solutions

1. ✅ Pass a ref variable as a prop:

```tsx
function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo} />
}
```

2. ✅ Set the macros option to true, then use the `defineVaporComponent` macro for wrapping.

  - Setup

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

  - Usage

    ```tsx
    import { defineVaporComponent, ref } from 'vue'

    const Comp = defineVaporComponent(({ foo }) => {
      return <>{foo}</>
    })
    // Will be converted to:
    const Comp = defineVaporComponent((_props) => {
      return <>{_props.foo}</>
    }, { props: ['foo'] })

    export default () => {
      const foo = ref('foo')
      return <Comp foo={foo.value} />
    }
    ```

## Migration from `react`

Suggest using the ESLint plugin [eslint-plugin-react2vue](https://github.com/zhiyuanzmj/eslint-plugin-react2vue) for converting the React Hooks API to the Vue Composition API and Macros.

### useState

```ts
// before
const [foo, setFoo] = useState(count)
console.log([foo, setFoo(1), setFoo])

// after
const foo = ref(0)
console.log([foo.value, foo.value = 1, val => foo.value = val])
```

### useEffect

Use `watchEffect` instead of `useEffect`.

```ts
// before
useEffect(() => {
  console.log(foo)
}, [foo])

// after
watchEffect(() => {
  console.log(foo)
})
```

### useMemo

Use `computed` instead of `useMemo`.

```ts
// before
const double = useMemo(() => foo * 2, [foo])
console.log({ double }, [double])

// after
const double = computed(() => foo * 2)
console.log({ double: double.value }, [double.value])
```

### defineVaporComponent

Use `defineVaporComponent` macro to support destructuring props.

```tsx
// before
const Comp = ({ count = 1 }) => {
  return <div>{count}</div>
}

// after
const Comp = defineVaporComponent(({ count = 1 }) => {
  return <div>{count}</div>
})
```

### defineSlots

Use `defineSlots` instead of `children` prop.

```tsx
// before
const Comp = ({ children }) => {
  return children
}

// after
const Comp = ({ children }) => {
  const slots = defineSlots()
  return <slots.default />
}
```

### useCallback

Remove useCallback.

```ts
// before
const callback = useCallback(() => {
  console.log(foo)
}, [foo])

// after
const callback = () => {
  console.log(foo)
}
```

### forwardRef

Remove forwardRef.

```tsx
// before
const Comp = forwardRef(({ count }, ref) => {
  return <div>{count}</div>
})

// after
const Comp = ({ count }) => {
  return <div>{count}</div>
}
```

### useImperativeHandle

Use `defineExpose` instead of `useImperativeHandle`.

```tsx
// before
const Comp = ({ count, ref }) => {
  useImperativeHandle(ref, () => {
    return {
      count: count * 2
    }
  }, [count])
  return <div>{count}</div>
}

// after
const Comp = ({ count }) => {
  defineExpose(computed(() => {
    return {
      count: count * 2
    }
  }))
  return <div>{count}</div>
}
```
