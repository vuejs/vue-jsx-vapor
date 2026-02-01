import { ref, VaporTransition } from 'vue'

export default () => {
  const show = ref(false)
  const count = ref(1)
  return [
    <button onClick={() => count.value++}>+</button>,
    <button onClick={() => count.value--}>-</button>,
    <VaporTransition>
      <div v-if={count.value === 1}>1</div>
      <div v-else-if={count.value === 2}>2</div>
      <div v-else>3</div>
    </VaporTransition>,
    <VaporTransition mode="out-in">
      <div v-if={count.value === 1}>1</div>
      <div v-else-if={count.value === 2}>2</div>
      <div v-else>3</div>
    </VaporTransition>,

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
    transition: opacity .5s ease;
  }

  .v-enter-from,
  .v-leave-to {
    opacity: 0;
  }
`)
