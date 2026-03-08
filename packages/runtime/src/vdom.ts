import {
  cloneVNode,
  Comment,
  createBlock,
  createElementBlock,
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
    if (!cacheMap.has(i)) cacheMap.set(i, [])
    const caches = cacheMap.get(i)
    return caches[index] || (caches[index] = [])
  } else {
    return []
  }
}

export function normalizeVNode(value: any = ' ', flag = 1): VNode {
  let create: any = createVNode
  const isFunction = typeof value === 'function'
  if (isFunction) {
    openBlock()
    create = createBlock
    value = value()
  }
  return isVNode(value)
    ? isFunction
      ? createBlock(cloneIfMounted(value))
      : cloneIfMounted(value)
    : Array.isArray(value)
      ? createElementBlock(
          Fragment,
          null,
          value.map((n) =>
            normalizeVNode(typeof n === 'function' ? n.toString() : () => n),
          ),
        )
      : value == null || typeof value === 'boolean'
        ? create(Comment)
        : create(Text, null, String(value), flag)
}

// optimized normalization for template-compiled render fns
function cloneIfMounted(child: VNode): VNode {
  return (child.el === null && child.patchFlag !== -1) ||
    // @ts-ignore
    child.memo
    ? child
    : cloneVNode(child)
}
