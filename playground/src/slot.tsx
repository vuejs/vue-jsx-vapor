import { defineVaporComponent, ref } from 'vue'

const Comp = (props: { foo: string }, { slots }) => {
  return (
    <>
      {slots.default ? (
        <slots.default foo={props.foo} />
      ) : (
        <div>default slot</div>
      )}
    </>
  )
}

const CompSlotlet = (props: { foo: string }) => {
  return (
    <slot foo={props.foo}>
      <div>default slot</div>
    </slot>
  )
}

const CompSlotlet1 = defineVaporComponent((props: { foo: string }) => {
  return (
    <slot foo={props.foo}>
      <div>default slot</div>
    </slot>
  )
})

const slots = {
  default: (scope) => <div>{scope.foo}</div>,
}

// eslint-disable-next-line unused-imports/no-unused-vars
const slotName = ref('default')
export default () => {
  const foo = ref('foo')
  return (
    <>
      <input v-model={foo.value} />
      <div style="display: flex;">
        <fieldset>
          <legend>v-slots</legend>
          <Comp foo={foo.value} v-slots={slots} />

          <Comp
            foo={foo.value}
            v-slots={{ default: (scope) => <div>{scope.foo}</div> }}
          />

          <Comp foo={foo.value}>
            {{ default: (scope) => <div>{scope.foo}</div> }}
          </Comp>

          <Comp foo={foo.value}>{(scope) => <div>{scope.foo}</div>}</Comp>
        </fieldset>

        <fieldset>
          <legend>v-slot</legend>
          <Comp foo={foo.value} v-slot:$slotName_value$={scope}>
            <div>{scope.foo}</div>
          </Comp>
          <CompSlotlet foo={foo.value} v-slot={scope}>
            <div>{scope.foo}</div>
          </CompSlotlet>
          <CompSlotlet1 foo={foo.value} v-slot={scope}>
            <div>{scope.foo}</div>
          </CompSlotlet1>
        </fieldset>
      </div>
    </>
  )
}
