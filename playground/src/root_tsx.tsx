/* eslint-disable unused-imports/no-unused-vars */
/* eslint-disable @typescript-eslint/no-unused-expressions */
/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { computed, defineComponent, defineVaporComponent } from 'vue'

const Comp = (
  props: { id: 1 },
  {
    slots,
  }: {
    attrs: any
    emit: any
    slots: { default: (props: { id: 1 }) => [] }
    expose: (exposed: { id: 1 }) => void
  },
) => {
  const A = (<a href=""></a>) as unknown as __InferJsxElement<'a'>
  A.href = '#foo'
  return A
}
const comp = (<Comp />) as unknown as __InferJsxElement<typeof Comp>
comp.slots
comp.exposed!.id
comp.block
comp === ({} as HTMLAnchorElement)

const VaporComp = (() => {
  const __setup = (
    __props: typeof __ctx.props & { id: number },
    __context?: typeof __ctx.context,
    __ctx = {} as Awaited<ReturnType<typeof __fn>>,
    __fn = (props: { id: number }) => {
      const __slots = defineSlots({
        default: (props: { id: 1 }) => [],
      })
      const __exposed = defineExpose({
        id: computed(() => 1),
      })
      const __render = (
        <div>{props.id}</div>
      ) as unknown as __InferJsxElement<'div'>
      return {} as {
        props: {}
        context: {} & {
          slots: Partial<typeof __slots>
          expose: (exposed: typeof __exposed) => void
          attrs: Record<string, any>
        }
        render: typeof __render
      }
    },
  ) => __ctx.render
  type __Setup = typeof __setup
  type __Props = Parameters<__Setup>[0]
  type __Slots = Parameters<__Setup>[1] extends
    | { slots?: infer Slots }
    | undefined
    ? Slots
    : {}
  type __Exposed = Parameters<__Setup>[1] extends
    | { expose?: (exposed: infer Exposed) => any }
    | undefined
    ? Exposed
    : {}
  const __component = // @ts-ignore
    (defineVaporComponent, await import('vue-jsx-vapor')).defineVaporComponent({
      ...({} as {
        setup: (props: __Props) => __Exposed
        render: () => ReturnType<__Setup>
        slots: __Slots
      }),
    })
  return {} as Omit<typeof __component, 'constructor'> & {
    new (props?: __Props): InstanceType<typeof __component> & {
      /** @deprecated This is only a type when used in Vapor Instances. */
      $props: __Props
    }
  }
})()
const vaporComp = (
  <VaporComp
    id={1}
    v-slots={{ default: ({ id }) => <></> } satisfies typeof __VLS_ctx_0.slots}
  />
) as unknown as __InferJsxElement<typeof VaporComp>
vaporComp.props.id
vaporComp.exposeProxy?.id === ({} as number)
vaporComp.block.style

const VDomComp = (() => {
  const __setup = (
    __props: typeof __ctx.props & { id: number },
    __context?: typeof __ctx.context,
    __ctx = {} as Awaited<ReturnType<typeof __fn>>,
    __fn = (props: { id: number }) => {
      const __slots = defineSlots({
        default: (props: { id: 1 }) => <div />,
      })
      const __render: () => JSX.Element = () =>
        (<div>{props.id}</div>) as unknown as __InferJsxElement<'div'>
      return {} as {
        props: {}
        context: {} & {
          slots: Partial<typeof __slots>
          expose: (exposed: {}) => void
          attrs: Record<string, any>
        }
        render: typeof __render
      }
    },
  ) => __ctx.render
  type __Setup = typeof __setup
  type __Props = Parameters<__Setup>[0]
  type __Slots = Parameters<__Setup>[1] extends
    | { slots?: infer Slots }
    | undefined
    ? Slots
    : {}
  type __Exposed = Parameters<__Setup>[1] extends
    | { expose?: (exposed: infer Exposed) => any }
    | undefined
    ? Exposed
    : {}
  const __component = defineComponent({
    ...({} as {
      setup: (props: __Props) => __Exposed
      render: () => ReturnType<__Setup>
      slots: import('vue').SlotsType<__Slots>
    }),
  })
  return {} as Omit<typeof __component, 'constructor'> & {
    new (props?: __Props): InstanceType<typeof __component> & {}
  }
})()
const vdompComp = (
  <VDomComp
    id={1}
    v-slots={{ default: ({}) => <></> } satisfies typeof __VLS_ctx_1.slots}
  />
) as unknown as __InferJsxElement<typeof VDomComp>
vdompComp.props

