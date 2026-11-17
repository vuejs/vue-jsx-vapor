import { defineComponent, ref } from 'vue'

const Comp = defineComponent((props: { modelValue: string }) => {
  const model = defineModel<string>()
  return () => (
    <input
      value={model.value}
      onInput={(e) => (model.value = (e.target as HTMLInputElement).value)}
    />
  )
})

export default defineComponent(() => {
  const msg = ref('Hello')
  return () => (
    <>
      <Comp modelValue={msg.value} onUpdate:modelValue={(v: string) => (msg.value = v)} />
      <p>{msg.value}</p>
    </>
  )
})
