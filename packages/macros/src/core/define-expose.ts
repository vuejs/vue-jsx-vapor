import { importHelperFn } from './utils'
import type { CallExpression } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineExpose(
  node: CallExpression,
  s: MagicString,
): void {
  const argument = node.arguments[0]
  const typeParameters = node.typeParameters ?? node.typeArguments
  s.overwrite(node.callee.start!, typeParameters?.end ?? node.callee.end!, ';')
  s.appendRight(
    argument?.start ?? node.end! - 1,
    `${importHelperFn(
      s,
      'getCurrentInstance',
      undefined,
      '/vue-jsx-vapor/props',
    )}().exposed = ${argument ? '' : '{}'}`,
  )
}
