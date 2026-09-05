import { defineComponent, defineVaporComponent, ref } from 'vue'

const VaporComp = (props: { model: string }) => (
  <div>Vapor Component: {props.model}</div>
)
const VDomComp = (props: { model: string }) => (
  <div>Virtual DOM Component: {props.model}</div>
)

const VDom = defineComponent(() => {
  const model = ref('')
  return () => [
    <input
      value={model.value}
      onInput={(event) => (model.value = event.currentTarget.value)}
    />,
    <VDomComp model={model.value} />,
    <VaporComp model={model.value} />,
  ]
})

export default defineVaporComponent(() => <VDom />, { name: 'interop' })
