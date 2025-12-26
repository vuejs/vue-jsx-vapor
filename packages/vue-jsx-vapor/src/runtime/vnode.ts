import {
  cloneVNode,
  Comment,
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

export function normalizeVNode(value: any = ' ', flag = 0): VNode {
  if (value == null || typeof value === 'boolean') {
    // empty placeholder
    return createVNode(Comment)
  } else if (typeof value !== 'function') {
    return createVNode(Text, null, String(value), flag)
  }
  openBlock()
  const child = value()
  if (value == null || typeof value === 'boolean') {
    // empty placeholder
    return createBlock(Comment)
  } else if (Array.isArray(child)) {
    // fragment
    return createBlock(Fragment, null, child.slice())
  } else if (isVNode(child)) {
    return createBlock(cloneIfMounted(child))
  } else {
    // strings and numbers
    return createBlock(Text, null, String(child), flag)
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
