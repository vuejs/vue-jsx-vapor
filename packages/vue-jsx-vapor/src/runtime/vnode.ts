import {
  cloneVNode,
  createBlock,
  createVNode,
  Fragment,
  getCurrentInstance,
  isVNode,
  openBlock,
  Text,
  type VNode,
} from 'vue'

const cacheMap = new WeakMap()

export function createVNodeCache(index: number) {
  const i = getCurrentInstance()
  if (i) {
    !cacheMap.has(i) && cacheMap.set(i, [])
    const caches = cacheMap.get(i)
    return caches[index] || (caches[index] = [])
  } else {
    return []
  }
}

export function normalizeVNode(value: any = ' ', flag = 1): VNode {
  if (isVNode(value)) {
    return cloneIfMounted(value)
  } else if (Array.isArray(value)) {
    // fragment
    return createVNode(
      Fragment,
      null,
      value.map((n) => normalizeVNode(n)),
    )
  } else if (typeof value !== 'function') {
    return createVNode(
      Text,
      null,
      value == null || typeof value === 'boolean' ? '' : String(value),
      flag,
    )
  }
  openBlock()
  const node = value()
  if (isVNode(node)) {
    return createBlock(cloneIfMounted(node))
  } else if (Array.isArray(node)) {
    // fragment
    return createBlock(
      Fragment,
      null,
      node.map((n) => normalizeVNode(n)),
    )
  } else {
    return createBlock(
      Text,
      null,
      node == null || typeof node === 'boolean' ? '' : String(node),
      flag,
    )
  }
}

// optimized normalization for template-compiled render fns
function cloneIfMounted(child: VNode): VNode {
  return (child.el === null && child.patchFlag !== -1) ||
    // @ts-ignore
    child.memo
    ? child
    : cloneVNode(child)
}
