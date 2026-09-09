import { ref } from 'vue'

export default () => {
  const count = ref(3)

  return (
    <>
      <button onClick={() => count.value++}>current: {count.value}</button>
      <div v-once>{count.value}</div>
    </>
  )
}
