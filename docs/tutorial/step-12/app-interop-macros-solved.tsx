import { defineComponent, ref } from 'vue'

const Comp = defineComponent((props: { modelValue: string }) => {
  const model = defineModel<string>()
  return () => (
    <input v-model={model.value} />
  )
})

export default defineComponent(() => {
  const msg = ref('Hello')
  return () => (
    <>
      <Comp v-model={msg.value} />
      <p>{msg.value}</p>
    </>
  )
})
