/* eslint-disable prefer-rest-params */
import * as Vue from 'vue'
import {
  createComponent,
  createNodes,
  createProxyComponent,
  type NodeChild,
} from './vapor'
import type {
  Block,
  Component,
  EmitsOptions,
  Fragment,
  FunctionalVaporComponent,
  NodeRef,
  Suspense,
  SuspenseProps,
  TeleportProps,
  VaporComponent,
  VaporTeleport,
} from 'vue'

type HTMLElementEventHandler = {
  [K in keyof HTMLElementEventMap as `on${Capitalize<K>}`]?: (
    ev: HTMLElementEventMap[K],
  ) => any
}

type ReservedProps = { key?: () => PropertyKey; ref?: NodeRef }
type RawProps = Record<string, any> & ReservedProps

type RawSlot = () => NodeChild
type RawChildren = NodeChild | RawSlot
type RawSlots = Record<string, RawSlot>

// The following is a series of overloads for providing props validation of
// manually written render functions.

// element / custom element / resolve component
export function h<K extends string>(
  type: K,
  props?:
    | (RawProps &
        (K extends keyof HTMLElementTagNameMap ? HTMLElementEventHandler : {}))
    | null,
  children?: K extends keyof HTMLElementTagNameMap ? RawChildren : RawSlots,
): Block

// fragment
export function h(
  type: typeof Fragment,
  props?: ReservedProps | null,
  children?: RawChildren,
): Block

// teleport (target prop is required)
export function h(
  type: typeof VaporTeleport,
  props: RawProps & TeleportProps,
  children: RawChildren | RawSlots,
): Block

// suspense
export function h(
  type: typeof Suspense,
  props?: (RawProps & SuspenseProps) | null,
  children?: RawChildren | RawSlots,
): Block

// functional component
export function h<
  P,
  E extends EmitsOptions = {},
  S extends Record<string, any> = RawSlots,
>(
  type: FunctionalVaporComponent<P, E, S>,
  props?: (RawProps & P) | ({} extends P ? null : never),
  children?: RawChildren | S,
): Block

// catch all types
export function h(
  type: Component | VaporComponent,
  props?: RawProps,
  children?: NodeChild | RawSlots,
): Block

/*@__NO_SIDE_EFFECTS__*/
export function h(type: any, props?: any, children?: any): any {
  const l = arguments.length
  if (l > 3) {
    children = Array.prototype.slice.call(arguments, 2)
  }
  const childrenIsArray = Array.isArray(children)
  const { props: resolvedProps, key, ref } = resolveProps(props)
  const render = () => {
    const comp = createComponent(
      type,
      resolvedProps,
      children
        ? typeof children === 'object' && !childrenIsArray
          ? new Proxy(children, {
              get: (target, key, receiver) =>
                createProxyComponent(Reflect.get(target, key, receiver)),
            })
          : {
              default: () =>
                createNodes(...(childrenIsArray ? children : [children])),
            }
        : undefined,
    )
    if (ref) {
      const setRef = Vue.createTemplateRefSetter()
      Vue.renderEffect(() => setRef(comp, ref!))
    }
    return comp
  }
  return key ? Vue.createKeyedFragment(key, render) : render()
}

type ResolvedProps = {
  props: Record<string, any>
} & ReservedProps
const EVENT_REGEX = /^on[A-Z]/
function resolveProps(props?: Record<string, any>): ResolvedProps {
  const resolvedProps: ResolvedProps = { props: {} }
  if (props) {
    // eslint-disable-next-line no-restricted-syntax
    for (const p in props) {
      const isFuncton = typeof props[p] === 'function'
      if (p === 'key') {
        resolvedProps.key = isFuncton ? props[p] : () => props[p]
      } else if (p === 'ref') {
        resolvedProps.ref = props[p]
      } else if (EVENT_REGEX.test(p[2])) {
        resolvedProps.props[p] = () => props[p]
      } else {
        resolvedProps.props[p] = props[p]
      }
    }
    return resolvedProps
  }
  return resolvedProps
}
