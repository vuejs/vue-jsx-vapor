import { ref } from 'vue'

const Comp = (props: { modelValue: string }) => {
  const model = defineModel<string>()
  return (
    <input v-model={model.value} />
  )
}

export default () => {
  const msg = ref('Hello')
  return (
    <>
      <Comp v-model={msg.value} />
      <p>{msg.value}</p>
    </>
  )
}
