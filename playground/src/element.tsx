/* eslint-disable unused-imports/no-unused-vars */
/* eslint-disable @typescript-eslint/no-unused-expressions */
/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { computed, defineComponent, defineVaporComponent } from 'vue'

const Comp = () => {
  const A = <a href=""></a>
  A.href = '#foo'
  return A
}
const comp = <Comp />
comp.block.href

const VaporComp = defineVaporComponent((props: { id: number }) => {
  defineSlots({
    default: (props: { id: 1 }) => [],
  })
  defineExpose({
    id: computed(() => 1),
  })
  return <div>{props.id}</div>
})
const vaporComp = <VaporComp id={1} v-slot={{ id }} />
vaporComp.props.id
vaporComp.exposeProxy?.id === ({} as number)
vaporComp.block.style

const VDomComp = defineComponent((props: { id: number }) => {
  defineSlots({
    default: (props: { id: 1 }) => <div />,
  })
  return () => <div>{props.id}</div>
})
const vdompComp = <VDomComp id={1} v-slot={{}} />
vdompComp.props
