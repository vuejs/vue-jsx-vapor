import {
  defineComponent as __defineComponent,
  normalizeClass as _normalizeClass,
  cloneVNode,
  Comment,
  createBlock,
  createElementBlock,
  createElementVNode,
  createVNode,
  Fragment,
  getCurrentInstance,
  isVNode,
  openBlock,
  renderList,
  Text,
  withCtx,
  type Component,
  type ComponentInjectOptions,
  type ComponentObjectPropsOptions,
  type ComponentOptions,
  type ComponentOptionsBase,
  type ComponentOptionsMixin,
  type ComponentProvideOptions,
  type ComponentTypeEmits,
  type ComputedOptions,
  type CreateComponentPublicInstanceWithMixins,
  type DefineComponent,
  type Directive,
  type EmitFn,
  type EmitsOptions,
  type EmitsToProps,
  type ExtractDefaultPropTypes,
  type ExtractPropTypes,
  type MethodOptions,
  type PublicProps,
  type Slot,
  type SlotsType,
  type VNode,
  type VNodeChild,
} from 'vue'
import type { EmitFnToEmits, RenderResult, SetupContextToProps } from './types'

const cacheMap = new WeakMap()

export function createVNodeCache(key: string) {
  const i = getCurrentInstance()
  if (i) {
    if (!cacheMap.has(i)) cacheMap.set(i, {})
    const caches = cacheMap.get(i)
    return caches[key] || (caches[key] = [])
  } else {
    return []
  }
}

export function normalizeVNode(
  value: VNodeChild | (() => VNodeChild),
  flag = 1,
): VNode {
  let create: any = createVNode
  let isBlock = false
  if (typeof value === 'function') {
    isBlock = true
    openBlock()
    create = createBlock
    value = value()
  }
  return isVNode(value)
    ? isBlock
      ? createBlock(cloneIfMounted(value))
      : cloneIfMounted(value)
    : Array.isArray(value)
      ? isBlock
        ? createElementBlock(
            Fragment,
            null,
            value.map((n) => normalizeVNode(() => n)),
            -2,
          )
        : createElementVNode(Fragment, null, value.slice())
      : value == null || typeof value === 'boolean'
        ? create(Comment)
        : create(Text, null, String(value), flag)
}

// optimized normalization for template-compiled render fns
function cloneIfMounted(child: VNode): VNode {
  return (child.el === null && child.patchFlag !== -1) ||
    // @ts-ignore
    child.memo
    ? child
    : cloneVNode(child)
}

const normalizeSlotValue = (value: unknown): VNode[] =>
  Array.isArray(value)
    ? value.map((n) => normalizeVNode(n))
    : [normalizeVNode(value as VNodeChild)]

export const normalizeSlot = (rawSlot: Function): Slot => {
  if ((rawSlot as any)._n) {
    // already normalized
    return rawSlot as Slot
  }
  return withCtx((...args: any[]) => {
    return normalizeSlotValue(rawSlot(...args))
  }) as Slot
}

export const normalizeSlots = (slots: any): Record<string, any> | Function => {
  return typeof slots === 'function' ||
    (Object.prototype.toString.call(slots) === '[object Object]' &&
      !isVNode(slots))
    ? slots
    : {
        default: withCtx(() => [normalizeVNode(() => slots)]),
      }
}

export const normalizeClass = (value: unknown) => _normalizeClass(value) || null

// defineComponent

type RenderFunction = () => RenderResult

export type DefineSetupFnComponent<
  P extends Record<string, any>,
  E extends EmitsOptions = {},
  S extends SlotsType = SlotsType,
  Exposed extends Record<string, any> = {},
  Props = Readonly<P> & SetupContextToProps<E, S, Exposed>,
  PP = PublicProps,
> = new (
  props: Props & PP,
) => CreateComponentPublicInstanceWithMixins<
  Props,
  Exposed,
  {},
  {},
  {},
  ComponentOptionsMixin,
  ComponentOptionsMixin,
  E,
  PP,
  {},
  false,
  {},
  S,
  {},
  {},
  keyof Exposed & string
>

// defineComponent is a utility that is primarily used for type inference
// when declaring components. Type inference is provided in the component
// options (provided as the argument). The returned value has artificial types
// for TSX / manual render function / IDE support.

// overload 1: direct setup function
// (uses user defined props interface)
declare function _defineComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends SlotsType | Record<string, any> = {},
  Exposed extends Record<string, any> = {},
  Emit = EmitFn<Emits>,
>(
  setup: (
    this: void,
    props: Props,
    ctx: {
      emit: Emit
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed?: Exposed) => void
    },
  ) => RenderFunction | Promise<RenderFunction>,
  options?: Omit<ComponentOptions, 'props' | 'emits' | 'slots'> & {
    props?: (keyof NoInfer<Props>)[]
    emits?: Emits | RuntimeEmitsKeys[]
    slots?: Slots
  },
): DefineSetupFnComponent<
  Props,
  [keyof Emits] extends [never] ? EmitFnToEmits<Emit> : Emits,
  Slots extends SlotsType ? Slots : SlotsType<Slots>,
  Exposed
