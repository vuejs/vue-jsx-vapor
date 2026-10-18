import { ref, watch } from 'vue'
import { defineVaporComponent, VaporFor } from 'vue-jsx'

const ForComp = defineVaporComponent(() => {
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
          const first = list.value[0]!
          list.value[0] = list.value.at(-1)!
          list.value[list.value.length - 1] = first
        }}
      >
        swap
      </button>
      <button
        onClick={() => {
          list.value = JSON.parse(JSON.stringify([...list.value]))
        }}
      >
        refresh
      </button>

      <fieldset>
        <legend>without key</legend>
        <VaporFor in={list.value}>
          {(item, index) => {
            return (
              <div>
                {item.id}.
                <input />
                <span onClick={() => list.value.splice(index.value, 1)}>x</span>
              </div>
            )
          }}
        </VaporFor>
      </fieldset>

      <fieldset>
        <legend>with key</legend>
        <VaporFor in={list.value} getKey={(item) => item.id}>
          {(item, index) => {
            return (
              <div>
                {item.value.id}.
                <input />
                <span onClick={() => list.value.splice(index.value, 1)}>x</span>
              </div>
            )
          }}
        </VaporFor>
      </fieldset>
    </>
  )
})

export default () => {
  const count = ref(3)
  const selected = ref(0)

  return (
    <div>
      <input
        type="number"
        value={count.value}
        onInput={(event) => (count.value = event.currentTarget.valueAsNumber)}
      />
      {Array.from({ length: count.value }, (_, id) => ({ id })).map((item) => (
        <div
          key={item.id}
          class={{ 'text-red': item.id === selected.value }}
          onClick={() => (selected.value = item.id)}
        >
          {item.id}
        </div>
      ))}

      <ForComp></ForComp>
    </div>
  )
}
