import { computed, defineComponent, shallowRef } from 'vue'

const Comp = defineComponent((props: { count: number }) => {
  const double = computed(() => props.count * 2)
  defineExpose({
    double,
  })
  return () => <span>{props.count} x 2 = </span>
})

export default defineComponent(() => {
  const compRef = shallowRef<InstanceType<typeof Comp>>()
  return () => (
    <>
      <Comp ref={compRef} count={1} />
      {compRef.value?.double}
    </>
  )
})
