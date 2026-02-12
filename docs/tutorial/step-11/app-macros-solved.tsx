import { computed } from 'vue'
import { useRef } from 'vue-jsx-vapor'

const Comp = (props: { count: number }) => {
  const double = computed(() => props.count * 2)
  defineExpose({
    double,
  })
  return <span>{props.count} x 2 = </span>
}

export default () => {
  const compRef = useRef()
  return (
    <>
      <Comp ref={compRef} count={1} />
      {compRef.value?.double}
    </>
  )
}
