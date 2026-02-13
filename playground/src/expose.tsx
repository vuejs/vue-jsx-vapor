import { computed, ref, type Ref } from 'vue'
import { useRef } from 'vue-jsx-vapor'

export const Comp = (
  props: { foo: number },
  { expose }: { expose: (exposed: { double: Ref<number> }) => any },
) => {
  const double = computed(() => props.foo * 2)
  expose?.({ double })
  return <span>{props.foo} x 2 = </span>
}

export default () => {
  const compRef = useRef()
  const foo = ref(1)
  return (
    <>
      <button onClick={() => foo.value++}>+</button>
      <br />
      <Comp ref={compRef} foo={foo.value}></Comp>
      {compRef.value?.double}
    </>
  )
}
