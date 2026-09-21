import { ref } from 'vue'

export default () => {
  const count = ref(1)
  const Foo = () => <div style="color: red">2</div>

  return (
    <div>
      <button onClick={() => count.value++}>+</button>
      <button onClick={() => count.value--}>-</button>
      {count.value === 1 ? (
        <div>{count.value}</div>
      ) : count.value === 2 ? (
        <Foo />
      ) : count.value >= 3 ? (
        <div>lg 3: {count.value}</div>
      ) : (
        <div>lt 0: {count.value}</div>
      )}
    </div>
  )
}
