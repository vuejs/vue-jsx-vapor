import { ref, watch } from 'vue'
import { defineVaporComponent, VaporFor } from 'vue-jsx-vapor'

export default defineVaporComponent(() => {
  const count = ref(1)
  const list = ref<{ id: number }[]>([])
  watch(
    count,
    () => {
      list.value.push({ id: count.value })
    },
    { immediate: true },
  )

  return (
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
      <VaporFor in={list.value}>
        {(item, index) => {
          return (
            <div key={item.id}>
              <input />
              <span onClick={() => list.value.splice(index.value, 1)}>x</span>
            </div>
          )
        }}
      </VaporFor>
    </>
  )
})
