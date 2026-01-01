import { importHelperFn } from './utils'
import type { CallExpression } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineExpose(
  node: CallExpression,
  s: MagicString,
): void {
  s.overwrite(node.callee.start!, node.callee.end!, ';')
  s.appendRight(
    node.arguments[0]?.start || node.end! - 1,
    `${importHelperFn(s, 'getCurrentInstance', undefined, '/vue-jsx-vapor/props')}().exposed = `,
  )
}
