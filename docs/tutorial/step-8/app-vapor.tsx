import { ref } from 'vue'

const Comp = ({ foo }) => {
  return <div>{foo}</div>
}

export default () => {
  const foo = ref(1)
  return (
    <>
      <button onClick={() => foo.value++}>+</button>
      <Comp foo={foo.value} />
    </>
  )
}
