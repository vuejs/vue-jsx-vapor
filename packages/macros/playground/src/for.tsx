import { ref } from 'vue'

export default () => {
  const count = ref(3)

  const Arr = (
    <>
      {Array.from({ length: count.value }).map((_, index) => {
        if (index > 1) {
          return (
            <>
              <div>({index}) lg 1</div>
            </>
          )
        } else {
          return [<span>({index}) lt 1</span>, <br />]
        }
      })}
    </>
  )

  const selected = ref(0)
  return (
    <div>
      <input v-model_number={count.value} />

      <div style="display: flex;">
        <fieldset>
          <legend>map</legend>
          {Arr}
        </fieldset>

        <fieldset>
          <legend>v-for</legend>
          <div
            v-for={
              (i, index) in
              Array.from({ length: count.value }).map((_, id) => ({ id }))
            }
            key={i.id}
            class={{ 'text-red': i.id === selected.value }}
            onClick={() => (selected.value = i.id)}
          >
            {i.id}
          </div>
        </fieldset>
      </div>
    </div>
  )
}

defineStyle(`
  .text-red {
    color: red;
  }
`)