type __VLS_IsAny<T> = 0 extends 1 & T ? true : false
type __VLS_PickNotAny<A, B> = __VLS_IsAny<A> extends true ? B : A
declare function __VLS_asFunctionalComponent<
  T,
  K = T extends new (...args: any) => any ? InstanceType<T> : unknown,
>(
  t: T,
  instance?: K,
): T extends new (...args: any) => any
  ? (
      props: K extends { $props: infer Props }
        ? Props
        : K extends { props: infer Props }
          ? Props
          : any,
      ctx?: any,
    ) => JSX.Element & {
      __ctx: {
        attrs?: Record<string, any>
        props: K extends { $props: infer Props }
          ? Props
          : K extends { props: infer Props }
            ? Props
            : any
        slots?: K extends { $slots: infer Slots }
          ? Slots
          : K extends { slots: infer Slots }
            ? Slots
            : any
        emit?: K extends { $emit: infer Emit }
          ? Emit
          : K extends { emit: infer Emit }
            ? Emit
            : any
        expose?: (
          exposed: K extends { exposeProxy: infer Exposed }
            ? keyof Exposed extends never
              ? K
              : Exposed
            : K,
        ) => void
      }
    }
  : T extends () => any
    ? (props: {}, ctx?: any) => ReturnType<T>
    : T extends (...args: any) => any
      ? T
      : (
          _: {},
          ctx?: any,
        ) => {
          __ctx: {
            attrs?: any
            expose?: any
            slots?: any
            emit?: any
          }
        }
const __VLS_nativeElements = {
  ...({} as SVGElementTagNameMap),
  ...({} as HTMLElementTagNameMap),
}
declare function __VLS_getFunctionalComponentCtx<T, K, const S>(
  comp: T,
  compInstance: K,
  s: S,
): S extends keyof typeof __VLS_nativeElements
  ? { expose: (exposed: (typeof __VLS_nativeElements)[S]) => any }
  : '__ctx' extends keyof __VLS_PickNotAny<K, {}>
    ? K extends { __ctx?: infer Ctx }
      ? Ctx
      : never
    : T extends (props: infer P, ctx: infer Ctx) => any
      ? { props: P } & Ctx
      : {}

const __VLS_ctx_0 = __VLS_getFunctionalComponentCtx(
  VaporComp,
  __VLS_asFunctionalComponent(VaporComp)({ id: 1 }),
  'VaporComp',
)

const __VLS_ctx_1 = __VLS_getFunctionalComponentCtx(
  VDomComp,
  __VLS_asFunctionalComponent(VDomComp)({ id: 1 }),
  'VDomComp',
)

declare const { defineModel }: typeof import('vue')
declare function defineSlots<T extends Record<string, any>>(): Partial<T>
declare function defineSlots<T extends Record<string, any>>(slots: T): T
declare function defineExpose<
  Exposed extends Record<string, any> = Record<string, any>,
>(exposed?: Exposed): Exposed
declare const defineStyle: {
  <T>(...args: __StyleArgs): T
  scss: <T>(...args: __StyleArgs) => T
  sass: <T>(...args: __StyleArgs) => T
  stylus: <T>(...args: __StyleArgs) => T
  less: <T>(...args: __StyleArgs) => T
  postcss: <T>(...args: __StyleArgs) => T
}
type __StyleArgs = [style: string, options?: { scoped?: boolean }]

type __InferJsxElement<T> = T extends keyof HTMLElementTagNameMap
  ? HTMLElementTagNameMap[T]
  : T extends keyof SVGElementTagNameMap
    ? SVGElementTagNameMap[T]
    : T extends (
          props: infer Props,
          ctx: {
            slots: infer Slots extends Record<string, any>
            expose: (exposed: infer Exposed extends Record<string, any>) => void
            attrs: any
            emit: any
          },
        ) => infer TypeBlock
      ? import('vue-jsx-vapor').VaporComponentInstance<
          Props,
          {},
          Slots,
          Exposed,
          TypeBlock
        >
      : T extends { new (...args: any[]): infer Instance }
        ? Instance extends { $: any }
          ? import('vue').VNode
          : Instance
        : JSX.Element
