import { defineVaporComponent, ref } from 'vue'

export default defineVaporComponent(() => {
  const count = ref(1)

  const Foo = () => <div style="color: red">2</div>

  return (
    <div>
      <button onClick={() => count.value++}>+</button>
      <button onClick={() => count.value--}>-</button>

      <div style="display: flex;">
        <fieldset>
          <legend>expression</legend>
          <div>
            {count.value === 1 ? (
              <div>{1}</div>
            ) : count.value === 2 ? (
              <Foo />
            ) : count.value >= 3 ? (
              <div>lg 3: {count.value}</div>
            ) : (
              <div>lt 0: {count.value}</div>
            )}
          </div>
        </fieldset>

        <fieldset>
          <legend>v-if</legend>
          <div>
            <div v-if={count.value === 1} a b c d e>
              {count.value}
            </div>
            <Foo v-else-if={count.value === 2} />
            <div v-else-if={count.value >= 3}>lg 3: {count.value}</div>
            <div v-else>lt 0: {count.value}</div>
          </div>
        </fieldset>
      </div>
    </div>
  )
  defineStyle(`
    .foo {
      color: red;
    }
  `)
})
