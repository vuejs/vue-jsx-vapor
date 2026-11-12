import { defineVaporComponent } from 'vue'

const key = 'b'

const Comp = defineVaporComponent(
  ({
    'foo-bar': bar = 1,
    ['baz']: baz,
    0: zero,
    other,
    qux: { 'a-b': ab = 2, [key]: dynamic },
  }: any) => {
    return <>{[bar, baz, zero, other, ab, dynamic]}</>
  },
)

const Comp1 = defineVaporComponent(({ 'max-size': maxSize }: any) => {
  return <>{maxSize}</>
})

const Comp2 = defineVaporComponent(({ ['size']: size = 1 }: any) => {
  return <>{size}</>
})

const Comp3 = defineVaporComponent(({ 0: zero }: any) => {
  return <>{zero}</>
})
