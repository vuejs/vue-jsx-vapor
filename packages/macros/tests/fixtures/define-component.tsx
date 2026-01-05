import {
  defineComponent,
  defineVaporComponent,
  nextTick,
  unref,
} from 'vue'

const $ = unref

const Comp = defineVaporComponent(
  ({
    bar = 'bar'! as string,
    Comp,
    ...attrs
  }: {
    bar: string
    baz: 'baz'
    Comp: any
  }) => {
    defineModel()
    const foo = $(
      defineModel('foo', {
        validator: (value) => {
          return value === 'foo'
        },
        type: String,
      })!,
    )
    return (
      <div>
        {[foo, bar, attrs.baz]}
        <Comp />
      </div>
    )
  },
  { name: 'Comp', props: { Comp: Object } },
)

const Comp1 = defineVaporComponent(
  (props: { bar: 'bar'; 'onUpdate:bar': any; comp: any }) => {
    const foo = defineModel('foo')
    return (
      <div>
        {[foo.value, props['bar'], props['onUpdate:bar']]}
        <props.comp />
      </div>
    )
  },
)

const Comp2 = defineComponent(async () => {
  await nextTick()
  let foo = await new Promise((resolve) => {
    setTimeout(() => resolve('foo'), 1000)
  })
  return () => <div>{foo}</div>
})

const foo = () => {}
defineVaporComponent(
  ({
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
  },
)

// #21
const Comp3 = defineComponent(<T,>() => {
  return () => <div>123</div>
})
const Comp4 = defineVaporComponent(<T,>() => {
  return <div>123</div>
})
