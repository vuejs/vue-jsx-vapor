import { ref, type Ref, type SlotsType } from 'vue'
import { defineComponent, defineVaporComponent, useRef } from 'vue-jsx-vapor'

const Comp = defineVaporComponent(
  (
    props,
    {
      slots,
      expose,
    }: {
      slots: {
        default?: (props: { foo: number }) => any
      }
      expose: (exposed: { baz: Ref<number> }) => void
    },
  ) => {
    expose({ baz: ref(1) })
    return slots.default?.({ foo: 1 })
  },
)

const Comp1 = defineComponent(
  (
    props: { foo: number },
    {
      slots,
      expose,
    }: {
      slots: {
        default?: (props: { foo: number }) => any
        other?: (props: { foo: number }) => any
      }
      expose: (exposed: { bar: Ref<number> }) => void
    },
  ) => {
    expose({ bar: ref(123) })
    return () => <div>{slots.default?.({ foo: props.foo })}</div>
  },
)

const Comp2 = defineComponent({
  slots: Object as SlotsType<{
    default?: (props: { foo: number }) => any
    other?: (props: { foo: number }) => any
  }>,
  setup: (props: { foo: number }, { slots }) => {
    return () => <div>{slots.default?.({ foo: props.foo })}</div>
  },
})

const Comp3 = () => <div></div>

export default () => {
  const foo = 1

  const exposed = useRef<InstanceType<typeof Comp>['exposed']>()
  const exposed1 = useRef<InstanceType<typeof Comp1>>()
  console.log(exposed.value!.baz, exposed1.value!.bar)
  return (
    <>
      <Comp>{1}</Comp>
      <Comp1
        v-model:foo={foo}
        foo={foo}
        onUpdate:foo={() => {}}
        v-slots={{
          default: ({ foo }) => [foo],
          other: ({ foo }) => [foo],
        }}
      ></Comp1>
      <Comp2 foo={1}>
        {{
          default: ({ foo }) => <div>{foo}</div>,
          other: ({ foo }) => <div>{foo}</div>,
        }}
      </Comp2>
      <Comp3>foo</Comp3>
    </>
  )
}
