import { defineComponent, ref } from 'vue'

const Comp = defineComponent((props, { slots }) => {
  return () => <>{slots.default ? <slots.default /> : 'Fallback content'}</>
})

export default defineComponent(() => {
  const msg = ref('from parent')
  return () => <Comp>{/* ... */}</Comp>
})
