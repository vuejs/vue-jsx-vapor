import { defineComponent, ref, Transition } from 'vue'

export default defineComponent(() => {
  const show = ref(false)
  return () => (
    <>
      <button onClick={() => (show.value = !show.value)}>Toggle</button>
      <Transition appear>{show.value ? <div>visible</div> : null}</Transition>
    </>
  )
})
