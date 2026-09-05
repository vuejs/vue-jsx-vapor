import { ref } from 'vue'

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
    </div>
  )
}
