import { defineComponent, ref, Transition } from 'vue'

export default defineComponent(() => {
  const show = ref(false)
  return () => (
    <>
      <button onClick={() => (show.value = !show.value)}>Toggle</button>
      <Transition
        appear
        onBeforeEnter={() => {
          console.error('should not trigger')
        }}
      >
        <div v-show={show.value}>1</div>
      </Transition>
      <Transition
        appear
        onBeforeEnter={() => {
          console.info('should trigger')
        }}
      >
        <div v-show={!show.value}>2</div>
      </Transition>
    </>
  )
})

defineStyle(`
  .v-enter-active,
  .v-leave-active {
    transition: opacity 0.5s ease;
  }

  .v-enter-from,
  .v-leave-to {
    opacity: 0;
  }
`)
