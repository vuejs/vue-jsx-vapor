import { defineComponent } from 'vue'

const Comp = defineComponent((props, { slots }) => {
  return () => <slots.default foo="from child" />
})

export default defineComponent(() => {
  return () => <Comp>{(slotProps) => <div>{slotProps.foo}</div>}</Comp>
})
