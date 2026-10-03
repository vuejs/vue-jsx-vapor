import { defineComponent, EmitsOptions, SetupContext, SlotsType } from 'vue'

const Comp = defineComponent(
  <T,>(
    props: { foo: T },
    {
      slots,
    }: SetupContext<
      EmitsOptions,
      SlotsType<{
        default: (props: { foo: T }) => any
      }>
    >,
  ) => {
    return () => <div>{slots.default(props)}</div>
  },
)

export default () => [<Comp foo={1}>{{ default: (props) => <>{props.foo}</> }}</Comp>]
