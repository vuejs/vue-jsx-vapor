import { computed, shallowRef, type Ref, type UnwrapRef } from 'vue'

const Comp = (
  props: { count: number },
  { expose }: { expose: (exposed: { double: Ref<number> }) => any },
) => {
  const double = computed(() => props.count * 2)
  expose({
    double,
  })
  return []
}

export default () => {
  const compRef =
    shallowRef<UnwrapRef<Parameters<Parameters<typeof Comp>[1]['expose']>[0]>>()
  return (
    <>
      <Comp count={1} />
      {compRef.value?.double}
    </>
  )
}
