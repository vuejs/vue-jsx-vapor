import { ref } from 'vue'

export default () => {
  const toggle = ref(true)
  return (
    <>
      <button
        onClick={() => {
          toggle.value = !toggle.value
        }}
      >
        Toggle
      </button>

      <h1>true</h1>
      <h1>false</h1>
    </>
  )
}
