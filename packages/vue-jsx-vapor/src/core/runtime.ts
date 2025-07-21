import {
  createComponent as _createComponent,
  createComponentWithFallback as _createComponentWithFallback,
  EffectScope,
  Fragment,
  insert,
  isFragment,
  isVaporComponent,
  proxyRefs,
  remove,
  renderEffect,
  toRefs,
  useAttrs,
  VaporFragment,
  type Block,
  type GenericComponentInstance,
  type VaporComponent,
} from 'vue'

import * as Vue from 'vue'

export function isBlock(val: NonNullable<unknown>): val is Block {
  return (
    val instanceof Node ||
    Array.isArray(val) ||
    isVaporComponent(val) ||
    isFragment(val)
  )
}

export function getCurrentInstance(): GenericComponentInstance | null {
  // @ts-ignore
  return Vue.currentInstance || Vue.getCurrentInstance()
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
    type = (_, { slots }) => slots.default()
    props = null
  }
  // @ts-ignore
  if (Vue.currentInstance && Vue.currentInstance.appContext.vapor) {
    typeof type === 'function' && ((type as VaporComponent).__vapor = true)
  }
  return createComponent(type as VaporComponent, props, ...args)
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
) => createProxyComponent(_createComponentWithFallback, type, ...args)

/**
 * Returns the props of the current component instance.
 *
 * @example
 * ```tsx
 * import { useProps } from 'vue-jsx-vapor'
 *
 * defineComponent(({ foo = '' })=>{
 *   const props = useProps() // { foo: '' }
 * })
 * ```
 */
export function useProps() {
  const i = getCurrentInstance()
  return i!.props
}

/**
 * Returns the merged props and attrs of the current component.\
 * Equivalent to `useProps()` + `useAttrs()`.
 *
 * @example
 * ```tsx
 * import { useFullProps } from 'vue-jsx-vapor'
 *
 * defineComponent((props) => {
 *   const fullProps = useFullProps() // = useAttrs() + useProps()
 * })
 */
export function useFullProps() {
  return proxyRefs({
    ...toRefs(useProps()),
    ...toRefs(useAttrs()),
  })
}

function createFragment(
  nodes: Block,
  anchor: Node | undefined = document.createTextNode(''),
) {
  const frag = new VaporFragment(nodes)
  frag.anchor = anchor
  return frag
}

function normalizeNode(node: any, anchor?: Node): Block {
  if (node instanceof Node || isFragment(node)) {
    anchor && (anchor.textContent = '')
    return node
  } else if (isVaporComponent(node)) {
    anchor && (anchor.textContent = '')
    return createFragment(node, anchor)
  } else if (Array.isArray(node)) {
    anchor && (anchor.textContent = '')
    return createFragment(
      node.map((i) => normalizeNode(i)),
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
  const node = normalizeNode(value, _anchor)
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
