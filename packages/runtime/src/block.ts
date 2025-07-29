import { createTextNode, isFragment, isVaporComponent, type Block } from 'vue'

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
