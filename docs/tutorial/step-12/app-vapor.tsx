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
      <Comp modelValue={msg.value} onUpdate:modelValue={(v) => (msg.value = v)} />
      <p>{msg.value}</p>
    </>
  )
}
