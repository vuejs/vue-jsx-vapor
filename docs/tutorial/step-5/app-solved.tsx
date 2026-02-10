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

      <h1 v-if={toggle.value}>true</h1>
      <h1 v-else>false</h1>
    </>
  )
}
