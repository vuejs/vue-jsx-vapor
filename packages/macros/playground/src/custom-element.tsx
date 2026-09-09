import { defineVaporCustomElement } from 'vue'

const Comp = defineVaporCustomElement(() => {
  return (
    <div>
      <slot name="name">default</slot>
    </div>
  )
})

export default () => {
  !customElements.get('ce-comp') && customElements.define('ce-comp', Comp)
  return (
    <>
      <ce-comp></ce-comp>
      <ce-comp v-for={i in [1, 2]}>
        <div slot="name">name</div>
      </ce-comp>
    </>
  )
}
