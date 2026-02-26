import { computed, defineVaporComponent, Fragment, ref, shallowRef } from 'vue'
import { h } from 'vue-jsx-vapor'

const Comp = defineVaporComponent(
  (props: { type: 'input' | 'select' }, { attrs }: any) => {
    const compRef = shallowRef()
    return h(Fragment, null, [
      // 1. JSX
      <>
        {(() => {
          const DynamicComp = computed(() => `el-${props.type}`)
          return <DynamicComp.value />
        })()}
      </>,
      // 2. HyperScript
      h(
        `el-input`,
        {
          id: '123', // id="123"
          type: () => props.type, // id={type.value}
          ref: (e) => (compRef.value = e), // ref={compRef}
          key: () => attrs.foo, // key={props.foo}
          onClick: () => alert(1), // onClick={() => alert(1)}
          $: [() => attrs], // {...attrs}
        },
        [`default slot:`, () => props.type], // 1. default slot without scopes
        // (slotProps) => [() => `default slot: ${slotProps.type}`], // 2. default slot with scopes
        // { default: (slotProps) => 'default slot' }, // 3. multiple slots
      ),
    ])
  },
  {
    props: {
      type: { type: String, default: 'input' },
    },
    components: {
      ElInput: (props: { type: string }) => (
        <div>
          input: <slot type={props.type} />
        </div>
      ),
      ElSelect: () => <div>select</div>,
    },
  },
)

export default () => {
  const type = ref('input')
  return (
    <>
      <input v-model={type.value} />
      <Comp type={type.value}></Comp>
    </>
  )
}
