import { ref } from 'vue'

export default () => {
  const show = ref(false)
  return (
    <>
      <input
        checked={show.value}
        type="checkbox"
        onChange={(event) => (show.value = event.currentTarget.checked)}
      />
      <span style={{ display: show.value ? '' : 'none' }}>
        {String(show.value)}
      </span>
    </>
  )
}
