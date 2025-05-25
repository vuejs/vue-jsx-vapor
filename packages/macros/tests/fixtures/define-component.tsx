import { defineComponent, nextTick } from 'vue'

const Comp = defineComponent(
  ({ bar = 'bar'!, Comp, ...attrs }: { bar: 'bar'; baz: 'baz', Comp: any }) => {
    defineModel()
    const foo = $(
      defineModel('foo', {
        validator: (value) => {
          return value === 'foo'
        },
        type: String,
      })!,
    )
    return <div>
      {[foo, bar, attrs.baz]}
      <Comp />
    </div>
  },
  { name: 'Comp', props: { Comp: Object } },
)

const Comp1 = defineComponent((props: { bar: 'bar'; 'onUpdate:bar': any, comp: any }) => {
  const foo = defineModel('foo')
  return <div>
      {[foo.value, props['bar'], props['onUpdate:bar']]}
      <props.comp />
    </div>
})

const Comp2 = defineComponent(async () => {
  await nextTick()
  let foo = await new Promise((resolve) => {
    setTimeout(() => resolve('foo'), 1000)
  })
  return () => <div>{foo}</div>
})

const foo = () => {}
defineComponent(({
  a = 0,
  b = 'b',
  c = true,
  d = () => {},
  e = {},
  f = [],
  g = foo,
  h = null,
  i = undefined!,
}) => {
  return (
    <>
      {a}
      {b}
      {c}
      {d}
      {e}
      {f}
      {g}
      {h}
      {i}
    </>
  )
})