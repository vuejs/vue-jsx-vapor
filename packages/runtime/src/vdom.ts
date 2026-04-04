import {
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
  Text,
  withCtx,
  type Component,
  type ComponentInjectOptions,
  type ComponentObjectPropsOptions,
  type ComponentOptions,
  type ComponentOptionsBase,
  type ComponentOptionsMixin,
  type ComponentPropsOptions,
  type ComponentProvideOptions,
  type ComponentPublicInstance,
  type ComponentTypeEmits,
  type ComputedOptions,
  type CreateComponentPublicInstanceWithMixins,
  type Directive,
  type EmitFn,
  type EmitsOptions,
  type EmitsToProps,
  type ExtractDefaultPropTypes,
  type ExtractPropTypes,
  type GlobalComponents,
  type GlobalDirectives,
  type MethodOptions,
  type PublicProps,
  type SetupContext,
  type Slot,
  type SlotsType,
  type TypeEmitsToOptions,
  type VNode,
  type VNodeChild,
} from 'vue'
import type {
  IsKeyValues,
  ResolvePropsWithSlots,
  ToResolvedProps,
} from './types'

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

// defineComponent

type RenderFunction = () => VNodeChild | {}

type ComponentPublicInstanceConstructor<
  T extends ComponentPublicInstance<
    Props,
    RawBindings,
    D,
    C,
    M
  > = ComponentPublicInstance<any>,
  Props = any,
  RawBindings = any,
  D = any,
  C extends ComputedOptions = ComputedOptions,
  M extends MethodOptions = MethodOptions,
> = {
  __isFragment?: never
  __isTeleport?: never
  __isSuspense?: never
  new (...args: any[]): T
}

type ResolveProps<PropsOrPropOptions, E extends EmitsOptions> = Readonly<
  PropsOrPropOptions extends ComponentPropsOptions
    ? ExtractPropTypes<PropsOrPropOptions>
    : PropsOrPropOptions
> &
  ({} extends E ? {} : EmitsToProps<E>)

export type DefineComponent<
  PropsOrPropOptions = {},
  RawBindings = {},
  D = {},
  C extends ComputedOptions = ComputedOptions,
  M extends MethodOptions = MethodOptions,
  Mixin extends ComponentOptionsMixin = ComponentOptionsMixin,
  Extends extends ComponentOptionsMixin = ComponentOptionsMixin,
  E extends EmitsOptions = {},
  EE extends string = string,
  PP = PublicProps,
  Props = ResolveProps<PropsOrPropOptions, E>,
  Defaults = ExtractDefaultPropTypes<PropsOrPropOptions>,
  S extends SlotsType = {},
  LC extends Record<string, Component> = {},
  Directives extends Record<string, Directive> = {},
  Exposed extends string = string,
  Provide extends ComponentProvideOptions = ComponentProvideOptions,
  MakeDefaultsOptional extends boolean = true,
  TypeRefs extends Record<string, unknown> = {},
  TypeEl extends Element = any,
> = ComponentPublicInstanceConstructor<
  CreateComponentPublicInstanceWithMixins<
    ResolvePropsWithSlots<Props, SetupContext<EmitsOptions, S>['slots']>,
    RawBindings,
    D,
    C,
    M,
    Mixin,
    Extends,
    E,
    PP,
    Defaults,
    MakeDefaultsOptional,
    {},
    S,
    LC & GlobalComponents,
    Directives & GlobalDirectives,
    Exposed,
    TypeRefs,
    TypeEl
  >
> &
  ComponentOptionsBase<
    Props,
    RawBindings,
    D,
    C,
    M,
    Mixin,
    Extends,
    E,
    EE,
    Defaults,
    {},
    string,
    S,
    LC & GlobalComponents,
    Directives & GlobalDirectives,
    Exposed,
    Provide
  > &
  PP

export type DefineSetupFnComponent<
  P extends Record<string, any>,
  E extends EmitsOptions = {},
  S extends SlotsType = SlotsType,
  Props = P & EmitsToProps<E>,
  PP = PublicProps,
> = new (
  props: Props & PP,
) => CreateComponentPublicInstanceWithMixins<
  ResolvePropsWithSlots<Props, SetupContext<EmitsOptions, S>['slots']>,
  {},
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
  S
>

// defineComponent is a utility that is primarily used for type inference
// when declaring components. Type inference is provided in the component
// options (provided as the argument). The returned value has artificial types
// for TSX / manual render function / IDE support.

// overload 1: direct setup function
// (uses user defined props interface)
export function defineComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends Record<string, any> = {},
  Exposed extends Record<string, any> = Record<string, any>,
>(
  setup: (
    this: void,
    props: Props,
    ctx: {
      emit: EmitFn<Emits>
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed: Exposed) => void
    },
  ) => RenderFunction | Promise<RenderFunction>,
  options?: Omit<ComponentOptions, 'props' | 'emits' | 'slots'> & {
    props?: (keyof NoInfer<Props>)[]
    emits?: Emits | RuntimeEmitsKeys[]
    slots?: Slots
  },
): DefineSetupFnComponent<Props, Emits, SlotsType<Slots>>
export function defineComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends Record<string, any> = {},
  Exposed extends Record<string, any> = Record<string, any>,
>(
  setup: (
    this: void,
    props: Props,
    ctx: {
      emit: EmitFn<Emits>
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed: Exposed) => void
    },
  ) => RenderFunction | Promise<RenderFunction>,
  options?: Omit<ComponentOptions, 'props' | 'emits' | 'slots'> & {
    props?: ComponentObjectPropsOptions<Props>
    emits?: Emits | RuntimeEmitsKeys[]
    slots?: Slots
  },
): DefineSetupFnComponent<Props, Emits, SlotsType<Slots>>

// overload 2: defineComponent with options object, infer props from options
export function defineComponent<
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
    ? TypeEmitsToOptions<TypeEmits>
    : RuntimeEmitsOptions,
  InferredProps = IsKeyValues<TypeProps> extends true
    ? TypeProps
    : string extends RuntimePropsKeys
      ? ComponentObjectPropsOptions extends RuntimePropsOptions
        ? {}
        : ExtractPropTypes<RuntimePropsOptions>
      : { [key in RuntimePropsKeys]?: any },
  TypeRefs extends Record<string, unknown> = {},
  TypeEl extends Element = any,
>(
  options: {
    props?: (RuntimePropsOptions & ThisType<void>) | RuntimePropsKeys[]
    /**
     * @private
     */
    __typeProps?: TypeProps
    /**
     * @private
     */
    __typeEmits?: TypeEmits
    /**
     * @private
     */
    __typeRefs?: TypeRefs
    /**
     * @private
     */
    __typeEl?: TypeEl
  } & ComponentOptionsBase<
    ToResolvedProps<InferredProps, ResolvedEmits>,
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
        ToResolvedProps<InferredProps, ResolvedEmits>,
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
  ToResolvedProps<InferredProps, ResolvedEmits>,
  ExtractDefaultPropTypes<RuntimePropsOptions>,
  Slots,
  LocalComponents,
  Directives,
  Exposed,
  Provide,
  // MakeDefaultsOptional - if TypeProps is provided, set to false to use
  // user props types verbatim
  unknown extends TypeProps ? true : false,
  TypeRefs,
  TypeEl
>

// implementation, close to no-op
/*@__NO_SIDE_EFFECTS__*/
export function defineComponent(
  options: unknown,
  extraOptions?: ComponentOptions,
) {
  return typeof options === 'function'
    ? Object.assign({ name: options.name }, extraOptions, { setup: options })
    : options
}
