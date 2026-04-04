import { defineComponent, defineVaporComponent } from 'vue-jsx-vapor'
import type { SlotsType } from 'vue'

const Comp = defineVaporComponent({
  setup: (
    props,
    {
      slots,
    }: {
      slots: {
        default?: (props: { foo: number }) => any
      }
    },
  ) => {
    return slots.default?.({ foo: 1 })
  },
})

const Comp1 = defineComponent(
  (
    props: { foo: number },
    {
      slots,
    }: {
      slots: {
        default?: (props: { foo: number }) => any
        other?: (props: { foo: number }) => any
      }
    },
  ) => {
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
  return (
    <>
      <Comp>{1}</Comp>
      <Comp1
        foo={1}
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
