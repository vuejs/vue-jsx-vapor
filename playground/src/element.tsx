import { computed, defineComponent, defineVaporComponent, type Ref } from 'vue'

export const NativeComp = () => {
  return <a href="#foo">foo</a>
}

const VaporComp = defineVaporComponent(
  (
    props: { id: number },
    {
      slots,
      expose,
    }: {
      slots: { default?: (props: { id: 1 }) => any }
      expose: (value: { id: Ref<number> }) => void
    },
  ) => {
    expose({ id: computed(() => 1) })
    return slots.default?.({ id: 1 }) ?? <div>{props.id}</div>
  },
)

export const vaporComp = (
  <VaporComp id={1} v-slots={{ default: ({ id }) => <div>{id}</div> }} />
)

const VDomComp = defineComponent(
  (
    props: { id: number },
    { slots }: { slots: { default?: (props: { id: 1 }) => any } },
  ) =>
    () =>
      slots.default?.({ id: 1 }) ?? <div>{props.id}</div>,
)

export const vdomComp = (
  <VDomComp
    id={1}
    v-slots={{ default: ({ id }: { id: 1 }) => <div>{id}</div> }}
  />
)

export default VaporComp
