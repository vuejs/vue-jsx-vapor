import { ref, shallowRef } from 'vue'
import type { EmitFn, Ref } from 'vue'

const Stepper = (
  props: { init: number },
  {
    emit,
    expose,
    slots,
  }: {
    emit: EmitFn<{ change: [value: number] }>
    expose: (exposed: { reset: () => void }) => void
    slots: { default?: (scope: { value: Ref<number> }) => any }
  },
) => {
  const value = ref(props.init)
  expose({ reset: () => (value.value = props.init) })

  return (
    <p class="count">
      <button onClick={() => emit('change', --value.value)}>-</button>
      {slots.default?.({ value })}
      <button onClick={() => emit('change', ++value.value)}>+</button>
    </p>
  )
}

export default () => {
  const stepper = shallowRef<{ reset: () => void } | null>(null)
  const last = ref(1)

  return (
    <section class="demo">
      <Stepper ref={(e) => (stepper.value = e)} init={1} onChange={(value) => (last.value = value)}>
        {({ value }) => <b>{value.value}</b>}
      </Stepper>
      <p class="output">last change: {last.value}</p>
      <button onClick={() => stepper.value?.reset()}>reset</button>
    </section>
  )
}
