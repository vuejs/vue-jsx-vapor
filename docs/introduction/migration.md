# Migration Guide

## Migrating from `vue-jsx`

### Key Differences

1. **Component Definition**: Use `defineVaporComponent` instead of `defineComponent` for Vapor components. Unlike `defineComponent`, the setup function in `defineVaporComponent` returns JSX directly rather than a render function.

2. **Naming Conventions**: Hyphenated prop names and component names are not automatically converted to camelCase. Use camelCase naming consistently throughout your codebase.

3. **`v-model` Syntax**: Array expressions are not supported with `v-model`. Use the explicit modifier syntax instead:
    ```tsx
      <>
        {/* ❌ Not supported */}
        <input v-model={[foo, ['trim']]} />
        
        {/* ✅ Use this instead */}
        <input v-model:$name$_trim={foo} />
      </>
    ```

4. **`v-models` Directive**: The `v-models` directive is not supported. Define multiple `v-model` bindings separately.

5. **Props Destructuring**:

> [!CAUTION]
> Destructuring props in a functional component breaks reactivity, as the destructured values become static snapshots.

```tsx
function Comp({ foo }) {
  return <div>{foo}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo.value} />
}
```

#### Solutions

**Option 1**: Pass the ref directly instead of its value:

```tsx
function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref('foo')
  return <Comp foo={foo} />
}
```

**Option 2**: Enable macros and wrap your component with `defineVaporComponent`:

- Configuration

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
  // Compiles to:
  const Comp = defineVaporComponent((_props) => {
    return <>{_props.foo}</>
  }, { props: ['foo'] })

  export default () => {
    const foo = ref('foo')
    return <Comp foo={foo.value} />
  }
  ```

## Migrating from React

For automated migration, consider using [eslint-plugin-react2vue](https://github.com/zhiyuanzmj/eslint-plugin-react2vue), which transforms React Hooks API to Vue Composition API equivalents.

### `useState` → `ref`

```ts
// React
const [foo, setFoo] = useState(count)
console.log([foo, setFoo(1), setFoo])

// Vue
const foo = ref(0)
console.log([foo.value, foo.value = 1, val => foo.value = val])
```

### `useEffect` → `watchEffect`

```ts
// React
useEffect(() => {
  console.log(foo)
}, [foo])

// Vue
watchEffect(() => {
  console.log(foo)
})
```

### `useMemo` → `computed`

```ts
// React
const double = useMemo(() => foo * 2, [foo])
console.log({ double }, [double])

// Vue
const double = computed(() => foo * 2)
console.log({ double: double.value }, [double.value])
```

### Functional Components → `defineVaporComponent`

The `defineVaporComponent` macro enables props destructuring with full reactivity:

```tsx
// React
const Comp = ({ count = 1 }) => {
  return <div>{count}</div>
}

// Vue
const Comp = defineVaporComponent(({ count = 1 }) => {
  return <div>{count}</div>
})
```

### `children` → `defineSlots`

Replace the `children` prop pattern with Vue's slot system:

```tsx
// React
const Comp = ({ children }) => {
  return children
}

// Vue
const Comp = () => {
  const slots = defineSlots()
  return <slots.default />
}
```

### `useCallback`

Vue's reactivity system eliminates the need for `useCallback`. Simply define your functions directly:

```ts
// React
const callback = useCallback(() => {
  console.log(foo)
}, [foo])

// Vue
const callback = () => {
  console.log(foo)
}
```

### `forwardRef`

Vue handles ref forwarding automatically. Remove the `forwardRef` wrapper:

```tsx
// React
const Comp = forwardRef(({ count }, ref) => {
  return <div>{count}</div>
})

// Vue
const Comp = ({ count }) => {
  return <div>{count}</div>
}
```

### `useImperativeHandle` → `defineExpose`

```tsx
// React
const Comp = ({ count, ref }) => {
  useImperativeHandle(ref, () => {
    return {
      count: count * 2
    }
  }, [count])
  return <div>{count}</div>
}

// Vue
const Comp = ({ count }) => {
  defineExpose(computed(() => {
    return {
      count: count * 2
    }
  }))
  return <div>{count}</div>
}
```
