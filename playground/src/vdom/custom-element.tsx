import { defineComponent, defineCustomElement } from 'vue'

const Comp = defineCustomElement(() => () => <slot name="name">default</slot>)

export default defineComponent(() => {
  !customElements.get('ce-comp') && customElements.define('ce-comp', Comp)
  return () => (
    <>
      <ce-comp />
      {[1, 2].map((id) => (
        <ce-comp key={id}>
          <div slot="name">bar {id}</div>
        </ce-comp>
      ))}
    </>
  )
})
