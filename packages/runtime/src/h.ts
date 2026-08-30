import * as Vue from 'vue'
import { createComponent, createProxyComponent, normalizeNode } from './vapor'
import type {
  EmitFnToProps,
  ExposedToProps,
  NodeChild,
  NodeRef,
  SlotsToProps,
} from './types'
import type {
  Block,
  Fragment,
  Suspense,
  SuspenseProps,
  TeleportProps,
  VaporComponentInstance,
  VaporTeleport,
} from 'vue'

type HTMLElementEventHandler = {
  [K in keyof HTMLElementEventMap as `on${Capitalize<K>}`]?: (
    ev: HTMLElementEventMap[K],
  ) => any
}

type ReservedProps = { key?: () => PropertyKey }
type RawProps = Record<string, any> & ReservedProps

type RawNodeChild = NodeChild<Block | (() => RawNodeChild)>
type RawSlot = (...args: any[]) => RawNodeChild
type RawChildren = RawNodeChild | RawSlot
type RawSlots = Record<string, RawSlot>

type VaporHType =
  | string
  | typeof Fragment
  | typeof Suspense
  | typeof VaporTeleport
  | ((...args: any[]) => any)
  | (new (...args: any[]) => VaporComponentInstance)

type ResolveProps<T> =
  T extends Record<string, any>
    ? {
        [K in keyof T]: T[K] | (() => T[K])
      }
    : T

type ResolveSlots<T> = T extends (...args: infer Args) => any
  ? (...args: Args) => RawNodeChild
  : T extends Record<string, any>
    ? {
        [K in keyof T]?: ResolveSlots<T[K]>
      }
    : RawNodeChild

type VaporHArgs<T extends VaporHType> = T extends string
  ? [
      props?:
        | (RawProps & {
            ref?: NodeRef<
              T extends keyof HTMLElementTagNameMap
                ? HTMLElementTagNameMap[T]
                : Element | VaporComponentInstance
            >
          } & (T extends keyof HTMLElementTagNameMap
              ? HTMLElementEventHandler
              : {}))
        | null,
      children?: T extends keyof HTMLElementTagNameMap
        ? RawChildren
        : RawChildren | RawSlots,
    ]
  : T extends typeof Fragment
    ? [props?: ReservedProps | null, children?: RawChildren]
    : T extends typeof Suspense
      ? [
          props?: (RawProps & SuspenseProps) | null,
          children?: RawChildren | RawSlots,
        ]
      : T extends typeof VaporTeleport
        ? [props: RawProps & TeleportProps, children: RawChildren | RawSlots]
        : T extends new (...args: any[]) => infer Instance
          ? Instance extends VaporComponentInstance
            ? [
                props?:
                  | (ResolveProps<Omit<Instance['props'], 'ref'>> &
                      ExposedToProps<
                        string extends keyof NonNullable<Instance['exposed']>
                          ? VaporComponentInstance
                          : NonNullable<Instance['exposed']>
                      >)
                  | null,
                children?: SlotsToProps<
                  Instance['slots']
                > extends infer SlotsProps
                  ? 'v-slots' extends keyof SlotsProps
                    ? ResolveSlots<SlotsProps['v-slots']>
                    : RawChildren | RawSlots
                  : RawChildren | RawSlots,
              ]
            : []
          : T extends (
                props: infer Props,
                ctx: {
                  slots: infer Slots extends Record<string, any>
                  expose: (
                    exposed: infer Exposed extends Record<string, any>,
                  ) => void
                  attrs: any
                  emit: infer Emit
                },
              ) => any
            ? [
                props?:
                  | (ResolveProps<Props> &
                      EmitFnToProps<Emit> &
                      ExposedToProps<Exposed>)
                  | null,
                children?: SlotsToProps<Slots> extends infer SlotsProps
                  ? 'v-slots' extends keyof SlotsProps
                    ? ResolveSlots<SlotsProps['v-slots']>
                    : RawChildren | RawSlots
                  : RawChildren | RawSlots,
              ]
            : never

/*@__NO_SIDE_EFFECTS__*/
export function vaporH<T extends VaporHType>(
  type: T,
  ...[props, children]: VaporHArgs<T>
): any {
  const { props: resolvedProps, key, ref } = resolveProps(props)
  const render = () => {
    const comp = createComponent(
      type as any,
      resolvedProps,
      (children
        ? typeof children === 'object' && !Array.isArray(children)
          ? new Proxy(children, {
              get: (target, key, receiver) =>
                createProxyComponent(
                  Reflect.get(target, key, receiver),
                  normalizeNode,
                ),
            })
          : {
              default:
                typeof children === 'function'
                  ? createProxyComponent(children as any, normalizeNode)
                  : () => normalizeNode(children as any),
            }
        : undefined) as any,
    )
    if (ref) {
      const setRef = Vue.createTemplateRefSetter()
      Vue.renderEffect(() => setRef(comp as any, ref as any))
    }
    return comp
  }
  return key ? Vue.createKeyedFragment(key, render) : render()
}

const EVENT_REGEX = /^on[A-Z]/
function resolveProps(props?: Record<string, any>) {
  const resolvedProps: {
    props: Record<string, any>
    ref?: NodeRef<VaporComponentInstance>
  } & ReservedProps = { props: {} }
  if (props) {
    // eslint-disable-next-line no-restricted-syntax
    for (const p in props) {
      const isFuncton = typeof props[p] === 'function'
      if (p === 'key') {
        resolvedProps.key = isFuncton ? props[p] : () => props[p]
      } else if (p === 'ref') {
        resolvedProps.ref = props[p]
      } else if (EVENT_REGEX.test(p)) {
        resolvedProps.props[p] = () => props[p]
      } else {
        resolvedProps.props[p] = props[p]
      }
    }
    return resolvedProps
  }
  return resolvedProps
}
