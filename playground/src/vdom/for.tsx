import { ref, watch } from 'vue'
import { defineComponent, For } from 'vue-jsx-vapor'

export default defineComponent(() => {
  const count = ref(1)
  const list = ref<{ id: number }[]>([])
  watch(
    count,
    () => {
      list.value.push({ id: count.value })
    },
    { immediate: true },
  )

  return () => (
    <>
      <button onClick={() => count.value++}>+</button>
      <button onClick={() => count.value--}>-</button>
      <button
        onClick={() => {
          const first = list.value[0]
          list.value[0] = list.value.at(-1)
          list.value[list.value.length - 1] = first
        }}
      >
        swap
      </button>
      <button
        onClick={() => {
          list.value = structuredClone(list.value)
        }}
      >
        refresh
      </button>

      <div></div>
      <For in={list.value}>
        {(item, index) => {
          return (
            <div key={item.id}>
              <input />
              <span onClick={() => list.value.splice(index, 1)}>x</span>
            </div>
          )
        }}
      </For>
    </>
  )
})
