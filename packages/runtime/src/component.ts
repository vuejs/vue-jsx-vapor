import {
  createComponent as _createComponent,
  createComponentWithFallback as _createComponentWithFallback,
  Fragment,
  type VaporComponent,
} from 'vue'
import { normalizeNode } from './block'
import { getCurrentInstance } from './helpers'

type Tail<T extends any[]> = T extends [any, ...infer R] ? R : never

export const createComponent = (
  type: VaporComponent | typeof Fragment,
  ...args: Tail<Parameters<typeof _createComponent>>
) => {
  return createProxyComponent(_createComponent, type, ...args)
}

export const createComponentWithFallback = (
  type: VaporComponent | typeof Fragment,
  ...args: Tail<Parameters<typeof _createComponentWithFallback>>
) => {
  const slots = args[1]
  if (
    typeof type === 'string' &&
    slots &&
    slots.default &&
    typeof slots.default === 'function'
  ) {
    const defaultSlot = slots.default
    slots.default = () => {
      return createProxyComponent(
        _createComponentWithFallback,
        defaultSlot,
        null,
        null,
      )
    }
  }
  return createProxyComponent(_createComponentWithFallback, type, ...args)
}

const createProxyComponent = (
  createComponent:
    | typeof _createComponent
    | typeof _createComponentWithFallback,
  type: VaporComponent | typeof Fragment,
  props: any,
  ...args: any[]
) => {
  if (type === Fragment) {
    type = (_, { slots }) => (slots.default ? slots.default() : [])
    props = null
  }

  const i = getCurrentInstance()
  if (typeof type === 'function') {
    type = new Proxy(type, {
      apply(target, ctx, args) {
        // @ts-ignore
        if (typeof target.__setup === 'function') {
          // @ts-ignore
          target.__setup.apply(ctx, args)
        }
        return normalizeNode(Reflect.apply(target, ctx, args))
      },
      get(target, p, receiver) {
        if ((i && (i.appContext as any)).vapor && p === '__vapor') {
          return true
        }
        return Reflect.get(target, p, receiver)
      },
    })
  } else if (type.__vapor && type.setup) {
    type.setup = new Proxy(type.setup, {
      apply(target, ctx, args) {
        return normalizeNode(Reflect.apply(target, ctx, args))
      },
    })
  }

  return createComponent(type as VaporComponent, props, ...args)
}
