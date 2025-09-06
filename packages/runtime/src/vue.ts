import type {
  AllowedComponentProps,
  Block,
  ComponentCustomProps,
  ComponentInternalOptions,
  ComponentObjectPropsOptions,
  ComponentPropsOptions,
  ComponentTypeEmits,
  EffectScope,
  EmitFn,
  EmitsOptions,
  EmitsToProps,
  ExtractDefaultPropTypes,
  ExtractPropTypes,
  GenericAppContext,
  GenericComponentInstance,
  LifecycleHook,
  NormalizedPropsOptions,
  ObjectEmitsOptions,
  RawProps,
  RawSlots,
  ReservedProps,
  ShallowRef,
  ShallowUnwrapRef,
  SharedInternalOptions,
  StaticSlots,
  SuspenseBoundary,
  VNode,
} from 'vue'

type Prettify<T> = {
  [K in keyof T]: T[K]
} & {}
type OverloadParameters<T extends (...args: any[]) => any> = Parameters<
  OverloadUnion<T>
>
type OverloadProps<TOverload> = Pick<TOverload, keyof TOverload>
type OverloadUnionRecursive<
  TOverload,
  TPartialOverload = unknown,
> = TOverload extends (...args: infer TArgs) => infer TReturn
  ? TPartialOverload extends TOverload
    ? never
    :
        | OverloadUnionRecursive<
            TPartialOverload & TOverload,
            TPartialOverload &
              ((...args: TArgs) => TReturn) &
              OverloadProps<TOverload>
          >
        | ((...args: TArgs) => TReturn)
  : never
type OverloadUnion<TOverload extends (...args: any[]) => any> = Exclude<
  OverloadUnionRecursive<(() => never) & TOverload>,
  TOverload extends () => never ? never : () => never
>
type IfAny<T, Y, N> = 0 extends 1 & T ? Y : N
type IsKeyValues<T, K = string> = IfAny<
  T,
  false,
  T extends object ? (keyof T extends K ? true : false) : false
>

export declare class VaporComponentInstance<
  Props extends Record<string, any> = {},
  Emits extends EmitsOptions = {},
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
  TypeRefs extends Record<string, any> = Record<string, any>,
> implements GenericComponentInstance
{
  vapor: true
  uid: number
  type: VaporComponent
  root: GenericComponentInstance | null
  parent: GenericComponentInstance | null
  appContext: GenericAppContext
  block: TypeBlock
  scope: EffectScope
  rawProps: RawProps
  rawSlots: RawSlots
  props: Readonly<Props>
  attrs: Record<string, any>
  propsDefaults: Record<string, any> | null
  slots: Slots
  rawPropsRef?: ShallowRef<any>
  rawSlotsRef?: ShallowRef<any>
  emit: EmitFn<Emits>
  emitted: Record<string, boolean> | null
  expose: (<T extends Record<string, any> = Exposed>(exposed: T) => void) &
    string[]
  exposed: Record<string, any> extends Exposed ? Exposed | null : Exposed
  exposeProxy: Record<string, any> extends Exposed
    ? Exposed | null
    : ShallowUnwrapRef<Exposed>
  refs: TypeRefs
  provides: Record<string, any>
  ids: [string, number, number]
  suspense: SuspenseBoundary | null
  hasFallthrough: boolean
  isMounted: boolean
  isUnmounted: boolean
  isDeactivated: boolean
  isUpdating: boolean
  bc?: LifecycleHook
  c?: LifecycleHook
  bm?: LifecycleHook
  m?: LifecycleHook
  bu?: LifecycleHook
  u?: LifecycleHook
  um?: LifecycleHook
  bum?: LifecycleHook
  da?: LifecycleHook
  a?: LifecycleHook
  rtg?: LifecycleHook
  rtc?: LifecycleHook
  ec?: LifecycleHook
  sp?: LifecycleHook<() => Promise<unknown>>
  setupState?: Exposed extends Block ? undefined : ShallowUnwrapRef<Exposed>
  devtoolsRawSetupState?: any
  hmrRerender?: () => void
  hmrReload?: (newComp: VaporComponent) => void
  propsOptions?: NormalizedPropsOptions
  emitsOptions?: ObjectEmitsOptions | null
  isSingleRoot?: boolean
  constructor(
    comp: VaporComponent,
    rawProps?: RawProps | null,
    rawSlots?: RawSlots | null,
    appContext?: GenericAppContext,
  )
  /**
   * Expose `getKeysFromRawProps` on the instance so it can be used in code
   * paths where it's needed, e.g. `useModel`
   */
  rawKeys(): string[]
}

