import { ref } from 'vue'

const Comp = () => {
  const model = defineModel<string>()
  return <input v-model={model.value} />
}

export default () => {
  const model = ref('model')

  return (
    <>
      <input v-model={model.value}></input>
      <Comp v-model={model.value} />
      {model.value}
    </>
  )
}
