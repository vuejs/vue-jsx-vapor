import { ref, VaporTransition } from 'vue'

export default () => {
  const show = ref(false)
  const count = ref(1)
  const content = () =>
    count.value === 1 ? (
      <div>1</div>
    ) : count.value === 2 ? (
      <div>2</div>
    ) : (
      <div>3</div>
    )

  return [
    <button onClick={() => count.value++}>+</button>,
    <button onClick={() => count.value--}>-</button>,
    <VaporTransition>{content()}</VaporTransition>,
    <button onClick={() => (show.value = !show.value)}>Toggle</button>,
    <VaporTransition appear>
      {show.value ? <div>visible</div> : null}
    </VaporTransition>,
  ]
}