export interface ObjectVaporComponent<
  Props = ComponentPropsOptions,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
  InferredProps = ComponentObjectPropsOptions extends Props
    ? {}
    : ExtractPropTypes<Props>,
> extends ComponentInternalOptions,
    SharedInternalOptions {
  inheritAttrs?: boolean
  props?: Props
  emits?: Emits | RuntimeEmitsKeys[]
  slots?: Slots
  setup?: VaporSetupFn<InferredProps, Emits, Slots, Exposed, TypeBlock>
  render?: (
    ctx: Exposed extends Block ? undefined : ShallowUnwrapRef<Exposed>,
    props: Readonly<InferredProps>,
    emit: EmitFn<Emits>,
    attrs: any,
    slots: Slots,
  ) => RenderReturn<TypeBlock>

  name?: string
  vapor?: boolean
}

export type VaporComponent =
  | FunctionalVaporComponent
  | ObjectVaporComponent
  | DefineVaporComponent

export type FunctionalVaporComponent = VaporSetupFn &
  Omit<ObjectVaporComponent, 'setup'> & {
    displayName?: string
  } & SharedInternalOptions

type VaporSetupFn<
  Props = {},
  Emits extends EmitsOptions = {},
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
> = (
  props: Readonly<Props>,
  ctx: {
    emit: EmitFn<Emits>
    slots: Slots
    attrs: Record<string, any>
    expose: <T extends Record<string, any> = Exposed>(exposed: T) => void
  },
) => TypeBlock | Exposed | Promise<Exposed> | void

export type TypeEmitsToOptions<T extends ComponentTypeEmits> = {
  [K in keyof T & string]: T[K] extends [...args: infer Args]
    ? (...args: Args) => any
    : () => any
} & (T extends (...args: any[]) => any
  ? ParametersToFns<OverloadParameters<T>>
  : {})

type ParametersToFns<T extends any[]> = {
  [K in T[0]]: IsStringLiteral<K> extends true
    ? (
        ...args: T extends [e: infer E, ...args: infer P]
          ? K extends E
            ? P
            : never
          : never
      ) => any
    : never
}

type IsStringLiteral<T> = T extends string
  ? string extends T
    ? false
    : true
  : false

export type VaporComponentInstanceConstructor<
  T extends VaporComponentInstance,
> = {
  __isFragment?: never
  __isTeleport?: never
  __isSuspense?: never
  new (props?: T['props']): T
}

export type VaporPublicProps = ReservedProps &
  AllowedComponentProps &
  ComponentCustomProps

export type RenderReturn<T extends Block = Block> =
  | VNode
  | T
  | RenderReturn<T>[]

export type DefineVaporComponent<
  RuntimePropsOptions = {},
  RuntimePropsKeys extends string = string,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
  TypeRefs extends Record<string, unknown> = {},
  MakeDefaultsOptional extends boolean = true,
  InferredProps = string extends RuntimePropsKeys
    ? ComponentObjectPropsOptions extends RuntimePropsOptions
      ? {}
      : ExtractPropTypes<RuntimePropsOptions>
    : { [key in RuntimePropsKeys]?: any },
  PublicProps = VaporPublicProps,
  ResolvedProps = InferredProps & EmitsToProps<Emits>,
  Defaults = ExtractDefaultPropTypes<RuntimePropsOptions>,
