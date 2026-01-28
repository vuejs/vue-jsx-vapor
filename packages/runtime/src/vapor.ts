import {
  createComponent as _createComponent,
  createComponentWithFallback as _createComponentWithFallback,
  createTextNode,
  // @ts-ignore
  currentInstance,
  EffectScope,
  Fragment,
  getCurrentInstance,
  insert,
  isFragment,
  isVaporComponent,
  remove,
  renderEffect,
  VaporFragment,
  type Block,
  type VaporComponent,
} from 'vue'

// component

/*@__NO_SIDE_EFFECTS__*/
export function defineVaporSSRComponent(
  comp: VaporComponent,
  extraOptions: VaporComponent,
): VaporComponent {
  if (typeof comp === 'function') {
    return Object.assign({ name: comp.name }, extraOptions, {
      setup(props: any, ctx: any) {
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

  const i = currentInstance || getCurrentInstance()
  // @ts-ignore #24
  if (!type.__proxyed) {
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
          if (i && i.appContext.vapor && p === '__vapor') {
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
    // @ts-ignore
    type.__proxyed = true
  }

  return createComponent(type as VaporComponent, props, ...args)
}

// block

type NodeChildAtom = Block | string | number | boolean | null | undefined | void

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
    return createTextNode(String(node))
  }
}

export function isBlock(val: NonNullable<unknown>): val is Block {
  return (
    val instanceof Node ||
    Array.isArray(val) ||
    isVaporComponent(val) ||
    isFragment(val)
  )
}

// node

function createFragment(
  nodes: Block,
  anchor: Node | undefined = document.createTextNode(''),
) {
  const frag = new VaporFragment(nodes)
  frag.anchor = anchor
  return frag
}

function normalizeBlock(node: any, anchor?: Node): Block {
  if (node instanceof Node || isFragment(node)) {
    anchor && (anchor.textContent = '')
    return node
  } else if (isVaporComponent(node)) {
    anchor && (anchor.textContent = '')
    return createFragment(node, anchor)
  } else if (Array.isArray(node)) {
    anchor && (anchor.textContent = '')
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

function resolveValue(current: Block, value: any, _anchor?: Node) {
  const node = normalizeBlock(value, _anchor)
  if (current) {
    if (isFragment(current)) {
      const { anchor } = current
      if (anchor && anchor.parentNode) {
        remove(current.nodes, anchor.parentNode)
        insert(node, anchor.parentNode, anchor)
        !_anchor && anchor.parentNode.removeChild(anchor)
      }
    } else if (current instanceof Node) {
      if (isFragment(node) && current.parentNode) {
        insert(node, current.parentNode, current)
        current.parentNode.removeChild(current)
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
  const frag = createFragment(nodes, _anchor)
  const scopes: EffectScope[] = []
  for (const [index, value] of values.entries()) {
    const anchor = index === values.length - 1 ? _anchor : undefined
    if (typeof value === 'function') {
      renderEffect(() => {
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
  return frag
}

export function setNodes(anchor: Node, ...values: any[]) {
  const resolvedValues = resolveValues(values, anchor)
  anchor.parentNode && insert(resolvedValues, anchor.parentNode, anchor)
}

export function createNodes(...values: any[]) {
  return resolveValues(values)
}
