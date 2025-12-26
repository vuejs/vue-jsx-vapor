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

export function normalizeVNode(child = ' ', flag = 0): VNode {
  if (child == null || typeof child === 'boolean') {
    // empty placeholder
    return createVNode(Comment)
  } else if (Array.isArray(child)) {
    // fragment
    return (
      openBlock(),
      createBlock(createVNode(Fragment, null, child.slice()))
    )
  } else if (isVNode(child)) {
    return (openBlock(), createBlock(cloneIfMounted(child)))
  } else {
    // strings and numbers
    return createVNode(Text, null, String(child), flag)
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
