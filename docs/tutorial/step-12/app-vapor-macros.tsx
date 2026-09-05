import { ref } from 'vue'

const Comp = (props: { modelValue: string }) => {
  const model = defineModel<string>()
  return (
    <input
      value={model.value}
      onInput={(e) => (model.value = e.currentTarget.value)}
    />
  )
}

export default () => {
  const msg = ref('Hello')
  return (
    <>
      <Comp modelValue={msg.value} onUpdate:modelValue={(v) => (msg.value = v)} />
      <p>{msg.value}</p>
    </>
  )
}
