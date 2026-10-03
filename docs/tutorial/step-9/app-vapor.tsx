import { ref } from 'vue'

const Comp = (props, { slots }) => {
  return <>{slots.default ? <slots.default /> : 'Fallback content'}</>
}

export default () => {
  const msg = ref('from parent')
  return <Comp>{/* ... */}</Comp>
}
