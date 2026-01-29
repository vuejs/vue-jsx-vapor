import { defineComponent, defineVaporComponent } from 'vue'

const Comp = defineComponent(() => {
  return () => {
    return <slot foo></slot>
  }
})

const Comp1 = () => {
  return <slot foo={1}>foo</slot>
}

export default defineComponent(() => {
  return () => (
    <>
      <Comp v-slot={{ foo }}>{foo}</Comp>
      <Comp1 v-slot={{ foo }}>{foo}</Comp1>
    </>
  )
})
