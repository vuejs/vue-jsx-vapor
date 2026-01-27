import { defineComponent, defineCustomElement, ref, useAttrs } from 'vue'

const Comp = defineCustomElement(() => {
  return () => (
    <div>
      <slot name="name">default</slot>
    </div>
  )
})

export default defineComponent(() => {
  !customElements.get('ce-comp') && customElements.define('ce-comp', Comp)
  return () => (
    <>
      <ce-comp></ce-comp>
      <ce-comp v-for={i in [1, 2]}>
        <div slot="name">bar</div>
      </ce-comp>
    </>
  )
})
