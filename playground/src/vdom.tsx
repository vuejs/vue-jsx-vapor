import { defineComponent, ref } from 'vue'

const Comp = (props) => <div>Virtual Dom Component:{props.model}</div>

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
        <Comp model={data.model}></Comp>
      </>
    )
  },
})
