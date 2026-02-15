# useRef

A utility function that provides automatic type inference for component refs. It serves as an alias for `shallowRef`.

## Basic Usage

```tsx twoslash
import { defineVaporComponent } from 'vue'
import { useRef } from 'vue-jsx-vapor'
// or
// import { shallowRef as useRef } from 'vue'

export const Comp = () => {
  defineExpose({
    foo: 1
  })

  return <div />
}

export default defineVaporComponent(() => {
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
