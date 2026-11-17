import { ref } from 'vue'

export default () => {
  const count = ref(3)

  return (
    <>
      <input v-model_number={count.value} />
      <div v-once>{count.value}</div>
    </>
  )
}
