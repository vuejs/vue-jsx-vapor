import { computed, defineComponent, shallowRef } from 'vue'

const Comp = defineComponent({
  props: ['count'],
  setup: (props: { count: number }) => {
    const double = computed(() => props.count * 2)
    return {
      double,
    }
  },
})

export default defineComponent(() => {
  const compRef = shallowRef<InstanceType<typeof Comp>>()
  return () => (
    <>
      <Comp count={1} />
      {compRef.value?.double}
    </>
  )
})
