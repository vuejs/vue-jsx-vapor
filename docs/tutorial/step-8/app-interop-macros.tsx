import { defineComponent, ref } from 'vue'

const Comp = defineComponent({
  setup: ({ foo }) => {
    return () => <div>{foo}</div>
  },
  props: ['foo'],
})

export default defineComponent(() => {
  const foo = ref(1)
  return () => (
    <>
      <button onClick={() => foo.value++}>+</button>
      <Comp foo={foo.value} />
    </>
  )
})
