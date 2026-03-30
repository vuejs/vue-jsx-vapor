import {
  cloneVNode,
  Comment,
  createBlock,
  createElementBlock,
  createElementVNode,
  createVNode,
  Fragment,
  getCurrentInstance,
  isVNode,
  openBlock,
  Text,
  type Slot,
  type VNode,
  type VNodeChild,
} from 'vue'

const cacheMap = new WeakMap()

export function createVNodeCache(key: string) {
  const i = getCurrentInstance()
  if (i) {
    if (!cacheMap.has(i)) cacheMap.set(i, {})
    const caches = cacheMap.get(i)
    return caches[key] || (caches[key] = [])
  } else {
    return []
  }
}

export function normalizeVNode(
  value: VNodeChild | (() => VNodeChild),
  flag = 1,
): VNode {
  let create: any = createVNode
  let isBlock = false
  if (typeof value === 'function') {
    isBlock = true
    openBlock()
    create = createBlock
    value = value()
  }
  return isVNode(value)
    ? isBlock
      ? createBlock(cloneIfMounted(value))
      : cloneIfMounted(value)
    : Array.isArray(value)
      ? isBlock
        ? createElementBlock(
            Fragment,
            null,
            value.map((n) => normalizeVNode(() => n)),
            -2,
          )
        : createElementVNode(Fragment, null, value.slice())
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

const normalizeSlotValue = (value: unknown): VNode[] =>
  Array.isArray(value)
    ? value.map((n) => normalizeVNode(n))
    : [normalizeVNode(value as VNodeChild)]

export const normalizeSlot = (rawSlot: Function): Slot => {
  if ((rawSlot as any)._n) {
    // already normalized
    return rawSlot as Slot
  }
  const normalized = (...args: any[]) => {
    return normalizeSlotValue(rawSlot(...args))
  }
  // NOT a compiled slot
  ;(normalized as any)._c = false
  return normalized
}