>
declare function _defineComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends SlotsType | Record<string, any> = {},
  Exposed extends Record<string, any> = {},
  Emit = EmitFn<Emits>,
>(
  setup: (
    this: void,
    props: Props,
    ctx: {
      emit: Emit
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed?: Exposed) => void
    },
  ) => RenderFunction | Promise<RenderFunction>,
  options?: Omit<ComponentOptions, 'props' | 'emits' | 'slots'> & {
    props?: ComponentObjectPropsOptions<Props>
    emits?: Emits | RuntimeEmitsKeys[]
    slots?: Slots
  },
): DefineSetupFnComponent<
  Props,
  [keyof Emits] extends [never] ? EmitFnToEmits<Emit> : Emits,
  Slots extends SlotsType ? Slots : SlotsType<Slots>,
  Exposed
>

// overload 2: defineComponent with options object, infer props from options
declare function _defineComponent<
  // props
  TypeProps,
  RuntimePropsOptions extends
    ComponentObjectPropsOptions = ComponentObjectPropsOptions,
  RuntimePropsKeys extends string = string,
  // emits
  TypeEmits extends ComponentTypeEmits = {},
  RuntimeEmitsOptions extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  // other options
  Data = {},
  SetupBindings = {},
  Computed extends ComputedOptions = {},
  Methods extends MethodOptions = {},
  Mixin extends ComponentOptionsMixin = ComponentOptionsMixin,
  Extends extends ComponentOptionsMixin = ComponentOptionsMixin,
  InjectOptions extends ComponentInjectOptions = {},
  InjectKeys extends string = string,
  Slots extends SlotsType = {},
  LocalComponents extends Record<string, Component> = {},
  Directives extends Record<string, Directive> = {},
  Exposed extends string = string,
  Provide extends ComponentProvideOptions = ComponentProvideOptions,
  // resolved types
  ResolvedEmits extends EmitsOptions = {} extends RuntimeEmitsOptions
    ? EmitFnToEmits<EmitFn<TypeEmits>>
    : RuntimeEmitsOptions,
  InferredProps = Readonly<
    unknown extends TypeProps
      ? string extends RuntimePropsKeys
        ? ComponentObjectPropsOptions extends RuntimePropsOptions
          ? {}
          : ExtractPropTypes<RuntimePropsOptions>
        : { [key in RuntimePropsKeys]?: any }
      : TypeProps
  > &
    EmitsToProps<ResolvedEmits>,
>(
  options: {
    props?: (RuntimePropsOptions & ThisType<void>) | RuntimePropsKeys[]
  } & ComponentOptionsBase<
    InferredProps,
    SetupBindings,
    Data,
    Computed,
    Methods,
    Mixin,
    Extends,
    RuntimeEmitsOptions,
    RuntimeEmitsKeys,
    {}, // Defaults
    InjectOptions,
    InjectKeys,
    Slots,
    LocalComponents,
    Directives,
    Exposed,
    Provide
  > &
    ThisType<
      CreateComponentPublicInstanceWithMixins<
        InferredProps,
        SetupBindings,
        Data,
        Computed,
        Methods,
        Mixin,
        Extends,
        ResolvedEmits,
        {},
        {},
        false,
        InjectOptions,
        Slots,
        LocalComponents,
        Directives,
        string
      >
    >,
): DefineComponent<
  InferredProps,
  SetupBindings,
  Data,
  Computed,
  Methods,
  Mixin,
  Extends,
  ResolvedEmits,
  RuntimeEmitsKeys,
  PublicProps,
  InferredProps & EmitsToProps<ResolvedEmits>,
  ExtractDefaultPropTypes<RuntimePropsOptions>,
  Slots,
  LocalComponents,
  Directives,
  Exposed,
  Provide,
  // MakeDefaultsOptional - if TypeProps is provided, set to false to use
  // user props types verbatim
  unknown extends TypeProps ? true : false
>

export const defineComponent = __defineComponent as typeof _defineComponent

// components

export const For = defineComponent(
  <
    T extends
      | any[]
      | Record<any, any>
      | number
      | string
      | Set<any>
      | Map<any, any>,
    Item = T extends number
      ? number
      : T extends string
        ? string
        : T extends any[]
          ? T[number]
          : T extends Iterable<infer T1>
            ? T1
            : Record<any, any>,
  >(
    props: {
      in: T
    },
    {
      slots,
    }: {
      slots: {
        default: (
          ...args: string extends keyof Item
            ? [item: T[keyof T], key: keyof T, index: number]
            : [item: Item, index: number]
        ) => any
      }
    },
  ) => {
    const defaultSlot = slots.default
    return () => (
      openBlock(true),
      createElementBlock(
        Fragment,
        null,
        renderList(props.in, (item, key, index) => {
          // @ts-ignore
          const result = defaultSlot(item, key, index)
          return Array.isArray(result)
            ? result.length === 1
              ? result[0]
              : normalizeVNode(result)
            : result
        }),
        128,
      )
    )
  },
  { props: ['in'] },
)
