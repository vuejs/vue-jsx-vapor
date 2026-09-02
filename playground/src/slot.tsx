import { ref } from 'vue'

type Slots = {
  default?: (scope: { foo: string }) => any
}

const Comp = (props: { foo: string }, { slots }: { slots: Slots }) =>
  slots.default?.({ foo: props.foo }) ?? <div>default slot</div>

export default () => {
  const foo = ref('foo')
  const slots: Slots = {
    default: (scope) => <div>{scope.foo}</div>,
  }

  return (
    <>
      <input
        value={foo.value}
        onInput={(event) => (foo.value = event.currentTarget.value)}
      />
      <Comp foo={foo.value} v-slots={slots} />
      <Comp foo={foo.value}>{(scope) => <div>{scope.foo}</div>}</Comp>
    </>
  )
}
