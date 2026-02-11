import { defineComponent, ref } from 'vue'

const Comp = defineComponent(
  ({ foo }) => {
    return () => <div>{foo.value}</div>
  },
  { props: ['foo'] },
)

export default defineComponent(() => {
  const foo = ref(1)
  return () => (
    <>
      <button onClick={() => foo.value++}>+</button>
      <Comp foo={foo} />
    </>
  )
})
