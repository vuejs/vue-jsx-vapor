import { ref } from 'vue'
import { defineComponent } from 'vue-jsx'

const Button = defineComponent(<T,>(
  props: { offset: T },
  {
    slots,
  }: {
    slots: {
      default?: (props: { offset: T }) => any
    }
  },
) => {
  return () => slots.default?.(props)
}, { props: ['offset'] })

export default defineComponent(() => {
  const count = ref(1)
  const Output = () => <div class="output"><slot /></div>
  let foo = 1
  let bar = 1

  return () => {
    const offset = 2 as const

    return (
      <main class="demo">
        <div class="count">count: {count.value}</div>
        <Button offset={offset}>
          {({ offset }) => (
            <button onClick={() => count.value += offset}>
              + {offset}
            </button>
          )}
        </Button>
        <Output>
          {() => [
            'should update: ',
            bar,
            ' + ',
            offset,
            ' = ',
            (bar += offset),
          ]}
        </Output>
        <Output>{() => ['should not update: ', foo++]}</Output>
      </main>
    )
  }
})
