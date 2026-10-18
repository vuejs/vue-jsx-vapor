import { ref } from 'vue'

export default () => {
  const foo = ref('<div style="color: red;">foo</div>')

  return (
    <>
      <input v-model={foo.value} style="width: 100%" />

      <div v-html={foo.value} />
    </>
  )
}
