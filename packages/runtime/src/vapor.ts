import {
  EffectScope,
  Fragment,
  getCurrentInstance,
  type Block,
  type VaporComponent,
  type VNode,
} from 'vue'
import * as Vue from 'vue'

// component

/*@__NO_SIDE_EFFECTS__*/
export function defineVaporSSRComponent(
  comp: VaporComponent,
  extraOptions: VaporComponent,
): VaporComponent {
  if (typeof comp === 'function') {
    return Object.assign({ name: comp.name }, extraOptions, {
      setup(props: any, ctx: any) {
        // @ts-ignore
        const result = comp(props, ctx)
        return () => result
      },
      __vapor: true,
    })
  }
  const setup = comp.setup
  if (setup) {
    comp.setup = (props, ctx) => {
      const result = setup(props, ctx)
      return () => result
    }
  }
  comp.__vapor = true
  return comp
}

type Tail<T extends any[]> = T extends [any, ...infer R] ? R : never

export const createComponent = (
  type: VaporComponent | typeof Fragment | string,
  ...args: Tail<Parameters<typeof Vue.createComponent>>
) => {
  if (type === Fragment) {
    const slots = args[1]
    type = (slots && (slots.default as any)) || (() => [])
  }
  return Vue.createComponentWithFallback(
    createProxyComponent(Vue.resolveDynamicComponent(type) as VaporComponent),
    ...args,
  )
}

export function createProxyComponent(type: VaporComponent) {
  if (typeof type === 'function') {
    // @ts-ignore
    const i = Vue.currentInstance || getCurrentInstance()
    return new Proxy(type, {
      apply(target, ctx, args) {
        // @ts-ignore
        if (typeof target.__setup === 'function') {
          // @ts-ignore
          target.__setup.apply(ctx, args)
        }
        return normalizeNode(Reflect.apply(target, ctx, args))
      },
      get(target, p, receiver) {
        if (i && i.appContext.vapor && p === '__vapor') {
          return true
        }
        return Reflect.get(target, p, receiver)
      },
    })
  } else if (
    type &&
    type.setup &&
    type.__vapor &&
    // @ts-ignore #24
    !type.__proxyed
  ) {
    type.setup = new Proxy(type.setup, {
      apply(target, ctx, args) {
        return normalizeNode(Reflect.apply(target, ctx, args))
      },
    })
    // @ts-ignore
    type.__proxyed = true
  }
  return type
}

// block

type NodeChildAtom =
  | VNode
  | Block
  | string
  | number
  | boolean
  | null
  | undefined
  | void

export type NodeArrayChildren = Array<NodeArrayChildren | NodeChildAtom>

export type NodeChild = NodeChildAtom | NodeArrayChildren

export function normalizeNode(node: NodeChild): Block {
  if (node == null || typeof node === 'boolean') {
    return document.createComment('')
  } else if (Array.isArray(node) && node.length) {
    return node.map(normalizeNode)
  } else if (isBlock(node)) {
    return node
  } else {
    // strings and numbers
    return document.createTextNode(String(node))
  }
}

export function isBlock(val: NonNullable<unknown>): val is Block {
  return (
    val instanceof Node ||
    Array.isArray(val) ||
    Vue.isVaporComponent(val) ||
    Vue.isFragment(val)
  )
}

// node

function createFragment(
  nodes: Block,
  anchor: Node | undefined = document.createTextNode(''),
) {
  const frag = new Vue.VaporFragment(nodes)
  frag.anchor = anchor
  return frag
}

function normalizeBlock(node: any, anchor?: Node): Block {
  if (node instanceof Node || Vue.isFragment(node)) {
    return node
  } else if (Vue.isVaporComponent(node)) {
    return createFragment(node, anchor)
  } else if (Array.isArray(node)) {
    return createFragment(
      node.map((i) => normalizeBlock(i)),
      anchor,
    )
  } else {
    const result = node == null || typeof node === 'boolean' ? '' : String(node)
    if (anchor) {
      anchor.textContent = result
      return anchor
    } else {
      return document.createTextNode(result)
    }
  }
}

function resolveValue(current: Block | undefined, value: any, anchor?: Node) {
  anchor =
    anchor ||
    (current instanceof Node && current.nodeType === 3 ? current : undefined)
  const node = normalizeBlock(value, anchor)
  if (current) {
    if (Vue.isFragment(current)) {
      if (current.anchor && current.anchor.parentNode) {
        Vue.remove(current.nodes, current.anchor.parentNode)
        Vue.insert(node, current.anchor.parentNode, current.anchor)
        !anchor && current.anchor.parentNode.removeChild(current.anchor)
        // @ts-ignore
        if (current.scope) current.scope.stop()
      }
    } else if (current instanceof Node) {
      if (
        current.nodeType === 3 &&
        (!(node instanceof Node) || node.nodeType !== 3)
      ) {
        current.textContent = ''
      }
      if (Vue.isFragment(node) && current.parentNode) {
        Vue.insert(node, current.parentNode, current)
        if (!anchor || current.nodeType !== 3) {
          current.parentNode.removeChild(current)
        }
      } else if (node instanceof Node) {
        if (current.nodeType === 3 && node.nodeType === 3) {
          current.textContent = node.textContent
          return current
        } else if (current.parentNode) {
          current.parentNode.replaceChild(node, current)
        }
      }
    }
  }
  return node
}

function resolveValues(values: any[] = [], _anchor?: Node) {
  const nodes: Block[] = []
  const scopes: EffectScope[] = []
  for (const [index, value] of values.entries()) {
    const anchor = index === values.length - 1 ? _anchor : undefined
    if (typeof value === 'function') {
      Vue.renderEffect(() => {
        if (scopes[index]) scopes[index].stop()
        scopes[index] = new EffectScope()
        nodes[index] = scopes[index].run(() =>
          resolveValue(nodes[index], value(), anchor),
        )!
      })
    } else {
      nodes[index] = resolveValue(nodes[index], value, anchor)
    }
  }
  return nodes
}

export function setNodes(anchor: Node, ...values: any[]) {
  const resolvedValues = resolveValues(values, anchor)
  anchor.parentNode && Vue.insert(resolvedValues, anchor.parentNode, anchor)
}

export function createNodes(...values: any[]) {
  return resolveValues(values)
}
