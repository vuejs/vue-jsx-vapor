import { ref } from 'vue'

function Comp({ foo }) {
  return <div>{foo.value}</div>
}

export default () => {
  const foo = ref(1)
  return (
    <>
      <button onClick={() => foo.value++}>+</button>
      <Comp foo={foo} />
    </>
  )
}
