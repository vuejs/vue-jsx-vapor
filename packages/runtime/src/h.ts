/* eslint-disable prefer-rest-params */
import {
  createComponentWithFallback,
  isBlock,
  type NodeArrayChildren,
  type NodeChild,
} from './vapor'
import type {
  Block,
  Component,
  ComponentOptions,
  ConcreteComponent,
  DefineComponent,
  EmitsOptions,
  Fragment,
  FunctionalComponent,
  Suspense,
  SuspenseProps,
  Teleport,
  TeleportProps,
  VNodeRef,
} from 'vue'

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

type RawProps = Record<string, any>

type ResolveProps<T> = T extends null | undefined ? T : (() => T) | T

// The following is a series of overloads for providing props validation of
// manually written render functions.

// element
export function h<K extends keyof HTMLElementTagNameMap>(
  type: K,
  children?: NodeChild,
): Block
export function h<K extends keyof HTMLElementTagNameMap>(
  type: K,
  props?: ResolveProps<RawProps & HTMLElementEventHandler> | null,
  children?: NodeChild,
): Block

// custom element
export function h(type: string, children?: NodeChild): Block
export function h(
  type: string,
  props?: ResolveProps<RawProps> | null,
  children?: NodeChild,
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
  props?: ResolveProps<{ key?: PropertyKey; ref?: VNodeRef }> | null,
  children?: NodeArrayChildren,
): Block

// teleport (target prop is required)
export function h(
  type: typeof Teleport,
  props: RawProps & TeleportProps,
  children: NodeChild,
): Block

// suspense
export function h(type: typeof Suspense, children?: NodeChild): Block
export function h(
  type: typeof Suspense,
  props?: ResolveProps<RawProps & SuspenseProps> | null,
  children?: NodeChild,
): Block

// functional component
export function h(type: FunctionalComponent, children?: NodeChild): Block
export function h<
  P,
  E extends EmitsOptions = {},
  S extends Record<string, any> = any,
>(
  type: FunctionalComponent<P, E, S>,
  props?: ResolveProps<(RawProps & P) | ({} extends P ? null : never)>,
  children?: NodeChild,
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
  children?: NodeChild,
): Block
export function h<P>(
  type:
    | string
    | ConcreteComponent<P>
    | Component<P>
    | ComponentOptions<P>
    | Constructor<P>
    | DefineComponent<P>,
  props?: ResolveProps<(RawProps & P) | ({} extends P ? null : never)>,
  children?: NodeChild,
): Block

/*@__NO_SIDE_EFFECTS__*/
export function h(type: any, propsOrChildren?: any, children?: any) {
  const l = arguments.length
  if (l === 2) {
    if (
      (typeof propsOrChildren === 'object' &&
        !Array.isArray(propsOrChildren)) ||
      typeof propsOrChildren === 'function'
    ) {
      // single block without props
      if (isBlock(propsOrChildren)) {
        return createComponentWithFallback(type, null, {
          default: () => propsOrChildren,
        })
      }

      // props without children
      return createComponentWithFallback(type, resolveProps(propsOrChildren))
    } else {
      // omit props
      return createComponentWithFallback(type, null, {
        default: () => propsOrChildren,
      })
    }
  } else {
    if (l > 3) {
      children = Array.prototype.slice.call(arguments, 2)
    }
    return createComponentWithFallback(
      type,
      resolveProps(propsOrChildren),
      children
        ? typeof children === 'object' && !Array.isArray(children)
          ? children
          : {
              default: () => children,
            }
        : undefined,
    )
  }
}

function resolveProps(
  props?: Record<string, any> | (() => Record<string, any>),
) {
  if (props) {
    if (typeof props === 'function') {
      return { $: [props] }
    }
    const resolvedProps: Record<string, any> = {}
    // eslint-disable-next-line no-restricted-syntax
    for (const key in props) {
      if (typeof props[key] === 'function' || key === '$') {
        resolvedProps[key] = props[key]
      } else {
        resolvedProps[key] = () => props[key]
      }
    }
    return resolvedProps
  }
  return null
}
