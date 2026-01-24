import { ref, VaporTransition } from 'vue'

export default () => {
  const show = ref(false)
  return [
    <button onClick={() => (show.value = !show.value)}>Toggle</button>,
    <VaporTransition
      appear
      onBeforeEnter={() => {
        console.error('should not trigger')
      }}
    >
      <div v-show={show.value}>1</div>
    </VaporTransition>,
    <VaporTransition
      appear
      onBeforeEnter={() => {
        console.info('should trigger')
      }}
    >
      <div v-show={!show.value}>2</div>
    </VaporTransition>,
  ]
}

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
