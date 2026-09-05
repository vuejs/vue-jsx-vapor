import { defineComponent } from 'vue'

const Comp = defineComponent(
  (_props, { slots }: { slots: { default?: (props: { foo: true }) => any } }) =>
    () =>
      slots.default?.({ foo: true }) ?? <div>fallback</div>,
)

const Comp1 = (
  _props: {},
  { slots }: { slots: { default?: (props: { foo: 1 }) => any } },
) => slots.default?.({ foo: 1 }) ?? <div>fallback</div>

export default defineComponent(() => () => (
  <>
    <Comp
      v-slots={{
        default: ({ foo }: { foo: true }) => <div>{String(foo)}</div>,
      }}
    />
    <Comp1>{({ foo }: { foo: 1 }) => <div>{foo}</div>}</Comp1>
  </>
))
