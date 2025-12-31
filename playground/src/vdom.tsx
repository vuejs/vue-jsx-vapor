import { defineComponent, ref } from 'vue'

const Comp = (props) => {
  const slots = defineSlots({
    default: (props: { foo: number }) => <div>{props.foo}</div>,
  })
  return (
    <div>
      Virtual Dom Component:{props.model}
      <slots.default foo={'foo'}></slots.default>
    </div>
  )
}

export default defineComponent({
  name: 'vdom',
  setup: () => {
    const model = ref()
    return { model }
  },
  render(data) {
    return (
      <>
        <input v-model={data.model}></input>
        {data.model && data.model}
        <Comp model={data.model}>
          {{ default: ({ foo }) => <div>{foo}</div> }}
        </Comp>
      </>
    )
  },
})
