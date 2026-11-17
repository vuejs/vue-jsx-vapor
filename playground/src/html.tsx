import { ref } from 'vue'

export default () => {
  const html = ref('<div style="color: red;">foo</div>')
  return (
    <>
      <input
        style="width: 100%"
        value={html.value}
        onInput={(event) => (html.value = event.currentTarget.value)}
      />
      <div innerHTML={html.value} />
    </>
  )
}
