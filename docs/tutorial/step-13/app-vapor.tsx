import { shallowRef } from 'vue'

const Foo = () => <div>Foo</div>
const Bar = () => <div>Bar</div>

export default () => {
  const DynamicComponent = shallowRef(Foo)
  const toggle = () => {
    DynamicComponent.value = DynamicComponent.value === Foo ? Bar : Foo
  }
  return (
    <>
      <button onClick={toggle}>Toggle</button>
      <DynamicComponent.value />
    </>
  )
}
