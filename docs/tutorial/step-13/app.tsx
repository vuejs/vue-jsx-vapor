import { defineComponent, shallowRef } from 'vue'

const Foo = defineComponent(() => () => <div>Foo</div>)
const Bar = defineComponent(() => () => <div>Bar</div>)

export default defineComponent(() => {
  const DynamicComponent = shallowRef<typeof Foo | typeof Bar>(Foo)
  const toggle = () => {
    DynamicComponent.value = DynamicComponent.value === Foo ? Bar : Foo
  }
  return () => (
    <>
      <button onClick={toggle}>Toggle</button>
      <DynamicComponent.value />
    </>
  )
})
