import { defineVaporComponent, ref } from 'vue'

const Comp = defineVaporComponent(({ foo }) => {
  return <div>{foo}</div>
})

export default defineVaporComponent(() => {
  const foo = ref(1)
  return (
    <>
      <button onClick={() => foo.value++}>+</button>
      <Comp foo={foo.value} />
    </>
  )
})
