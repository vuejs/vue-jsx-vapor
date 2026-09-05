import { defineComponent, ref } from 'vue'

const Comp = (
  props: { model: string },
  { slots }: { slots: { default?: (props: { foo: number }) => any } },
) => (
  <div>
    Virtual DOM Component: {props.model}
    {slots.default?.({ foo: 1 })}
  </div>
)

export default defineComponent(() => {
  const model = ref('')
  return () => (
    <>
      <input
        value={model.value}
        onInput={(event) => (model.value = event.currentTarget.value)}
      />
      <Comp
        model={model.value}
        v-slots={{ default: ({ foo }) => <div>{foo}</div> }}
      />
    </>
  )
})
