import { ref } from 'vue'

const Comp = (props: { modelValue: string; 'onUpdate:modelValue': (v: string) => void }) => {
  return (
    <input
      value={props.modelValue}
      onInput={(e) => props['onUpdate:modelValue']((e.target as HTMLInputElement).value)}
    />
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
