import {
  computed,
  defineVaporComponent,
  Fragment,
  ref,
  shallowRef,
  type EmitFn,
  type VaporComponentInstance,
} from 'vue'
import { h } from 'vue-jsx/vapor'

function expectType<T>(_value: T): void {}

const Comp = defineVaporComponent({
  setup: (
    props: { type: 'input' | 'select' | (string & {}) },
    {
      attrs,
      expose,
    }: {
      attrs: any
      slots: { default: (props: { id: 1 }) => [] }
      expose: (exposed: { foo: 1 }) => void
    },
  ) => {
    expose({ foo: 1 })
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
})

const CompFn = defineVaporComponent(
  (
    props: { foo: 1 },
    {
      slots,
    }: {
      emit: EmitFn<{ foo: [1] }>
      slots: { default: (props: { foo: 1 }) => [] }
      expose: (exposed: { foo: 1 }) => void
    },
  ) => {
    return <div>Comp Fn{slots.default?.(props)}</div>
  },
)

const FnComp = (
  props: { foo: 1 },
  {
    slots,
  }: {
    emit: EmitFn<{ foo: [1] }>
    expose: (exposed: { foo: 1 }) => void
    slots: { default: (props: { foo: 1 }) => [] }
  },
) => {
  return slots.default?.(props)
}

export default defineVaporComponent(() => {
  const type = ref('input')
  return (
    <>
      <input v-model={type.value} />
      <Comp ref={(e) => e} type="input"></Comp>
      {h(
        Comp,
        {
          type: () => type.value,
          // @ts-expect-error should error
          type1: () => type.value,
          ref: (e) => expectType<VaporComponentInstance | null>(e),
        },
        {
          default: (props) => [expectType<1>(props.id)],
        },
      )}
      {h(
        CompFn,
        {
          foo: 1,
          ref: (exposed) => expectType<1 | undefined>(exposed?.foo),
        },
        {
          default: (props) => () => [() => expectType<1>(props.foo)],
        },
      )}
      {h(
        FnComp,
        {
          foo: 1,
          onFoo: (e) => {
            expectType<1>(e)
          },
          ref: (exposed) => expectType<1 | undefined>(exposed?.foo),
        },
        { default: (props) => [expectType<1>(props.foo)] },
      )}
      {h('input', {
        ref: (element) => expectType<HTMLInputElement | null>(element),
        onClick: (event) => expectType<MouseEvent>(event),
      })}
    </>
  )
})
