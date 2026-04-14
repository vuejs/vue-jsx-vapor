import type {
  EmitsOptions,
  EmitsToProps,
  SetupContext,
  SlotsType,
  VNodeChild,
} from 'vue'

export type Prettify<T> = { [K in keyof T]: T[K] } & {}

export type IfAny<T, Y, N> = 0 extends 1 & T ? Y : N
export type IsKeyValues<T, K = string> = IfAny<
  T,
  false,
  T extends object ? (keyof T extends K ? true : false) : false
>

export type ToResolvedProps<
  Props,
  Emits extends EmitsOptions,
> = Readonly<Props> & Readonly<EmitsToProps<Emits>>

export type SlotsToProps<
  RawSlots extends SlotsType | Record<string, any> = Record<string, any>,
  Element = VNodeChild,
  Slots = RawSlots extends SlotsType
    ? SetupContext<EmitsOptions, RawSlots>['slots']
    : RawSlots,
> = string extends keyof Slots
  ? {}
  : {
      'v-slots'?:
        | ('default' extends keyof Slots ? Slots['default'] | Slots : Slots)
        | NoInfer<Element>
    }
