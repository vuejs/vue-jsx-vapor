import { defineComponent, ref } from 'vue'

const Comp = defineComponent(({ foo }) => {
  return () => <div>{foo}</div>
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
