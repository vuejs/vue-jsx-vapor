import { defineComponent, ref } from 'vue'

const Comp = defineComponent(() => {
  const slots = defineSlots({
    default: () => <>Fallback content</>,
  })
  return () => <slots.default />
})

export default defineComponent(() => {
  const msg = ref('from parent')
  return () => <Comp>{msg.value}</Comp>
})
