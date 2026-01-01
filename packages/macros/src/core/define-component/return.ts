import { isFunctionalNode, type FunctionalNode } from '../utils'
import type MagicString from 'magic-string'

export function transformReturn(root: FunctionalNode, s: MagicString): void {
  const node =
    root.body.type === 'BlockStatement'
      ? root.body.body.find((node) => node.type === 'ReturnStatement')?.argument
      : root.body
  if (!node || isFunctionalNode(node)) return

  s.appendRight(
    node.extra?.parenthesized ? (node.extra.parenStart as number) : node.start!,
    '() => ',
  )
}
