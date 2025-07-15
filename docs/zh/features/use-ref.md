# useRef

自动为 `useRef` 推断类型。它是 `shallowRef` 的别名。

## 基本用法

```tsx twoslash
import { defineComponent } from 'vue'
import { useRef } from 'vue-jsx-vapor'
// 或者
// import { shallowRef as useRef } from 'vue'

export const Comp = () => {
  defineExpose({
    foo: 1,
  })

  return <div />
}

export default defineComponent(() => {
  const comp = useRef()
  comp.value?.foo
  //           ^?

  return (
    <div>
      <Comp ref={comp} />
    </div>
  )
})
```
