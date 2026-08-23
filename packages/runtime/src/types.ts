import type {
  Directive,
  EmitsOptions,
  EmitsToProps,
  Ref,
  SetupContext,
  SlotsType,
  VaporComponentInstance,
  VNode,
} from 'vue'

declare module 'vue' {
  interface VaporComponentInstance {
    // @ts-expect-error Compatible with vue3.5
    block: never
  }
  interface RenderResultExtensions {
    render: RenderResult
  }
}

export type RenderResult<T = VaporComponentInstance['block']> =
  | T
  | VNode
  | RenderResult[]

export type Prettify<T> = { [K in keyof T]: T[K] } & {}

export type IfAny<T, Y, N> = 0 extends 1 & T ? Y : N
export type IsKeyValues<T, K = string> = IfAny<
  T,
  false,
  T extends object ? (keyof T extends K ? true : false) : false
>

export type DirectiveArgs<T extends Directive> =
  T extends Directive<any, infer Value, infer Modifiers, infer Argument>
    ?
        | Value
        | [Value]
        | [Value, Argument]
        | [Value, Array<Modifiers>]
        | [Value, Argument, Array<Modifiers>]
    : unknown

type NodeChildAtom<T> =
  | T
  | VNode
  | string
  | number
  | boolean
  | null
  | undefined
  | void

export type NodeArrayChildren<T> = Array<
  NodeArrayChildren<T> | NodeChildAtom<T>
>
export type NodeChild<T = VaporComponentInstance['block']> =
  | NodeChildAtom<T>
  | NodeArrayChildren<T>

export type NodeRef<T> =
  | ((ref: T | null, refs: Record<string, any>) => void)
  | Ref
  | string

type ResolveSlots<Slots> = {
  readonly [Key in keyof Slots]?: Slots[Key] extends (
    ...args: infer Args
  ) => VNode | VNode[]
    ? (...args: Args) => NodeChild
    : Slots[Key]
}
export type SlotsToProps<
  RawSlots extends SlotsType | Record<string, any> = Record<string, any>,
  Slots = ResolveSlots<
    RawSlots extends SlotsType
      ? SetupContext<EmitsOptions, RawSlots>['slots']
      : RawSlots
  >,
> = string extends keyof Slots
  ? {}
  : [keyof Slots] extends [never]
    ? {}
    : {
        readonly 'v-slots'?:
          | ('default' extends keyof Slots ? Slots['default'] | Slots : Slots)
          | NoInfer<NodeChild>
      }

declare const exposedType: unique symbol
export type ExtractExposed<
  Props,
  Default = never,
> = typeof exposedType extends keyof Props
  ? Exclude<Props[typeof exposedType], undefined>
  : Default
export type ExposedToProps<T extends Record<string, any>> =
  string extends keyof T
    ? {}
    : [keyof T] extends [never]
      ? {}
      : {
          readonly [exposedType]?: T
          readonly ref?: NodeRef<T>
        }

export type EmitFnToProps<T, ExcludeKeys extends PropertyKey = ''> = T extends (
  event: infer Event extends string,
  ...args: infer Args
) => any
  ? string extends Event
    ? {}
    : {
        readonly [K in Event as `on${Capitalize<K>}` extends ExcludeKeys
          ? never
          : `on${Capitalize<K>}`]?: (...args: Args) => any
      }
  : {}

export type SetupContextToProps<
  Emits extends EmitsOptions = {},
  Slots extends SlotsType | Record<string, any> = {},
  Exposed extends Record<string, any> = {},
> = EmitsToProps<Emits> & SlotsToProps<Slots> & ExposedToProps<Exposed>