> = VaporComponentInstanceConstructor<
  VaporComponentInstance<
    MakeDefaultsOptional extends true
      ? keyof Defaults extends never
        ? Prettify<ResolvedProps> & PublicProps
        : Partial<Defaults> &
            Omit<Prettify<ResolvedProps> & PublicProps, keyof Defaults>
      : Prettify<ResolvedProps> & PublicProps,
    Emits,
    Slots,
    Exposed,
    TypeBlock,
    TypeRefs
  >
> &
  ObjectVaporComponent<
    RuntimePropsOptions | RuntimePropsKeys[],
    Emits,
    RuntimeEmitsKeys,
    Slots,
    Exposed
  >

export type DefineVaporSetupFnComponent<
  Props extends Record<string, any> = {},
  Emits extends EmitsOptions = {},
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
  ResolvedProps extends Record<string, any> = Props &
    EmitsToProps<Emits> &
    VaporPublicProps,
> = new (
  props?: ResolvedProps,
) => VaporComponentInstance<ResolvedProps, Emits, Slots, Exposed, TypeBlock>

// overload 1: direct setup function
// (uses user defined props interface)
export function defineVaporComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
>(
  setup: (
    props: Props,
    ctx: {
      emit: EmitFn<Emits>
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed: Exposed) => void
    },
  ) => RenderReturn<TypeBlock>,
  extraOptions?: ObjectVaporComponent<
    (keyof Props)[],
    Emits,
    RuntimeEmitsKeys,
    Slots,
    Exposed
  > &
    ThisType<void>,
): DefineVaporSetupFnComponent<Props, Emits, Slots, Exposed, TypeBlock>
export function defineVaporComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
>(
  setup: (
    props: Props,
    ctx: {
      emit: EmitFn<Emits>
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed: Exposed) => void
    },
  ) => RenderReturn<TypeBlock>,
  extraOptions?: ObjectVaporComponent<
    ComponentObjectPropsOptions<Props>,
    Emits,
    RuntimeEmitsKeys,
    Slots,
    Exposed
  > &
    ThisType<void>,
): DefineVaporSetupFnComponent<Props, Emits, Slots, Exposed, TypeBlock>

// overload 2: defineVaporComponent with options object, infer props from options
export function defineVaporComponent<
  // props
  TypeProps,
  RuntimePropsOptions extends
    ComponentObjectPropsOptions = ComponentObjectPropsOptions,
  RuntimePropsKeys extends string = string,
  // emits
  TypeEmits extends ComponentTypeEmits = {},
  RuntimeEmitsOptions extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
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
  TypeBlock extends Block = Block,
>(
  options: ObjectVaporComponent<
    RuntimePropsOptions | RuntimePropsKeys[],
    ResolvedEmits,
    RuntimeEmitsKeys,
    Slots,
    Exposed,
    TypeBlock,
    InferredProps
  > & {
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
    __typeEl?: TypeBlock
  } & ThisType<void>,
): DefineVaporComponent<
  RuntimePropsOptions,
  RuntimePropsKeys,
  ResolvedEmits,
  RuntimeEmitsKeys,
  Slots,
  Exposed extends Block ? Record<string, any> : Exposed,
  TypeBlock,
  TypeRefs,
  // MakeDefaultsOptional - if TypeProps is provided, set to false to use
  // user props types verbatim
  unknown extends TypeProps ? true : false,
  InferredProps
>

/*! #__NO_SIDE_EFFECTS__ */
export function defineVaporComponent(comp: VaporComponent, extraOptions?: any) {
  if (typeof comp === 'function') {
    // #8236: extend call and options.name access are considered side-effects
    // by Rollup, so we have to wrap it in a pure-annotated IIFE.
    return /*@__PURE__*/ (() =>
      Object.assign({ name: comp.name }, extraOptions, {
        setup: comp,
        __vapor: true,
      }))()
  }
  comp.__vapor = true
  return comp
}
