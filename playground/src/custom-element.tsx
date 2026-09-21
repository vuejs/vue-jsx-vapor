import { defineVaporCustomElement } from 'vue'

const Comp = defineVaporCustomElement(() => <slot name="name">default</slot>)

export default () => {
  !customElements.get('ce-comp') && customElements.define('ce-comp', Comp)
  return (
    <>
      <ce-comp />
      {[1, 2].map((id) => (
        <ce-comp key={id}>
          <div slot="name">name {id}</div>
        </ce-comp>
      ))}
    </>
  )
}
