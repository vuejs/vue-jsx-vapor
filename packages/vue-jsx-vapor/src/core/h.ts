/* eslint-disable prefer-rest-params */
import {
  createTextNode,
  type Block,
  type Component,
  type ComponentOptions,
  type ConcreteComponent,
  type DefineComponent,
  type EmitsOptions,
  type Fragment,
  type FunctionalComponent,
  type RawSlots,
  type Suspense,
  type SuspenseProps,
  type Teleport,
  type TeleportProps,
  type VNodeRef,
} from 'vue'
import { createComponentWithFallback, isBlock } from './runtime'

type NodeChildAtom = Block | string | number | boolean | null | undefined | void

type NodeArrayChildren = Array<NodeArrayChildren | NodeChildAtom>

type RawChildren = NodeChildAtom | NodeArrayChildren

function normalizeNode(node: RawChildren): Block {
  if (node == null || typeof node === 'boolean') {
    return []
  } else if (Array.isArray(node) && node.length) {
    return node.map(normalizeNode)
  } else if (isBlock(node)) {
    return node
  } else {
    // strings and numbers
    return createTextNode(String(node))
  }
}

// fake constructor type returned from `defineComponent`
interface Constructor<P = any> {
  __isFragment?: never
  __isTeleport?: never
  __isSuspense?: never
  new (...args: any[]): { $props: P }
}

type HTMLElementEventHandler = {
  [K in keyof HTMLElementEventMap as `on${Capitalize<K>}`]?: (
    ev: HTMLElementEventMap[K],
  ) => any
}

type IfAny<T, Y, N> = 0 extends 1 & T ? Y : N

type RawProps = Record<string, any>

// The following is a series of overloads for providing props validation of
// manually written render functions.

// element
export function h<K extends keyof HTMLElementTagNameMap>(
  type: K,
  children?: RawChildren,
): Block
export function h<K extends keyof HTMLElementTagNameMap>(
  type: K,
  props?: (RawProps & HTMLElementEventHandler) | null,
  children?: RawChildren | RawSlots,
): Block

// custom element
export function h(type: string, children?: RawChildren): Block
export function h(
  type: string,
  props?: RawProps | null,
  children?: RawChildren | RawSlots,
): Block

// text/comment
export function h(
  type: typeof Text | typeof Comment,
  children?: string | number | boolean,
): Block
export function h(
  type: typeof Text | typeof Comment,
  props?: null,
  children?: string | number | boolean,
): Block

// fragment
export function h(type: typeof Fragment, children?: NodeArrayChildren): Block
export function h(
  type: typeof Fragment,
  props?: { key?: PropertyKey; ref?: VNodeRef } | null,
  children?: NodeArrayChildren,
): Block

// teleport (target prop is required)
export function h(
  type: typeof Teleport,
  props: RawProps & TeleportProps,
  children: RawChildren | RawSlots,
): Block

// suspense
export function h(type: typeof Suspense, children?: RawChildren): Block
export function h(
  type: typeof Suspense,
  props?: (RawProps & SuspenseProps) | null,
  children?: RawChildren | RawSlots,
): Block

// functional component
export function h(type: FunctionalComponent, children?: RawChildren): Block
export function h<
  P,
  E extends EmitsOptions = {},
  S extends Record<string, any> = any,
>(
  type: FunctionalComponent<P, E, S>,
  props?: (RawProps & P) | ({} extends P ? null : never),
  children?: RawChildren | IfAny<S, RawSlots, S>,
): Block

// catch all types
export function h(
  type:
    | string
    | ConcreteComponent
    | Component
    | ComponentOptions
    | Constructor
    | DefineComponent,
  children?: RawChildren,
): Block
export function h<P>(
  type:
    | string
    | ConcreteComponent<P>
    | Component<P>
    | ComponentOptions<P>
    | Constructor<P>
    | DefineComponent<P>,
  props?: (RawProps & P) | ({} extends P ? null : never),
  children?: RawChildren | RawSlots,
): Block

export function h(type: any, propsOrChildren?: any, children?: any) {
  const l = arguments.length
  if (l === 2) {
    if (
      typeof propsOrChildren === 'object' &&
      !Array.isArray(propsOrChildren)
    ) {
      // single block without props
      if (isBlock(propsOrChildren)) {
        return createComponentWithFallback(type, null, {
          default: () => normalizeNode(propsOrChildren),
        })
      }

      // props without children
      return createComponentWithFallback(
        type,
        propsOrChildren ? { $: [() => propsOrChildren] } : null,
      )
    } else {
      // omit props
      return createComponentWithFallback(type, null, {
        default: () => normalizeNode(propsOrChildren),
      })
    }
  } else {
    if (l > 3) {
      children = Array.prototype.slice.call(arguments, 2)
    }
    return createComponentWithFallback(
      type,
      propsOrChildren ? { $: [() => propsOrChildren] } : null,
      children
        ? typeof children === 'object' && !Array.isArray(children)
          ? children
          : {
              default: () => normalizeNode(children),
            }
        : undefined,
    )
  }
}
