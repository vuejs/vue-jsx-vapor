import { importHelperFn } from './utils'
import type { CallExpression } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineSlots(
  node: CallExpression,
  s: MagicString,
): void {
  s.overwrite(
    node.start!,
    (node.arguments[0]?.start && node.arguments[0].start - 1) ||
      node.typeArguments?.end ||
      node.callee.end!,
    `Object.assign`,
  )
  const slots = `${importHelperFn(s, 'useSlots')}()`
  s.appendLeft(node.end! - 1, `${node.arguments[0] ? ',' : '{}, '}${slots}`)
}
