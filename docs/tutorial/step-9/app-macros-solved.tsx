import { ref } from 'vue'

const Comp = () => {
  const slots = defineSlots({
    default: () => <>Fallback content</>,
  })
  return <slots.default />
}

export default () => {
  const msg = ref('from parent')
  return <Comp>{msg.value}</Comp>
}
