import { defineComponent, ref } from 'vue'

const Comp = defineComponent({
  props: ['modelValue'],
  emits: ['update:modelValue'],
  setup(props: { modelValue: string }, { emit }) {
    return () => (
      <input
        value={props.modelValue}
        onInput={(e) => emit('update:modelValue', (e.target as HTMLInputElement).value)}
      />
    )
  },
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
