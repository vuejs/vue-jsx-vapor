import {
  defineComponent,
  defineVaporComponent,
  ref,
  shallowRef,
  type Block,
  type EmitFn,
  type EmitsOptions,
  type FunctionalComponent,
  type FunctionalVaporComponent,
  type Ref,
  type SetupContext,
  type SlotsType,
  type VaporComponentInstance,
} from 'vue'
import {
  defineComponent as _defineComponent,
  defineVaporComponent as _defineVaporComponent,
  type ExtractExposed,
  type SetupContextToProps,
} from 'vue-jsx'

declare function expectType<T>(value: T): void

;<div slot="name" />

const Comp = defineComponent({
  setup: (
    props: { id: 1 },
    {
      slots,
    }: SetupContext<
      EmitsOptions,
      SlotsType<{ default: (props: { id: 1 }) => any }>
    >,
  ) => {
    slots.default({ id: props.id })
    return { foo: props.id }
  },
})
;<Comp
  ref={(e) => {
    expectType<1>(e!.$props.id)
  }}
  id={1}
>
  {(props) => <>{props.id}</>}
</Comp>

const Comp1 = defineComponent(
  <T,>(
    props: { foo: T } & SetupContextToProps<
      { foo: [T] },
      { default: (props: { foo: T }) => any },
      { foo: T }
    >,
    {
      emit,
      slots,
      expose,
    }: {
      emit: EmitFn<{ foo: [T] }>
      slots: { default?: (props: { foo: T }) => any }
      expose: (exposed: { foo: T }) => void
    },
  ) => {
    emit('foo', props.foo)
    expose({ foo: props.foo })
    return () => slots.default?.({ foo: props.foo }) || []
  },
)
const comp1Ref =
  shallowRef<ExtractExposed<InstanceType<typeof Comp1>['$props']>>()
expectType<unknown>(comp1Ref.value?.foo)
;<Comp1
  ref={(exposed) => {
    expectType<1>(exposed!.foo)
  }}
  foo={1 as const}
  onFoo={(e) => expectType<1>(e)}
>
  {(props) => {
    expectType<1>(props.foo)
    return []
  }}
</Comp1>

const _Comp1 = _defineComponent<{ foo: 1 }, { foo: [1] }>((props, { emit }) => {
  emit('foo', props.foo)
  return () => []
})
;<_Comp1
  foo={1}
  onFoo={(e) => {
    expectType<1>(e)
  }}
></_Comp1>

const _Comp2 = _defineComponent(
  <T,>(
    props: { foo: T },
    {
      emit,
      slots,
      expose,
    }: {
      emit: EmitFn<{ foo: [T] }>
      slots: { default?: (props: { foo: T }) => JSX.Element }
      expose: (exposed: { foo: T }) => void
    },
  ) => {
    emit('foo', props.foo)
    expose({ foo: props.foo })
    return () => slots.default?.({ foo: props.foo })
  },
)
const _comp2Ref =
  shallowRef<ExtractExposed<InstanceType<typeof _Comp2>['$props']>>()
expectType<unknown>(_comp2Ref.value?.foo)
;<_Comp2
  ref={(exposed) => {
    expectType<1>(exposed!.foo)
  }}
  foo={1 as const}
  onFoo={(e) => expectType<1>(e)}
>
  {(props) => {
    expectType<1>(props.foo)
    return []
  }}
</_Comp2>

const Comp2: FunctionalComponent<
  { id: 1 },
  {},
  { default: (props: { id: 1 }) => [] }
> = () => <div></div>
;<Comp2 id={1}>{(props) => props.id}</Comp2>

const Comp3: FunctionalVaporComponent<
  { id: 1 },
  { change: (props: { id: 1 }) => void },
  { default: (foo: { id?: 1 }) => any },
  { foo: Ref<string> }
> = (props, { emit, slots, expose }) => {
  emit('change', { id: 1 })
  expose({ foo: ref('1') })
  return [
    <div
      ref={(element: HTMLDivElement | null) => {
        expectType<HTMLDivElement | null>(element)
      }}
    >
      <slots.default />
    </div>,
  ]
}
;<Comp3
  ref={(e) => {
    expectType<{ foo: string } | null>(e)
  }}
  id={1}
  onChange={(props) => props.id}
>
  {{ default: (foo) => <div>{foo.id}</div> }}
</Comp3>

const Comp4 = <
  T,
  Slots extends Record<string, any> = {
    default: (props: { id: T }) => JSX.Element
  },
>(
  props: { id: T } & SetupContextToProps<{ change: [T] }, Slots, { foo: T }>,
  {
    emit,
    expose,
  }: {
    slots: Slots
    emit: EmitFn<{ change: [T] }>
    expose: (Exposed: { foo: T }) => void
  },
) => {
  emit('change', props.id)
  expose({ foo: props.id })
  return <div></div>
}
;<Comp4
  ref={(e) => {
    expectType<{ foo: 1 }>(e!)
  }}
  id={1 as const}
  onChange={(e) => {
    expectType<1>(e)
  }}
>
  {(props) => <>{props.id}</>}
</Comp4>

const Comp5 = defineVaporComponent(
  <T extends number>(
    props: { foo: T },
    {
      slots,
    }: {
      expose: (exposed: { foo: Ref<1> }) => void
      slots: { default: (props: { id: T }) => any }
    },
  ) => <slots.default id={props.foo}></slots.default>,
)
;<Comp5
  ref={(e) => {
    expectType<1>(e!.foo)
  }}
  foo={1}
>
  {(props) => <>{expectType<number>(props.id)}</>}
</Comp5>

const Comp6 = defineVaporComponent((props: { foo: 1 }) => (
  <div>{props.foo}</div>
))
;<Comp6
  ref={(e) => {
    expectType<VaporComponentInstance>(e!)
  }}
  foo={1}
>
  <div></div>
</Comp6>

const _Comp6 = _defineVaporComponent(
  (props: { foo: 1 }, _: { emit: EmitFn<{ foo: [1] }> }) => (
    <div>{props.foo}</div>
  ),
)
;<_Comp6 foo={1} onFoo={(e) => expectType<1>(e)}>
  <div></div>
</_Comp6>

const Comp7 = defineVaporComponent({
  setup: (
    props: { foo: 1 },
    {
      slots,
    }: {
      slots: { default: (scope: { foo: 1 }) => [] }
    },
  ) => {
    return (<slots.default foo={props.foo}>{props.foo}</slots.default>) as Block
  },
})
;<Comp7
  ref={(e) => {
    expectType<VaporComponentInstance>(e!)
  }}
  foo={1}
>
  {(scope) => <div>{scope.foo}</div>}
</Comp7>

const _Comp7 = _defineVaporComponent({
  setup: (
    props: { foo: 1 },
    {
      slots,
    }: {
      slots: { default: (scope: { foo: 1 }) => any }
    },
  ) => {
    return (<slots.default foo={props.foo}>{props.foo}</slots.default>) as Block
  },
})
;<_Comp7
  ref={(e) => {
    expectType<VaporComponentInstance>(e!)
  }}
  foo={1}
>
  {(scope) => <div>{scope.foo}</div>}
</_Comp7>

// Adt.tsx
export interface A {
  type: 'a'
  left: string
}

export interface B {
  type: 'b'
  right: string
}

const Adt = (props: A | B) => {
  if (props.type === 'a') {
    return <p>{props.left}</p>
  }

  return <p>{props.right}</p>
}

export default () => {
  return <Adt left="valueLeft" type="a" />
}
